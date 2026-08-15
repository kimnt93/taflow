//! Incremental Exponential Moving Average (EMA).

use multiversion::multiversion;

use crate::error::TaResult;

use crate::stream::{invalid_period, SimpleMovingAverage, StreamingIndicator};

/// Steady-state EMA recurrence used by
/// [`ExponentialMovingAverage::extend_slice_into`].
///
/// A free function so it can carry `#[multiversion]`: a portable build without
/// runtime dispatch lowers `mul_add` to a libm `fma()` call. `mul_add` is an
/// explicitly fused operation in both cases, so the dispatched FMA variant
/// returns bit-identical values.
#[allow(unexpected_cfgs)]
#[multiversion(targets("x86_64+avx2+fma", "x86_64+avx", "x86_64+sse4.2"))]
fn ema_steady_loop(inputs: &[f64], k: f64, seed: f64, outputs: &mut Vec<f64>) -> f64 {
    let mut previous = seed;
    for &input in inputs {
        previous = k.mul_add(input - previous, previous);
        outputs.push(previous);
    }
    previous
}

/// Computes an aligned Exponential Moving Average vector using the stream
/// Compute the exponential moving average result for the supplied aligned series.
///
/// # Parameters
///
/// * `input` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
/// Stateful EMA with the same SMA seed as TA-Lib's batch EMA.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `ExponentialMovingAverage`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct ExponentialMovingAverage {
    seed: SimpleMovingAverage,
    k: f64,
    period: usize,
    samples: usize,
    value: Option<f64>,
}

impl ExponentialMovingAverage {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(period: usize) -> TaResult<Self> {
        if period == 0 {
            return Err(invalid_period("timeperiod", period, 1));
        }
        Ok(Self {
            seed: SimpleMovingAverage::new(period)?,
            k: 2.0 / (period as f64 + 1.0),
            period,
            samples: 0,
            value: None,
        })
    }

    /// The smoothing constant `k = 2 / (period + 1)` used by the recurrence.
    #[inline]
    pub(crate) fn smoothing(&self) -> f64 {
        self.k
    }

    /// The current EMA value, if the state is warm.
    #[inline]
    pub(crate) fn current(&self) -> Option<f64> {
        self.value
    }

    /// Writes back the scalar recurrence state after a fused bulk loop.
    ///
    /// This mirrors exactly what `appended` warm calls of [`Self::append`]
    /// would have left behind: `samples` advances by `appended` and the value
    /// becomes `value`. Callers must only use this once the state is warm
    /// (`current().is_some()`), so the SMA seed is untouched.
    #[inline]
    pub(crate) fn store_bulk_state(&mut self, value: f64, appended: usize) {
        debug_assert!(self.value.is_some());
        self.samples += appended;
        self.value = Some(value);
    }
}

impl StreamingIndicator for ExponentialMovingAverage {
    type Output = f64;

    /// Bulk initialization writes aligned `f64` output directly, avoiding the
    /// former intermediate `Vec<Option<f64>>` and adapter conversion pass.
    /// Continued and short chunks replay [`Self::append`] so every split leaves
    /// exactly the same seed and recurrence state as scalar execution.
    fn extend_slice_into(&mut self, inputs: &[f64], output: &mut Vec<f64>) {
        if inputs.is_empty() {
            return;
        }
        output.reserve(inputs.len());
        if self.samples != 0 || inputs.len() < self.period {
            output.extend(
                inputs
                    .iter()
                    .map(|&input| self.append(input).unwrap_or(f64::NAN)),
            );
            return;
        }

        output.resize(output.len() + self.period - 1, f64::NAN);
        let seed = crate::simd::sum_f64(&inputs[..self.period]) / self.period as f64;
        output.push(seed);
        let previous = ema_steady_loop(&inputs[self.period..], self.k, seed, output);

        self.samples = inputs.len();
        self.value = Some(previous);
    }

    fn append(&mut self, input: f64) -> Option<f64> {
        self.samples += 1;
        self.value = match self.value {
            Some(previous) => Some(self.k.mul_add(input - previous, previous)),
            None => self.seed.append(input),
        };
        self.value
    }

    fn value(&self) -> Option<f64> {
        self.value
    }

    fn reset(&mut self) {
        self.seed.reset();
        self.samples = 0;
        self.value = None;
    }
}
