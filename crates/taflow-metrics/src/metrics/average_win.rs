use crate::{
    primitives::GainLossState, MetricError, MetricInputKind, MetricInputState, MetricResult,
    NanPolicy,
};

/// Arithmetic mean of strictly positive observations.
#[derive(Debug, Clone)]
pub struct AverageWin {
    input: MetricInputState,
    observations: GainLossState,
}

impl AverageWin {
    /// Construct an empty state for returns, raw period P&L, or closed trades.
    pub fn new(nan_policy: NanPolicy) -> MetricResult<Self> {
        Ok(Self {
            input: MetricInputState::unbound(nan_policy),
            observations: GainLossState::new(),
        })
    }

    /// Append one chronological observation and return the mean positive value.
    pub fn append(&mut self, value: f64) -> MetricResult<Option<f64>> {
        if let Some(observation) = self.input.append(value)? {
            self.observations.append(observation);
        }
        Ok(self.value())
    }

    /// Append a chronological slice through the same persistent state.
    pub fn extend(&mut self, values: &[f64]) -> MetricResult<Option<f64>> {
        self.input.extend(values, |observation| {
            self.observations.append(observation);
            Ok(())
        })?;
        Ok(self.value())
    }

    /// Return the arithmetic mean of wins, or `None` when no win exists.
    pub fn value(&self) -> Option<f64> {
        self.observations.average_gain()
    }

    /// Return the current result without replaying prior observations.
    pub fn compute(&self) -> Option<f64> {
        self.value()
    }

    /// Restore fresh-state behavior while preserving the semantic input domain.
    pub fn reset(&mut self) {
        self.input.reset();
        self.observations.reset();
    }

    /// Return the number of usable observations, including losses and breakevens.
    pub fn len(&self) -> usize {
        self.input.len()
    }

    /// Return whether no usable observations have been processed.
    pub fn is_empty(&self) -> bool {
        self.input.is_empty()
    }
}

crate::impl_observation_metric_lifecycle!(AverageWin);
