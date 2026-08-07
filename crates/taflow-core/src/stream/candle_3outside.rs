//! Incremental Three Outside pattern recognition (CDL3OUTSIDE).
use super::pattern::*;
use crate::error::TaResult;
use std::collections::VecDeque;
#[derive(Clone, Copy)]
struct Candle {
    open: f64,
    close: f64,
}
/// Incremental CDL3OUTSIDE state.
/// Persistent Rust state or aligned output type for `CandleThreeOutside`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct CandleThreeOutside {
    candles: VecDeque<Candle>,
    value: Option<i32>,
}
impl Default for CandleThreeOutside {
    fn default() -> Self {
        Self::new()
    }
}
impl CandleThreeOutside {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new() -> Self {
        Self {
            candles: VecDeque::with_capacity(2),
            value: None,
        }
    }
    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, open: f64, _high: f64, _low: f64, close: f64) -> Option<i32> {
        let output = if self.candles.len() == 2 {
            let first = self.candles[0];
            let second = self.candles[1];
            let bull = first.close < first.open
                && second.close >= first.open
                && second.open <= first.close
                && close > second.close;
            let bear = first.close >= first.open
                && second.close < second.open
                && second.open >= first.close
                && second.close <= first.open
                && close < second.close;
            Some((bull as i32) * 100 - (bear as i32) * 100)
        } else {
            None
        };
        if self.candles.len() == 2 {
            self.candles.pop_front();
        }
        self.candles.push_back(Candle { open, close });
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
/// Compute the candle three outside result for the supplied aligned series.
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
pub fn candle_three_outside(
    open: &[f64],
    high: &[f64],
    low: &[f64],
    close: &[f64],
) -> TaResult<Vec<i32>> {
    let len = validate_ohlc(open, high, low, close)?;
    let mut output = vec![0i32; len];
    // lookback = 3
    if len < 3 {
        return Ok(output);
    }

    for i in 2..len {
        // Bullish: 1st black, 2nd white engulfs, 3rd closes higher
        let bull = candle_color(open[i - 2], close[i - 2]) == -1
            && candle_color(open[i - 1], close[i - 1]) == 1
            && close[i - 1] >= open[i - 2]
            && open[i - 1] <= close[i - 2]
            && close[i] > close[i - 1];
        // Bearish: 1st white, 2nd black engulfs, 3rd closes lower
        let bear = candle_color(open[i - 2], close[i - 2]) == 1
            && candle_color(open[i - 1], close[i - 1]) == -1
            && open[i - 1] >= close[i - 2]
            && close[i - 1] <= open[i - 2]
            && close[i] < close[i - 1];
        output[i] = (bull as i32) * 100 - (bear as i32) * 100;
    }
    Ok(output)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn matches_batch() {
        let open = vec![10., 9., 11., 10., 8.];
        let high = vec![12.; 5];
        let low = vec![7.; 5];
        let close = vec![9., 11., 12., 8., 7.];
        let expected = crate::stream::candle_three_outside(&open, &high, &low, &close).unwrap();
        let mut s = CandleThreeOutside::new();
        for (((&o, &h), &l), (&c, &e)) in open
            .iter()
            .zip(&high)
            .zip(&low)
            .zip(close.iter().zip(&expected))
        {
            match s.append(o, h, l, c) {
                Some(v) => assert_eq!(v, e),
                None => assert_eq!(e, 0),
            }
        }
    }
}
