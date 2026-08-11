use crate::{
    primitives::DrawdownState, MetricError, MetricInputKind, MetricInputState, MetricResult,
    NanPolicy,
};

/// Absolute arithmetic return sum divided by absolute maximum drawdown.
#[derive(Debug, Clone)]
pub struct RecoveryFactor {
    input: MetricInputState,
    drawdown: DrawdownState,
    return_sum: f64,
    return_sum_compensation: f64,
}

impl RecoveryFactor {
    /// Construct an empty state with an explicitly selected semantic input mode.
    pub fn new(input_kind: MetricInputKind, nan_policy: NanPolicy) -> MetricResult<Self> {
        if matches!(
            input_kind,
            MetricInputKind::RawPnl | MetricInputKind::Trades
        ) {
            return Err(MetricError::InvalidParameter {
                name: "input_kind",
                value: format!("{input_kind:?}"),
                reason: "recovery factor requires returns, log returns, equity, or period P&L with initial equity",
            });
        }
        Ok(Self {
            input: MetricInputState::new(input_kind, nan_policy)?,
            drawdown: DrawdownState::new(),
            return_sum: 0.0,
            return_sum_compensation: 0.0,
        })
    }

    /// Append one chronological observation and return the factor to date.
    pub fn append(&mut self, value: f64) -> MetricResult<Option<f64>> {
        if let Some(simple_return) = self.input.append(value)? {
            self.drawdown.append(simple_return)?;

            // Neumaier summation preserves O(1) state while reducing cancellation
            // error against the oracle's vectorized arithmetic sum.
            let updated_sum = self.return_sum + simple_return;
            if self.return_sum.abs() >= simple_return.abs() {
                self.return_sum_compensation += (self.return_sum - updated_sum) + simple_return;
            } else {
                self.return_sum_compensation += (simple_return - updated_sum) + self.return_sum;
            }
            self.return_sum = updated_sum;
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

    /// Return the recovery factor, or `None` while empty or without drawdown.
    pub fn value(&self) -> Option<f64> {
        let maximum_drawdown = self.drawdown.maximum_drawdown()?;
        if maximum_drawdown == 0.0 {
            return None;
        }
        let numerator = (self.return_sum + self.return_sum_compensation).abs();
        let factor = numerator / maximum_drawdown.abs();
        factor.is_finite().then_some(factor)
    }

    /// Return the current result without replaying prior observations.
    pub fn compute(&self) -> Option<f64> {
        self.value()
    }

    /// Restore fresh-state behavior while preserving input configuration.
    pub fn reset(&mut self) {
        self.input.reset();
        self.drawdown.reset();
        self.return_sum = 0.0;
        self.return_sum_compensation = 0.0;
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
