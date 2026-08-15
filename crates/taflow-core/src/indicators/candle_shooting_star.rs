//! Incremental Shooting Star candlestick recognition (CDLSHOOTINGSTAR).
use crate::error::TaResult;
use crate::stream::pattern::*;
#[derive(Clone, Copy, Default)]
struct Candle {
    o: f64,
    h: f64,
    l: f64,
    c: f64,
}
impl Candle {
    fn body(self) -> f64 {
        (self.c - self.o).abs()
    }
    fn upper(self) -> f64 {
        self.h - self.o.max(self.c)
    }
    fn lower(self) -> f64 {
        self.o.min(self.c) - self.l
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
/// Compute the candle shooting star result for the supplied aligned series.
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
pub struct CandleShootingStar {
    candles: [Candle; 11],
    head: usize,
    len: usize,
    body_sum: f64,
    shadow_vs_sum: f64,
    value: Option<i32>,
}
impl Default for CandleShootingStar {
    fn default() -> Self {
        Self::new()
    }
}
impl CandleShootingStar {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new() -> Self {
        Self {
            candles: [Candle::default(); 11],
            head: 0,
            len: 0,
            body_sum: 0.0,
            shadow_vs_sum: 0.0,
            value: None,
        }
    }
    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, o: f64, h: f64, l: f64, c: f64) -> Option<i32> {
        let cur = Candle { o, h, l, c };
        let value = if self.len == 11 {
            let prev = self.candles[(self.head + 10) % 11];
            let evicted = self.candles[(self.head + 1) % 11];
            let body = ca_realbody_scalar(BODY_SHORT, self.body_sum, o, c);
            let vs = ca_highlow_scalar(SHADOW_VERY_SHORT, self.shadow_vs_sum, h, l);
            // Slide sums exactly like the batch loop: sum += cr(bar) - cr(bar - 10).
            self.body_sum += cr_realbody_scalar(o, c) - cr_realbody_scalar(evicted.o, evicted.c);
            self.shadow_vs_sum += cr_highlow_scalar(h, l) - cr_highlow_scalar(evicted.h, evicted.l);
            Some(
                (cur.body() < body
                    && cur.upper() > cur.body()
                    && cur.lower() < vs
                    && cur.o.min(cur.c) > prev.o.max(prev.c)) as i32
                    * -100,
            )
        } else {
            // Warm-up: seed the sums exactly like the batch prologue.
            let i = self.len;
            if (1..11).contains(&i) {
                self.body_sum += cr_realbody_scalar(o, c);
                self.shadow_vs_sum += cr_highlow_scalar(h, l);
            }
            None
        };
        if self.len == 11 {
            self.candles[self.head] = cur;
            self.head = (self.head + 1) % 11;
        } else {
            self.candles[(self.head + self.len) % 11] = cur;
            self.len += 1;
        }
        self.value = value;
        value
    }
    /// Bulk-append aligned OHLC slices, pushing one score per bar into `output`.
    ///
    /// From a pristine state this runs directly over the slices and rebuilds
    /// the bounded candle ring once after the loop. A non-pristine state falls
    /// back to the per-bar loop. Either route is bit-identical to calling
    /// `append` once per bar (warm-up `None` becomes `0`, matching the batch
    /// prologue).
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
        const LOOKBACK: usize = 11;
        if self.len != 0 || len <= LOOKBACK {
            output.reserve(len);
            for i in 0..len {
                output.push(self.append(open[i], high[i], low[i], close[i]).unwrap_or(0));
            }
            return Ok(());
        }

        let start = output.len();
        output.resize(start + len, 0);
        let mut body_sum = (1..11).fold(0.0, |sum, i| sum + cr_realbody_scalar(open[i], close[i]));
        let mut shadow_sum = (1..11).fold(0.0, |sum, i| sum + cr_highlow_scalar(high[i], low[i]));
        for i in LOOKBACK..len {
            let body = (close[i] - open[i]).abs();
            output[start + i] = ((body
                < ca_realbody_scalar(BODY_SHORT, body_sum, open[i], close[i])
                && high[i] - open[i].max(close[i]) > body
                && open[i].min(close[i]) - low[i]
                    < ca_highlow_scalar(SHADOW_VERY_SHORT, shadow_sum, high[i], low[i])
                && open[i].min(close[i]) > open[i - 1].max(close[i - 1]))
                as i32)
                * -100;
            body_sum += cr_realbody_scalar(open[i], close[i])
                - cr_realbody_scalar(open[i - 10], close[i - 10]);
            shadow_sum +=
                cr_highlow_scalar(high[i], low[i]) - cr_highlow_scalar(high[i - 10], low[i - 10]);
        }

        self.body_sum = body_sum;
        self.shadow_vs_sum = shadow_sum;
        for (slot, i) in (len - LOOKBACK..len).enumerate() {
            self.candles[slot] = Candle {
                o: open[i],
                h: high[i],
                l: low[i],
                c: close[i],
            };
        }
        self.head = 0;
        self.len = LOOKBACK;
        self.value = Some(output[start + len - 1]);
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
        self.head = 0;
        self.len = 0;
        self.body_sum = 0.0;
        self.shadow_vs_sum = 0.0;
        self.value = None;
    }
}
