//! Persistent Accumulation/Distribution Oscillator state.

use crate::error::{TaError, TaResult};
use crate::stream::accumulation_distribution_helper::money_flow_volume;

/// Difference between fast and slow first-value-seeded EMAs of the A/D line.
#[derive(Debug, Clone)]
pub struct AccumulationDistributionOscillator {
    lookback: usize,
    index: usize,
    fast_smoothing: f64,
    slow_smoothing: f64,
    accumulation_distribution: f64,
    fast_average: Option<f64>,
    slow_average: Option<f64>,
    value: Option<f64>,
}

impl AccumulationDistributionOscillator {
    /// Create an oscillator with TA-Lib-compatible periods and warm-up.
    pub fn new(fast_period: usize, slow_period: usize) -> TaResult<Self> {
        if fast_period < 2 || slow_period < 2 {
            return Err(TaError::InvalidParameter {
                name: "fastperiod/slowperiod",
                value: format!("{fast_period}/{slow_period}"),
                reason: "both periods must be >= 2",
            });
        }
        Ok(Self {
            lookback: fast_period.max(slow_period) - 1,
            index: 0,
            fast_smoothing: 2.0 / (fast_period as f64 + 1.0),
            slow_smoothing: 2.0 / (slow_period as f64 + 1.0),
            accumulation_distribution: 0.0,
            fast_average: None,
            slow_average: None,
            value: None,
        })
    }

    /// Append one chronological high/low/close/volume tuple.
    pub fn append(&mut self, high: f64, low: f64, close: f64, volume: f64) -> Option<f64> {
        self.accumulation_distribution += money_flow_volume(high, low, close, volume);
        match (self.fast_average, self.slow_average) {
            (Some(fast), Some(slow)) => {
                self.fast_average = Some(
                    self.fast_smoothing
                        .mul_add(self.accumulation_distribution - fast, fast),
                );
                self.slow_average = Some(
                    self.slow_smoothing
                        .mul_add(self.accumulation_distribution - slow, slow),
                );
            }
            _ => {
                self.fast_average = Some(self.accumulation_distribution);
                self.slow_average = Some(self.accumulation_distribution);
            }
        }
        if self.index >= self.lookback {
            self.value = Some(
                self.fast_average.expect("fast average is initialized")
                    - self.slow_average.expect("slow average is initialized"),
            );
        }
        self.index += 1;
        self.value
    }

    /// Append aligned slices in scalar replay order, NaN-filling warm-up.
    pub fn extend_slices_into(
        &mut self,
        high: &[f64],
        low: &[f64],
        close: &[f64],
        volume: &[f64],
        output: &mut Vec<f64>,
    ) -> TaResult<()> {
        let len = high.len();
        for actual in [low.len(), close.len(), volume.len()] {
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
                self.append(high[index], low[index], close[index], volume[index])
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
        self.index = 0;
        self.accumulation_distribution = 0.0;
        self.fast_average = None;
        self.slow_average = None;
        self.value = None;
    }
}
