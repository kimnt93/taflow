use crate::{
    primitives::DrawdownState, MetricError, MetricInputKind, MetricInputState, MetricResult,
    NanPolicy,
};

/// Most negative peak-to-trough decline in compounded wealth.
#[derive(Debug, Clone)]
pub struct MaximumDrawdown {
    input: MetricInputState,
    drawdown: DrawdownState,
}

impl MaximumDrawdown {
    /// Construct an empty state with an explicitly selected semantic input mode.
    pub fn new(nan_policy: NanPolicy) -> MetricResult<Self> {
        Ok(Self {
            input: MetricInputState::unbound(nan_policy),
            drawdown: DrawdownState::new(),
        })
    }

    /// Append one chronological observation and return maximum drawdown to date.
    pub fn append(&mut self, value: f64) -> MetricResult<Option<f64>> {
        if let Some(simple_return) = self.input.append(value)? {
            self.drawdown.append(simple_return)?;
        }
        Ok(self.value())
    }

    /// Append a chronological slice through the same persistent state.
    pub fn extend(&mut self, values: &[f64]) -> MetricResult<Option<f64>> {
        self.input
            .extend(values, |simple_return| self.drawdown.append(simple_return))?;
        Ok(self.value())
    }

    pub(crate) fn extend_normalized(&mut self, values: &[f64]) -> MetricResult<()> {
        self.input
            .extend_normalized_returns(values, |value| self.drawdown.append(value))
    }

    /// Return the signed, non-positive maximum drawdown, or `None` when empty.
    pub fn value(&self) -> Option<f64> {
        self.drawdown.maximum_drawdown()
    }

    /// Return the current result without replaying prior observations.
    pub fn compute(&self) -> Option<f64> {
        self.value()
    }

    /// Restore fresh-state behavior while preserving input configuration.
    pub fn reset(&mut self) {
        self.input.reset();
        self.drawdown.reset();
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

crate::impl_return_metric_lifecycle!(MaximumDrawdown);
