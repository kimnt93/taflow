//! Incremental High-Wave candlestick recognition (CDLHIGHWAVE).

use std::collections::VecDeque;

/// Incremental CDLHIGHWAVE state using TA-Lib's rolling short-body average.
pub struct CandleHighWave {
    bodies: VecDeque<f64>,
    sum: f64,
    value: Option<i32>,
}
impl Default for CandleHighWave {
    fn default() -> Self {
        Self::new()
    }
}
impl CandleHighWave {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new() -> Self {
        Self {
            bodies: VecDeque::with_capacity(10),
            sum: 0.0,
            value: None,
        }
    }
    /// Appends OHLC data and returns a signed high-wave signal after the ten-bar warmup.
    pub fn append(&mut self, open: f64, high: f64, low: f64, close: f64) -> Option<i32> {
        let body = (close - open).abs();
        let output = if self.bodies.len() == 10 {
            let upper = high - open.max(close);
            let lower = open.min(close) - low;
            Some(
                (body < self.sum / 10.0 && upper > body * 2.0 && lower > body * 2.0) as i32
                    * if close >= open { 100 } else { -100 },
            )
        } else {
            None
        };
        if self.bodies.len() == 10 {
            self.sum -= self.bodies.pop_front().expect("window is full");
        }
        self.bodies.push_back(body);
        self.sum += body;
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
            .map(|(i, x)| x + if i % 3 == 0 { -0.1 } else { 0.1 })
            .collect();
        let expected = crate::stream::candle_high_wave(&open, &high, &low, &close).unwrap();
        let mut state = CandleHighWave::new();
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
