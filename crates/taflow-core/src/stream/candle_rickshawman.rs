//! Incremental Rickshaw Man candlestick recognition (CDLRICKSHAWMAN).

use std::collections::VecDeque;

/// Incremental CDLRICKSHAWMAN state using TA-Lib's doji and near range averages.
pub struct CandleRickshawman {
    body_ranges: VecDeque<f64>,
    body_sum: f64,
    near_ranges: VecDeque<f64>,
    near_sum: f64,
    value: Option<i32>,
}
impl Default for CandleRickshawman {
    fn default() -> Self {
        Self::new()
    }
}
impl CandleRickshawman {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new() -> Self {
        Self {
            body_ranges: VecDeque::with_capacity(10),
            body_sum: 0.0,
            near_ranges: VecDeque::with_capacity(5),
            near_sum: 0.0,
            value: None,
        }
    }
    fn push(window: &mut VecDeque<f64>, sum: &mut f64, capacity: usize, value: f64) {
        if window.len() == capacity {
            *sum -= window.pop_front().expect("window is full");
        }
        window.push_back(value);
        *sum += value;
    }
    /// Appends OHLC data and returns +100 for a rickshaw man after the ten-bar warmup.
    pub fn append(&mut self, open: f64, high: f64, low: f64, close: f64) -> Option<i32> {
        let range = high - low;
        let body = (close - open).abs();
        let output = if self.body_ranges.len() == 10 && self.near_ranges.len() == 5 {
            let midpoint = low + range / 2.0;
            let near = self.near_sum * 0.04;
            Some(
                (body <= self.body_sum * 0.01
                    && open.min(close) - low > body
                    && high - open.max(close) > body
                    && open.min(close) <= midpoint + near
                    && open.max(close) >= midpoint - near) as i32
                    * 100,
            )
        } else {
            None
        };
        Self::push(&mut self.body_ranges, &mut self.body_sum, 10, range);
        Self::push(&mut self.near_ranges, &mut self.near_sum, 5, range);
        self.value = output;
        output
    }
    /// Computes or updates `value` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn value(&self) -> Option<i32> {
        self.value
    }
    /// Reset the persistent state and clear the latest value.
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
        let expected = crate::stream::candle_rickshawman(&open, &high, &low, &close).unwrap();
        let mut state = CandleRickshawman::new();
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
