use crate::{
    primitives::GainLossState, MetricError, MetricInputKind, MetricInputState, MetricResult,
    NanPolicy,
};

/// Historical binary Kelly fraction from win probability and payoff ratio.
#[derive(Debug, Clone)]
pub struct KellyCriterion {
    input: MetricInputState,
    observations: GainLossState,
}

impl KellyCriterion {
    /// Construct an empty state for period returns or realized closed-trade P&L.
    pub fn new(nan_policy: NanPolicy) -> MetricResult<Self> {
        Ok(Self {
            input: MetricInputState::unbound(nan_policy),
            observations: GainLossState::new(),
        })
    }

    /// Append one chronological observation and return the Kelly fraction to date.
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

    /// Return the historical binary Kelly fraction when wins and losses exist.
    pub fn value(&self) -> Option<f64> {
        let win_count = self.observations.gain_count();
        let loss_count = self.observations.loss_count();
        let decisive_count = win_count + loss_count;
        if win_count == 0 || loss_count == 0 || decisive_count == 0 {
            return None;
        }

        let win_probability = win_count as f64 / decisive_count as f64;
        let loss_probability = loss_count as f64 / decisive_count as f64;
        let average_win = self.observations.average_gain()?;
        let average_loss = self.observations.average_loss()?;
        let payoff_ratio = average_win / average_loss.abs();
        if payoff_ratio == 0.0 || !payoff_ratio.is_finite() {
            return None;
        }
        let fraction = ((payoff_ratio * win_probability) - loss_probability) / payoff_ratio;
        fraction.is_finite().then_some(fraction)
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

    /// Return usable observations, including zero-valued breakevens.
    pub fn len(&self) -> usize {
        self.input.len()
    }

    /// Return whether no usable observations have been processed.
    pub fn is_empty(&self) -> bool {
        self.input.is_empty()
    }
}

crate::impl_return_trade_metric_lifecycle!(KellyCriterion);
