use crate::{
    primitives::GainLossState, MetricError, MetricInputKind, MetricInputState, MetricResult,
    NanPolicy,
};

/// Expected P&L per period or closed trade, including breakeven observations.
#[derive(Debug, Clone)]
pub struct Expectancy {
    input: MetricInputState,
    observations: GainLossState,
}

impl Expectancy {
    /// Construct an empty state for raw period P&L or closed-trade P&L.
    pub fn new(input_kind: MetricInputKind, nan_policy: NanPolicy) -> MetricResult<Self> {
        if !matches!(
            input_kind,
            MetricInputKind::RawPnl | MetricInputKind::Trades
        ) {
            return Err(MetricError::InvalidParameter {
                name: "input_kind",
                value: format!("{input_kind:?}"),
                reason: "expectancy requires raw period P&L or realized closed-trade P&L",
            });
        }
        Ok(Self {
            input: MetricInputState::new(input_kind, nan_policy)?,
            observations: GainLossState::new(),
        })
    }

    /// Append one P&L observation and return the current expectancy.
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

    /// Return win probability times average win plus loss probability times average loss.
    ///
    /// Both probabilities use all usable observations as their denominator, so
    /// breakevens contribute zero. Empty input returns `None`.
    pub fn value(&self) -> Option<f64> {
        let count = self.observations.len();
        (count != 0).then(|| {
            (self.observations.gross_gain() + self.observations.gross_loss()) / count as f64
        })
    }

    /// Return the current result without replaying prior observations.
    pub fn compute(&self) -> Option<f64> {
        self.value()
    }

    /// Restore fresh-state behavior while preserving input configuration.
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
