//! Incremental Long Line candlestick recognition (CDLLONGLINE).

use std::collections::VecDeque;

/// Incremental CDLLONGLINE state using TA-Lib's long-body and short-shadow averages.
pub struct CandleLongLine {
    bodies: VecDeque<f64>,
    body_sum: f64,
    shadows: VecDeque<f64>,
    shadow_sum: f64,
    value: Option<i32>,
}

impl Default for CandleLongLine {
    fn default() -> Self {
        Self::new()
    }
}

impl CandleLongLine {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new() -> Self {
        Self {
            bodies: VecDeque::with_capacity(10),
            body_sum: 0.0,
            shadows: VecDeque::with_capacity(10),
            shadow_sum: 0.0,
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
    /// Appends OHLC data and returns a signed long-line signal after warmup.
    pub fn append(&mut self, open: f64, high: f64, low: f64, close: f64) -> Option<i32> {
        let body = (close - open).abs();
        let upper = high - open.max(close);
        let lower = open.min(close) - low;
        let value = if self.bodies.len() == 10 && self.shadows.len() == 10 {
            Some(
                (body > self.body_sum / 10.0
                    && upper < self.shadow_sum / 10.0
                    && lower < self.shadow_sum / 10.0) as i32
                    * if close >= open { 100 } else { -100 },
            )
        } else {
            None
        };
        Self::push(&mut self.bodies, &mut self.body_sum, body);
        Self::push(&mut self.shadows, &mut self.shadow_sum, upper + lower);
        self.value = value;
        value
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
            .map(|(i, x)| x + if i % 3 == 0 { -1.0 } else { 1.0 })
            .collect();
        let expected = crate::pattern::cdl_longline(&open, &high, &low, &close).unwrap();
        let mut state = CandleLongLine::new();
        for (((&o, &h), &l), (&c, &expected)) in open
            .iter()
            .zip(&high)
            .zip(&low)
            .zip(close.iter().zip(&expected))
        {
            assert_eq!(state.append(o, h, l, c).unwrap_or(0), expected);
        }
    }
}
