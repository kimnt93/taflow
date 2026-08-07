//! Incremental Breakaway candlestick recognition (CDLBREAKAWAY).
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
/// Stateful CandleBreakaway candle recognizer.
/// Consumes causal OHLC bars and returns an aligned pattern score.
pub struct CandleBreakaway {
    candles: VecDeque<Candle>,
    value: Option<i32>,
}
impl Default for CandleBreakaway {
    fn default() -> Self {
        Self::new()
    }
}
impl CandleBreakaway {
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
            let long = self.candles.iter().take(10).map(|x| x.body()).sum::<f64>() / 10.0;
            let base = a.body() > long
                && a.color() == b.color()
                && b.color() == d.color()
                && d.color() == -cur.color();
            let bear_first = base
                && a.color() == -1
                && b.o.max(b.c) < a.o.min(a.c)
                && cnd.h < b.h
                && cnd.l < b.l
                && d.h < cnd.h
                && d.l < cnd.l
                && cur.c > b.o
                && cur.c < a.c;
            let bull_first = base
                && a.color() == 1
                && b.o.min(b.c) > a.o.max(a.c)
                && cnd.h > b.h
                && cnd.l > b.l
                && d.h > cnd.h
                && d.l > cnd.l
                && cur.c < b.o
                && cur.c > a.c;
            Some((bear_first as i32 | bull_first as i32) * cur.color() * 100)
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
        let o: Vec<f64> = (0..48).map(|i| 100.0 + i as f64 * 0.2).collect();
        let h: Vec<f64> = o.iter().map(|x| x + 2.0).collect();
        let l: Vec<f64> = o.iter().map(|x| x - 2.0).collect();
        let c: Vec<f64> = o
            .iter()
            .enumerate()
            .map(|(i, x)| x + if i % 3 == 0 { -1.0 } else { 1.0 })
            .collect();
        let e = crate::pattern::cdl_breakaway(&o, &h, &l, &c).unwrap();
        let mut s = CandleBreakaway::new();
        for ((((&o, &h), &l), &c), &e) in o.iter().zip(&h).zip(&l).zip(&c).zip(&e) {
            match s.append(o, h, l, c) {
                Some(v) => assert_eq!(v, e),
                None => assert_eq!(e, 0),
            }
        }
    }
}
