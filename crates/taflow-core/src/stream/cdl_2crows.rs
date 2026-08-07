//! Incremental Two Crows candlestick recognition (CDL2CROWS).

use std::collections::VecDeque;
#[derive(Clone, Copy)]
struct Candle {
    open: f64,
    close: f64,
    body: f64,
}
/// Incremental CDL2CROWS state.
pub struct Candle2Crows {
    candles: VecDeque<Candle>,
    bodies: VecDeque<f64>,
    sum: f64,
    value: Option<i32>,
}
impl Default for Candle2Crows {
    fn default() -> Self {
        Self::new()
    }
}
impl Candle2Crows {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new() -> Self {
        Self {
            candles: VecDeque::with_capacity(3),
            bodies: VecDeque::with_capacity(10),
            sum: 0.0,
            value: None,
        }
    }
    fn push_body(&mut self, value: f64) {
        if self.bodies.len() == 10 {
            self.sum -= self.bodies.pop_front().expect("window full");
        }
        self.bodies.push_back(value);
        self.sum += value;
    }
    /// Appends OHLC data and returns -100 for a two-crows pattern after warmup.
    pub fn append(&mut self, open: f64, _high: f64, _low: f64, close: f64) -> Option<i32> {
        let current = Candle {
            open,
            close,
            body: (close - open).abs(),
        };
        let output = if self.bodies.len() == 10 && self.candles.len() == 2 {
            let first = self.candles[0];
            let second = self.candles[1];
            let pattern = first.close >= first.open
                && first.body > self.sum / 10.0
                && second.close < second.open
                && second.open.min(second.close) > first.open.max(first.close)
                && close < open
                && open < second.open
                && open > second.close
                && close > first.open
                && close < first.close;
            Some(-(pattern as i32) * 100)
        } else {
            None
        };
        if self.candles.len() == 2 {
            self.push_body(self.candles[0].body);
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
    pub fn reset(&mut self) {
        *self = Self::new();
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn matches_batch() {
        let open: Vec<f64> = (0..40).map(|i| 100. + i as f64 * 0.2).collect();
        let high: Vec<f64> = open.iter().map(|x| x + 2.).collect();
        let low: Vec<f64> = open.iter().map(|x| x - 2.).collect();
        let close: Vec<f64> = open.iter().map(|x| x + 1.).collect();
        let expected = crate::pattern::cdl_2crows(&open, &high, &low, &close).unwrap();
        let mut state = Candle2Crows::new();
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
