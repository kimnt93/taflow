use crate::{
    primitives::DrawdownState, MetricError, MetricInputKind, MetricInputState, MetricResult,
    NanPolicy,
};

/// Root-mean-square percentage drawdown over the complete return path.
#[derive(Debug, Clone)]
pub struct UlcerIndex {
    input: MetricInputState,
    drawdown: DrawdownState,
    squared_drawdown_sum: f64,
}

impl UlcerIndex {
    /// Construct an empty state with an explicitly selected semantic input mode.
    pub fn new(nan_policy: NanPolicy) -> MetricResult<Self> {
        Ok(Self {
            input: MetricInputState::unbound(nan_policy),
            drawdown: DrawdownState::new(),
            squared_drawdown_sum: 0.0,
        })
    }

    /// Append one chronological observation and return the index to date.
    pub fn append(&mut self, value: f64) -> MetricResult<Option<f64>> {
        if let Some(simple_return) = self.input.append(value)? {
            self.drawdown.append(simple_return)?;
            let current_drawdown = self
                .drawdown
                .current_drawdown()
                .expect("an appended return always produces a drawdown");
            self.squared_drawdown_sum += current_drawdown * current_drawdown;
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

    /// Return the positive root-mean-square drawdown, or `None` before two returns.
    pub fn value(&self) -> Option<f64> {
        let count = self.input.len();
        (count >= 2).then(|| (self.squared_drawdown_sum / (count - 1) as f64).sqrt())
    }

    /// Return the current result without replaying prior observations.
    pub fn compute(&self) -> Option<f64> {
        self.value()
    }

    /// Restore fresh-state behavior while preserving input configuration.
    pub fn reset(&mut self) {
        self.input.reset();
        self.drawdown.reset();
        self.squared_drawdown_sum = 0.0;
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

crate::impl_return_metric_lifecycle!(UlcerIndex);
