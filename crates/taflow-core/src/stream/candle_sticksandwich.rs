//! Incremental Stick Sandwich recognition (CDLSTICKSANDWICH).
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
/// Incremental CDLSTICKSANDWICH state.
/// Persistent Rust state or aligned output type for `CandleStickSandwich`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct CandleStickSandwich {
    candles: VecDeque<Candle>,
    value: Option<i32>,
}
impl Default for CandleStickSandwich {
    fn default() -> Self {
        Self::new()
    }
}
impl CandleStickSandwich {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new() -> Self {
        Self {
            candles: VecDeque::with_capacity(7),
            value: None,
        }
    }
    fn equal(&self) -> f64 {
        self.candles.iter().take(5).map(|c| c.range()).sum::<f64>() * 0.01
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
        let output = if self.candles.len() == 7 {
            let first = self.candles[5];
            let second = self.candles[6];
            Some(
                ((first.color() == -1
                    && second.color() == 1
                    && current.color() == -1
                    && second.low > first.close
                    && (close - first.close).abs() <= self.equal()) as i32)
                    * 100,
            )
        } else {
            None
        };
        if self.candles.len() == 7 {
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

/// Compute the candle pattern signal for aligned OHLC bars.
///
/// # Parameters
///
/// * `open`, `high`, `low`, `close` - Equal-length chronological OHLC series.
///
/// # Returns
///
/// A same-length vector containing -100, 0, or 100 pattern signals; bars
/// Compute the candle stick sandwich result for the supplied aligned series.
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
pub fn candle_stick_sandwich(
    open: &[f64],
    high: &[f64],
    low: &[f64],
    close: &[f64],
) -> TaResult<Vec<i32>> {
    let len = validate_ohlc(open, high, low, close)?;
    let mut output = vec![0i32; len];
    let lookback = EQUAL.avg_period + 2;
    if len <= lookback {
        return Ok(output);
    }

    let mut equal_sum = 0.0;
    let start = lookback;
    for i in (start - 2 - EQUAL.avg_period)..(start - 2) {
        equal_sum += cr(EQUAL, open, high, low, close, i);
    }

    for i in start..len {
        output[i] = (candle_color(open[i - 2], close[i - 2]) == -1
            && candle_color(open[i - 1], close[i - 1]) == 1
            && candle_color(open[i], close[i]) == -1
            && low[i - 1] > close[i - 2]
            && (close[i] - close[i - 2]).abs()
                <= ca(EQUAL, equal_sum, open, high, low, close, i - 2)) as i32
            * 100;
        equal_sum += cr(EQUAL, open, high, low, close, i - 2)
            - cr(EQUAL, open, high, low, close, i - 2 - EQUAL.avg_period);
    }
    Ok(output)
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
        let e = crate::stream::candle_stick_sandwich(&open, &high, &low, &close).unwrap();
        let mut s = CandleStickSandwich::new();
        for (((&o, &h), &l), (&c, &e)) in open.iter().zip(&high).zip(&low).zip(close.iter().zip(&e))
        {
            match s.append(o, h, l, c) {
                Some(v) => assert_eq!(v, e),
                None => assert_eq!(e, 0),
            }
        }
    }
}
