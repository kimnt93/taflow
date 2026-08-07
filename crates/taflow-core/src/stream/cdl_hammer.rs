//! Incremental Hammer candlestick recognition (CDLHAMMER).

use std::collections::VecDeque;

/// Incremental CDLHAMMER state using TA-Lib's body, range, and near windows.
pub struct CandleHammer {
    bodies: VecDeque<f64>,
    body_sum: f64,
    ranges: VecDeque<f64>,
    range_sum: f64,
    near: VecDeque<f64>,
    near_sum: f64,
    previous: Option<(f64, f64)>,
    value: Option<i32>,
}
impl Default for CandleHammer {
    fn default() -> Self {
        Self::new()
    }
}
impl CandleHammer {
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
            near: VecDeque::with_capacity(5),
            near_sum: 0.0,
            previous: None,
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
    /// Appends OHLC data and returns +100 for a hammer after warmup.
    pub fn append(&mut self, open: f64, high: f64, low: f64, close: f64) -> Option<i32> {
        let range = high - low;
        let body = (close - open).abs();
        let output = if self.bodies.len() == 10 && self.ranges.len() == 10 && self.near.len() == 5 {
            let (previous_low, previous_range) = self.previous.expect("history exists");
            let short_body = body < self.body_sum / 10.0;
            let long_lower = open.min(close) - low > body;
            let short_upper = high - open.max(close) < self.range_sum * 0.01;
            let near_low = open.min(close) <= previous_low + self.near_sum * 0.04;
            let _ = previous_range;
            Some((short_body && long_lower && short_upper && near_low) as i32 * 100)
        } else {
            None
        };
        if let Some((_, previous_range)) = self.previous {
            Self::push(&mut self.near, &mut self.near_sum, 5, previous_range);
        }
        Self::push(&mut self.bodies, &mut self.body_sum, 10, body);
        Self::push(&mut self.ranges, &mut self.range_sum, 10, range);
        self.previous = Some((low, range));
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
    pub fn reset(&mut self) {
        *self = Self::new();
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn matches_batch() {
        let open: Vec<f64> = (0..50).map(|i| 100.0 + i as f64 * 0.1).collect();
        let high: Vec<f64> = open.iter().map(|x| x + 2.0).collect();
        let low: Vec<f64> = open.iter().map(|x| x - 2.0).collect();
        let close: Vec<f64> = open
            .iter()
            .enumerate()
            .map(|(i, x)| x + if i % 5 == 0 { 0.1 } else { 1.0 })
            .collect();
        let expected = crate::pattern::cdl_hammer(&open, &high, &low, &close).unwrap();
        let mut state = CandleHammer::new();
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
