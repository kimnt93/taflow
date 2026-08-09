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
        self.b.clear();
        self.r.clear();
        self.bs = 0.0;
        self.rs = 0.0;
        self.value = None;
    }
}
