//! Persistent Normalized Average True Range state.

use super::AverageTrueRange;
use crate::error::{TaError, TaResult};

/// Scale Average True Range by the current close as a percentage.
#[derive(Debug, Clone)]
pub struct NormalizedAverageTrueRange {
    average_true_range: AverageTrueRange,
    normalize: bool,
    value: Option<f64>,
}

impl NormalizedAverageTrueRange {
    /// Create a normalized state with a positive Average True Range period.
    pub fn new(period: usize) -> TaResult<Self> {
        Ok(Self {
            average_true_range: AverageTrueRange::new(period)?,
            normalize: period > 1,
            value: None,
        })
    }

    /// Append one chronological high/low/close tuple.
    pub fn append(&mut self, high: f64, low: f64, close: f64) -> Option<f64> {
        self.value = self
            .average_true_range
            .append(high, low, close)
            .map(|average_true_range| {
                if !self.normalize {
                    average_true_range
                } else if close == 0.0 {
                    0.0
                } else {
                    average_true_range / close * 100.0
                }
            });
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
        self.average_true_range.reset();
        self.value = None;
    }
}
