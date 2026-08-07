//! Incremental Engulfing candlestick recognition (CDLENGULFING).

use super::pattern::*;
use crate::error::TaResult;
/// Incremental CDLENGULFING state.
/// Persistent Rust state or aligned output type for `CandleEngulfing`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct CandleEngulfing {
    previous: Option<(f64, f64)>,
    value: Option<i32>,
}
impl Default for CandleEngulfing {
    fn default() -> Self {
        Self::new()
    }
}
impl CandleEngulfing {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new() -> Self {
        Self {
            previous: None,
            value: None,
        }
    }
    /// Appends OHLC data; high and low are accepted for a uniform pattern API.
    pub fn append(&mut self, open: f64, _high: f64, _low: f64, close: f64) -> Option<i32> {
        let previous = self.previous.replace((open, close));
        let (previous_open, previous_close) = previous?;
        let bullish =
            previous_close < previous_open && close >= previous_open && open <= previous_close;
        let bearish = previous_close >= previous_open
            && close < open
            && open >= previous_close
            && close <= previous_open;
        self.value = Some((bullish as i32) * 100 - (bearish as i32) * 100);
        self.value
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
/// Compute the candle engulfing result for the supplied aligned series.
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
pub fn candle_engulfing(
    open: &[f64],
    high: &[f64],
    low: &[f64],
    close: &[f64],
) -> TaResult<Vec<i32>> {
    let len = validate_ohlc(open, high, low, close)?;
    let mut output = vec![0i32; len];
    // Engulfing has no candle settings, lookback = 2 (need i-1)
    if len < 2 {
        return Ok(output);
    }

    for i in 1..len {
        // Bullish: prev black, curr white, curr close >= prev open, curr open <= prev close
        let bull = candle_color(open[i - 1], close[i - 1]) == -1
            && candle_color(open[i], close[i]) == 1
            && close[i] >= open[i - 1]
            && open[i] <= close[i - 1];
        // Bearish: prev white, curr black, curr open >= prev close, curr close <= prev open
        let bear = candle_color(open[i - 1], close[i - 1]) == 1
            && candle_color(open[i], close[i]) == -1
            && open[i] >= close[i - 1]
            && close[i] <= open[i - 1];
        output[i] = (bull as i32) * 100 - (bear as i32) * 100;
    }
    Ok(output)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn matches_batch() {
        let open = vec![10., 9., 12., 10., 8.];
        let high = vec![11.; 5];
        let low = vec![7.; 5];
        let close = vec![9., 11., 10., 8., 11.];
        let expected = crate::stream::candle_engulfing(&open, &high, &low, &close).unwrap();
        let mut state = CandleEngulfing::new();
        for (((&o, &h), &l), (&c, &expected)) in open
            .iter()
            .zip(&high)
            .zip(&low)
            .zip(close.iter().zip(&expected))
        {
            match state.append(o, h, l, c) {
                Some(value) => assert_eq!(value, expected),
                None => assert_eq!(expected, 0),
            }
        }
    }
}
