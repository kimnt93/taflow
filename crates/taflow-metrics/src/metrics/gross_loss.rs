use crate::{
    primitives::GainLossState, MetricError, MetricInputKind, MetricInputState, MetricResult,
    NanPolicy,
};

/// Signed sum of strictly negative raw P&L observations.
#[derive(Debug, Clone)]
pub struct GrossLoss {
    input: MetricInputState,
    observations: GainLossState,
}

impl GrossLoss {
    /// Construct an empty state for raw period P&L or realized closed-trade P&L.
    pub fn new(input_kind: MetricInputKind, nan_policy: NanPolicy) -> MetricResult<Self> {
        if !matches!(
            input_kind,
            MetricInputKind::RawPnl | MetricInputKind::Trades
        ) {
            return Err(MetricError::InvalidParameter {
                name: "input_kind",
                value: format!("{input_kind:?}"),
                reason: "gross loss requires raw period P&L or realized closed-trade P&L",
            });
        }
        Ok(Self {
            input: MetricInputState::new(input_kind, nan_policy)?,
            observations: GainLossState::new(),
        })
    }

    /// Append one chronological P&L observation and return signed gross loss.
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

    /// Return signed gross loss, or `None` when no usable P&L was processed.
    pub fn value(&self) -> Option<f64> {
        (!self.input.is_empty()).then(|| self.observations.gross_loss())
    }

    /// Return the current result without replaying prior observations.
    pub fn compute(&self) -> Option<f64> {
        self.value()
    }

    /// Restore fresh-state behavior while preserving the P&L input domain.
    pub fn reset(&mut self) {
        self.input.reset();
        self.observations.reset();
    }

    /// Return the number of usable P&L observations processed.
    pub fn len(&self) -> usize {
        self.input.len()
    }

    /// Return whether no usable observations have been processed.
    pub fn is_empty(&self) -> bool {
        self.input.is_empty()
    }
}
