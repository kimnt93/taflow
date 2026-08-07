//! Incremental Advance Block candlestick recognition (CDLADVANCEBLOCK).
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
    fn shadow(self) -> f64 {
        self.range() - self.body()
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
/// Stateful CandleAdvanceBlock candle recognizer.
/// Consumes causal OHLC bars and returns an aligned pattern score.
pub struct CandleAdvanceBlock {
    candles: VecDeque<Candle>,
    value: Option<i32>,
}
impl Default for CandleAdvanceBlock {
    fn default() -> Self {
        Self::new()
    }
}
impl CandleAdvanceBlock {
    pub fn new() -> Self {
        Self {
            candles: VecDeque::with_capacity(12),
            value: None,
        }
    }
    pub fn append(&mut self, o: f64, h: f64, l: f64, c: f64) -> Option<i32> {
        let cur = Candle { o, h, l, c };
        let value = if self.candles.len() == 12 {
            let a = self.candles[10];
            let b = self.candles[11];
            let long = self.candles.iter().take(10).map(|x| x.body()).sum::<f64>() / 10.0;
            let shadow_short = self.candles.iter().take(10).map(|x| x.shadow()).sum::<f64>() / 20.0;
            let near1 = self.candles.iter().skip(6).take(5).map(|x| x.range()).sum::<f64>() * 0.04;
            let near2 = self.candles.iter().skip(7).take(5).map(|x| x.range()).sum::<f64>() * 0.04;
            let far1 = self.candles.iter().skip(6).take(5).map(|x| x.range()).sum::<f64>() * 0.12;
            let far2 = self.candles.iter().skip(7).take(5).map(|x| x.range()).sum::<f64>() * 0.12;
            let base = a.color() == 1
                && b.color() == 1
                && cur.color() == 1
                && b.c > a.c
                && cur.c > b.c
                && b.o > a.o
                && b.o <= a.c + near1
                && cur.o > b.o
                && cur.o <= b.c + near2
                && a.body() > long
                && a.upper() < shadow_short;
            let weakness = base
                && ((b.body() < a.body() - far1 && cur.body() < b.body() + near2)
                    || (cur.body() < b.body()
                        && b.body() < a.body()
                        && (cur.upper() > cur.body() || b.upper() > b.body()))
                    || cur.body() < b.body() - far2);
            Some(weakness as i32 * -100)
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
        let e = crate::pattern::cdl_advanceblock(&o, &h, &l, &c).unwrap();
        let mut s = CandleAdvanceBlock::new();
        for ((((&o, &h), &l), &c), &e) in o.iter().zip(&h).zip(&l).zip(&c).zip(&e) {
            match s.append(o, h, l, c) {
                Some(v) => assert_eq!(v, e),
                None => assert_eq!(e, 0),
            }
        }
    }
}
