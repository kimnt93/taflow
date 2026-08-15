//! Incremental Rickshaw Man candlestick recognition (CDLRICKSHAWMAN).

use std::collections::VecDeque;

use crate::error::TaResult;
use crate::stream::pattern::*;
/// Incremental CDLRICKSHAWMAN state using TA-Lib's doji and near range averages.
/// Persistent Rust state or aligned output type for `CandleRickshawman`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct CandleRickshawman {
    body_ranges: VecDeque<f64>,
    body_sum: f64,
    near_ranges: VecDeque<f64>,
    near_sum: f64,
    value: Option<i32>,
}
impl Default for CandleRickshawman {
    fn default() -> Self {
        Self::new()
    }
}
impl CandleRickshawman {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new() -> Self {
        Self {
            body_ranges: VecDeque::with_capacity(10),
            body_sum: 0.0,
            near_ranges: VecDeque::with_capacity(5),
            near_sum: 0.0,
            value: None,
        }
    }
    fn push(window: &mut VecDeque<f64>, sum: &mut f64, capacity: usize, value: f64) {
        if window.len() == capacity {
            // Slide exactly like the batch loop: sum += cr(new) - cr(evicted).
            let old = window.pop_front().expect("window is full");
            *sum += value - old;
        } else {
            *sum += value;
        }
        window.push_back(value);
    }
    /// Appends OHLC data and returns +100 for a rickshaw man after the ten-bar warmup.
    pub fn append(&mut self, open: f64, high: f64, low: f64, close: f64) -> Option<i32> {
        let range = high - low;
        let body = (close - open).abs();
        let output = if self.body_ranges.len() == 10 && self.near_ranges.len() == 5 {
            let midpoint = low + range / 2.0;
            let near = ca_highlow_scalar(NEAR, self.near_sum, high, low);
            Some(
                (body <= ca_highlow_scalar(BODY_DOJI, self.body_sum, high, low)
                    && open.min(close) - low > body
                    && high - open.max(close) > body
                    && open.min(close) <= midpoint + near
                    && open.max(close) >= midpoint - near) as i32
                    * 100,
            )
        } else {
            None
        };
        Self::push(&mut self.body_ranges, &mut self.body_sum, 10, range);
        Self::push(&mut self.near_ranges, &mut self.near_sum, 5, range);
        self.value = output;
        output
    }
    /// Bulk-append aligned OHLC slices, pushing one score per bar into `output`.
    ///
    /// From a pristine state this runs directly over the slices and rebuilds
    /// the rolling sums and bounded range windows once after the loop. A non-pristine
    /// state falls back to the per-bar loop. Either route is bit-identical to
    /// calling `append` once per bar (warm-up `None` becomes `0`, matching the
    /// batch prologue).
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
        const LOOKBACK: usize = 10;
        if !self.body_ranges.is_empty() || len <= LOOKBACK {
            for i in 0..len {
                output.push(self.append(open[i], high[i], low[i], close[i]).unwrap_or(0));
            }
            return Ok(());
        }

        let output_start = output.len();
        output.resize(output_start + len, 0);
        let mut body_sum = high[..LOOKBACK]
            .iter()
            .zip(&low[..LOOKBACK])
            .fold(0.0, |sum, (&high, &low)| sum + high - low);
        let mut near_sum = high[5..LOOKBACK]
            .iter()
            .zip(&low[5..LOOKBACK])
            .fold(0.0, |sum, (&high, &low)| sum + high - low);

        for (((open_window, high_window), low_window), (close_window, output)) in open
            .windows(LOOKBACK + 1)
            .zip(high.windows(LOOKBACK + 1))
            .zip(low.windows(LOOKBACK + 1))
            .zip(
                close
                    .windows(LOOKBACK + 1)
                    .zip(&mut output[output_start + LOOKBACK..]),
            )
        {
            let current_open = open_window[LOOKBACK];
            let current_high = high_window[LOOKBACK];
            let current_low = low_window[LOOKBACK];
            let current_close = close_window[LOOKBACK];
            let range = current_high - current_low;
            let body = (current_close - current_open).abs();
            let midpoint = current_low + range / 2.0;
            let near = ca_highlow_scalar(NEAR, near_sum, current_high, current_low);
            *output = ((body <= ca_highlow_scalar(BODY_DOJI, body_sum, current_high, current_low)
                && current_open.min(current_close) - current_low > body
                && current_high - current_open.max(current_close) > body
                && current_open.min(current_close) <= midpoint + near
                && current_open.max(current_close) >= midpoint - near)
                as i32)
                * 100;
            body_sum += range - (high_window[0] - low_window[0]);
            near_sum += range - (high_window[5] - low_window[5]);
        }

        self.body_ranges.extend(
            high[len - LOOKBACK..]
                .iter()
                .zip(&low[len - LOOKBACK..])
                .map(|(&high, &low)| high - low),
        );
        self.near_ranges.extend(
            high[len - 5..]
                .iter()
                .zip(&low[len - 5..])
                .map(|(&high, &low)| high - low),
        );
        self.body_sum = body_sum;
        self.near_sum = near_sum;
        self.value = Some(output[output_start + len - 1]);
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
        self.body_ranges.clear();
        self.body_sum = 0.0;
        self.near_ranges.clear();
        self.near_sum = 0.0;
        self.value = None;
    }
}
