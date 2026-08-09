//! Persistent Williams Percent R state.

use super::{invalid_period, vhgw, MonotonicMax, MonotonicMin};
use crate::error::{TaError, TaResult};

/// Locate the close within the trailing high-low range on a -100 to 0 scale.
#[derive(Debug, Clone)]
pub struct WilliamsPercentR {
    highs: MonotonicMax,
    lows: MonotonicMin,
    value: Option<f64>,
}

impl WilliamsPercentR {
    /// Create a state with a lookback of at least two bars.
    pub fn new(period: usize) -> TaResult<Self> {
        if period < 2 {
            return Err(invalid_period("timeperiod", period, 2));
        }
        Ok(Self {
            highs: MonotonicMax::new(period)?,
            lows: MonotonicMin::new(period)?,
            value: None,
        })
    }

    /// Append one chronological high/low/close tuple.
    pub fn append(&mut self, high: f64, low: f64, close: f64) -> Option<f64> {
        let maximum = self.highs.append(high);
        let minimum = self.lows.append(low);
        self.value = maximum.zip(minimum).map(|(maximum, minimum)| {
            let range = maximum - minimum;
            if range > 0.0 {
                -100.0 * (maximum - close) / range
            } else {
                0.0
            }
        });
        self.value
    }

    /// Append aligned slices and NaN-fill warm-up positions.
    ///
    /// A fresh sufficiently long state uses the portable vHGW extrema kernel,
    /// then rebuilds both bounded deques so continuation is scalar-equivalent.
    pub fn extend_slices_into(
        &mut self,
        high: &[f64],
        low: &[f64],
        close: &[f64],
        output: &mut Vec<f64>,
    ) -> TaResult<()> {
        let len = high.len();
        if low.len() != len {
            return Err(TaError::LengthMismatch {
                expected: len,
                got: low.len(),
            });
        }
        if close.len() != len {
            return Err(TaError::LengthMismatch {
                expected: len,
                got: close.len(),
            });
        }
        let period = self.highs.period();
        if self.highs.count() != 0 || len < period {
            output.reserve(len);
            for index in 0..len {
                output.push(
                    self.append(high[index], low[index], close[index])
                        .unwrap_or(f64::NAN),
                );
            }
            return Ok(());
        }
        let start = output.len();
        output.resize(start + len, f64::NAN);
        let warm = start + period - 1;
        let mut lowest = vec![0.0_f64; len - (period - 1)];
        vhgw::sliding_max_into(high, period, &mut output[warm..]);
        vhgw::sliding_min_into(low, period, &mut lowest);
        for (offset, (slot, &minimum)) in output[warm..].iter_mut().zip(&lowest).enumerate() {
            let maximum = *slot;
            let range = maximum - minimum;
            *slot = if range > 0.0 {
                -100.0 * (maximum - close[period - 1 + offset]) / range
            } else {
                0.0
            };
        }
        self.highs.rebuild_from_full_run(high);
        self.lows.rebuild_from_full_run(low);
        self.value = output.last().copied();
        Ok(())
    }

    /// Return the latest result, or `None` during warm-up.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Restore fresh-state behavior without reallocating.
    pub fn reset(&mut self) {
        self.highs.reset();
        self.lows.reset();
        self.value = None;
    }
}
