use crate::{
    primitives::DrawdownState, MetricError, MetricInputKind, MetricInputState, MetricResult,
    NanPolicy,
};

/// Mean absolute percentage drawdown over the complete return path.
#[derive(Debug, Clone)]
pub struct PainIndex {
    input: MetricInputState,
    drawdown: DrawdownState,
    absolute_drawdown_sum: f64,
}

impl PainIndex {
    /// Construct an empty state with an explicitly selected semantic input mode.
    pub fn new(nan_policy: NanPolicy) -> MetricResult<Self> {
        Ok(Self {
            input: MetricInputState::unbound(nan_policy),
            drawdown: DrawdownState::new(),
            absolute_drawdown_sum: 0.0,
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
            self.absolute_drawdown_sum += current_drawdown.abs();
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

    /// Return mean absolute drawdown, or `None` when no return was processed.
    pub fn value(&self) -> Option<f64> {
        let count = self.input.len();
        (count != 0).then(|| self.absolute_drawdown_sum / count as f64)
    }

    /// Return the current result without replaying prior observations.
    pub fn compute(&self) -> Option<f64> {
        self.value()
    }

    /// Restore fresh-state behavior while preserving input configuration.
    pub fn reset(&mut self) {
        self.input.reset();
        self.drawdown.reset();
        self.absolute_drawdown_sum = 0.0;
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

crate::impl_return_metric_lifecycle!(PainIndex);
