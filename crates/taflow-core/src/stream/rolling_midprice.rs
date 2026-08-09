//! Persistent rolling midprice state.

use super::vhgw;
use crate::error::{TaError, TaResult};
use crate::stream::{MonotonicMax, MonotonicMin};

/// Rolling midpoint of the highest high and lowest low over `period` bars.
#[derive(Debug, Clone)]
pub struct RollingMidprice {
    highs: MonotonicMax,
    lows: MonotonicMin,
    value: Option<f64>,
}

impl RollingMidprice {
    /// Creates a rolling midprice state with the validated lookback period.
    pub fn new(period: usize) -> TaResult<Self> {
        Ok(Self {
            highs: MonotonicMax::new(period)?,
            lows: MonotonicMin::new(period)?,
            value: None,
        })
    }

    /// Appends one high/low pair and returns the latest value after warm-up.
    pub fn append(&mut self, high: f64, low: f64) -> Option<f64> {
        let maximum = self.highs.append(high);
        let minimum = self.lows.append(low);
        self.value = maximum.zip(minimum).map(|(high, low)| (high + low) * 0.5);
        self.value
    }

    /// Extends the state with aligned high and low slices.
    pub fn extend_slices_into(
        &mut self,
        high: &[f64],
        low: &[f64],
        output: &mut Vec<f64>,
    ) -> TaResult<()> {
        if high.len() != low.len() {
            return Err(TaError::LengthMismatch {
                expected: high.len(),
                got: low.len(),
            });
        }
        let n = high.len();
        let period = self.highs.period();
        if self.highs.count() != 0 || n < period {
            output.reserve(n);
            for index in 0..n {
                output.push(self.append(high[index], low[index]).unwrap_or(f64::NAN));
            }
            return Ok(());
        }
        let start = output.len();
        output.resize(start + n, f64::NAN);
        let warm = start + period - 1;
        let mut lowest = vec![0.0_f64; n - (period - 1)];
        vhgw::sliding_max_into(high, period, &mut output[warm..]);
        vhgw::sliding_min_into(low, period, &mut lowest);
        for (slot, &minimum) in output[warm..].iter_mut().zip(&lowest) {
            *slot = (*slot + minimum) * 0.5;
        }
        self.highs.rebuild_from_full_run(high);
        self.lows.rebuild_from_full_run(low);
        self.value = output.last().copied();
        Ok(())
    }

    /// Returns the latest value after warm-up.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Resets the state without reallocating its bounded buffers.
    pub fn reset(&mut self) {
        self.highs.reset();
        self.lows.reset();
        self.value = None;
    }
}
