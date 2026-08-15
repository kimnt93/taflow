//! Persistent higher-high relation.

use crate::error::{TaError, TaResult};

/// Emit `1` when the current high exceeds the previous high, otherwise `0`.
#[derive(Debug, Clone, Default)]
pub struct HigherHigh {
    previous: Option<(f64, f64)>,
    value: Option<f64>,
}

impl HigherHigh {
    /// Create an empty state; the first appended bar is warm-up.
    pub fn new() -> Self {
        Self::default()
    }

    /// Append one high/low bar in chronological order.
    pub fn append(&mut self, high: f64, low: f64) -> Option<f64> {
        self.value = self
            .previous
            .map(|(previous_high, _)| f64::from(high > previous_high));
        self.previous = Some((high, low));
        self.value
    }

    /// Append aligned high/low slices directly into an aligned output history.
    pub fn extend_slices_into(
        &mut self,
        high: &[f64],
        low: &[f64],
        output: &mut Vec<f64>,
    ) -> TaResult<()> {
        let len = high.len();
        if low.len() != len {
            return Err(TaError::LengthMismatch {
                expected: len,
                got: low.len(),
            });
        }
        if len == 0 {
            return Ok(());
        }

        output.reserve(len);
        let first_value = self.previous.map_or(f64::NAN, |(previous_high, _)| {
            f64::from(high[0] > previous_high)
        });
        output.push(first_value);
        output.extend(high.windows(2).map(|pair| f64::from(pair[1] > pair[0])));

        self.previous = Some((high[len - 1], low[len - 1]));
        self.value = if self.value.is_none() && len == 1 && first_value.is_nan() {
            None
        } else {
            Some(*output.last().expect("non-empty batch has an output"))
        };
        Ok(())
    }

    /// Return the latest relation value, or `None` before two bars exist.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Restore fresh-state behavior.
    pub fn reset(&mut self) {
        self.previous = None;
        self.value = None;
    }
}
