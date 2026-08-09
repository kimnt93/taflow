//! Incremental Triple Exponential Average Rate of Change (TRIX).

use multiversion::multiversion;

use crate::error::TaResult;

use super::{invalid_period, ExponentialMovingAverage, StreamingIndicator};

/// Steady-state kernel for the triple EMA cascade plus the ROC step.
///
/// Extracted from [`TripleExponentialRateOfChange::extend_slice_into`] so it
/// can carry `#[multiversion]`; without runtime dispatch a portable build
/// lowers each `mul_add` to a libm `fma()` call. `mul_add` is explicitly fused
/// either way, so the dispatched variants are bit-identical.
#[allow(unexpected_cfgs)]
#[multiversion(targets("x86_64+avx2+fma", "x86_64+avx", "x86_64+sse4.2"))]
fn trix_steady_loop(
    inputs: &[f64],
    k: [f64; 3],
    state: &mut [f64; 3],
    output: &mut Vec<f64>,
) -> Option<f64> {
    let [k1, k2, k3] = k;
    let [mut e1, mut e2, mut e3] = *state;
    let mut last = None;
    for &input in inputs {
        e1 = k1.mul_add(input - e1, e1);
        e2 = k2.mul_add(e1 - e2, e2);
        let previous = e3;
        e3 = k3.mul_add(e2 - e3, e3);
        let value = if previous != 0.0 {
            (e3 - previous) / previous * 100.0
        } else {
            0.0
        };
        output.push(value);
        last = Some(value);
    }
    *state = [e1, e2, e3];
    last
}

/// Persistent TRIX with a triple TA-Lib-seeded EMA cascade and O(1) updates.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `TripleExponentialRateOfChange`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct TripleExponentialRateOfChange {
    ema1: ExponentialMovingAverage,
    ema2: ExponentialMovingAverage,
    ema3: ExponentialMovingAverage,
    previous_ema3: Option<f64>,
    value: Option<f64>,
}

impl TripleExponentialRateOfChange {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(period: usize) -> TaResult<Self> {
        if period < 2 {
            return Err(invalid_period("timeperiod", period, 2));
        }
        Ok(Self {
            ema1: ExponentialMovingAverage::new(period)?,
            ema2: ExponentialMovingAverage::new(period)?,
            ema3: ExponentialMovingAverage::new(period)?,
            previous_ema3: None,
            value: None,
        })
    }
}

impl StreamingIndicator for TripleExponentialRateOfChange {
    type Output = f64;

    /// Bulk kernel: advances the triple EMA cascade and the ROC step in one
    /// loop with the scalar states held in locals. Bit-identical to per-bar
    /// [`Self::append`] in outputs and post-run streaming state.
    fn extend_slice_into(&mut self, inputs: &[f64], output: &mut Vec<f64>) {
        output.reserve(inputs.len());
        let mut index = 0;
        // Warm-up prologue: per-bar appends until EMA3 has produced a value
        // and it has been latched as the previous ROC reference.
        while index < inputs.len() && self.previous_ema3.is_none() {
            output.push(self.append(inputs[index]).unwrap_or(f64::NAN));
            index += 1;
        }
        if index == inputs.len() {
            return;
        }

        let k = [
            self.ema1.smoothing(),
            self.ema2.smoothing(),
            self.ema3.smoothing(),
        ];
        let mut state = [
            self.ema1.current().expect("warm EMA1"),
            self.ema2.current().expect("warm EMA2"),
            self.ema3.current().expect("warm EMA3"),
        ];
        let last = trix_steady_loop(&inputs[index..], k, &mut state, output).or(self.value);

        let appended = inputs.len() - index;
        self.ema1.store_bulk_state(state[0], appended);
        self.ema2.store_bulk_state(state[1], appended);
        self.ema3.store_bulk_state(state[2], appended);
        self.previous_ema3 = Some(state[2]);
        self.value = last;
    }

    fn append(&mut self, input: f64) -> Option<f64> {
        let e1 = self.ema1.append(input)?;
        let e2 = self.ema2.append(e1)?;
        let e3 = self.ema3.append(e2)?;
        let previous = self.previous_ema3.replace(e3)?;
        self.value = Some(if previous != 0.0 {
            (e3 - previous) / previous * 100.0
        } else {
            0.0
        });
        self.value
    }

    fn value(&self) -> Option<f64> {
        self.value
    }

    fn reset(&mut self) {
        self.ema1.reset();
        self.ema2.reset();
        self.ema3.reset();
        self.previous_ema3 = None;
        self.value = None;
    }
}
