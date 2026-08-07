//! Incremental Three Line Strike recognition (CDL3LINESTRIKE).
use super::pattern::*;
use crate::error::TaResult;
use std::collections::VecDeque;
#[derive(Clone, Copy)]
struct Candle {
    open: f64,
    high: f64,
    low: f64,
    close: f64,
}
impl Candle {
    fn range(self) -> f64 {
        self.high - self.low
    }
    fn color(self) -> i32 {
        if self.close >= self.open {
            1
        } else {
            -1
        }
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
/// Compute the candle three line strike result for the supplied aligned series.
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
pub fn candle_three_line_strike(
    open: &[f64],
    high: &[f64],
    low: &[f64],
    close: &[f64],
) -> TaResult<Vec<i32>> {
    let len = validate_ohlc(open, high, low, close)?;
    let mut output = vec![0i32; len];
    let lookback = NEAR.avg_period + 3;
    if len <= lookback {
        return Ok(output);
    }

    let mut near_sum = [0.0f64; 4];
    let start = lookback;
    // Init near sums for bars i-3 and i-2
    for k in [2usize, 3] {
        let bar = start - k;
        if bar >= NEAR.avg_period {
            for j in (bar - NEAR.avg_period)..bar {
                near_sum[k] += cr(NEAR, open, high, low, close, j);
            }
        }
    }

    for i in start..len {
        let c3 = candle_color(open[i - 3], close[i - 3]);
        let c2 = candle_color(open[i - 2], close[i - 2]);
        let c1 = candle_color(open[i - 1], close[i - 1]);
        let c0 = candle_color(open[i], close[i]);

        if c3 == c2 && c2 == c1 && c0 != c1 {
            // Three same-color, 4th opposite
            let progressive = if c3 == 1 {
                close[i - 2] > close[i - 3] && close[i - 1] > close[i - 2]
            } else {
                close[i - 2] < close[i - 3] && close[i - 1] < close[i - 2]
            };
            let opens_near = if c3 == 1 {
                open[i - 2] >= open[i - 3].min(close[i - 3])
                    && open[i - 2]
                        <= close[i - 3] + ca(NEAR, near_sum[3], open, high, low, close, i - 3)
                    && open[i - 1] >= open[i - 2].min(close[i - 2])
                    && open[i - 1]
                        <= close[i - 2] + ca(NEAR, near_sum[2], open, high, low, close, i - 2)
            } else {
                open[i - 2] <= open[i - 3].max(close[i - 3])
                    && open[i - 2]
                        >= close[i - 3] - ca(NEAR, near_sum[3], open, high, low, close, i - 3)
                    && open[i - 1] <= open[i - 2].max(close[i - 2])
                    && open[i - 1]
                        >= close[i - 2] - ca(NEAR, near_sum[2], open, high, low, close, i - 2)
            };
            let strike = if c3 == 1 {
                open[i] >= close[i - 1] && close[i] <= open[i - 3]
            } else {
                open[i] <= close[i - 1] && close[i] >= open[i - 3]
            };
            output[i] = (progressive && opens_near && strike) as i32 * c3 * 100;
        }
        // Update near sums
        for k in [2usize, 3] {
            let bar = i - k;
            if bar >= NEAR.avg_period && NEAR.avg_period > 0 {
                near_sum[k] += cr(NEAR, open, high, low, close, bar)
                    - cr(NEAR, open, high, low, close, bar - NEAR.avg_period);
            }
        }
    }
    Ok(output)
}
/// Incremental CDL3LINESTRIKE state.
/// Persistent Rust state or aligned output type for `CandleThreeLineStrike`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct CandleThreeLineStrike {
    candles: VecDeque<Candle>,
    value: Option<i32>,
}
impl Default for CandleThreeLineStrike {
    fn default() -> Self {
        Self::new()
    }
}
impl CandleThreeLineStrike {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new() -> Self {
        Self {
            candles: VecDeque::with_capacity(8),
            value: None,
        }
    }
    fn near(&self, start: usize) -> f64 {
        self.candles
            .iter()
            .skip(start)
            .take(5)
            .map(|c| c.range())
            .sum::<f64>()
            * 0.04
    }
    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, open: f64, high: f64, low: f64, close: f64) -> Option<i32> {
        let current = Candle {
            open,
            high,
            low,
            close,
        };
        let output = if self.candles.len() == 8 {
            let a = self.candles[5];
            let b = self.candles[6];
            let c = self.candles[7];
            let color = a.color();
            let same = color == b.color() && color == c.color() && current.color() != color;
            let progressive = if color == 1 {
                b.close > a.close && c.close > b.close
            } else {
                b.close < a.close && c.close < b.close
            };
            let opens = if color == 1 {
                b.open >= a.open.min(a.close)
                    && b.open <= a.close + self.near(0)
                    && c.open >= b.open.min(b.close)
                    && c.open <= b.close + self.near(1)
            } else {
                b.open <= a.open.max(a.close)
                    && b.open >= a.close - self.near(0)
                    && c.open <= b.open.max(b.close)
                    && c.open >= b.close - self.near(1)
            };
            let strike = if color == 1 {
                open >= c.close && close <= a.open
            } else {
                open <= c.close && close >= a.open
            };
            Some((same && progressive && opens && strike) as i32 * color * 100)
        } else {
            None
        };
        if self.candles.len() == 8 {
            self.candles.pop_front();
        }
        self.candles.push_back(current);
        self.value = output;
        output
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
        *self = Self::new()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn matches_batch() {
        let open: Vec<f64> = (0..30).map(|i| 100. + i as f64 * 0.1).collect();
        let high: Vec<f64> = open.iter().map(|x| x + 2.).collect();
        let low: Vec<f64> = open.iter().map(|x| x - 2.).collect();
        let close: Vec<f64> = open.iter().map(|x| x + 1.).collect();
        let e = crate::stream::candle_three_line_strike(&open, &high, &low, &close).unwrap();
        let mut s = CandleThreeLineStrike::new();
        for (((&o, &h), &l), (&c, &e)) in open.iter().zip(&high).zip(&low).zip(close.iter().zip(&e))
        {
            match s.append(o, h, l, c) {
                Some(v) => assert_eq!(v, e),
                None => assert_eq!(e, 0),
            }
        }
    }
}
