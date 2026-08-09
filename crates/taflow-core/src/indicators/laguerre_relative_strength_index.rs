//! Stateful Laguerre Relative Strength Index.

use crate::error::{TaError, TaResult};
use crate::stream::StreamingIndicator;

/// Ehlers' four-stage Laguerre oscillator on a zero-to-one-hundred scale.
#[derive(Debug, Clone)]
pub struct LaguerreRelativeStrengthIndex {
    gamma: f64,
    stages: [f64; 4],
    initialized: bool,
    value: Option<f64>,
}

impl LaguerreRelativeStrengthIndex {
    /// Creates an oscillator with `gamma` in the half-open interval `[0, 1)`.
    pub fn new(gamma: f64) -> TaResult<Self> {
        if !(0.0..1.0).contains(&gamma) {
            return Err(TaError::InvalidParameter {
                name: "gamma",
                value: gamma.to_string(),
                reason: "must be in [0, 1)",
            });
        }
        Ok(Self {
            gamma,
            stages: [0.0; 4],
            initialized: false,
            value: None,
        })
    }
}

impl StreamingIndicator for LaguerreRelativeStrengthIndex {
    type Output = f64;

    fn append(&mut self, input: f64) -> Option<f64> {
        if !self.initialized {
            self.stages = [input; 4];
            self.initialized = true;
            self.value = Some(0.0);
            return self.value;
        }

        let gamma = self.gamma;
        let [previous_stage_zero, previous_stage_one, previous_stage_two, previous_stage_three] =
            self.stages;
        let stage_zero = (1.0 - gamma) * input + gamma * previous_stage_zero;
        let stage_one = -gamma * stage_zero + previous_stage_zero + gamma * previous_stage_one;
        let stage_two = -gamma * stage_one + previous_stage_one + gamma * previous_stage_two;
        let stage_three = -gamma * stage_two + previous_stage_two + gamma * previous_stage_three;
        let upward = (stage_zero - stage_one).max(0.0)
            + (stage_one - stage_two).max(0.0)
            + (stage_two - stage_three).max(0.0);
        let downward = (stage_one - stage_zero).max(0.0)
            + (stage_two - stage_one).max(0.0)
            + (stage_three - stage_two).max(0.0);
        self.stages = [stage_zero, stage_one, stage_two, stage_three];
        let total = upward + downward;
        self.value = Some(if total == 0.0 {
            0.0
        } else {
            100.0 * upward / total
        });
        self.value
    }

    fn value(&self) -> Option<f64> {
        self.value
    }

    fn reset(&mut self) {
        self.stages = [0.0; 4];
        self.initialized = false;
        self.value = None;
    }

    fn extend_slice_into(&mut self, inputs: &[f64], output: &mut Vec<f64>) {
        output.reserve(inputs.len());
        for &input in inputs {
            output.push(self.append(input).unwrap_or(f64::NAN));
        }
    }
}
