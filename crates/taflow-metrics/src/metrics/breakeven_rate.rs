use crate::{MetricError, MetricInputKind, MetricInputState, MetricResult, NanPolicy};

/// Fraction of valid observations exactly equal to zero.
#[derive(Debug, Clone)]
pub struct BreakevenRate {
    input: MetricInputState,
    breakevens: usize,
}

impl BreakevenRate {
    /// Construct an empty state for returns, raw period P&L, or closed trades.
    pub fn new(input_kind: MetricInputKind, nan_policy: NanPolicy) -> MetricResult<Self> {
        if !matches!(
            input_kind,
            MetricInputKind::Returns | MetricInputKind::RawPnl | MetricInputKind::Trades
        ) {
            return Err(MetricError::InvalidParameter {
                name: "input_kind",
                value: format!("{input_kind:?}"),
                reason: "breakeven rate accepts returns, raw period P&L, or closed trades",
            });
        }
        Ok(Self {
            input: MetricInputState::new(input_kind, nan_policy)?,
            breakevens: 0,
        })
    }
    /// Append one observation and return the fraction exactly equal to zero.
    pub fn append(&mut self, value: f64) -> MetricResult<Option<f64>> {
        if let Some(observation) = self.input.append(value)? {
            self.breakevens += usize::from(observation == 0.0);
        }
        Ok(self.value())
    }
    /// Append a slice through the same persistent state.
    pub fn extend(&mut self, values: &[f64]) -> MetricResult<Option<f64>> {
        for &value in values {
            self.append(value)?;
        }
        Ok(self.value())
    }
    /// Return exact-zero count divided by valid observation count.
    pub fn value(&self) -> Option<f64> {
        (!self.input.is_empty()).then(|| self.breakevens as f64 / self.input.len() as f64)
    }
    /// Return the current scalar without replaying input.
    pub fn compute(&self) -> Option<f64> {
        self.value()
    }
    /// Restore fresh-state behavior while preserving domain configuration.
    pub fn reset(&mut self) {
        self.input.reset();
        self.breakevens = 0;
    }
    /// Return valid observation count.
    pub fn len(&self) -> usize {
        self.input.len()
    }
    /// Return whether no valid observation was processed.
    pub fn is_empty(&self) -> bool {
        self.input.is_empty()
    }
}
