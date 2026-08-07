//! Incremental Triangular Moving Average (TRIMA).

use crate::error::TaResult;

use super::{invalid_period, SimpleMovingAverage, StreamingIndicator};

/// Computes an aligned Triangular Moving Average vector.
pub fn triangular_moving_average(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    let mut state = TriangularMovingAverage::new(timeperiod)?;
    Ok(input.iter().map(|&value| state.append(value).unwrap_or(f64::NAN)).collect())
}

/// Stateful triangular moving average as two cascaded SMA windows.
#[derive(Debug, Clone)]
pub struct TriangularMovingAverage {
    sma1: SimpleMovingAverage,
    sma2: SimpleMovingAverage,
    value: Option<f64>,
}

impl TriangularMovingAverage {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(period: usize) -> TaResult<Self> {
        if period == 0 {
            return Err(invalid_period("timeperiod", period, 1));
        }
        let (p1, p2) = if period % 2 == 1 {
            let half = (period + 1) / 2;
            (half, half)
        } else {
            (period / 2 + 1, period / 2)
        };
        Ok(Self {
            sma1: SimpleMovingAverage::new(p1)?,
            sma2: SimpleMovingAverage::new(p2)?,
            value: None,
        })
    }
}

impl StreamingIndicator for TriangularMovingAverage {
    type Output = f64;

    fn append(&mut self, input: f64) -> Option<f64> {
        self.value = self
            .sma1
            .append(input)
            .and_then(|first| self.sma2.append(first));
        self.value
    }

    fn value(&self) -> Option<f64> {
        self.value
    }

    fn reset(&mut self) {
        self.sma1.reset();
        self.sma2.reset();
        self.value = None;
    }
}
