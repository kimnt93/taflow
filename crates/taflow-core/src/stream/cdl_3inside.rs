//! Incremental Three Inside pattern recognition (CDL3INSIDE).
use std::collections::VecDeque;
#[derive(Clone, Copy)]
struct Candle {
    open: f64,
    close: f64,
}
impl Candle {
    fn body(self) -> f64 {
        (self.close - self.open).abs()
    }
    fn color(self) -> i32 {
        if self.close >= self.open {
            1
        } else {
            -1
        }
    }
}
/// Incremental CDL3INSIDE state.
pub struct Cdl3Inside {
    candles: VecDeque<Candle>,
    value: Option<i32>,
}
impl Default for Cdl3Inside {
    fn default() -> Self {
        Self::new()
    }
}
impl Cdl3Inside {
    pub fn new() -> Self {
        Self {
            candles: VecDeque::with_capacity(12),
            value: None,
        }
    }
    fn average(&self, start: usize) -> f64 {
        self.candles
            .iter()
            .skip(start)
            .take(10)
            .map(|c| c.body())
            .sum::<f64>()
            / 10.0
    }
    /// Appends OHLC data and returns a three-inside signal after warmup.
    pub fn append(&mut self, open: f64, _high: f64, _low: f64, close: f64) -> Option<i32> {
        let current = Candle { open, close };
        let output = if self.candles.len() == 12 {
            let first = self.candles[10];
            let second = self.candles[11];
            let inside = first.body() > self.average(0)
                && second.body() <= self.average(1)
                && second.open.max(second.close) < first.open.max(first.close)
                && second.open.min(second.close) > first.open.min(first.close);
            let reversal =
                (first.color() == 1 && current.color() == -1 && current.close < first.open)
                    || (first.color() == -1 && current.color() == 1 && current.close > first.open);
            Some(-((inside && reversal) as i32) * first.color() * 100)
        } else {
            None
        };
        if self.candles.len() == 12 {
            self.candles.pop_front();
        }
        self.candles.push_back(current);
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
        let open: Vec<f64> = (0..40).map(|i| 100. + i as f64 * 0.2).collect();
        let high: Vec<f64> = open.iter().map(|x| x + 2.).collect();
        let low: Vec<f64> = open.iter().map(|x| x - 2.).collect();
        let close: Vec<f64> = open.iter().map(|x| x + 1.).collect();
        let expected = crate::pattern::cdl_3inside(&open, &high, &low, &close).unwrap();
        let mut s = Cdl3Inside::new();
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
