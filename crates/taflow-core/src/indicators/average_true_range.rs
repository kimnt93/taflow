//! Persistent Average True Range state.

use crate::error::{TaError, TaResult};
use crate::stream::invalid_period;

/// Smooth true range with Wilder's recurrence after an arithmetic seed.
#[derive(Debug, Clone)]
pub struct AverageTrueRange {
    period: usize,
    previous_close: Option<f64>,
    true_range_count: usize,
    true_range_sum: f64,
    value: Option<f64>,
}

impl AverageTrueRange {
    /// Create an Average True Range state with a positive period.
    pub fn new(period: usize) -> TaResult<Self> {
        if period == 0 {
            return Err(invalid_period("timeperiod", period, 1));
        }
        Ok(Self {
            period,
            previous_close: None,
            true_range_count: 0,
            true_range_sum: 0.0,
            value: None,
        })
    }

    /// Append one chronological high/low/close tuple.
    pub fn append(&mut self, high: f64, low: f64, close: f64) -> Option<f64> {
        let previous_close = self.previous_close.replace(close)?;
        let true_range = (high - low)
            .max((high - previous_close).abs())
            .max((low - previous_close).abs());
        self.true_range_count += 1;

        if self.true_range_count < self.period {
            self.true_range_sum += true_range;
            return None;
        }
        if self.true_range_count == self.period {
            self.value = Some((self.true_range_sum + true_range) / self.period as f64);
        } else if let Some(previous) = self.value {
            let period = self.period as f64;
            self.value = Some((previous * (period - 1.0) + true_range) / period);
        }
        self.value
    }

    /// Append aligned slices in scalar replay order with NaN warm-up.
    pub fn extend_slices_into(
        &mut self,
        high: &[f64],
        low: &[f64],
        close: &[f64],
        output: &mut Vec<f64>,
    ) -> TaResult<()> {
        let len = high.len();
        for actual in [low.len(), close.len()] {
            if actual != len {
                return Err(TaError::LengthMismatch {
                    expected: len,
                    got: actual,
                });
            }
        }
        output.reserve(len);
        for index in 0..len {
            output.push(
                self.append(high[index], low[index], close[index])
                    .unwrap_or(f64::NAN),
            );
        }
        Ok(())
    }

    /// Return the latest result, or `None` during warm-up.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Restore fresh-state behavior without reallocating.
    pub fn reset(&mut self) {
        self.previous_close = None;
        self.true_range_count = 0;
        self.true_range_sum = 0.0;
        self.value = None;
    }
}
