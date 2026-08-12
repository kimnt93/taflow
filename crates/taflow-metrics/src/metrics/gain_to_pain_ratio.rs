use crate::{
    primitives::GainLossState, MetricError, MetricInputKind, MetricInputState, MetricResult,
    NanPolicy,
};

/// Net return sum divided by the absolute sum of negative returns.
#[derive(Debug, Clone)]
pub struct GainToPainRatio {
    input: MetricInputState,
    returns: GainLossState,
}

impl GainToPainRatio {
    /// Construct an empty state with an explicitly selected semantic input mode.
    pub fn new(nan_policy: NanPolicy) -> MetricResult<Self> {
        Ok(Self {
            input: MetricInputState::unbound(nan_policy),
            returns: GainLossState::new(),
        })
    }

    /// Append one chronological observation and return the ratio to date.
    pub fn append(&mut self, value: f64) -> MetricResult<Option<f64>> {
        if let Some(simple_return) = self.input.append(value)? {
            self.returns.append(simple_return);
        }
        Ok(self.value())
    }

    /// Append a chronological slice through the same persistent state.
    pub fn extend(&mut self, values: &[f64]) -> MetricResult<Option<f64>> {
        self.input.extend(values, |simple_return| {
            self.returns.append(simple_return);
            Ok(())
        })?;
        Ok(self.value())
    }

    /// Return net gains divided by pain, or `None` without a negative return.
    pub fn value(&self) -> Option<f64> {
        let pain = -self.returns.gross_loss();
        (pain > 0.0).then(|| (self.returns.gross_gain() + self.returns.gross_loss()) / pain)
    }

    /// Return the current result without replaying prior observations.
    pub fn compute(&self) -> Option<f64> {
        self.value()
    }

    /// Restore fresh-state behavior while preserving input configuration.
    pub fn reset(&mut self) {
        self.input.reset();
        self.returns.reset();
    }

    /// Return the number of usable normalized returns processed.
    pub fn len(&self) -> usize {
        self.input.len()
    }

    /// Return whether no usable normalized returns have been processed.
    pub fn is_empty(&self) -> bool {
        self.input.is_empty()
    }
}

crate::impl_return_metric_lifecycle!(GainToPainRatio);
