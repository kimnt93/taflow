//! Incremental Long Line candlestick recognition (CDLLONGLINE).

use std::collections::VecDeque;

use super::pattern::*;
use crate::error::TaResult;
/// Incremental CDLLONGLINE state using TA-Lib's long-body and short-shadow averages.
/// Persistent Rust state or aligned output type for `CandleLongLine`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct CandleLongLine {
    bodies: VecDeque<f64>,
    body_sum: f64,
    shadows: VecDeque<f64>,
    shadow_sum: f64,
    value: Option<i32>,
}

impl Default for CandleLongLine {
    fn default() -> Self {
        Self::new()
    }
}

impl CandleLongLine {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new() -> Self {
        Self {
            bodies: VecDeque::with_capacity(10),
            body_sum: 0.0,
            shadows: VecDeque::with_capacity(10),
            shadow_sum: 0.0,
            value: None,
        }
    }
    fn push(window: &mut VecDeque<f64>, sum: &mut f64, value: f64) {
        if window.len() == 10 {
            // Slide exactly like the batch loop: sum += cr(new) - cr(evicted).
            let old = window.pop_front().expect("window is full");
            *sum += value - old;
        } else {
            *sum += value;
        }
        window.push_back(value);
    }
    /// Appends OHLC data and returns a signed long-line signal after warmup.
    pub fn append(&mut self, open: f64, high: f64, low: f64, close: f64) -> Option<i32> {
        let body = (close - open).abs();
        let upper = high - open.max(close);
        let lower = open.min(close) - low;
        let value = if self.bodies.len() == 10 && self.shadows.len() == 10 {
            Some(
                (body > self.body_sum / 10.0
                    && upper < self.shadow_sum / 20.0
                    && lower < self.shadow_sum / 20.0) as i32
                    * if close >= open { 100 } else { -100 },
            )
        } else {
            None
        };
        Self::push(&mut self.bodies, &mut self.body_sum, body);
        // SHADOW_SHORT range value, computed exactly like the batch cr_shadows.
        Self::push(
            &mut self.shadows,
            &mut self.shadow_sum,
            cr_shadows_scalar(open, high, low, close),
        );
        self.value = value;
        value
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
        if !self.bodies.is_empty() {
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
        self.bodies.clear();
        self.body_sum = 0.0;
        self.shadows.clear();
        self.shadow_sum = 0.0;
        self.value = None;
    }
}
