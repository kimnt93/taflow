use crate::{MetricError, MetricInputKind, MetricInputState, MetricResult, NanPolicy};

/// Longest run of strictly negative observations.
#[derive(Debug, Clone)]
pub struct LongestLosingStreak {
    input: MetricInputState,
    current: usize,
    longest: usize,
}
impl LongestLosingStreak {
    /// Construct an empty state for returns, raw period P&L, or closed trades.
    pub fn new(nan_policy: NanPolicy) -> MetricResult<Self> {
        Ok(Self {
            input: MetricInputState::unbound(nan_policy),
            current: 0,
            longest: 0,
        })
    }
    /// Append one observation and return the longest losing run.
    pub fn append(&mut self, value: f64) -> MetricResult<Option<usize>> {
        if let Some(observation) = self.input.append(value)? {
            if observation < 0.0 {
                self.current += 1;
                self.longest = self.longest.max(self.current);
            } else {
                self.current = 0;
            }
        }
        Ok(self.value())
    }
    /// Append observations through the same state.
    pub fn extend(&mut self, values: &[f64]) -> MetricResult<Option<usize>> {
        for &value in values {
            self.append(value)?;
        }
        Ok(self.value())
    }
    /// Return longest losing run; a nonempty no-loss sample returns zero.
    pub fn value(&self) -> Option<usize> {
        (!self.input.is_empty()).then_some(self.longest)
    }
    /// Return current scalar without replay.
    pub fn compute(&self) -> Option<usize> {
        self.value()
    }
    /// Reset observations and streaks.
    pub fn reset(&mut self) {
        self.input.reset();
        self.current = 0;
        self.longest = 0;
    }
    /// Return valid observation count.
    pub fn len(&self) -> usize {
        self.input.len()
    }
    /// Return whether empty.
    pub fn is_empty(&self) -> bool {
        self.input.is_empty()
    }
}

crate::impl_observation_metric_lifecycle!(LongestLosingStreak);
