//! Stateful Ehlers Laguerre Relative Strength Index.

use crate::error::{TaError, TaResult};
use super::StreamingIndicator;

/// Computes Laguerre RSI with four causal Laguerre stages.
#[derive(Debug, Clone)]
pub struct LaguerreRelativeStrengthIndex {
    gamma: f64,
    stages: [f64; 4],
    value: Option<f64>,
}

impl LaguerreRelativeStrengthIndex {
    /// Creates the oscillator with `gamma` in the half-open interval `[0, 1)`.
    pub fn new(gamma: f64) -> TaResult<Self> {
        if !(0.0..1.0).contains(&gamma) {
            return Err(TaError::InvalidParameter { name: "gamma", value: gamma.to_string(), reason: "must be in [0, 1)" });
        }
        Ok(Self { gamma, stages: [0.0; 4], value: None })
    }
}

impl StreamingIndicator for LaguerreRelativeStrengthIndex {
    type Output = f64;

    fn append(&mut self, input: f64) -> Option<f64> {
        let g = self.gamma;
        let [a, b, c, d] = self.stages;
        let l0 = (1.0 - g) * input + g * a;
        let l1 = -g * l0 + a + g * b;
        let l2 = -g * l1 + b + g * c;
        let l3 = -g * l2 + c + g * d;
        let cu = (l0 - l1).max(0.0) + (l1 - l2).max(0.0) + (l2 - l3).max(0.0);
        let cd = (l1 - l0).max(0.0) + (l2 - l1).max(0.0) + (l3 - l2).max(0.0);
        self.stages = [l0, l1, l2, l3];
        self.value = Some(if cu + cd == 0.0 { 0.0 } else { cu / (cu + cd) });
        self.value
    }

    fn value(&self) -> Option<f64> { self.value }
    fn reset(&mut self) { self.stages = [0.0; 4]; self.value = None; }
}
