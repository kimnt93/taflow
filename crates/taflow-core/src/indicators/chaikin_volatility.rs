use crate::error::TaResult;
use crate::indicators::ExponentialMovingAverage;
use crate::stream::{validate_period, StreamingIndicator};
use std::collections::VecDeque;

/// Rate of change of an EMA-smoothed high-low spread.
#[derive(Debug, Clone)]
pub struct ChaikinVolatility {
    spread_average: ExponentialMovingAverage,
    roc_period: usize,
    smoothed_history: VecDeque<f64>,
    value: Option<f64>,
}

impl ChaikinVolatility {
    /// Create an indicator with explicit EMA and rate-of-change periods.
    pub fn new(ema_period: usize, roc_period: usize) -> TaResult<Self> {
        validate_period(ema_period)?;
        validate_period(roc_period)?;
        Ok(Self {
            spread_average: ExponentialMovingAverage::new(ema_period)?,
            roc_period,
            smoothed_history: VecDeque::with_capacity(roc_period + 1),
            value: None,
        })
    }

    /// Append one high/low bar and return percentage spread expansion.
    pub fn append(&mut self, high: f64, low: f64) -> Option<f64> {
        let Some(smoothed) = self.spread_average.append(high - low) else {
            return None;
        };
        if self.smoothed_history.len() == self.roc_period + 1 {
            self.smoothed_history.pop_front();
        }
        self.smoothed_history.push_back(smoothed);
        self.value = (self.smoothed_history.len() == self.roc_period + 1).then(|| {
            let previous = self.smoothed_history[0];
            if previous == 0.0 {
                0.0
            } else {
                100.0 * (smoothed - previous) / previous
            }
        });
        self.value
    }

    /// Return the latest percentage value, or `None` during warm-up.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Clear the EMA, rate-of-change history, and latest value.
    pub fn reset(&mut self) {
        self.spread_average.reset();
        self.smoothed_history.clear();
        self.value = None;
    }
}
