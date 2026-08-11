use crate::{
    primitives::DrawdownState, MetricError, MetricInputKind, MetricInputState, MetricResult,
    NanPolicy,
};

/// Mean trough magnitude across distinct drawdown episodes.
#[derive(Debug, Clone)]
pub struct AverageDrawdown {
    input: MetricInputState,
    drawdown: DrawdownState,
    completed_depth_sum: f64,
    completed_episode_count: usize,
    current_episode_depth: Option<f64>,
}

impl AverageDrawdown {
    /// Construct an empty state with an explicitly selected semantic input mode.
    pub fn new(input_kind: MetricInputKind, nan_policy: NanPolicy) -> MetricResult<Self> {
        if matches!(
            input_kind,
            MetricInputKind::RawPnl | MetricInputKind::Trades
        ) {
            return Err(MetricError::InvalidParameter {
                name: "input_kind",
                value: format!("{input_kind:?}"),
                reason: "average drawdown requires returns, log returns, equity, or period P&L with initial equity",
            });
        }
        Ok(Self {
            input: MetricInputState::new(input_kind, nan_policy)?,
            drawdown: DrawdownState::new(),
            completed_depth_sum: 0.0,
            completed_episode_count: 0,
            current_episode_depth: None,
        })
    }

    /// Append one chronological observation and return the current average.
    pub fn append(&mut self, value: f64) -> MetricResult<Option<f64>> {
        if let Some(simple_return) = self.input.append(value)? {
            self.drawdown.append(simple_return)?;
            let current_drawdown = self
                .drawdown
                .current_drawdown()
                .expect("an appended return always produces a drawdown");
            if current_drawdown < 0.0 {
                self.current_episode_depth = Some(
                    self.current_episode_depth
                        .map_or(current_drawdown, |depth| depth.min(current_drawdown)),
                );
            } else if let Some(depth) = self.current_episode_depth.take() {
                self.completed_depth_sum += depth.abs();
                self.completed_episode_count += 1;
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

    /// Return the mean positive trough magnitude across observed episodes.
    ///
    /// A current unrecovered episode participates alongside completed episodes.
    /// A non-empty path without a negative drawdown returns zero; an empty path
    /// returns `None`.
    pub fn value(&self) -> Option<f64> {
        if self.input.is_empty() {
            return None;
        }
        if let Some(depth) = self.current_episode_depth {
            return Some(
                (self.completed_depth_sum + depth.abs())
                    / (self.completed_episode_count + 1) as f64,
            );
        }
        if self.completed_episode_count == 0 {
            Some(0.0)
        } else {
            Some(self.completed_depth_sum / self.completed_episode_count as f64)
        }
    }

    /// Return the current result without replaying prior observations.
    pub fn compute(&self) -> Option<f64> {
        self.value()
    }

    /// Restore fresh-state behavior while preserving input configuration.
    pub fn reset(&mut self) {
        self.input.reset();
        self.drawdown.reset();
        self.completed_depth_sum = 0.0;
        self.completed_episode_count = 0;
        self.current_episode_depth = None;
    }

    /// Return the number of usable normalized returns processed.
    pub fn len(&self) -> usize {
        self.input.len()
    }

    /// Return whether no usable normalized returns have been processed.
    pub fn is_empty(&self) -> bool {
        self.input.is_empty()
    }
}
