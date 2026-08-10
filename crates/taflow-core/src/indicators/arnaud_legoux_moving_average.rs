//! Persistent Arnaud Legoux moving average state.

use crate::error::{TaError, TaResult};
use crate::stream::operator_states::validate_period;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `ArnaudLegouxMovingAverage`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct ArnaudLegouxMovingAverage {
    values: VecDeque<f64>,
    period: usize,
    weights: Vec<f64>,
    value: Option<f64>,
}

impl ArnaudLegouxMovingAverage {
    /// Create a new empty state.
    ///
    pub fn new(period: usize, offset: f64, sigma: f64) -> TaResult<Self> {
        validate_period(period)?;
        if !(0.0..=1.0).contains(&offset) || sigma <= 0.0 {
            return Err(TaError::InvalidParameter {
                name: "offset/sigma",
                value: format!("{offset}/{sigma}"),
                reason: "offset must be 0..1 and sigma must be positive",
            });
        }
        let m = offset * (period - 1) as f64;
        let scale = period as f64 / sigma;
        let denominator = 2.0 * scale * scale;
        let weights = (0..period)
            .map(|i| (-(i as f64 - m).powi(2) / denominator).exp())
            .collect();
        Ok(Self {
            values: VecDeque::with_capacity(period),
            period,
            weights,
            value: None,
        })
    }
    /// Append one causal observation and return the latest result.
    ///
    pub fn append(&mut self, input: f64) -> Option<f64> {
        if self.values.len() == self.period {
            self.values.pop_front();
        }
        self.values.push_back(input);
        self.value = (self.values.len() == self.period).then(|| {
            let total = self.weights.iter().sum::<f64>();
            self.values
                .iter()
                .zip(&self.weights)
                .map(|(&v, &w)| v * w)
                .sum::<f64>()
                / total
        });
        self.value
    }
    /// Return the latest computed result, if warm-up is complete.
    ///
    pub fn value(&self) -> Option<f64> {
        self.value
    }
    /// Reset the state and clear its accumulated history.
    ///
    pub fn reset(&mut self) {
        self.values.clear();
        self.value = None;
    }
}
