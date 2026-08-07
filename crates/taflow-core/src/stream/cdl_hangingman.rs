//! Incremental Hanging Man candlestick recognition (CDLHANGINGMAN).
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
}
pub struct CdlHangingMan {
    candles: VecDeque<Candle>,
    value: Option<i32>,
}
impl Default for CdlHangingMan {
    fn default() -> Self {
        Self::new()
    }
}
impl CdlHangingMan {
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
            let body = self.candles.iter().skip(1).map(|x| x.body()).sum::<f64>() / 10.0;
            let vs = self.candles.iter().skip(1).map(|x| x.range()).sum::<f64>() * 0.01;
            let near = self.candles.iter().skip(5).take(5).map(|x| x.range()).sum::<f64>() * 0.04;
            Some(
                (cur.body() < body
                    && cur.lower() > cur.body()
                    && cur.upper() < vs
                    && cur.o.min(cur.c) >= prev.h - near) as i32
                    * -100,
            )
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
        let e = crate::pattern::cdl_hangingman(&o, &h, &l, &c).unwrap();
        let mut s = CdlHangingMan::new();
        for ((((&o, &h), &l), &c), &e) in o.iter().zip(&h).zip(&l).zip(&c).zip(&e) {
            match s.append(o, h, l, c) {
                Some(v) => assert_eq!(v, e),
                None => assert_eq!(e, 0),
            }
        }
    }
}
