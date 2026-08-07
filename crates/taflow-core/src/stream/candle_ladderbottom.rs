//! Incremental Ladder Bottom candlestick recognition (CDLLADDERBOTTOM).
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
    fn color(self) -> i32 {
        if self.c >= self.o {
            1
        } else {
            -1
        }
    }
}
/// Stateful CandleLadderBottom candle recognizer.
/// Consumes causal OHLC bars and returns an aligned pattern score.
pub struct CandleLadderBottom {
    candles: VecDeque<Candle>,
    value: Option<i32>,
}
impl Default for CandleLadderBottom {
    fn default() -> Self {
        Self::new()
    }
}
impl CandleLadderBottom {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new() -> Self {
        Self {
            candles: VecDeque::with_capacity(14),
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
        let value = if self.candles.len() == 14 {
            let a = self.candles[10];
            let b = self.candles[11];
            let cnd = self.candles[12];
            let d = self.candles[13];
            let shadow = self
                .candles
                .iter()
                .skip(3)
                .take(SHADOW_VERY_SHORT.avg_period)
                .map(|x| x.range())
                .sum::<f64>()
                * SHADOW_VERY_SHORT.factor
                / SHADOW_VERY_SHORT.avg_period as f64;
            Some(
                (a.color() == -1
                    && b.color() == -1
                    && cnd.color() == -1
                    && d.color() == -1
                    && a.o > b.o
                    && b.o > cnd.o
                    && a.c > b.c
                    && b.c > cnd.c
                    && d.color() == -1
                    && d.upper() > shadow
                    && cur.color() == 1
                    && cur.o > d.o
                    && cur.c > d.h) as i32
                    * 100,
            )
        } else {
            None
        };
        if self.candles.len() == 14 {
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
/// Compute the candle ladder bottom result for the supplied aligned series.
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
pub fn candle_ladder_bottom(
    open: &[f64],
    high: &[f64],
    low: &[f64],
    close: &[f64],
) -> TaResult<Vec<i32>> {
    let len = validate_ohlc(open, high, low, close)?;
    let mut output = vec![0i32; len];
    let lookback = SHADOW_VERY_SHORT.avg_period + 4;
    if len <= lookback {
        return Ok(output);
    }

    let mut shadow_sum = 0.0;
    let start = lookback;
    for i in (start - 1 - SHADOW_VERY_SHORT.avg_period)..(start - 1) {
        shadow_sum += cr(SHADOW_VERY_SHORT, open, high, low, close, i);
    }

    for i in start..len {
        output[i] = (candle_color(open[i-4], close[i-4]) == -1
            && candle_color(open[i-3], close[i-3]) == -1
            && candle_color(open[i-2], close[i-2]) == -1
            && candle_color(open[i-1], close[i-1]) == -1
            && open[i-4] > open[i-3] && open[i-3] > open[i-2]
            && close[i-4] > close[i-3] && close[i-3] > close[i-2]
            // 4th: upper shadow exceeds very short
            && candle_color(open[i-1], close[i-1]) == -1
            && upper_shadow(open[i-1], high[i-1], close[i-1]) > ca(SHADOW_VERY_SHORT, shadow_sum, open, high, low, close, i-1)
            // 5th: white, opens above 4th open, closes above 4th high
            && candle_color(open[i], close[i]) == 1
            && open[i] > open[i-1]
            && close[i] > high[i-1]) as i32
            * 100;
        shadow_sum += cr(SHADOW_VERY_SHORT, open, high, low, close, i - 1)
            - cr(
                SHADOW_VERY_SHORT,
                open,
                high,
                low,
                close,
                i - 1 - SHADOW_VERY_SHORT.avg_period,
            );
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
        let e = crate::stream::candle_ladder_bottom(&o, &h, &l, &c).unwrap();
        let mut s = CandleLadderBottom::new();
        for ((((&o, &h), &l), &c), &e) in o.iter().zip(&h).zip(&l).zip(&c).zip(&e) {
            match s.append(o, h, l, c) {
                Some(v) => assert_eq!(v, e),
                None => assert_eq!(e, 0),
            }
        }
    }
}
