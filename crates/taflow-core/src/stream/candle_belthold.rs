//! Incremental Belt Hold candlestick recognition (CDLBELTHOLD).
use super::pattern::*;
use crate::error::TaResult;
use std::collections::VecDeque;
/// Stateful CandleBeltHold candle recognizer.
/// Consumes causal OHLC bars and returns an aligned pattern score.
pub struct CandleBeltHold {
    b: VecDeque<f64>,
    r: VecDeque<f64>,
    bs: f64,
    rs: f64,
    value: Option<i32>,
}
impl Default for CandleBeltHold {
    fn default() -> Self {
        Self::new()
    }
}
impl CandleBeltHold {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new() -> Self {
        Self {
            b: VecDeque::with_capacity(10),
            r: VecDeque::with_capacity(10),
            bs: 0.,
            rs: 0.,
            value: None,
        }
    }
    fn push(q: &mut VecDeque<f64>, s: &mut f64, v: f64) {
        if q.len() == 10 {
            // Slide exactly like the batch loop: sum += cr(new) - cr(evicted).
            let old = q.pop_front().unwrap();
            *s += v - old;
        } else {
            *s += v;
        }
        q.push_back(v);
    }
    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, o: f64, h: f64, l: f64, c: f64) -> Option<i32> {
        let body = (c - o).abs();
        let range = h - l;
        let upper = h - o.max(c);
        let lower = o.min(c) - l;
        let v = if self.b.len() == 10 {
            let long = body > self.bs / 10.0;
            let lim = ca_highlow_scalar(SHADOW_VERY_SHORT, self.rs, h, l);
            Some(if long && c >= o && lower < lim {
                100
            } else if long && c < o && upper < lim {
                -100
            } else {
                0
            })
        } else {
            None
        };
        Self::push(&mut self.b, &mut self.bs, body);
        Self::push(&mut self.r, &mut self.rs, range);
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
        let scores = candle_belt_hold(open, high, low, close)?;
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
        self.r.clear();
        self.bs = 0.0;
        self.rs = 0.0;
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
/// Compute the candle belt hold result for the supplied aligned series.
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
pub fn candle_belt_hold(
    open: &[f64],
    high: &[f64],
    low: &[f64],
    close: &[f64],
) -> TaResult<Vec<i32>> {
    let len = validate_ohlc(open, high, low, close)?;
    let mut output = vec![0i32; len];
    let lookback = BODY_LONG.avg_period.max(SHADOW_VERY_SHORT.avg_period);
    if len <= lookback {
        return Ok(output);
    }

    let mut body_sum = 0.0;
    let mut shadow_sum = 0.0;
    let start = lookback;
    for i in (start - BODY_LONG.avg_period)..start {
        body_sum += cr_realbody(open, high, low, close, i);
    }
    for i in (start - SHADOW_VERY_SHORT.avg_period)..start {
        shadow_sum += cr_highlow(open, high, low, close, i);
    }

    for i in start..len {
        let long_body = real_body(open[i], close[i])
            > ca_realbody(BODY_LONG, body_sum, open, high, low, close, i);
        let bull = long_body
            && candle_color(open[i], close[i]) == 1
            && lower_shadow(open[i], low[i], close[i])
                < ca_highlow(SHADOW_VERY_SHORT, shadow_sum, open, high, low, close, i);
        let bear = long_body
            && candle_color(open[i], close[i]) == -1
            && upper_shadow(open[i], high[i], close[i])
                < ca_highlow(SHADOW_VERY_SHORT, shadow_sum, open, high, low, close, i);
        output[i] = (bull as i32) * 100 - (bear as i32) * 100;
        body_sum += cr_realbody(open, high, low, close, i)
            - cr_realbody(open, high, low, close, i - BODY_LONG.avg_period);
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
        let o: Vec<f64> = (0..45).map(|i| 100.0 + i as f64 * 0.2).collect();
        let h: Vec<f64> = o.iter().map(|x| x + 2.0).collect();
        let l: Vec<f64> = o.iter().map(|x| x - 2.0).collect();
        let c: Vec<f64> = o
            .iter()
            .enumerate()
            .map(|(i, x)| x + if i % 3 == 0 { -1.0 } else { 1.0 })
            .collect();
        let e = crate::stream::candle_belt_hold(&o, &h, &l, &c).unwrap();
        let mut s = CandleBeltHold::new();
        for ((((&o, &h), &l), &c), &e) in o.iter().zip(&h).zip(&l).zip(&c).zip(&e) {
            assert_eq!(s.append(o, h, l, c).unwrap_or(0), e)
        }
    }
}
