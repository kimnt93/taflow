//! Incremental Three Black Crows candlestick recognition (CDL3BLACKCROWS).
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
    fn lower(self) -> f64 {
        self.open.min(self.close) - self.low
    }
    fn black(self) -> bool {
        self.close < self.open
    }
}
/// Incremental CDL3BLACKCROWS state.
pub struct CandleThreeBlackCrows {
    candles: VecDeque<Candle>,
    value: Option<i32>,
}
impl Default for CandleThreeBlackCrows {
    fn default() -> Self {
        Self::new()
    }
}
impl CandleThreeBlackCrows {
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
    fn average(&self, start: usize) -> f64 {
        self.candles
            .iter()
            .skip(start)
            .take(10)
            .map(|c| c.range())
            .sum::<f64>()
            * 0.01
    }
    /// Appends OHLC data and returns -100 for three black crows after warmup.
    pub fn append(&mut self, open: f64, high: f64, low: f64, close: f64) -> Option<i32> {
        let current = Candle {
            open,
            high,
            low,
            close,
        };
        let output = if self.candles.len() == 13 {
            let a = self.candles[10];
            let b = self.candles[11];
            let pattern = a.black()
                && b.black()
                && current.black()
                && b.close < a.close
                && current.close < b.close
                && a.open <= self.candles[9].open.max(self.candles[9].close)
                && b.open <= a.open
                && b.open >= a.close
                && current.open <= b.open
                && current.open >= b.close
                && a.lower() < self.average(1)
                && b.lower() < self.average(2)
                && current.lower() < self.average(3);
            Some(-(pattern as i32) * 100)
        } else {
            None
        };
        if self.candles.len() == 13 {
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
        let open: Vec<f64> = (0..50).map(|i| 100. + i as f64 * 0.2).collect();
        let high: Vec<f64> = open.iter().map(|x| x + 2.).collect();
        let low: Vec<f64> = open.iter().map(|x| x - 2.).collect();
        let close: Vec<f64> = open.iter().map(|x| x + 1.).collect();
        let expected = crate::pattern::cdl_3blackcrows(&open, &high, &low, &close).unwrap();
        let mut state = CandleThreeBlackCrows::new();
        for (((&o, &h), &l), (&c, &expected)) in open
            .iter()
            .zip(&high)
            .zip(&low)
            .zip(close.iter().zip(&expected))
        {
            match state.append(o, h, l, c) {
                Some(v) => assert_eq!(v, expected),
                None => assert_eq!(expected, 0),
            }
        }
    }
}
