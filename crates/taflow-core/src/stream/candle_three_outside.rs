//! Incremental Three Outside pattern recognition (CDL3OUTSIDE).
use super::pattern::*;
use crate::error::TaResult;
use std::collections::VecDeque;
#[derive(Clone, Copy)]
struct Candle {
    open: f64,
    close: f64,
}
/// Incremental CDL3OUTSIDE state.
/// Persistent Rust state or aligned output type for `CandleThreeOutside`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct CandleThreeOutside {
    candles: VecDeque<Candle>,
    value: Option<i32>,
}
impl Default for CandleThreeOutside {
    fn default() -> Self {
        Self::new()
    }
}
impl CandleThreeOutside {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new() -> Self {
        Self {
            candles: VecDeque::with_capacity(2),
            value: None,
        }
    }
    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, open: f64, _high: f64, _low: f64, close: f64) -> Option<i32> {
        let output = if self.candles.len() == 2 {
            let first = self.candles[0];
            let second = self.candles[1];
            let bull = first.close < first.open
                && second.close >= first.open
                && second.open <= first.close
                && close > second.close;
            let bear = first.close >= first.open
                && second.close < second.open
                && second.open >= first.close
                && second.close <= first.open
                && close < second.close;
            Some((bull as i32) * 100 - (bear as i32) * 100)
        } else {
            None
        };
        if self.candles.len() == 2 {
            self.candles.pop_front();
        }
        self.candles.push_back(Candle { open, close });
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
/// Compute the candle three outside result for the supplied aligned series.
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
impl CandleThreeOutside {
    fn batch(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> TaResult<Vec<i32>> {
        let len = validate_ohlc(open, high, low, close)?;
        let mut output = vec![0i32; len];
        // lookback = 3
        if len < 3 {
            return Ok(output);
        }

        for i in 2..len {
            // Bullish: 1st black, 2nd white engulfs, 3rd closes higher
            let bull = candle_color(open[i - 2], close[i - 2]) == -1
                && candle_color(open[i - 1], close[i - 1]) == 1
                && close[i - 1] >= open[i - 2]
                && open[i - 1] <= close[i - 2]
                && close[i] > close[i - 1];
            // Bearish: 1st white, 2nd black engulfs, 3rd closes lower
            let bear = candle_color(open[i - 2], close[i - 2]) == 1
                && candle_color(open[i - 1], close[i - 1]) == -1
                && open[i - 1] >= close[i - 2]
                && close[i - 1] <= open[i - 2]
                && close[i] < close[i - 1];
            output[i] = (bull as i32) * 100 - (bear as i32) * 100;
        }
        Ok(output)
    }
}
