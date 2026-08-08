//! Incremental Modified Hikkake recognition (CDLHIKKAKEMOD).
use super::pattern::*;
use crate::error::TaResult;
use std::collections::VecDeque;
#[derive(Clone, Copy)]
struct Candle {
    high: f64,
    low: f64,
    close: f64,
}
impl Candle {}
/// Incremental CDLHIKKAKEMOD state.
/// Persistent Rust state or aligned output type for `CandleHikkakeModified`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct CandleHikkakeModified {
    candles: VecDeque<Candle>,
    index: usize,
    pending: Option<(usize, i32, f64)>,
    near_sum: f64,
    value: Option<i32>,
}
impl Default for CandleHikkakeModified {
    fn default() -> Self {
        Self::new()
    }
}
impl CandleHikkakeModified {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new() -> Self {
        Self {
            candles: VecDeque::with_capacity(8),
            index: 0,
            pending: None,
            near_sum: 0.0,
            value: None,
        }
    }
    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, _open: f64, high: f64, low: f64, close: f64) -> Option<i32> {
        let current = Candle { high, low, close };
        let i = self.index;
        self.index += 1;
        let mut result = 0;
        let mut new_pattern = false;
        // Deque holds bars i-8..=i-1; bar j maps to index 8 - (i - j).
        if i >= 10 && self.candles.len() == 8 {
            let a = self.candles[5]; // bar i-3
            let b = self.candles[6]; // bar i-2
            let c = self.candles[7]; // bar i-1
            if c.high < b.high && c.low > b.low && b.high < a.high && b.low > a.low {
                let near = ca_highlow_scalar(NEAR, self.near_sum, b.high, b.low);
                if high < c.high && low < c.low && b.close <= b.low + near {
                    self.pending = Some((i, 100, c.high));
                    result = 100;
                    new_pattern = true;
                } else if high > c.high && low > c.low && b.close >= b.high - near {
                    self.pending = Some((i, -100, c.low));
                    result = -100;
                    new_pattern = true;
                }
            }
            if !new_pattern {
                if let Some((setup, direction, threshold)) = self.pending {
                    if i <= setup + 3
                        && ((direction > 0 && close > threshold)
                            || (direction < 0 && close < threshold))
                    {
                        result = direction + direction.signum() * 100;
                        self.pending = None;
                    } else if i > setup + 3 {
                        self.pending = None;
                    }
                }
            }
            // Slide the sum exactly like the batch loop: sum += cr(bar) - cr(bar - 5).
            self.near_sum += cr_highlow_scalar(self.candles[6].high, self.candles[6].low)
                - cr_highlow_scalar(self.candles[1].high, self.candles[1].low);
        } else if (3..8).contains(&i) {
            // Warm-up: seed the sum exactly like the batch prologue.
            self.near_sum += cr_highlow_scalar(high, low);
        }
        if self.candles.len() == 8 {
            self.candles.pop_front();
        }
        self.candles.push_back(current);
        self.value = (i >= 10).then_some(result);
        self.value
    }
    /// Bulk-append aligned OHLC slices, pushing one score per bar into `output`.
    ///
    /// This state carries a monotonic bar counter and a pending setup that can
    /// outlive any fixed window, so no from-empty fast path is safe: the bulk
    /// entry point is the per-bar `append` loop with the `Option` unwrapped in
    /// place. Bit-identical to calling `append` once per bar.
    ///
    /// # Parameters
    ///
    /// * `open`, `high`, `low`, `close` - Equal-length chronological OHLC series.
    /// * `output` - Destination the aligned scores are appended to.
    ///
    /// # Returns
    ///
    /// `Ok(())`, or a validation error when the inputs are not aligned.
    pub fn extend_slices_into(
        &mut self,
        open: &[f64],
        high: &[f64],
        low: &[f64],
        close: &[f64],
        output: &mut Vec<i32>,
    ) -> TaResult<()> {
        let len = validate_ohlc(open, high, low, close)?;
        output.reserve(len);
        for i in 0..len {
            output.push(self.append(open[i], high[i], low[i], close[i]).unwrap_or(0));
        }
        Ok(())
    }

    /// Computes or updates `value` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn value(&self) -> Option<i32> {
        self.value
    }
    /// Reset the persistent state and clear the latest value.
    pub fn reset(&mut self) {
        self.candles.clear();
        self.index = 0;
        self.pending = None;
        self.near_sum = 0.0;
        self.value = None;
    }
}

