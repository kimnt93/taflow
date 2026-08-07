//! Incremental Tasuki Gap candlestick recognition (CDLTASUKIGAP).
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
    fn color(self) -> i32 {
        if self.c >= self.o {
            1
        } else {
            -1
        }
    }
}
pub struct CdlTasukiGap {
    candles: VecDeque<Candle>,
    value: Option<i32>,
}
impl Default for CdlTasukiGap {
    fn default() -> Self {
        Self::new()
    }
}
impl CdlTasukiGap {
    pub fn new() -> Self {
        Self {
            candles: VecDeque::with_capacity(7),
            value: None,
        }
    }
    pub fn append(&mut self, o: f64, h: f64, l: f64, c: f64) -> Option<i32> {
        let cur = Candle { o, h, l, c };
        let value = if self.candles.len() == 7 {
            let a = self.candles[5];
            let b = self.candles[6];
            let near = self.candles.iter().skip(1).take(5).map(|x| x.range()).sum::<f64>() * 0.04;
            let c1 = b.color();
            let c0 = cur.color();
            let near_same = (b.body() - cur.body()).abs() < near;
            let bull = b.o.min(b.c) > a.o.max(a.c)
                && c1 == 1
                && c0 == -1
                && cur.o < b.c
                && cur.o > b.o
                && cur.c < b.o
                && cur.c > a.o.max(a.c)
                && near_same;
            let bear = b.o.max(b.c) < a.o.min(a.c)
                && c1 == -1
                && c0 == 1
                && cur.o < b.o
                && cur.o > b.c
                && cur.c > b.o
                && cur.c < a.o.min(a.c)
                && near_same;
            Some((bull as i32 | bear as i32) * c1 * 100)
        } else {
            None
        };
        if self.candles.len() == 7 {
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
        let e = crate::pattern::cdl_tasukigap(&o, &h, &l, &c).unwrap();
        let mut s = CdlTasukiGap::new();
        for ((((&o, &h), &l), &c), &e) in o.iter().zip(&h).zip(&l).zip(&c).zip(&e) {
            match s.append(o, h, l, c) {
                Some(v) => assert_eq!(v, e),
                None => assert_eq!(e, 0),
            }
        }
    }
}
