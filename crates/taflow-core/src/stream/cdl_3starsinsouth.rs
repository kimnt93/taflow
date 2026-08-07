//! Incremental Three Stars In The South recognition (CDL3STARSINSOUTH).
use std::collections::VecDeque;
#[derive(Clone, Copy)]
struct Candle {
    open: f64,
    high: f64,
    low: f64,
    close: f64,
}
impl Candle {
    fn body(self) -> f64 {
        (self.close - self.open).abs()
    }
    fn range(self) -> f64 {
        self.high - self.low
    }
    fn lower(self) -> f64 {
        self.open.min(self.close) - self.low
    }
    fn upper(self) -> f64 {
        self.high - self.open.max(self.close)
    }
    fn black(self) -> bool {
        self.close < self.open
    }
}
/// Incremental CDL3STARSINSOUTH state.
pub struct Candle3StarsInSouth {
    candles: VecDeque<Candle>,
    value: Option<i32>,
}
impl Default for Candle3StarsInSouth {
    fn default() -> Self {
        Self::new()
    }
}
impl Candle3StarsInSouth {
    pub fn new() -> Self {
        Self {
            candles: VecDeque::with_capacity(12),
            value: None,
        }
    }
    fn avg_body(&self, start: usize) -> f64 {
        self.candles
            .iter()
            .skip(start)
            .take(10)
            .map(|c| c.body())
            .sum::<f64>()
            / 10.0
    }
    fn avg_range(&self, start: usize) -> f64 {
        self.candles
            .iter()
            .skip(start)
            .take(10)
            .map(|c| c.range())
            .sum::<f64>()
            * 0.01
    }
    pub fn append(&mut self, open: f64, high: f64, low: f64, close: f64) -> Option<i32> {
        let current = Candle {
            open,
            high,
            low,
            close,
        };
        let output = if self.candles.len() == 12 {
            let first = self.candles[10];
            let second = self.candles[11];
            let pattern = first.black()
                && second.black()
                && current.black()
                && first.body() > self.avg_body(0)
                && first.lower() > first.body()
                && second.open.min(second.close) > first.open.min(first.close)
                && second.open.max(second.close) < first.open.max(first.close)
                && second.low < first.low
                && current.body() < self.avg_body(2)
                && current.upper() < self.avg_range(2)
                && current.lower() < self.avg_range(2)
                && current.low > second.low
                && current.high < second.high;
            Some((pattern as i32) * 100)
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
        let open: Vec<f64> = (0..30).map(|i| 100. + i as f64 * 0.1).collect();
        let high: Vec<f64> = open.iter().map(|x| x + 2.).collect();
        let low: Vec<f64> = open.iter().map(|x| x - 2.).collect();
        let close: Vec<f64> = open.iter().map(|x| x + 1.).collect();
        let e = crate::pattern::cdl_3starsinsouth(&open, &high, &low, &close).unwrap();
        let mut s = Candle3StarsInSouth::new();
        for (((&o, &h), &l), (&c, &e)) in open.iter().zip(&high).zip(&low).zip(close.iter().zip(&e))
        {
            match s.append(o, h, l, c) {
                Some(v) => assert_eq!(v, e),
                None => assert_eq!(e, 0),
            }
        }
    }
}
