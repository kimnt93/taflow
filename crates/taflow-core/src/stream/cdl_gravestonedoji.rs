//! Incremental Gravestone Doji candlestick recognition (CDLGRAVESTONEDOJI).

use std::collections::VecDeque;

/// Incremental CDLGRAVESTONEDOJI state using TA-Lib's ten-bar range average.
pub struct CandleGravestoneDoji {
    ranges: VecDeque<f64>,
    sum: f64,
    value: Option<i32>,
}
impl Default for CandleGravestoneDoji {
    fn default() -> Self {
        Self::new()
    }
}
impl CandleGravestoneDoji {
    pub fn new() -> Self {
        Self {
            ranges: VecDeque::with_capacity(10),
            sum: 0.0,
            value: None,
        }
    }
    /// Appends OHLC data and returns +100 for a gravestone doji after warmup.
    pub fn append(&mut self, open: f64, high: f64, low: f64, close: f64) -> Option<i32> {
        let range = high - low;
        let body = (close - open).abs();
        let output = if self.ranges.len() == 10 {
            let limit = self.sum * 0.01;
            Some(
                (body <= limit && open.min(close) - low < limit && high - open.max(close) > limit)
                    as i32
                    * 100,
            )
        } else {
            None
        };
        if self.ranges.len() == 10 {
            self.sum -= self.ranges.pop_front().expect("window is full");
        }
        self.ranges.push_back(range);
        self.sum += range;
        self.value = output;
        output
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
        let open: Vec<f64> = (0..45).map(|i| 100.0 + i as f64 * 0.2).collect();
        let high: Vec<f64> = open.iter().map(|x| x + 2.0).collect();
        let low: Vec<f64> = open.iter().map(|x| x - 2.0).collect();
        let close: Vec<f64> = open
            .iter()
            .enumerate()
            .map(|(i, x)| x + if i % 3 == 0 { 0.1 } else { 1.0 })
            .collect();
        let expected = crate::pattern::cdl_gravestonedoji(&open, &high, &low, &close).unwrap();
        let mut state = CandleGravestoneDoji::new();
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
