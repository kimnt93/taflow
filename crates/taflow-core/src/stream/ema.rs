//! Incremental Exponential Moving Average (EMA).

use crate::error::TaResult;

use super::{invalid_period, SimpleMovingAverage, StreamingIndicator};

/// Computes an aligned Exponential Moving Average vector using the stream
/// recurrence and SMA seed. Warm-up entries are `NaN`.
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
