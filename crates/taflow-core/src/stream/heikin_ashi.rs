//! Stateful causal Heikin-Ashi OHLC transform.

use crate::error::TaResult;

/// Computes transformed open, high, low, and close values from OHLC bars.
#[derive(Debug, Clone)]
pub struct HeikinAshi {
    previous_open: Option<f64>,
    previous_close: Option<f64>,
    value: Option<(f64, f64, f64, f64)>,
}

impl HeikinAshi {
    /// Creates an empty Heikin-Ashi state.
    pub fn new() -> TaResult<Self> {
        Ok(Self { previous_open: None, previous_close: None, value: None })
    }

    /// Appends one OHLC bar and returns transformed OHLC values.
    pub fn append(&mut self, open: f64, high: f64, low: f64, close: f64) -> (f64, f64, f64, f64) {
        let transformed_close = (open + high + low + close) / 4.0;
        let transformed_open = match (self.previous_open, self.previous_close) {
            (Some(previous_open), Some(previous_close)) => (previous_open + previous_close) / 2.0,
            _ => (open + close) / 2.0,
        };
        let transformed_high = high.max(transformed_open).max(transformed_close);
        let transformed_low = low.min(transformed_open).min(transformed_close);
        let value = (transformed_open, transformed_high, transformed_low, transformed_close);
        self.previous_open = Some(transformed_open);
        self.previous_close = Some(transformed_close);
        self.value = Some(value);
        value
    }

    /// Returns the latest transformed OHLC tuple.
    pub fn value(&self) -> Option<(f64, f64, f64, f64)> { self.value }

    /// Clears previous-candle state.
    pub fn reset(&mut self) { self.previous_open = None; self.previous_close = None; self.value = None; }
}
