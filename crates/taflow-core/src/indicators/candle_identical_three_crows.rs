//! Incremental Identical Three Crows candlestick recognition (CDLIDENTICAL3CROWS).
use crate::error::TaResult;
use crate::stream::pattern::*;
use std::collections::VecDeque;
#[derive(Clone, Copy)]
struct Candle {
    o: f64,
    h: f64,
    l: f64,
    c: f64,
}
impl Candle {
    fn lower(self) -> f64 {
        self.o.min(self.c) - self.l
    }
    fn color(self) -> i32 {
        if self.c >= self.o {
            1
        } else {
            -1
        }
    }
}
/// Stateful CandleIdenticalThreeCrows candle recognizer.
/// Consumes causal OHLC bars and returns an aligned pattern score.
pub struct CandleIdenticalThreeCrows {
    candles: VecDeque<Candle>,
    shadow_sum: [f64; 3],
    equal_sum: [f64; 2],
    value: Option<i32>,
}
impl Default for CandleIdenticalThreeCrows {
    fn default() -> Self {
        Self::new()
    }
}
impl CandleIdenticalThreeCrows {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new() -> Self {
        Self {
            candles: VecDeque::with_capacity(12),
            shadow_sum: [0.0; 3],
            equal_sum: [0.0; 2],
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
        // Deque holds bars i-12..=i-1; bar j maps to index 12 - (i - j).
        let value = if self.candles.len() == 12 {
            let a = self.candles[10]; // bar i-2
            let b = self.candles[11]; // bar i-1
            let shadow0 = ca_highlow_scalar(SHADOW_VERY_SHORT, self.shadow_sum[0], a.h, a.l);
            let shadow1 = ca_highlow_scalar(SHADOW_VERY_SHORT, self.shadow_sum[1], b.h, b.l);
            let shadow2 = ca_highlow_scalar(SHADOW_VERY_SHORT, self.shadow_sum[2], h, l);
            let equal0 = ca_highlow_scalar(EQUAL, self.equal_sum[0], a.h, a.l);
            let equal1 = ca_highlow_scalar(EQUAL, self.equal_sum[1], b.h, b.l);
            // Slide sums exactly like the batch loop: sum += cr(bar) - cr(bar - period).
            self.shadow_sum[0] += cr_highlow_scalar(a.h, a.l)
                - cr_highlow_scalar(self.candles[0].h, self.candles[0].l);
            self.shadow_sum[1] += cr_highlow_scalar(b.h, b.l)
                - cr_highlow_scalar(self.candles[1].h, self.candles[1].l);
            self.shadow_sum[2] +=
                cr_highlow_scalar(h, l) - cr_highlow_scalar(self.candles[2].h, self.candles[2].l);
            self.equal_sum[0] += cr_highlow_scalar(a.h, a.l)
                - cr_highlow_scalar(self.candles[5].h, self.candles[5].l);
            self.equal_sum[1] += cr_highlow_scalar(b.h, b.l)
                - cr_highlow_scalar(self.candles[6].h, self.candles[6].l);
            Some(
                (a.color() == -1
                    && b.color() == -1
                    && cur.color() == -1
                    && b.c < a.c
                    && cur.c < b.c
                    && a.lower() < shadow0
                    && b.lower() < shadow1
                    && cur.lower() < shadow2
                    && (b.o - a.c).abs() <= equal0
                    && (cur.o - b.c).abs() <= equal1) as i32
                    * -100,
            )
        } else {
            // Warm-up: seed the sums exactly like the batch prologue.
            let i = self.candles.len();
            for k in 0..3 {
                if i >= k && i < 10 + k {
                    self.shadow_sum[k] += cr_highlow_scalar(h, l);
                }
            }
            if (5..10).contains(&i) {
                self.equal_sum[0] += cr_highlow_scalar(h, l);
            }
            if (6..11).contains(&i) {
                self.equal_sum[1] += cr_highlow_scalar(h, l);
            }
            None
        };
        if self.candles.len() == 12 {
            self.candles.pop_front();
        }
        self.candles.push_back(cur);
        self.value = value;
        value
    }
    /// Bulk-append aligned OHLC slices, pushing one score per bar into `output`.
    ///
    /// From a pristine state this runs a direct slice kernel and reconstructs
    /// the bounded trailing state without replaying the input. A non-pristine
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
        const LOOKBACK: usize = 12;
        if !self.candles.is_empty() || len <= LOOKBACK {
            for i in 0..len {
                output.push(self.append(open[i], high[i], low[i], close[i]).unwrap_or(0));
            }
            return Ok(());
        }
        let start = output.len();
        output.resize(start + len, 0);
        let range = |i: usize| cr_highlow_scalar(high[i], low[i]);
        let mut shadow_sum = [0.0; 3];
        for k in 0..3 {
            shadow_sum[k] = (k..10 + k).fold(0.0, |sum, i| sum + range(i));
        }
        let mut equal_sum = [
            (5..10).fold(0.0, |sum, i| sum + range(i)),
            (6..11).fold(0.0, |sum, i| sum + range(i)),
        ];
        for ((((opens, highs), lows), closes), out) in open
            .windows(13)
            .zip(high.windows(13))
            .zip(low.windows(13))
            .zip(close.windows(13))
            .zip(output[start + LOOKBACK..].iter_mut())
        {
            let a = Candle {
                o: opens[10],
                h: highs[10],
                l: lows[10],
                c: closes[10],
            };
            let b = Candle {
                o: opens[11],
                h: highs[11],
                l: lows[11],
                c: closes[11],
            };
            let cur = Candle {
                o: opens[12],
                h: highs[12],
                l: lows[12],
                c: closes[12],
            };
            let shadow0 = ca_highlow_scalar(SHADOW_VERY_SHORT, shadow_sum[0], a.h, a.l);
            let shadow1 = ca_highlow_scalar(SHADOW_VERY_SHORT, shadow_sum[1], b.h, b.l);
            let shadow2 = ca_highlow_scalar(SHADOW_VERY_SHORT, shadow_sum[2], cur.h, cur.l);
            let equal0 = ca_highlow_scalar(EQUAL, equal_sum[0], a.h, a.l);
            let equal1 = ca_highlow_scalar(EQUAL, equal_sum[1], b.h, b.l);
            *out = (a.color() == -1
                && b.color() == -1
                && cur.color() == -1
                && b.c < a.c
                && cur.c < b.c
                && a.lower() < shadow0
                && b.lower() < shadow1
                && cur.lower() < shadow2
                && (b.o - a.c).abs() <= equal0
                && (cur.o - b.c).abs() <= equal1) as i32
                * -100;
            shadow_sum[0] += cr_highlow_scalar(a.h, a.l) - cr_highlow_scalar(highs[0], lows[0]);
            shadow_sum[1] += cr_highlow_scalar(b.h, b.l) - cr_highlow_scalar(highs[1], lows[1]);
            shadow_sum[2] += cr_highlow_scalar(cur.h, cur.l) - cr_highlow_scalar(highs[2], lows[2]);
            equal_sum[0] += cr_highlow_scalar(a.h, a.l) - cr_highlow_scalar(highs[5], lows[5]);
            equal_sum[1] += cr_highlow_scalar(b.h, b.l) - cr_highlow_scalar(highs[6], lows[6]);
        }
        self.shadow_sum = shadow_sum;
        self.equal_sum = equal_sum;
        self.candles.extend((len - LOOKBACK..len).map(|i| Candle {
            o: open[i],
            h: high[i],
            l: low[i],
            c: close[i],
        }));
        self.value = output.last().copied();
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
        self.shadow_sum = [0.0; 3];
        self.equal_sum = [0.0; 2];
        self.value = None;
    }
}
