//! Incremental Three Outside pattern recognition (CDL3OUTSIDE).
use std::collections::VecDeque;
#[derive(Clone, Copy)]
struct Candle {
    open: f64,
    close: f64,
}
/// Incremental CDL3OUTSIDE state.
pub struct Cdl3Outside {
    candles: VecDeque<Candle>,
    value: Option<i32>,
}
impl Default for Cdl3Outside {
    fn default() -> Self {
        Self::new()
    }
}
impl Cdl3Outside {
    pub fn new() -> Self {
        Self {
            candles: VecDeque::with_capacity(2),
            value: None,
        }
    }
    pub fn append(&mut self, open: f64, _high: f64, _low: f64, close: f64) -> Option<i32> {
        let output = if self.candles.len() == 2 {
            let first = self.candles[0];
            let second = self.candles[1];
            let bull = first.close < first.open
                && second.close >= first.open
                && second.open <= first.close
                && close > second.close;
            let bear = first.close >= first.open
                && second.close < second.open
                && second.open >= first.close
                && second.close <= first.open
                && close < second.close;
            Some((bull as i32) * 100 - (bear as i32) * 100)
        } else {
            None
        };
        if self.candles.len() == 2 {
            self.candles.pop_front();
        }
        self.candles.push_back(Candle { open, close });
        self.value = output;
        output
    }
    pub fn value(&self) -> Option<i32> {
        self.value
    }
    pub fn reset(&mut self) {
        *self = Self::new()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn matches_batch() {
        let open = vec![10., 9., 11., 10., 8.];
        let high = vec![12.; 5];
        let low = vec![7.; 5];
        let close = vec![9., 11., 12., 8., 7.];
        let expected = crate::pattern::cdl_3outside(&open, &high, &low, &close).unwrap();
        let mut s = Cdl3Outside::new();
        for (((&o, &h), &l), (&c, &e)) in open
            .iter()
            .zip(&high)
            .zip(&low)
            .zip(close.iter().zip(&expected))
        {
            match s.append(o, h, l, c) {
                Some(v) => assert_eq!(v, e),
                None => assert_eq!(e, 0),
            }
        }
    }
}
