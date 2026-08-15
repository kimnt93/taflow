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
        if len == 0 {
            return Ok(());
        }
        output.reserve(len);
        let mut index = self.index;
        let mut accumulation_distribution = self.accumulation_distribution;
        let mut value = self.value;
        let mut offset = 0;

        let (mut fast_average, mut slow_average) = match (self.fast_average, self.slow_average) {
            (Some(fast), Some(slow)) => (fast, slow),
            _ => {
                accumulation_distribution +=
                    money_flow_volume(high[0], low[0], close[0], volume[0]);
                let seeded = accumulation_distribution;
                if index >= self.lookback {
                    value = Some(0.0);
                }
                output.push(value.unwrap_or(f64::NAN));
                index += 1;
                offset = 1;
                (seeded, seeded)
            }
        };

        for position in offset..len {
            accumulation_distribution += money_flow_volume(
                high[position],
                low[position],
                close[position],
                volume[position],
            );
            fast_average = self
                .fast_smoothing
                .mul_add(accumulation_distribution - fast_average, fast_average);
            slow_average = self
                .slow_smoothing
                .mul_add(accumulation_distribution - slow_average, slow_average);
            if index >= self.lookback {
                value = Some(fast_average - slow_average);
            }
            output.push(value.unwrap_or(f64::NAN));
            index += 1;
        }

        self.index = index;
        self.accumulation_distribution = accumulation_distribution;
        self.fast_average = Some(fast_average);
        self.slow_average = Some(slow_average);
        self.value = value;
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
