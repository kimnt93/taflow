use super::operator_states::*;
use crate::error::TaResult;
use std::collections::VecDeque;

pub struct SpreadZScore {
    values: VecDeque<(f64, f64)>,
    timeperiod: usize,
    value: Option<f64>,
}

impl SpreadZScore {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(timeperiod: usize) -> TaResult<Self> {
        validate_period(timeperiod)?;
        Ok(Self {
            values: VecDeque::with_capacity(timeperiod),
            timeperiod,
            value: None,
        })
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, x: f64, y: f64) -> Option<f64> {
        if self.values.len() == self.timeperiod {
            self.values.pop_front();
        }
        self.values.push_back((x, y));
        self.value = if self.values.len() == self.timeperiod {
            let n = self.timeperiod as f64;
            // Contiguous two-slice scans with fused accumulators: each
            // accumulator adds the same terms in the same order as the
            // original per-quantity passes, so results are bit-identical.
            let (front, back) = self.values.as_slices();
            let mut sum_x = 0.0;
            let mut sum_y = 0.0;
            for &(x, y) in front {
                sum_x += x;
                sum_y += y;
            }
            for &(x, y) in back {
                sum_x += x;
                sum_y += y;
            }
            let mean_x = sum_x / n;
            let mean_y = sum_y / n;
            let mut covariance = 0.0;
            let mut variance = 0.0;
            for &(x, y) in front {
                let delta_x = x - mean_x;
                covariance += delta_x * (y - mean_y);
                variance += delta_x * delta_x;
            }
            for &(x, y) in back {
                let delta_x = x - mean_x;
                covariance += delta_x * (y - mean_y);
                variance += delta_x * delta_x;
            }
            let beta = if variance > 0.0 {
                covariance / variance
            } else {
                0.0
            };
            let spread = y - beta * x;
            let mut spread_sum = 0.0;
            for &(x, y) in front {
                spread_sum += y - beta * x;
            }
            for &(x, y) in back {
                spread_sum += y - beta * x;
            }
            let mean_spread = spread_sum / n;
            let mut spread_squared = 0.0;
            for &(x, y) in front {
                let delta = y - beta * x - mean_spread;
                spread_squared += delta * delta;
            }
            for &(x, y) in back {
                let delta = y - beta * x - mean_spread;
                spread_squared += delta * delta;
            }
            let std_spread = (spread_squared / n).sqrt();
            Some(if std_spread > 0.0 {
                (spread - mean_spread) / std_spread
            } else {
                0.0
            })
        } else {
            None
        };
        self.value
    }

    /// Computes or updates `value` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Reset the persistent state and clear the latest value.
    pub fn reset(&mut self) {
        self.values.clear();
        self.value = None;
    }
}
