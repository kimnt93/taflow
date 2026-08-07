//! Incremental Marubozu candlestick recognition (CDLMARUBOZU).

use std::collections::VecDeque;

/// Incremental CDLMARUBOZU state using TA-Lib's long-body and very-short-shadow averages.
pub struct CandleMarubozu {
    bodies: VecDeque<f64>,
    body_sum: f64,
    ranges: VecDeque<f64>,
    range_sum: f64,
    value: Option<i32>,
}
impl Default for CandleMarubozu {
    fn default() -> Self {
        Self::new()
    }
}
impl CandleMarubozu {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new() -> Self {
        Self {
            bodies: VecDeque::with_capacity(10),
            body_sum: 0.0,
            ranges: VecDeque::with_capacity(10),
            range_sum: 0.0,
            value: None,
        }
    }
    fn push(window: &mut VecDeque<f64>, sum: &mut f64, value: f64) {
        if window.len() == 10 {
            *sum -= window.pop_front().expect("window is full");
        }
        window.push_back(value);
        *sum += value;
    }
    /// Appends OHLC data and returns a signed marubozu signal after the ten-bar warmup.
    pub fn append(&mut self, open: f64, high: f64, low: f64, close: f64) -> Option<i32> {
        let body = (close - open).abs();
        let range = high - low;
        let output = if self.bodies.len() == 10 && self.ranges.len() == 10 {
            let upper = high - open.max(close);
            let lower = open.min(close) - low;
            Some(
                (body > self.body_sum / 10.0
                    && upper < self.range_sum * 0.01
                    && lower < self.range_sum * 0.01) as i32
                    * if close >= open { 100 } else { -100 },
            )
        } else {
            None
        };
        Self::push(&mut self.bodies, &mut self.body_sum, body);
        Self::push(&mut self.ranges, &mut self.range_sum, range);
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
        let open: Vec<f64> = (0..40).map(|i| 100.0 + i as f64 * 0.2).collect();
        let high: Vec<f64> = open.iter().map(|x| x + 2.0).collect();
        let low: Vec<f64> = open.iter().map(|x| x - 2.0).collect();
        let close: Vec<f64> = open
            .iter()
            .enumerate()
            .map(|(i, x)| x + if i % 3 == 0 { 0.1 } else { 1.0 })
            .collect();
        let expected = crate::pattern::cdl_marubozu(&open, &high, &low, &close).unwrap();
        let mut state = CandleMarubozu::new();
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
