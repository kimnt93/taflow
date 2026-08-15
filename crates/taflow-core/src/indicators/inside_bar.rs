//! Persistent inside-bar relation.

use crate::error::{TaError, TaResult};

/// Emit `1` when the current range is strictly inside the previous range.
#[derive(Debug, Clone, Default)]
pub struct InsideBar {
    previous: Option<(f64, f64)>,
    value: Option<f64>,
}

impl InsideBar {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn append(&mut self, high: f64, low: f64) -> Option<f64> {
        self.value = self.previous.map(|(previous_high, previous_low)| {
            f64::from(high < previous_high && low > previous_low)
        });
        self.previous = Some((high, low));
        self.value
    }

    /// Append aligned high/low slices through a direct adjacent comparison.
    pub fn extend_slices_into(
        &mut self,
        high: &[f64],
        low: &[f64],
        output: &mut Vec<f64>,
    ) -> TaResult<()> {
        Self::validate_lengths(high, low)?;
        if high.is_empty() {
            return Ok(());
        }
        let base = output.len();
        output.resize(base + high.len(), f64::NAN);
        output[base] = self.append(high[0], low[0]).unwrap_or(f64::NAN);
        for index in 1..high.len() {
            output[base + index] =
                f64::from(high[index] < high[index - 1] && low[index] > low[index - 1]);
        }
        if high.len() > 1 {
            self.previous = Some((*high.last().unwrap(), *low.last().unwrap()));
            self.value = Some(*output.last().unwrap());
        }
        Ok(())
    }

    pub fn value(&self) -> Option<f64> {
        self.value
    }

    pub fn reset(&mut self) {
        self.previous = None;
        self.value = None;
    }

    fn validate_lengths(high: &[f64], low: &[f64]) -> TaResult<()> {
        if high.len() != low.len() {
            return Err(TaError::LengthMismatch {
                expected: high.len(),
                got: low.len(),
            });
        }
        Ok(())
    }
}
