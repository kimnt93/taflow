//! Incremental Rising/Falling Three Methods candlestick recognition (CDLRISEFALL3METHODS).
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
    fn color(self) -> i32 {
        if self.c >= self.o {
            1
        } else {
            -1
        }
    }
}
/// Stateful CandleRiseFall3Methods candle recognizer.
/// Consumes causal OHLC bars and returns an aligned pattern score.
pub struct CandleRiseFall3Methods {
    candles: VecDeque<Candle>,
    value: Option<i32>,
}
impl Default for CandleRiseFall3Methods {
    fn default() -> Self {
        Self::new()
    }
}
impl CandleRiseFall3Methods {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new() -> Self {
        Self {
            candles: VecDeque::with_capacity(14),
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
        let value = if self.candles.len() == 14 {
            let a = self.candles[10];
            let b = self.candles[11];
            let cnd = self.candles[12];
            let d = self.candles[13];
            let long0 = self.candles.iter().skip(4).map(|x| x.body()).sum::<f64>() / 10.0;
            let long4 = self.candles.iter().take(10).map(|x| x.body()).sum::<f64>() / 10.0;
            let short0 = self.candles.iter().skip(1).take(10).map(|x| x.body()).sum::<f64>() / 10.0;
            let short1 = self.candles.iter().skip(2).take(10).map(|x| x.body()).sum::<f64>() / 10.0;
            let short2 = self.candles.iter().skip(3).map(|x| x.body()).sum::<f64>() / 10.0;
            let c4 = a.color();
            let c0 = cur.color();
            let mut out = 0;
            if a.body() > long4 && cur.body() > long0 {
                let mid_short = b.body() < short0 && cnd.body() < short1 && d.body() < short2;
                let bull = c4 == 1
                    && mid_short
                    && b.color() == -1
                    && cnd.color() == -1
                    && d.color() == -1
                    && b.c < a.c
                    && cnd.c < b.c
                    && d.c < cnd.c
                    && b.l > a.l
                    && cnd.l > a.l
                    && d.l > a.l
                    && b.h < a.h
                    && cnd.h < a.h
                    && d.h < a.h
                    && c0 == 1
                    && cur.o > d.c
                    && cur.c > a.c;
                let bear = c4 == -1
                    && mid_short
                    && b.color() == 1
                    && cnd.color() == 1
                    && d.color() == 1
                    && b.c > a.c
                    && cnd.c > b.c
                    && d.c > cnd.c
                    && b.h < a.h
                    && cnd.h < a.h
                    && d.h < a.h
                    && b.l > a.l
                    && cnd.l > a.l
                    && d.l > a.l
                    && c0 == -1
                    && cur.o < d.c
                    && cur.c < a.c;
                out = (bull as i32) * 100 - (bear as i32) * 100;
            }
            Some(out)
        } else {
            None
        };
        if self.candles.len() == 14 {
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
        let o: Vec<f64> = (0..48).map(|i| 100.0 + i as f64 * 0.2).collect();
        let h: Vec<f64> = o.iter().map(|x| x + 2.0).collect();
        let l: Vec<f64> = o.iter().map(|x| x - 2.0).collect();
        let c: Vec<f64> = o
            .iter()
            .enumerate()
            .map(|(i, x)| x + if i % 3 == 0 { -1.0 } else { 1.0 })
            .collect();
        let e = crate::pattern::cdl_risefall3methods(&o, &h, &l, &c).unwrap();
        let mut s = CandleRiseFall3Methods::new();
        for ((((&o, &h), &l), &c), &e) in o.iter().zip(&h).zip(&l).zip(&c).zip(&e) {
            match s.append(o, h, l, c) {
                Some(v) => assert_eq!(v, e),
                None => assert_eq!(e, 0),
            }
        }
    }
}
