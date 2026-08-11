use crate::{MetricError, MetricResult};

/// Stable chronological simple-return compounding state.
#[derive(Debug, Clone, Default)]
pub struct CompoundedGrowth {
    count: usize,
    logarithmic_growth: f64,
    total_loss: bool,
}

impl CompoundedGrowth {
    /// Construct an empty growth accumulator.
    pub fn new() -> Self {
        Self::default()
    }

    /// Append one finite simple return.
    pub fn append(&mut self, simple_return: f64) -> MetricResult<()> {
        if !simple_return.is_finite() || simple_return < -1.0 {
            return Err(MetricError::InvalidObservation {
                domain: "return",
                position: self.count,
                value: simple_return.to_string(),
                reason: "compounding requires a finite simple return greater than or equal to -1",
            });
        }
        if simple_return == -1.0 {
            self.total_loss = true;
        } else if !self.total_loss {
            self.logarithmic_growth += simple_return.ln_1p();
        }
        self.count += 1;
        Ok(())
    }

    /// Clear all observations.
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    /// Number of returns accumulated.
    pub fn len(&self) -> usize {
        self.count
    }

    /// Whether no returns have been accumulated.
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    /// Sum of `log1p` returns, or negative infinity after total loss.
    pub fn logarithmic_growth(&self) -> Option<f64> {
        (self.count != 0).then_some(if self.total_loss {
            f64::NEG_INFINITY
        } else {
            self.logarithmic_growth
        })
    }

    /// Compounded growth factor, or `None` when empty.
    pub fn growth_factor(&self) -> Option<f64> {
        (self.count != 0).then_some(if self.total_loss {
            0.0
        } else {
            self.logarithmic_growth.exp()
        })
    }

    /// Whether the accumulated path contains a negative-one return.
    pub fn is_total_loss(&self) -> bool {
        self.total_loss
    }
}
