//! Incremental Three Black Crows candlestick recognition (CDL3BLACKCROWS).
use super::pattern::*;
use crate::error::TaResult;
use std::collections::VecDeque;
#[derive(Clone, Copy)]
struct Candle {
    open: f64,
    high: f64,
    low: f64,
    close: f64,
}
impl Candle {
    fn lower(self) -> f64 {
        self.open.min(self.close) - self.low
    }
    fn black(self) -> bool {
        self.close < self.open
    }
}
/// Incremental CDL3BLACKCROWS state.
/// Persistent Rust state or aligned output type for `CandleThreeBlackCrows`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct CandleThreeBlackCrows {
    candles: VecDeque<Candle>,
    shadow_sum: [f64; 3],
    value: Option<i32>,
}
impl Default for CandleThreeBlackCrows {
    fn default() -> Self {
        Self::new()
    }
}
impl CandleThreeBlackCrows {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new() -> Self {
        Self {
            candles: VecDeque::with_capacity(13),
            shadow_sum: [0.0; 3],
            value: None,
        }
    }
    /// Appends OHLC data and returns -100 for three black crows after warmup.
    pub fn append(&mut self, open: f64, high: f64, low: f64, close: f64) -> Option<i32> {
        let current = Candle {
            open,
            high,
            low,
            close,
        };
        // Deque holds bars i-13..=i-1; bar j maps to index 13 - (i - j).
        let output = if self.candles.len() == 13 {
            let p3 = self.candles[10]; // bar i-3
            let a = self.candles[11]; // bar i-2
            let b = self.candles[12]; // bar i-1
            let pattern = a.black()
                && b.black()
                && current.black()
                && b.close < a.close
                && current.close < b.close
                && a.open <= p3.open.max(p3.close)
                && b.open <= a.open
                && b.open >= a.close
                && current.open <= b.open
                && current.open >= b.close
                && a.lower()
                    < ca_highlow_scalar(SHADOW_VERY_SHORT, self.shadow_sum[0], a.high, a.low)
                && b.lower()
                    < ca_highlow_scalar(SHADOW_VERY_SHORT, self.shadow_sum[1], b.high, b.low)
                && current.lower()
                    < ca_highlow_scalar(SHADOW_VERY_SHORT, self.shadow_sum[2], high, low);
            // Slide sums exactly like the batch loop: sum += cr(bar) - cr(bar - 10).
            self.shadow_sum[0] += cr_highlow_scalar(a.high, a.low)
                - cr_highlow_scalar(self.candles[1].high, self.candles[1].low);
            self.shadow_sum[1] += cr_highlow_scalar(b.high, b.low)
                - cr_highlow_scalar(self.candles[2].high, self.candles[2].low);
            self.shadow_sum[2] += cr_highlow_scalar(high, low)
                - cr_highlow_scalar(self.candles[3].high, self.candles[3].low);
            Some(-(pattern as i32) * 100)
        } else {
            // Warm-up: seed the sums exactly like the batch prologue.
            let i = self.candles.len();
            for k in 0..3 {
                if i >= k && i < 10 + k {
                    self.shadow_sum[k] += cr_highlow_scalar(high, low);
                }
            }
            None
        };
        if self.candles.len() == 13 {
            self.candles.pop_front();
        }
        self.candles.push_back(current);
        self.value = output;
        output
    }
    /// Bulk-append aligned OHLC slices, pushing one score per bar into `output`.
    ///
    /// No from-empty fast path here. The warm-up prologue seeds `shadow_sum`
    /// one bar earlier than the steady-state slide evicts (`+= cr(i-2)` against
    /// `-= cr(i-12)` on a window seeded at bars `0..=9`), which the batch
    /// kernel mirrors exactly - the two agree with each other, but the sums
    /// keep a `cr(bar 0) - cr(bar 10)` term forever. The state is therefore not
    /// a function of a bounded window and a tail replay cannot reconstruct it,
    /// so the bulk entry point is the per-bar `append` loop with the `Option`
    /// unwrapped in place. Bit-identical to calling `append` once per bar.
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
        self.shadow_sum = [0.0; 3];
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
/// Compute the candle three black crows result for the supplied aligned series.
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
pub fn candle_three_black_crows(
    open: &[f64],
    high: &[f64],
    low: &[f64],
    close: &[f64],
) -> TaResult<Vec<i32>> {
    let len = validate_ohlc(open, high, low, close)?;
    let mut output = vec![0i32; len];
    let lookback = SHADOW_VERY_SHORT.avg_period + 3;
    if len <= lookback {
        return Ok(output);
    }

    let mut shadow_sum = [0.0f64; 3];
    let start = lookback;
    for k in 0..3 {
        let bar_offset = start - 3 + k;
        if bar_offset >= SHADOW_VERY_SHORT.avg_period {
            for j in (bar_offset - SHADOW_VERY_SHORT.avg_period)..bar_offset {
                shadow_sum[k] += cr_highlow(open, high, low, close, j);
            }
        }
    }

    for i in start..len {
        output[i] = (candle_color(open[i - 2], close[i - 2]) == -1
            && candle_color(open[i - 1], close[i - 1]) == -1
            && candle_color(open[i], close[i]) == -1
            && close[i - 1] < close[i - 2]
            && close[i] < close[i - 1]
            && open[i - 2] <= open[i - 3].max(close[i - 3])
            && open[i - 1] <= open[i - 2]
            && open[i - 1] >= close[i - 2]
            && open[i] <= open[i - 1]
            && open[i] >= close[i - 1]
            && lower_shadow(open[i - 2], low[i - 2], close[i - 2])
                < ca_highlow(
                    SHADOW_VERY_SHORT,
                    shadow_sum[0],
                    open,
                    high,
                    low,
                    close,
                    i - 2,
                )
            && lower_shadow(open[i - 1], low[i - 1], close[i - 1])
                < ca_highlow(
                    SHADOW_VERY_SHORT,
                    shadow_sum[1],
                    open,
                    high,
                    low,
                    close,
                    i - 1,
                )
            && lower_shadow(open[i], low[i], close[i])
                < ca_highlow(SHADOW_VERY_SHORT, shadow_sum[2], open, high, low, close, i))
            as i32
            * -100;
        for k in 0..3 {
            let bar = i - 2 + k;
            if bar >= SHADOW_VERY_SHORT.avg_period {
                shadow_sum[k] += cr_highlow(open, high, low, close, bar)
                    - cr_highlow(open, high, low, close, bar - SHADOW_VERY_SHORT.avg_period);
            }
        }
    }
    Ok(output)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn matches_batch() {
        let open: Vec<f64> = (0..50).map(|i| 100. + i as f64 * 0.2).collect();
        let high: Vec<f64> = open.iter().map(|x| x + 2.).collect();
        let low: Vec<f64> = open.iter().map(|x| x - 2.).collect();
        let close: Vec<f64> = open.iter().map(|x| x + 1.).collect();
        let expected = crate::stream::candle_three_black_crows(&open, &high, &low, &close).unwrap();
        let mut state = CandleThreeBlackCrows::new();
        for (((&o, &h), &l), (&c, &expected)) in open
            .iter()
            .zip(&high)
            .zip(&low)
            .zip(close.iter().zip(&expected))
        {
            match state.append(o, h, l, c) {
                Some(v) => assert_eq!(v, expected),
                None => assert_eq!(expected, 0),
            }
        }
    }
}
