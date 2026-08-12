use crate::{
    primitives::CompoundedGrowth, MetricError, MetricInputKind, MetricInputState, MetricResult,
    NanPolicy,
};

/// Compounded return over every usable observation processed by this state.
#[derive(Debug, Clone)]
pub struct TotalReturn {
    input: MetricInputState,
    growth: CompoundedGrowth,
}

impl TotalReturn {
    /// Construct an empty state with an explicitly selected semantic input mode.
    pub fn new(nan_policy: NanPolicy) -> MetricResult<Self> {
        Ok(Self {
            input: MetricInputState::unbound(nan_policy),
            growth: CompoundedGrowth::new(),
        })
    }

    /// Append one chronological observation and return the current total return.
    pub fn append(&mut self, value: f64) -> MetricResult<Option<f64>> {
        if let Some(simple_return) = self.input.append(value)? {
            self.growth.append(simple_return)?;
        }
        Ok(self.value())
    }

    /// Append a chronological slice through the same persistent state.
    pub fn extend(&mut self, values: &[f64]) -> MetricResult<Option<f64>> {
        self.input
            .extend(values, |simple_return| self.growth.append(simple_return))?;
        Ok(self.value())
    }

    pub(crate) fn extend_normalized(&mut self, values: &[f64]) -> MetricResult<()> {
        self.input
            .extend_normalized_returns(values, |value| self.growth.append(value))
    }

    /// Return compounded simple return, or `None` when no usable return exists.
    pub fn value(&self) -> Option<f64> {
        self.growth.growth_factor().map(|factor| factor - 1.0)
    }

    /// Return the current result without replaying prior observations.
    pub fn compute(&self) -> Option<f64> {
        self.value()
    }

    /// Restore fresh-state behavior while preserving input configuration.
    pub fn reset(&mut self) {
        self.input.reset();
        self.growth.reset();
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

crate::impl_return_metric_lifecycle!(TotalReturn);
