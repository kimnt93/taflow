//! Incremental Separating Lines candlestick recognition (CDLSEPARATINGLINES).
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
/// Stateful CandleSeparatingLines candle recognizer.
/// Consumes causal OHLC bars and returns an aligned pattern score.
pub struct CandleSeparatingLines {
    candles: VecDeque<Candle>,
    value: Option<i32>,
}
impl Default for CandleSeparatingLines {
    fn default() -> Self {
        Self::new()
    }
}
impl CandleSeparatingLines {
    pub fn new() -> Self {
        Self {
            candles: VecDeque::with_capacity(11),
            value: None,
        }
    }
    pub fn append(&mut self, o: f64, h: f64, l: f64, c: f64) -> Option<i32> {
        let cur = Candle { o, h, l, c };
        let value = if self.candles.len() == 11 {
            let prev = self.candles[10];
            let vs = self.candles.iter().skip(1).map(|x| x.range()).sum::<f64>() * 0.01;
            let long = self.candles.iter().skip(1).map(|x| x.body()).sum::<f64>() / 10.0;
            let equal = self.candles.iter().skip(5).take(5).map(|x| x.range()).sum::<f64>() * 0.01;
            let color_prev = prev.color();
            let color_cur = cur.color();
            let base = color_prev != color_cur
                && (cur.o - prev.o).abs() <= equal
                && cur.body() > long;
            let bull = base && color_cur == 1 && cur.lower() < vs;
            let bear = base && color_cur == -1 && cur.upper() < vs;
            Some((bull as i32) * 100 - (bear as i32) * 100)
        } else {
            None
        };
        if self.candles.len() == 11 {
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
        let o: Vec<f64> = (0..40).map(|i| 100.0 + i as f64 * 0.2).collect();
        let h: Vec<f64> = o.iter().map(|x| x + 2.0).collect();
        let l: Vec<f64> = o.iter().map(|x| x - 2.0).collect();
        let c: Vec<f64> = o
            .iter()
            .enumerate()
            .map(|(i, x)| x + if i % 3 == 0 { -1.0 } else { 1.0 })
            .collect();
        let e = crate::pattern::cdl_separatinglines(&o, &h, &l, &c).unwrap();
        let mut s = CandleSeparatingLines::new();
        for ((((&o, &h), &l), &c), &e) in o.iter().zip(&h).zip(&l).zip(&c).zip(&e) {
            match s.append(o, h, l, c) {
                Some(v) => assert_eq!(v, e),
                None => assert_eq!(e, 0),
            }
        }
    }
}
