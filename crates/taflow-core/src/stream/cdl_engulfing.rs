//! Incremental Engulfing candlestick recognition (CDLENGULFING).

/// Incremental CDLENGULFING state.
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
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn matches_batch() {
        let open = vec![10., 9., 12., 10., 8.];
        let high = vec![11.; 5];
        let low = vec![7.; 5];
        let close = vec![9., 11., 10., 8., 11.];
        let expected = crate::pattern::cdl_engulfing(&open, &high, &low, &close).unwrap();
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
