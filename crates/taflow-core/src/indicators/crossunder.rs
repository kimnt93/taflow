//! Persistent `Crossunder` state.

use crate::error::{TaError, TaResult};

#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `Crossunder`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct Crossunder {
    previous_left: Option<f64>,
    previous_right: Option<f64>,
    value: Option<f64>,
}

impl Crossunder {
    /// Create a new empty state.
    ///
    pub fn new() -> Self {
        Self {
            previous_left: None,
            previous_right: None,
            value: None,
        }
    }
    /// Append one causal observation and return the latest result.
    ///
    pub fn append(&mut self, left: f64, right: f64) -> f64 {
        let value = match (self.previous_left, self.previous_right) {
            (Some(pl), Some(pr)) if pl >= pr && left < right => 1.0,
            _ => 0.0,
        };
        self.previous_left = Some(left);
        self.previous_right = Some(right);
        self.value = Some(value);
        value
    }
    /// Append aligned slices using one direct comparison pass.
    pub fn extend_slices_into(
        &mut self,
        left: &[f64],
        right: &[f64],
        output: &mut Vec<f64>,
    ) -> TaResult<()> {
        let len = left.len();
        if right.len() != len {
            return Err(TaError::LengthMismatch {
                expected: len,
                got: right.len(),
            });
        }
        if len == 0 {
            return Ok(());
        }

        output.reserve(len);
        output.push(match (self.previous_left, self.previous_right) {
            (Some(previous_left), Some(previous_right))
                if previous_left >= previous_right && left[0] < right[0] =>
            {
                1.0
            }
            _ => 0.0,
        });
        output.extend(
            left.windows(2)
                .zip(right.windows(2))
                .map(|(left_pair, right_pair)| {
                    f64::from(left_pair[0] >= right_pair[0] && left_pair[1] < right_pair[1])
                }),
        );

        self.previous_left = Some(left[len - 1]);
        self.previous_right = Some(right[len - 1]);
        self.value = output.last().copied();
        Ok(())
    }
    /// Return the latest computed result, if warm-up is complete.
    ///
    pub fn value(&self) -> Option<f64> {
        self.value
    }
    /// Reset the state and clear its accumulated history.
    ///
    pub fn reset(&mut self) {
        self.previous_left = None;
        self.previous_right = None;
        self.value = None;
    }
}

impl Default for Crossunder {
    fn default() -> Self {
        Self::new()
    }
}
