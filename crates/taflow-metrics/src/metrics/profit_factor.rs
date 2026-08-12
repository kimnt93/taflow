use crate::{
    primitives::GainLossState, MetricError, MetricInputKind, MetricInputState, MetricResult,
    NanPolicy,
};

/// Gross positive observations divided by absolute gross negative observations.
#[derive(Debug, Clone)]
pub struct ProfitFactor {
    input: MetricInputState,
    observations: GainLossState,
}

impl ProfitFactor {
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
                    "profit factor requires simple returns, raw period P&L, or closed-trade P&L",
            });
        }
        Ok(Self {
            input: MetricInputState::new(input_kind, nan_policy)?,
            observations: GainLossState::new(),
        })
    }

    /// Append one observation and return the current profit factor.
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

    /// Return gross profit divided by absolute gross loss.
    ///
    /// Positive-only input returns positive infinity. Empty and all-zero input
    /// return `None`; loss-only input returns zero.
    pub fn value(&self) -> Option<f64> {
        let gross_profit = self.observations.gross_gain();
        let absolute_gross_loss = -self.observations.gross_loss();
        if absolute_gross_loss > 0.0 {
            Some(gross_profit / absolute_gross_loss)
        } else if gross_profit > 0.0 {
            Some(f64::INFINITY)
        } else {
            None
        }
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

    /// Return the number of usable observations processed.
    pub fn len(&self) -> usize {
        self.input.len()
    }

    /// Return whether no usable observations have been processed.
    pub fn is_empty(&self) -> bool {
        self.input.is_empty()
    }
}
