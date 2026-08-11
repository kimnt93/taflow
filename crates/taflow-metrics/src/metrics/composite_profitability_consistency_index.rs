use crate::{
    primitives::GainLossState, MetricError, MetricInputKind, MetricInputState, MetricResult,
    NanPolicy,
};

/// Profit factor multiplied by decisive win rate and payoff ratio.
#[derive(Debug, Clone)]
pub struct CompositeProfitabilityConsistencyIndex {
    input: MetricInputState,
    gains: GainLossState,
}
impl CompositeProfitabilityConsistencyIndex {
    /// Construct an empty return or closed-trade state.
    pub fn new(input_kind: MetricInputKind, nan_policy: NanPolicy) -> MetricResult<Self> {
        if !matches!(
            input_kind,
            MetricInputKind::Returns | MetricInputKind::Trades
        ) {
            return Err(MetricError::InvalidParameter {
                name: "input_kind",
                value: format!("{input_kind:?}"),
                reason:
                    "composite profitability consistency index accepts returns or closed trades",
            });
        }
        Ok(Self {
            input: MetricInputState::new(input_kind, nan_policy)?,
            gains: GainLossState::new(),
        })
    }
    /// Append one observation and return the composite when defined.
    pub fn append(&mut self, value: f64) -> MetricResult<Option<f64>> {
        if let Some(observation) = self.input.append(value)? {
            self.gains.append(observation);
        }
        Ok(self.value())
    }
    /// Append observations through the same state.
    pub fn extend(&mut self, values: &[f64]) -> MetricResult<Option<f64>> {
        for &value in values {
            self.append(value)?;
        }
        Ok(self.value())
    }
    /// Return profit factor times decisive win rate times payoff ratio.
    pub fn value(&self) -> Option<f64> {
        let wins = self.gains.gain_count();
        let losses = self.gains.loss_count();
        if wins == 0 || losses == 0 {
            return None;
        }
        let gross_gain = self.gains.gross_gain();
        let gross_loss = self.gains.gross_loss().abs();
        let profit_factor = gross_gain / gross_loss;
        let win_rate = wins as f64 / (wins + losses) as f64;
        let payoff = (gross_gain / wins as f64) / (gross_loss / losses as f64);
        Some(profit_factor * win_rate * payoff)
    }
    /// Return current scalar without replay.
    pub fn compute(&self) -> Option<f64> {
        self.value()
    }
    /// Reset while preserving domain.
    pub fn reset(&mut self) {
        self.input.reset();
        self.gains.reset();
    }
    /// Return valid observation count, including breakevens.
    pub fn len(&self) -> usize {
        self.input.len()
    }
    /// Return whether no observations were processed.
    pub fn is_empty(&self) -> bool {
        self.input.is_empty()
    }
}
