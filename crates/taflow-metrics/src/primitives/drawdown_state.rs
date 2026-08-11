use crate::{MetricError, MetricResult};

/// Wealth, running-peak, and percentage-drawdown recurrence.
#[derive(Debug, Clone)]
pub struct DrawdownState {
    count: usize,
    wealth: f64,
    peak: f64,
    current_drawdown: f64,
    maximum_drawdown: f64,
}

impl Default for DrawdownState {
    fn default() -> Self {
        Self {
            count: 0,
            wealth: 1.0,
            peak: 1.0,
            current_drawdown: 0.0,
            maximum_drawdown: 0.0,
        }
    }
}

impl DrawdownState {
    /// Construct a state with phantom starting wealth of one.
    pub fn new() -> Self {
        Self::default()
    }

    /// Append one finite simple return greater than or equal to negative one.
    pub fn append(&mut self, simple_return: f64) -> MetricResult<()> {
        if !simple_return.is_finite() || simple_return < -1.0 {
            return Err(MetricError::InvalidObservation {
                domain: "return",
                position: self.count,
                value: simple_return.to_string(),
                reason: "drawdown requires a finite simple return greater than or equal to -1",
            });
        }
        self.wealth *= 1.0 + simple_return;
        self.peak = self.peak.max(self.wealth);
        self.current_drawdown = self.wealth / self.peak - 1.0;
        self.maximum_drawdown = self.maximum_drawdown.min(self.current_drawdown);
        self.count += 1;
        Ok(())
    }

    /// Restore phantom starting wealth and clear all observations.
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    /// Number of returns accumulated.
    pub fn len(&self) -> usize {
        self.count
    }

    /// Whether no returns have been accumulated.
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    /// Current wealth relative to the phantom starting value.
    pub fn wealth(&self) -> f64 {
        self.wealth
    }

    /// Current running peak wealth.
    pub fn peak(&self) -> f64 {
        self.peak
    }

    /// Latest non-positive percentage drawdown, or `None` when empty.
    pub fn current_drawdown(&self) -> Option<f64> {
        (self.count != 0).then_some(self.current_drawdown)
    }

    /// Most negative percentage drawdown, or `None` when empty.
    pub fn maximum_drawdown(&self) -> Option<f64> {
        (self.count != 0).then_some(self.maximum_drawdown)
    }
}
