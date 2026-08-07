//! Incremental Stalled Pattern candlestick recognition (CDLSTALLEDPATTERN).
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
    fn upper(self) -> f64 {
        self.h - self.o.max(self.c)
    }
    fn color(self) -> i32 {
        if self.c >= self.o {
            1
        } else {
            -1
        }
    }
}
/// Stateful CandleStalledPattern candle recognizer.
/// Consumes causal OHLC bars and returns an aligned pattern score.
pub struct CandleStalledPattern {
    candles: VecDeque<Candle>,
    value: Option<i32>,
}
impl Default for CandleStalledPattern {
    fn default() -> Self {
        Self::new()
    }
}
impl CandleStalledPattern {
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
            let long0 = self.candles.iter().take(10).map(|x| x.body()).sum::<f64>() / 10.0;
            let long1 = self.candles.iter().skip(1).take(10).map(|x| x.body()).sum::<f64>() / 10.0;
            let short = self.candles.iter().skip(2).map(|x| x.body()).sum::<f64>() / 10.0;
            let shadow = self.candles.iter().skip(1).take(10).map(|x| x.range()).sum::<f64>() * 0.01;
            let near0 = self.candles.iter().skip(5).take(5).map(|x| x.range()).sum::<f64>() * 0.04;
            let near1 = self.candles.iter().skip(6).take(5).map(|x| x.range()).sum::<f64>() * 0.04;
            Some(
                (a.color() == 1
                    && b.color() == 1
                    && cur.color() == 1
                    && b.c > a.c
                    && cur.c > b.c
                    && a.body() > long0
                    && b.body() > long1
                    && b.upper() < shadow
                    && b.o > a.o
                    && b.o <= a.c + near0
                    && cur.body() < short
                    && cur.o >= b.c - cur.body() - near1) as i32
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
        let e = crate::pattern::cdl_stalledpattern(&o, &h, &l, &c).unwrap();
        let mut s = CandleStalledPattern::new();
        for ((((&o, &h), &l), &c), &e) in o.iter().zip(&h).zip(&l).zip(&c).zip(&e) {
            match s.append(o, h, l, c) {
                Some(v) => assert_eq!(v, e),
                None => assert_eq!(e, 0),
            }
        }
    }
}
