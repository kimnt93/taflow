use crate::{
    primitives::GainLossState, MetricError, MetricInputKind, MetricInputState, MetricResult,
    NanPolicy,
};

/// Mean strictly positive observation divided by absolute mean negative observation.
#[derive(Debug, Clone)]
pub struct PayoffRatio {
    input: MetricInputState,
    observations: GainLossState,
}

impl PayoffRatio {
    /// Construct an empty state for returns, raw period P&L, or closed trades.
    pub fn new(nan_policy: NanPolicy) -> MetricResult<Self> {
        Ok(Self {
            input: MetricInputState::unbound(nan_policy),
            observations: GainLossState::new(),
        })
    }

    /// Append one chronological observation and return the current ratio.
    pub fn append(&mut self, value: f64) -> MetricResult<Option<f64>> {
        if let Some(observation) = self.input.append(value)? {
            self.observations.append(observation);
        }
        Ok(self.value())
    }

    /// Append a chronological slice through the same persistent state.
    pub fn extend(&mut self, values: &[f64]) -> MetricResult<Option<f64>> {
        for &value in values {
            self.append(value)?;
        }
        Ok(self.value())
    }

    /// Return average win divided by absolute average loss when both exist.
    pub fn value(&self) -> Option<f64> {
        let average_win = self.observations.average_gain()?;
        let average_loss = self.observations.average_loss()?;
        let ratio = average_win / average_loss.abs();
        ratio.is_finite().then_some(ratio)
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

    /// Return the number of usable observations, including breakevens.
    pub fn len(&self) -> usize {
        self.input.len()
    }

    /// Return whether no usable observations have been processed.
    pub fn is_empty(&self) -> bool {
        self.input.is_empty()
    }
}

crate::impl_observation_metric_lifecycle!(PayoffRatio);
