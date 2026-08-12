use crate::{MetricError, MetricInputKind, MetricInputState, MetricResult, NanPolicy};

/// Fraction of valid observations exactly equal to zero.
#[derive(Debug, Clone)]
pub struct BreakevenRate {
    input: MetricInputState,
    breakevens: usize,
}

impl BreakevenRate {
    /// Construct an empty state for returns, raw period P&L, or closed trades.
    pub fn new(nan_policy: NanPolicy) -> MetricResult<Self> {
        Ok(Self {
            input: MetricInputState::unbound(nan_policy),
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
        self.input.extend(values, |observation| {
            self.breakevens += usize::from(observation == 0.0);
            Ok(())
        })?;
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

crate::impl_observation_metric_lifecycle!(BreakevenRate);
