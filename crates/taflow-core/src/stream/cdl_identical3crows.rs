//! Incremental Identical Three Crows candlestick recognition (CDLIDENTICAL3CROWS).
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
/// Stateful CandleIdentical3Crows candle recognizer.
/// Consumes causal OHLC bars and returns an aligned pattern score.
pub struct CandleIdentical3Crows {
    candles: VecDeque<Candle>,
    value: Option<i32>,
}
impl Default for CandleIdentical3Crows {
    fn default() -> Self {
        Self::new()
    }
}
impl CandleIdentical3Crows {
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
            let shadow1 = self.candles.iter().skip(1).take(10).map(|x| x.range()).sum::<f64>() * 0.01;
            let shadow2 = self.candles.iter().skip(2).map(|x| x.range()).sum::<f64>() * 0.01;
            let equal0 = self.candles.iter().skip(5).take(5).map(|x| x.range()).sum::<f64>() * 0.01;
            let equal1 = self.candles.iter().skip(6).take(5).map(|x| x.range()).sum::<f64>() * 0.01;
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
    pub fn reset(&mut self) {
        *self = Self::new();
    }
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
        let e = crate::pattern::cdl_identical3crows(&o, &h, &l, &c).unwrap();
        let mut s = CandleIdentical3Crows::new();
        for ((((&o, &h), &l), &c), &e) in o.iter().zip(&h).zip(&l).zip(&c).zip(&e) {
            match s.append(o, h, l, c) {
                Some(v) => assert_eq!(v, e),
                None => assert_eq!(e, 0),
            }
        }
    }
}
