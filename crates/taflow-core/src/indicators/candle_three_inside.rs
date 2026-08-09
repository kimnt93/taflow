//! Incremental Three Inside pattern recognition (CDL3INSIDE).
use crate::error::TaResult;
use crate::stream::pattern::*;
use std::collections::VecDeque;
#[derive(Clone, Copy)]
struct Candle {
    open: f64,
    close: f64,
}
impl Candle {
    fn body(self) -> f64 {
        (self.close - self.open).abs()
    }
    fn color(self) -> i32 {
        if self.close >= self.open {
            1
        } else {
            -1
        }
    }
}
/// Incremental CDL3INSIDE state.
/// Persistent Rust state or aligned output type for `CandleThreeInside`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct CandleThreeInside {
    candles: VecDeque<Candle>,
    body_long_sum: f64,
    body_short_sum: f64,
    value: Option<i32>,
}
impl Default for CandleThreeInside {
    fn default() -> Self {
        Self::new()
    }
}
impl CandleThreeInside {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new() -> Self {
        Self {
            candles: VecDeque::with_capacity(12),
            body_long_sum: 0.0,
            body_short_sum: 0.0,
            value: None,
        }
    }
    /// Appends OHLC data and returns a three-inside signal after warmup.
    pub fn append(&mut self, open: f64, _high: f64, _low: f64, close: f64) -> Option<i32> {
        let current = Candle { open, close };
        // Deque holds bars i-12..=i-1; bar j maps to index 12 - (i - j).
        let output = if self.candles.len() == 12 {
            let first = self.candles[10]; // bar i-2
            let second = self.candles[11]; // bar i-1
            let inside = first.body()
                > ca_realbody_scalar(BODY_LONG, self.body_long_sum, first.open, first.close)
                && second.body()
                    <= ca_realbody_scalar(
                        BODY_SHORT,
                        self.body_short_sum,
                        second.open,
                        second.close,
                    )
                && second.open.max(second.close) < first.open.max(first.close)
                && second.open.min(second.close) > first.open.min(first.close);
            let reversal =
                (first.color() == 1 && current.color() == -1 && current.close < first.open)
                    || (first.color() == -1 && current.color() == 1 && current.close > first.open);
            // Slide sums exactly like the batch loop: sum += cr(bar) - cr(bar - 10).
            self.body_long_sum += cr_realbody_scalar(first.open, first.close)
                - cr_realbody_scalar(self.candles[0].open, self.candles[0].close);
            self.body_short_sum += cr_realbody_scalar(second.open, second.close)
                - cr_realbody_scalar(self.candles[1].open, self.candles[1].close);
            Some(-((inside && reversal) as i32) * first.color() * 100)
        } else {
            // Warm-up: seed the sums exactly like the batch prologue.
            let i = self.candles.len();
            if i < 10 {
                self.body_long_sum += cr_realbody_scalar(open, close);
            }
            if (1..11).contains(&i) {
                self.body_short_sum += cr_realbody_scalar(open, close);
            }
            None
        };
        if self.candles.len() == 12 {
            self.candles.pop_front();
        }
        self.candles.push_back(current);
        self.value = output;
        output
    }
    /// Bulk-append aligned OHLC slices, pushing one score per bar into `output`.
    ///
    /// From a pristine state this runs the incremental batch kernel over the
    /// slices and then replays only the trailing bars through `append` to
    /// rebuild the window-bounded streaming state; the replayed scores are
    /// discarded because the batch pass already emitted them. A non-pristine
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
        if !self.candles.is_empty() {
            for i in 0..len {
                output.push(self.append(open[i], high[i], low[i], close[i]).unwrap_or(0));
            }
            return Ok(());
        }
        let scores = Self::batch(open, high, low, close)?;
        output.extend_from_slice(&scores);
        // Every field of this state is a function of the last `BULK_REPLAY_BARS`
        // bars at most (deepest candle window is 10-bar average + 4 offset), so
        // replaying that tail from empty reproduces the full-run state exactly,
        // including `value` (set by the final `append`).
        let replay = len.min(BULK_REPLAY_BARS);
        for i in (len - replay)..len {
            self.append(open[i], high[i], low[i], close[i]);
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
        self.body_long_sum = 0.0;
        self.body_short_sum = 0.0;
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
/// Compute the candle three inside result for the supplied aligned series.
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
impl CandleThreeInside {
    fn batch(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> TaResult<Vec<i32>> {
        let len = validate_ohlc(open, high, low, close)?;
        let mut output = vec![0i32; len];
        let lookback = BODY_SHORT.avg_period.max(BODY_LONG.avg_period) + 2;
        if len <= lookback {
            return Ok(output);
        }

        let mut body_long_sum = 0.0;
        let mut body_short_sum = 0.0;
        let start = lookback;
        for i in (start - 2 - BODY_LONG.avg_period)..(start - 2) {
            body_long_sum += cr_realbody(open, high, low, close, i);
        }
        for i in (start - 1 - BODY_SHORT.avg_period)..(start - 1) {
            body_short_sum += cr_realbody(open, high, low, close, i);
        }

        for i in start..len {
            output[i] = (real_body(open[i - 2], close[i - 2])
                > ca_realbody(BODY_LONG, body_long_sum, open, high, low, close, i - 2)
                && real_body(open[i - 1], close[i - 1])
                    <= ca_realbody(BODY_SHORT, body_short_sum, open, high, low, close, i - 1)
                && open[i - 1].max(close[i - 1]) < open[i - 2].max(close[i - 2])
                && open[i - 1].min(close[i - 1]) > open[i - 2].min(close[i - 2])
                && ((candle_color(open[i - 2], close[i - 2]) == 1
                    && candle_color(open[i], close[i]) == -1
                    && close[i] < open[i - 2])
                    || (candle_color(open[i - 2], close[i - 2]) == -1
                        && candle_color(open[i], close[i]) == 1
                        && close[i] > open[i - 2]))) as i32
                * -candle_color(open[i - 2], close[i - 2])
                * 100;
            body_long_sum += cr_realbody(open, high, low, close, i - 2)
                - cr_realbody(open, high, low, close, i - 2 - BODY_LONG.avg_period);
            body_short_sum += cr_realbody(open, high, low, close, i - 1)
                - cr_realbody(open, high, low, close, i - 1 - BODY_SHORT.avg_period);
        }
        Ok(output)
    }
}
