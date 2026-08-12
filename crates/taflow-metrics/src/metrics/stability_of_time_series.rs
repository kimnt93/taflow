use crate::{
    primitives::PairedMoments, MetricError, MetricInputKind, MetricInputState, MetricResult,
    NanPolicy,
};

/// R-squared of cumulative log return regressed on observation number.
#[derive(Debug, Clone)]
pub struct StabilityOfTimeSeries {
    input: MetricInputState,
    moments: PairedMoments,
    cumulative_log_return: f64,
    total_loss: bool,
}

impl StabilityOfTimeSeries {
    /// Construct an empty state with an explicitly selected semantic input mode.
    pub fn new(nan_policy: NanPolicy) -> MetricResult<Self> {
        Ok(Self {
            input: MetricInputState::unbound(nan_policy),
            moments: PairedMoments::new(),
            cumulative_log_return: 0.0,
            total_loss: false,
        })
    }

    /// Append one chronological observation and return current path stability.
    pub fn append(&mut self, value: f64) -> MetricResult<Option<f64>> {
        if let Some(simple_return) = self.input.append(value)? {
            if simple_return == -1.0 {
                self.total_loss = true;
            } else if !self.total_loss {
                self.cumulative_log_return += simple_return.ln_1p();
                self.moments
                    .append((self.input.len() - 1) as f64, self.cumulative_log_return);
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

    /// Return regression R-squared, or `None` until two varying points exist.
    pub fn value(&self) -> Option<f64> {
        if self.total_loss || self.input.len() < 2 {
            return None;
        }
        self.moments
            .correlation()
            .map(|correlation| correlation * correlation)
    }

    /// Return the current result without replaying prior observations.
    pub fn compute(&self) -> Option<f64> {
        self.value()
    }

    /// Restore fresh-state behavior while preserving input configuration.
    pub fn reset(&mut self) {
        self.input.reset();
        self.moments.reset();
        self.cumulative_log_return = 0.0;
        self.total_loss = false;
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

crate::impl_return_metric_lifecycle!(StabilityOfTimeSeries);
