//! Incremental Doji candlestick recognition (CDLDOJI).

use std::collections::VecDeque;

/// Incremental CDLDOJI state using TA-Lib's ten-bar High-Low average.
pub struct CandleDoji {
    ranges: VecDeque<f64>,
    sum: f64,
    value: Option<i32>,
}
impl Default for CandleDoji {
    fn default() -> Self {
        Self::new()
    }
}
impl CandleDoji {
    pub fn new() -> Self {
        Self {
            ranges: VecDeque::with_capacity(10),
            sum: 0.0,
            value: None,
        }
    }
    /// Appends OHLC data and returns +100 for a doji after the ten-bar warmup.
    pub fn append(&mut self, open: f64, high: f64, low: f64, close: f64) -> Option<i32> {
        if self.ranges.len() < 10 {
            self.ranges.push_back(high - low);
            self.sum += high - low;
            return None;
        }
        let threshold = self.sum * 0.01;
        self.value = Some(if (close - open).abs() <= threshold {
            100
        } else {
            0
        });
        self.sum += high - low - self.ranges.pop_front().expect("window is full");
        self.ranges.push_back(high - low);
        self.value
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
        let open: Vec<f64> = (0..40).map(|i| 100.0 + i as f64 * 0.2).collect();
        let high: Vec<f64> = open.iter().map(|x| x + 2.0).collect();
        let low: Vec<f64> = open.iter().map(|x| x - 2.0).collect();
        let close: Vec<f64> = open
            .iter()
            .enumerate()
            .map(|(i, x)| x + if i % 3 == 0 { 0.1 } else { 1.0 })
            .collect();
        let expected = crate::pattern::cdl_doji(&open, &high, &low, &close).unwrap();
        let mut state = CandleDoji::new();
        for (((&o, &h), &l), (&c, &expected)) in open
            .iter()
            .zip(&high)
            .zip(&low)
            .zip(close.iter().zip(&expected))
        {
            match state.append(o, h, l, c) {
                Some(value) => assert_eq!(value, expected),
                None => assert_eq!(expected, 0),
            }
        }
    }
}
