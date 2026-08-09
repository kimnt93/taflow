//! Incremental Two Crows candlestick recognition (CDL2CROWS).

use crate::error::TaResult;
use crate::stream::pattern::*;
use std::collections::VecDeque;
#[derive(Clone, Copy)]
struct Candle {
    open: f64,
    close: f64,
    body: f64,
}
/// Incremental CDL2CROWS state.
/// Persistent Rust state or aligned output type for `CandleTwoCrows`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct CandleTwoCrows {
    candles: VecDeque<Candle>,
    bodies: VecDeque<f64>,
    sum: f64,
    value: Option<i32>,
}
impl Default for CandleTwoCrows {
    fn default() -> Self {
        Self::new()
    }
}
impl CandleTwoCrows {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new() -> Self {
        Self {
            candles: VecDeque::with_capacity(3),
            bodies: VecDeque::with_capacity(10),
            sum: 0.0,
            value: None,
        }
    }
    fn push_body(&mut self, value: f64) {
        if self.bodies.len() == 10 {
            // Slide exactly like the batch loop: sum += cr(new) - cr(evicted).
            let old = self.bodies.pop_front().expect("window full");
            self.sum += value - old;
        } else {
            self.sum += value;
        }
        self.bodies.push_back(value);
    }
    /// Appends OHLC data and returns -100 for a two-crows pattern after warmup.
    pub fn append(&mut self, open: f64, _high: f64, _low: f64, close: f64) -> Option<i32> {
        let current = Candle {
            open,
            close,
            body: (close - open).abs(),
        };
        let output = if self.bodies.len() == 10 && self.candles.len() == 2 {
            let first = self.candles[0];
            let second = self.candles[1];
            let pattern = first.close >= first.open
                && first.body > self.sum / 10.0
                && second.close < second.open
                && second.open.min(second.close) > first.open.max(first.close)
                && close < open
                && open < second.open
                && open > second.close
                && close > first.open
                && close < first.close;
            Some(-(pattern as i32) * 100)
        } else {
            None
        };
        if self.candles.len() == 2 {
            self.push_body(self.candles[0].body);
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
        self.bodies.clear();
        self.sum = 0.0;
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
/// Compute the candle two crows result for the supplied aligned series.
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
impl CandleTwoCrows {
    fn batch(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> TaResult<Vec<i32>> {
        let len = validate_ohlc(open, high, low, close)?;
        let mut output = vec![0i32; len];
        let lookback = BODY_LONG.avg_period + 2;
        if len <= lookback {
            return Ok(output);
        }

        let mut body_sum = 0.0;
        let start = lookback;
        for i in (start - 2 - BODY_LONG.avg_period)..(start - 2) {
            body_sum += cr_realbody(open, high, low, close, i);
        }

        for i in start..len {
            // 1st: long white
            output[i] = (candle_color(open[i-2], close[i-2]) == 1
            && real_body(open[i-2], close[i-2]) > ca_realbody(BODY_LONG, body_sum, open, high, low, close, i-2)
            // 2nd: black, gap up
            && candle_color(open[i-1], close[i-1]) == -1
            && real_body_gap_up(open, close, i-1, i-2)
            // 3rd: black, opens within 2nd body, closes within 1st body
            && candle_color(open[i], close[i]) == -1
            && open[i] < open[i-1] && open[i] > close[i-1]
            && close[i] > open[i-2] && close[i] < close[i-2]) as i32
                * -100;
            body_sum += cr_realbody(open, high, low, close, i - 2)
                - cr_realbody(open, high, low, close, i - 2 - BODY_LONG.avg_period);
        }
        Ok(output)
    }
}
