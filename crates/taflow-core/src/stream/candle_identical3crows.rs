//! Incremental Identical Three Crows candlestick recognition (CDLIDENTICAL3CROWS).
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
        let value = if self.candles.len() == 12 {
            let a = self.candles[10];
            let b = self.candles[11];
            let shadow0 = self.candles.iter().take(10).map(|x| x.range()).sum::<f64>() * 0.01;
            let shadow1 = self
                .candles
                .iter()
                .skip(1)
                .take(10)
                .map(|x| x.range())
                .sum::<f64>()
                * 0.01;
            let shadow2 = self.candles.iter().skip(2).map(|x| x.range()).sum::<f64>() * 0.01;
            let equal0 = self
                .candles
                .iter()
                .skip(5)
                .take(5)
                .map(|x| x.range())
                .sum::<f64>()
                * 0.01;
            let equal1 = self
                .candles
                .iter()
                .skip(6)
                .take(5)
                .map(|x| x.range())
                .sum::<f64>()
                * 0.01;
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
            None
        };
        if self.candles.len() == 12 {
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
/// Compute the candle identical three crows result for the supplied aligned series.
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
pub fn candle_identical_three_crows(
    open: &[f64],
    high: &[f64],
    low: &[f64],
    close: &[f64],
) -> TaResult<Vec<i32>> {
    let len = validate_ohlc(open, high, low, close)?;
    let mut output = vec![0i32; len];
    let lookback = SHADOW_VERY_SHORT.avg_period.max(EQUAL.avg_period) + 2;
    if len <= lookback {
        return Ok(output);
    }

    let mut shadow_sum = [0.0f64; 3];
    let mut equal_sum = [0.0f64; 3];
    let start = lookback;
    for k in 0..3 {
        let bar = start - 2 + k;
        if bar >= SHADOW_VERY_SHORT.avg_period {
            for j in (bar - SHADOW_VERY_SHORT.avg_period)..bar {
                shadow_sum[k] += cr(SHADOW_VERY_SHORT, open, high, low, close, j);
            }
        }
        if k < 2 && bar >= EQUAL.avg_period {
            for j in (bar - EQUAL.avg_period)..bar {
                equal_sum[k] += cr(EQUAL, open, high, low, close, j);
            }
        }
    }

    for i in start..len {
        output[i] = (candle_color(open[i-2], close[i-2]) == -1
            && candle_color(open[i-1], close[i-1]) == -1
            && candle_color(open[i], close[i]) == -1
            && close[i-1] < close[i-2] && close[i] < close[i-1]
            // Very short lower shadows
            && lower_shadow(open[i-2], low[i-2], close[i-2]) < ca(SHADOW_VERY_SHORT, shadow_sum[0], open, high, low, close, i-2)
            && lower_shadow(open[i-1], low[i-1], close[i-1]) < ca(SHADOW_VERY_SHORT, shadow_sum[1], open, high, low, close, i-1)
            && lower_shadow(open[i], low[i], close[i]) < ca(SHADOW_VERY_SHORT, shadow_sum[2], open, high, low, close, i)
            // Each opens equal to prior close
            && (open[i-1] - close[i-2]).abs() <= ca(EQUAL, equal_sum[0], open, high, low, close, i-2)
            && (open[i] - close[i-1]).abs() <= ca(EQUAL, equal_sum[1], open, high, low, close, i-1))
            as i32
            * -100;
        for k in 0..3 {
            let bar = i - 2 + k;
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
        for k in 0..2 {
            let bar = i - 2 + k;
            if EQUAL.avg_period > 0 && bar >= EQUAL.avg_period {
                equal_sum[k] += cr(EQUAL, open, high, low, close, bar)
                    - cr(EQUAL, open, high, low, close, bar - EQUAL.avg_period);
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
        let o: Vec<f64> = (0..45).map(|i| 100.0 + i as f64 * 0.2).collect();
        let h: Vec<f64> = o.iter().map(|x| x + 2.0).collect();
        let l: Vec<f64> = o.iter().map(|x| x - 2.0).collect();
        let c: Vec<f64> = o
            .iter()
            .enumerate()
            .map(|(i, x)| x + if i % 3 == 0 { -1.0 } else { 1.0 })
            .collect();
        let e = crate::stream::candle_identical_three_crows(&o, &h, &l, &c).unwrap();
        let mut s = CandleIdenticalThreeCrows::new();
        for ((((&o, &h), &l), &c), &e) in o.iter().zip(&h).zip(&l).zip(&c).zip(&e) {
            match s.append(o, h, l, c) {
                Some(v) => assert_eq!(v, e),
                None => assert_eq!(e, 0),
            }
        }
    }
}
