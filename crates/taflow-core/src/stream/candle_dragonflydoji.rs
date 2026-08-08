//! Incremental Dragonfly Doji candlestick recognition (CDLDRAGONFLYDOJI).

use std::collections::VecDeque;

use super::pattern::*;
use crate::error::TaResult;
/// Incremental CDLDRAGONFLYDOJI state using TA-Lib's two ten-bar range averages.
/// Persistent Rust state or aligned output type for `CandleDragonflyDoji`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct CandleDragonflyDoji {
    ranges: VecDeque<f64>,
    sum: f64,
    value: Option<i32>,
}
impl Default for CandleDragonflyDoji {
    fn default() -> Self {
        Self::new()
    }
}
impl CandleDragonflyDoji {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new() -> Self {
        Self {
            ranges: VecDeque::with_capacity(10),
            sum: 0.0,
            value: None,
        }
    }
    /// Appends OHLC data and returns +100 for a dragonfly doji after warmup.
    pub fn append(&mut self, open: f64, high: f64, low: f64, close: f64) -> Option<i32> {
        let range = high - low;
        let body = (close - open).abs();
        let output = if self.ranges.len() == 10 {
            let shadow_limit = ca_highlow_scalar(BODY_DOJI, self.sum, high, low);
            Some(
                (body <= shadow_limit
                    && high - open.max(close) < shadow_limit
                    && open.min(close) - low > shadow_limit) as i32
                    * 100,
            )
        } else {
            None
        };
        if self.ranges.len() == 10 {
            // Slide exactly like the batch loop: sum += cr(new) - cr(evicted).
            let old = self.ranges.pop_front().expect("window is full");
            self.sum += range - old;
        } else {
            self.sum += range;
        }
        self.ranges.push_back(range);
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
        if !self.ranges.is_empty() {
            for i in 0..len {
                output.push(self.append(open[i], high[i], low[i], close[i]).unwrap_or(0));
            }
            return Ok(());
        }
        let scores = candle_dragonfly_doji(open, high, low, close)?;
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
        self.ranges.clear();
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
/// Compute the candle dragonfly doji result for the supplied aligned series.
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
pub fn candle_dragonfly_doji(
    open: &[f64],
    high: &[f64],
    low: &[f64],
    close: &[f64],
) -> TaResult<Vec<i32>> {
    let len = validate_ohlc(open, high, low, close)?;
    let mut output = vec![0i32; len];
    let lookback = BODY_DOJI.avg_period.max(SHADOW_VERY_SHORT.avg_period);
    if len <= lookback {
        return Ok(output);
    }

    let mut body_sum = 0.0;
    let mut shadow_sum = 0.0;
    let start = lookback;
    for i in (start - BODY_DOJI.avg_period)..start {
        body_sum += cr_highlow(open, high, low, close, i);
    }
    for i in (start - SHADOW_VERY_SHORT.avg_period)..start {
        shadow_sum += cr_highlow(open, high, low, close, i);
    }

    for i in start..len {
        output[i] = (real_body(open[i], close[i])
            <= ca_highlow(BODY_DOJI, body_sum, open, high, low, close, i)
            && upper_shadow(open[i], high[i], close[i])
                < ca_highlow(SHADOW_VERY_SHORT, shadow_sum, open, high, low, close, i)
            && lower_shadow(open[i], low[i], close[i])
                > ca_highlow(SHADOW_VERY_SHORT, shadow_sum, open, high, low, close, i))
            as i32
            * 100;
        body_sum += cr_highlow(open, high, low, close, i)
            - cr_highlow(open, high, low, close, i - BODY_DOJI.avg_period);
        shadow_sum += cr_highlow(open, high, low, close, i)
            - cr_highlow(open, high, low, close, i - SHADOW_VERY_SHORT.avg_period);
    }
    Ok(output)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn matches_batch() {
        let open: Vec<f64> = (0..45).map(|i| 100.0 + i as f64 * 0.2).collect();
        let high: Vec<f64> = open.iter().map(|x| x + 2.0).collect();
        let low: Vec<f64> = open.iter().map(|x| x - 2.0).collect();
        let close: Vec<f64> = open
            .iter()
            .enumerate()
            .map(|(i, x)| x + if i % 3 == 0 { 0.1 } else { 1.0 })
            .collect();
        let expected = crate::stream::candle_dragonfly_doji(&open, &high, &low, &close).unwrap();
        let mut state = CandleDragonflyDoji::new();
        for (((&o, &h), &l), (&c, &expected)) in open
            .iter()
            .zip(&high)
            .zip(&low)
            .zip(close.iter().zip(&expected))
        {
            match state.append(o, h, l, c) {
                Some(value) => assert_eq!(value, expected),
                None => assert_eq!(expected, 0),
            }
        }
    }
}
