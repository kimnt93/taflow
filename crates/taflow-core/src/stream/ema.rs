//! Incremental Exponential Moving Average (EMA).

use crate::error::TaResult;

use super::{invalid_period, SimpleMovingAverage, StreamingIndicator};

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
pub fn exponential_moving_average(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    let mut state = ExponentialMovingAverage::new(timeperiod)?;
    Ok(state
        .extend_slice(input)
        .into_iter()
        .map(|value| value.unwrap_or(f64::NAN))
        .collect())
}

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

    /// Extends an empty state through the optimized contiguous bulk path.
    ///
    /// Partial and continued chunks use `append` so chunk boundaries preserve
    /// the exact recurrence state.
    pub fn extend_slice(&mut self, inputs: &[f64]) -> Vec<Option<f64>> {
        if self.samples != 0 || inputs.len() < self.period {
            return inputs.iter().map(|&input| self.append(input)).collect();
        }

        let mut outputs = Vec::with_capacity(inputs.len());
        outputs.resize(self.period - 1, None);

        let seed = crate::simd::sum_f64(&inputs[..self.period]) / self.period as f64;
        outputs.push(Some(seed));

        let mut previous = seed;
        for &input in &inputs[self.period..] {
            previous = self.k.mul_add(input - previous, previous);
            outputs.push(Some(previous));
        }

        self.samples = inputs.len();
        self.value = Some(previous);
        outputs
    }
}

impl StreamingIndicator for ExponentialMovingAverage {
    type Output = f64;

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
