use crate::{
    primitives::GainLossState, MetricError, MetricInputKind, MetricInputState, MetricResult,
    NanPolicy,
};

/// Sum of strictly positive raw period or closed-trade P&L observations.
#[derive(Debug, Clone)]
pub struct GrossProfit {
    input: MetricInputState,
    observations: GainLossState,
}

impl GrossProfit {
    /// Construct an empty state for raw period P&L or closed-trade P&L.
    pub fn new(nan_policy: NanPolicy) -> MetricResult<Self> {
        Ok(Self {
            input: MetricInputState::unbound(nan_policy),
            observations: GainLossState::new(),
        })
    }

    /// Append one chronological P&L observation and return gross profit to date.
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

    /// Return summed positive P&L, or `None` when no observation exists.
    pub fn value(&self) -> Option<f64> {
        (!self.input.is_empty()).then(|| self.observations.gross_gain())
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

    /// Return whether no usable P&L observations have been processed.
    pub fn is_empty(&self) -> bool {
        self.input.is_empty()
    }
}

crate::impl_pnl_trade_metric_lifecycle!(GrossProfit);
