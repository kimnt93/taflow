use crate::{
    primitives::DrawdownState, MetricError, MetricInputKind, MetricInputState, MetricResult,
    NanPolicy,
};

/// Longest drawdown episode length in usable observations.
#[derive(Debug, Clone)]
pub struct MaximumDrawdownDuration {
    input: MetricInputState,
    drawdown: DrawdownState,
    underwater_observations: usize,
    maximum_duration: usize,
}

impl MaximumDrawdownDuration {
    /// Construct an empty path-duration state.
    pub fn new(input_kind: MetricInputKind, nan_policy: NanPolicy) -> MetricResult<Self> {
        if matches!(
            input_kind,
            MetricInputKind::RawPnl | MetricInputKind::Trades
        ) {
            return Err(MetricError::InvalidParameter {
                name: "input_kind",
                value: format!("{input_kind:?}"),
                reason: "drawdown duration requires a normalized wealth-return path",
            });
        }
        Ok(Self {
            input: MetricInputState::new(input_kind, nan_policy)?,
            drawdown: DrawdownState::new(),
            underwater_observations: 0,
            maximum_duration: 0,
        })
    }

    /// Append one chronological observation and return the longest episode length.
    pub fn append(&mut self, value: f64) -> MetricResult<Option<usize>> {
        if let Some(simple_return) = self.input.append(value)? {
            self.drawdown.append(simple_return)?;
            let underwater = self
                .drawdown
                .current_drawdown()
                .is_some_and(|drawdown| drawdown < 0.0);
            if underwater {
                self.underwater_observations += 1;
                self.maximum_duration = self.maximum_duration.max(self.underwater_observations + 1);
            } else if self.underwater_observations != 0 {
                self.maximum_duration = self.maximum_duration.max(self.underwater_observations + 1);
                self.underwater_observations = 0;
            }
        }
        Ok(self.value())
    }

    /// Append a chronological slice through the same state.
    pub fn extend(&mut self, values: &[f64]) -> MetricResult<Option<usize>> {
        for &value in values {
            self.append(value)?;
        }
        Ok(self.value())
    }

    /// Return the longest negative drawdown run including its peak boundary.
    pub fn value(&self) -> Option<usize> {
        (self.maximum_duration != 0).then_some(self.maximum_duration)
    }
    /// Return the current result without replaying input.
    pub fn compute(&self) -> Option<usize> {
        self.value()
    }
    /// Restore fresh-state behavior while preserving input configuration.
    pub fn reset(&mut self) {
        self.input.reset();
        self.drawdown.reset();
        self.underwater_observations = 0;
        self.maximum_duration = 0;
    }
    /// Return the number of usable normalized returns processed.
    pub fn len(&self) -> usize {
        self.input.len()
    }
    /// Return whether no usable returns have been processed.
    pub fn is_empty(&self) -> bool {
        self.input.is_empty()
    }
}
