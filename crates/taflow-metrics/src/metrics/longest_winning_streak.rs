use crate::{MetricError, MetricInputKind, MetricInputState, MetricResult, NanPolicy};

/// Longest consecutive run of strictly positive observations.
#[derive(Debug, Clone)]
pub struct LongestWinningStreak {
    input: MetricInputState,
    current_streak: usize,
    longest_streak: usize,
}

impl LongestWinningStreak {
    /// Construct an empty state for returns, raw period P&L, or closed trades.
    pub fn new(input_kind: MetricInputKind, nan_policy: NanPolicy) -> MetricResult<Self> {
        if !matches!(
            input_kind,
            MetricInputKind::Returns | MetricInputKind::RawPnl | MetricInputKind::Trades
        ) {
            return Err(MetricError::InvalidParameter {
                name: "input_kind",
                value: format!("{input_kind:?}"),
                reason: "longest winning streak requires simple returns, raw period P&L, or closed-trade P&L",
            });
        }
        Ok(Self {
            input: MetricInputState::new(input_kind, nan_policy)?,
            current_streak: 0,
            longest_streak: 0,
        })
    }

    /// Append one observation and return the longest winning streak to date.
    pub fn append(&mut self, value: f64) -> MetricResult<Option<f64>> {
        if let Some(observation) = self.input.append(value)? {
            if observation > 0.0 {
                self.current_streak += 1;
                self.longest_streak = self.longest_streak.max(self.current_streak);
            } else {
                self.current_streak = 0;
            }
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

    /// Return the longest run length, or `None` when no observation exists.
    pub fn value(&self) -> Option<f64> {
        (!self.input.is_empty()).then_some(self.longest_streak as f64)
    }

    /// Return the current result without replaying prior observations.
    pub fn compute(&self) -> Option<f64> {
        self.value()
    }

    /// Restore fresh-state behavior while preserving input configuration.
    pub fn reset(&mut self) {
        self.input.reset();
        self.current_streak = 0;
        self.longest_streak = 0;
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
