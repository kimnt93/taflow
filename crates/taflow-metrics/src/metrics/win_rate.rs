use crate::{MetricError, MetricInputKind, MetricInputState, MetricResult, NanPolicy};

/// Share of decisive observations that are strictly positive.
#[derive(Debug, Clone)]
pub struct WinRate {
    input: MetricInputState,
    wins: usize,
    decisive_observations: usize,
}

impl WinRate {
    /// Construct an empty state for returns, raw period P&L, or closed trades.
    pub fn new(input_kind: MetricInputKind, nan_policy: NanPolicy) -> MetricResult<Self> {
        if !matches!(
            input_kind,
            MetricInputKind::Returns | MetricInputKind::RawPnl | MetricInputKind::Trades
        ) {
            return Err(MetricError::InvalidParameter {
                name: "input_kind",
                value: format!("{input_kind:?}"),
                reason: "win rate requires returns, raw period P&L, or closed-trade P&L",
            });
        }
        Ok(Self {
            input: MetricInputState::new(input_kind, nan_policy)?,
            wins: 0,
            decisive_observations: 0,
        })
    }

    /// Append one chronological observation and return the rate to date.
    pub fn append(&mut self, value: f64) -> MetricResult<Option<f64>> {
        if let Some(observation) = self.input.append(value)? {
            if observation > 0.0 {
                self.wins += 1;
                self.decisive_observations += 1;
            } else if observation < 0.0 {
                self.decisive_observations += 1;
            }
        }
        Ok(self.value())
    }

    /// Append a chronological slice through the same persistent state.
    pub fn extend(&mut self, values: &[f64]) -> MetricResult<Option<f64>> {
        self.input.extend(values, |observation| {
            if observation > 0.0 {
                self.wins += 1;
                self.decisive_observations += 1;
            } else if observation < 0.0 {
                self.decisive_observations += 1;
            }
            Ok(())
        })?;
        Ok(self.value())
    }

    /// Return wins divided by non-zero observations, or `None` without one.
    pub fn value(&self) -> Option<f64> {
        (self.decisive_observations != 0)
            .then(|| self.wins as f64 / self.decisive_observations as f64)
    }

    /// Return the current result without replaying prior observations.
    pub fn compute(&self) -> Option<f64> {
        self.value()
    }

    /// Restore fresh-state behavior while preserving input configuration.
    pub fn reset(&mut self) {
        self.input.reset();
        self.wins = 0;
        self.decisive_observations = 0;
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
