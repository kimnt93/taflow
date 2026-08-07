//! Incremental Concealing Baby Swallow candlestick recognition (CDLCONCEALBABYSWALL).
use super::pattern::*;
use crate::error::TaResult;
use std::collections::VecDeque;
#[derive(Clone, Copy)]
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
    fn range(self) -> f64 {
        self.h - self.l
    }
    fn upper(self) -> f64 {
        self.h - self.o.max(self.c)
    }
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
/// Stateful CandleConcealBabySwall candle recognizer.
/// Consumes causal OHLC bars and returns an aligned pattern score.
pub struct CandleConcealBabySwall {
    candles: VecDeque<Candle>,
    value: Option<i32>,
}
impl Default for CandleConcealBabySwall {
    fn default() -> Self {
        Self::new()
    }
}
impl CandleConcealBabySwall {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new() -> Self {
        Self {
            candles: VecDeque::with_capacity(13),
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
        let value = if self.candles.len() == 13 {
            let a = self.candles[10];
            let b = self.candles[11];
            let cnd = self.candles[12];
            let s0 = self.candles.iter().take(10).map(|x| x.range()).sum::<f64>() * 0.01;
            let s1 = self
                .candles
                .iter()
                .skip(1)
                .take(10)
                .map(|x| x.range())
                .sum::<f64>()
                * 0.01;
            Some(
                (a.color() == -1
                    && b.color() == -1
                    && cnd.color() == -1
                    && cur.color() == -1
                    && a.upper() < s0
                    && a.lower() < s0
                    && b.upper() < s1
                    && b.lower() < s1
                    && cnd.o.max(cnd.c) < b.o.min(b.c)
                    && cnd.h > b.c
                    && cur.o >= cnd.h
                    && cur.c <= cnd.l) as i32
                    * 100,
            )
        } else {
            None
        };
        if self.candles.len() == 13 {
            self.candles.pop_front();
        }
        self.candles.push_back(cur);
        self.value = value;
        value
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
        *self = Self::new();
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
/// Compute the candle conceal baby swall result for the supplied aligned series.
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
pub fn candle_conceal_baby_swall(
    open: &[f64],
    high: &[f64],
    low: &[f64],
    close: &[f64],
) -> TaResult<Vec<i32>> {
    let len = validate_ohlc(open, high, low, close)?;
    let mut output = vec![0i32; len];
    let lookback = SHADOW_VERY_SHORT.avg_period + 3;
    if len <= lookback {
        return Ok(output);
    }

    let mut shadow_sum = [0.0f64; 4];
    let start = lookback;
    for k in 0..4 {
        let bar = start - 3 + k;
        if bar >= SHADOW_VERY_SHORT.avg_period {
            for j in (bar - SHADOW_VERY_SHORT.avg_period)..bar {
                shadow_sum[k] += cr(SHADOW_VERY_SHORT, open, high, low, close, j);
            }
        }
    }

    for i in start..len {
        output[i] = (candle_color(open[i-3], close[i-3]) == -1
            && candle_color(open[i-2], close[i-2]) == -1
            && candle_color(open[i-1], close[i-1]) == -1
            && candle_color(open[i], close[i]) == -1
            // 1st and 2nd: marubozu (very short shadows)
            && upper_shadow(open[i-3], high[i-3], close[i-3]) < ca(SHADOW_VERY_SHORT, shadow_sum[0], open, high, low, close, i-3)
            && lower_shadow(open[i-3], low[i-3], close[i-3]) < ca(SHADOW_VERY_SHORT, shadow_sum[0], open, high, low, close, i-3)
            && upper_shadow(open[i-2], high[i-2], close[i-2]) < ca(SHADOW_VERY_SHORT, shadow_sum[1], open, high, low, close, i-2)
            && lower_shadow(open[i-2], low[i-2], close[i-2]) < ca(SHADOW_VERY_SHORT, shadow_sum[1], open, high, low, close, i-2)
            // 3rd: gaps down, upper shadow into 2nd body
            && real_body_gap_down(open, close, i-1, i-2)
            && high[i-1] > close[i-2]
            // 4th: engulfs 3rd including shadows
            && open[i] >= high[i-1] && close[i] <= low[i-1]) as i32
            * 100;
        for k in 0..4 {
            let bar = i - 3 + k;
            if SHADOW_VERY_SHORT.avg_period > 0 && bar >= SHADOW_VERY_SHORT.avg_period {
                shadow_sum[k] += cr(SHADOW_VERY_SHORT, open, high, low, close, bar)
                    - cr(
                        SHADOW_VERY_SHORT,
                        open,
                        high,
                        low,
                        close,
                        bar - SHADOW_VERY_SHORT.avg_period,
                    );
            }
        }
    }
    Ok(output)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn matches_batch() {
        let o: Vec<f64> = (0..48).map(|i| 100.0 + i as f64 * 0.2).collect();
        let h: Vec<f64> = o.iter().map(|x| x + 2.0).collect();
        let l: Vec<f64> = o.iter().map(|x| x - 2.0).collect();
        let c: Vec<f64> = o
            .iter()
            .enumerate()
            .map(|(i, x)| x + if i % 3 == 0 { -1.0 } else { 1.0 })
            .collect();
        let e = crate::stream::candle_conceal_baby_swall(&o, &h, &l, &c).unwrap();
        let mut s = CandleConcealBabySwall::new();
        for ((((&o, &h), &l), &c), &e) in o.iter().zip(&h).zip(&l).zip(&c).zip(&e) {
            match s.append(o, h, l, c) {
                Some(v) => assert_eq!(v, e),
                None => assert_eq!(e, 0),
            }
        }
    }
}
