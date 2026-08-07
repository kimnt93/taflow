//! Incremental Three Line Strike recognition (CDL3LINESTRIKE).
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
/// Incremental CDL3LINESTRIKE state.
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
        let e = crate::pattern::cdl_3linestrike(&open, &high, &low, &close).unwrap();
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
