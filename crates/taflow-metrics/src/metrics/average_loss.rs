use crate::{
    primitives::GainLossState, MetricError, MetricInputKind, MetricInputState, MetricResult,
    NanPolicy,
};

/// Arithmetic mean of strictly negative observations, retaining its sign.
#[derive(Debug, Clone)]
pub struct AverageLoss {
    input: MetricInputState,
    observations: GainLossState,
}

impl AverageLoss {
    /// Construct an empty state for returns, raw period P&L, or closed trades.
    pub fn new(input_kind: MetricInputKind, nan_policy: NanPolicy) -> MetricResult<Self> {
        if !matches!(
            input_kind,
            MetricInputKind::Returns | MetricInputKind::RawPnl | MetricInputKind::Trades
        ) {
            return Err(MetricError::InvalidParameter {
                name: "input_kind",
                value: format!("{input_kind:?}"),
                reason:
                    "average loss requires returns, raw period P&L, or realized closed-trade P&L",
            });
        }
        Ok(Self {
            input: MetricInputState::new(input_kind, nan_policy)?,
            observations: GainLossState::new(),
        })
    }

    /// Append one chronological observation and return the mean negative value.
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

    /// Return the signed arithmetic mean of losses, or `None` without one.
    pub fn value(&self) -> Option<f64> {
        self.observations.average_loss()
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

    /// Return usable observations, including wins and breakevens.
    pub fn len(&self) -> usize {
        self.input.len()
    }

    /// Return whether no usable observations have been processed.
    pub fn is_empty(&self) -> bool {
        self.input.is_empty()
    }
}
