use super::pattern::*;
use crate::error::TaResult;
use std::collections::VecDeque;
/// Stateful CandleShortLine candle recognizer.
/// Consumes causal OHLC bars and returns an aligned pattern score.
pub struct CandleShortLine {
    b: VecDeque<f64>,
    s: VecDeque<f64>,
    bs: f64,
    ss: f64,
    value: Option<i32>,
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
/// Compute the candle short line result for the supplied aligned series.
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
pub fn candle_short_line(
    open: &[f64],
    high: &[f64],
    low: &[f64],
    close: &[f64],
) -> TaResult<Vec<i32>> {
    let len = validate_ohlc(open, high, low, close)?;
    let mut output = vec![0i32; len];
    let lookback = BODY_SHORT.avg_period.max(SHADOW_SHORT.avg_period);
    if len <= lookback {
        return Ok(output);
    }

    let mut body_sum = 0.0;
    let mut shadow_sum = 0.0;
    let start = lookback;
    for i in (start - BODY_SHORT.avg_period)..start {
        body_sum += cr_realbody(open, high, low, close, i);
    }
    for i in (start - SHADOW_SHORT.avg_period)..start {
        shadow_sum += cr_shadows(open, high, low, close, i);
    }

    for i in start..len {
        output[i] = (real_body(open[i], close[i])
            < ca_realbody(BODY_SHORT, body_sum, open, high, low, close, i)
            && upper_shadow(open[i], high[i], close[i])
                < ca_shadows(SHADOW_SHORT, shadow_sum, open, high, low, close, i)
            && lower_shadow(open[i], low[i], close[i])
                < ca_shadows(SHADOW_SHORT, shadow_sum, open, high, low, close, i))
            as i32
            * candle_color(open[i], close[i])
            * 100;
        body_sum += cr_realbody(open, high, low, close, i)
            - cr_realbody(open, high, low, close, i - BODY_SHORT.avg_period);
        shadow_sum += cr_shadows(open, high, low, close, i)
            - cr_shadows(open, high, low, close, i - SHADOW_SHORT.avg_period);
    }
    Ok(output)
}
impl CandleShortLine {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new() -> Self {
        Self {
            b: VecDeque::with_capacity(10),
            s: VecDeque::with_capacity(10),
            bs: 0.,
            ss: 0.,
            value: None,
        }
    }
    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, o: f64, h: f64, l: f64, c: f64) -> Option<i32> {
        let body = (c - o).abs();
        // SHADOW_SHORT range value, computed exactly like the batch cr_shadows.
        let sh = cr_shadows_scalar(o, h, l, c);
        let v = if self.b.len() == 10 {
            Some(
                (body < self.bs / 10.
                    && h - o.max(c) < self.ss / 20.
                    && o.min(c) - l < self.ss / 20.) as i32
                    * if c >= o { 100 } else { -100 },
            )
        } else {
            None
        };
        if self.b.len() == 10 {
            // Slide exactly like the batch loop: sum += cr(new) - cr(evicted).
            let old_b = self.b.pop_front().unwrap();
            let old_s = self.s.pop_front().unwrap();
            self.bs += body - old_b;
            self.ss += sh - old_s;
        } else {
            self.bs += body;
            self.ss += sh;
        }
        self.b.push_back(body);
        self.s.push_back(sh);
        self.value = v;
        v
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
        if !self.b.is_empty() {
            for i in 0..len {
                output.push(self.append(open[i], high[i], low[i], close[i]).unwrap_or(0));
            }
            return Ok(());
        }
        let scores = candle_short_line(open, high, low, close)?;
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
        self.b.clear();
        self.s.clear();
        self.bs = 0.0;
        self.ss = 0.0;
        self.value = None;
    }
}