/// Compute the candle pattern signal for aligned OHLC bars.
///
/// # Parameters
///
/// * `open`, `high`, `low`, `close` - Equal-length chronological OHLC series.
///
/// # Returns
///
/// A same-length vector containing -100, 0, or 100 pattern signals; bars
/// Compute the candle hikkake modified result for the supplied aligned series.
///
/// # Parameters
///
/// * `open` - Input series or configuration value.
/// * `high` - Input series or configuration value.
/// * `low` - Input series or configuration value.
/// * `close` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn candle_hikkake_modified(
    open: &[f64],
    high: &[f64],
    low: &[f64],
    close: &[f64],
) -> TaResult<Vec<i32>> {
    let len = validate_ohlc(open, high, low, close)?;
    let mut output = vec![0i32; len];
    // C TA-Lib: lookback = max(1, TA_CandleAvgPeriod(Near)) + 5
    let lookback = 1_usize.max(NEAR.avg_period) + 5;
    if len <= lookback {
        return Ok(output);
    }

    // Initialize Near sum for bar (start - 3), i.e. the "2nd candle" at start
    let mut near_sum = 0.0;
    // The first evaluated fourth candle is `lookback`; its second candle is
    // two bars earlier, and Near averages the five bars immediately before it.
    let near_bar = lookback - 2;
    if NEAR.avg_period > 0 && near_bar >= NEAR.avg_period {
        for j in (near_bar - NEAR.avg_period)..near_bar {
            near_sum += cr_highlow(open, high, low, close, j);
        }
    }

    let mut pattern_idx: i32 = -10; // no active pattern
    let mut pattern_result: i32 = 0;

    for i in lookback..len {
        // C TA-Lib indices: i is current bar
        // Pattern: bar[i-3] contains bar[i-2], bar[i-2] contains bar[i-1]
        // Then bar[i] breaks out
        let mut new_pattern = false;
        if high[i-1] < high[i-2] && low[i-1] > low[i-2]   // bar[i-1] inside bar[i-2]
            && high[i-2] < high[i-3] && low[i-2] > low[i-3]
        // bar[i-2] inside bar[i-3]
        {
            let near_avg = ca_highlow(NEAR, near_sum, open, high, low, close, i - 2);
            // Bullish: bar[i] breaks down (lower high AND lower low)
            if high[i] < high[i-1] && low[i] < low[i-1]
                // 2nd bar close near the low
                && close[i-2] <= low[i-2] + near_avg
            {
                pattern_result = 100;
                pattern_idx = i as i32;
                output[i] = pattern_result;
                new_pattern = true;
            }
            // Bearish: bar[i] breaks up (higher high AND higher low)
            else if high[i] > high[i-1] && low[i] > low[i-1]
                // 2nd bar close near the high
                && close[i-2] >= high[i-2] - near_avg
            {
                pattern_result = -100;
                pattern_idx = i as i32;
                output[i] = pattern_result;
                new_pattern = true;
            }
        }

        // Confirmation: within 3 bars of pattern
        if !new_pattern && pattern_idx >= 0 && (i as i32) <= pattern_idx + 3 {
            if pattern_result > 0 && close[i] > high[(pattern_idx - 1) as usize] {
                output[i] = pattern_result + 100;
                pattern_idx = -10;
            } else if pattern_result < 0 && close[i] < low[(pattern_idx - 1) as usize] {
                output[i] = pattern_result - 100;
                pattern_idx = -10;
            }
        }

        // Update Near sum (for the "2nd bar" position, which is i-2)
        if NEAR.avg_period > 0 && (i - 2) >= NEAR.avg_period {
            near_sum += cr_highlow(open, high, low, close, i - 2)
                - cr_highlow(open, high, low, close, i - 2 - NEAR.avg_period);
        }
    }
    Ok(output)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn matches_batch() {
        let open = vec![10.; 20];
        let high: Vec<f64> = (0..20).map(|i| 20. - i as f64 * 0.2).collect();
        let low: Vec<f64> = (0..20).map(|i| i as f64 * 0.1).collect();
        let close = high.clone();
        let e = crate::stream::candle_hikkake_modified(&open, &high, &low, &close).unwrap();
        let mut s = CandleHikkakeModified::new();
        for (((&o, &h), &l), (&c, &e)) in open.iter().zip(&high).zip(&low).zip(close.iter().zip(&e))
        {
            match s.append(o, h, l, c) {
                Some(v) => assert_eq!(v, e),
                None => assert_eq!(e, 0),
            }
        }
    }
}
