//! Incremental Concealing Baby Swallow candlestick recognition (CDLCONCEALBABYSWALL).
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
/// Stateful CandleConcealBabySwall candle recognizer.
/// Consumes causal OHLC bars and returns an aligned pattern score.
pub struct CandleConcealBabySwall {
    candles: VecDeque<Candle>,
    value: Option<i32>,
}
impl Default for CandleConcealBabySwall {
    fn default() -> Self {
        Self::new()
    }
}
impl CandleConcealBabySwall {
    pub fn new() -> Self {
        Self {
            candles: VecDeque::with_capacity(13),
            value: None,
        }
    }
    pub fn append(&mut self, o: f64, h: f64, l: f64, c: f64) -> Option<i32> {
        let cur = Candle { o, h, l, c };
        let value = if self.candles.len() == 13 {
            let a = self.candles[10];
            let b = self.candles[11];
            let cnd = self.candles[12];
            let s0 = self.candles.iter().take(10).map(|x| x.range()).sum::<f64>() * 0.01;
            let s1 = self.candles.iter().skip(1).take(10).map(|x| x.range()).sum::<f64>() * 0.01;
            Some(
                (a.color() == -1
                    && b.color() == -1
                    && cnd.color() == -1
                    && cur.color() == -1
                    && a.upper() < s0
                    && a.lower() < s0
                    && b.upper() < s1
                    && b.lower() < s1
                    && cnd.o.max(cnd.c) < b.o.min(b.c)
                    && cnd.h > b.c
                    && cur.o >= cnd.h
                    && cur.c <= cnd.l) as i32
                    * 100,
            )
        } else {
            None
        };
        if self.candles.len() == 13 {
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
        let o: Vec<f64> = (0..48).map(|i| 100.0 + i as f64 * 0.2).collect();
        let h: Vec<f64> = o.iter().map(|x| x + 2.0).collect();
        let l: Vec<f64> = o.iter().map(|x| x - 2.0).collect();
        let c: Vec<f64> = o
            .iter()
            .enumerate()
            .map(|(i, x)| x + if i % 3 == 0 { -1.0 } else { 1.0 })
            .collect();
        let e = crate::pattern::cdl_concealbabyswall(&o, &h, &l, &c).unwrap();
        let mut s = CandleConcealBabySwall::new();
        for ((((&o, &h), &l), &c), &e) in o.iter().zip(&h).zip(&l).zip(&c).zip(&e) {
            match s.append(o, h, l, c) {
                Some(v) => assert_eq!(v, e),
                None => assert_eq!(e, 0),
            }
        }
    }
}
