use crate::{
    primitives::ExactOrderStatistics, MetricError, MetricInputKind, MetricInputState, MetricResult,
    NanPolicy,
};

/// Ratio of the absolute 95th-percentile return to the absolute 5th-percentile return.
#[derive(Debug, Clone)]
pub struct TailRatio {
    input: MetricInputState,
    order_statistics: ExactOrderStatistics,
}

impl TailRatio {
    /// Construct an empty exact tail-ratio state.
    pub fn new(input_kind: MetricInputKind, nan_policy: NanPolicy) -> MetricResult<Self> {
        if matches!(
            input_kind,
            MetricInputKind::RawPnl | MetricInputKind::Trades
        ) {
            return Err(MetricError::InvalidParameter {
                name: "input_kind",
                value: format!("{input_kind:?}"),
                reason: "tail ratio requires returns, log returns, equity, or period P&L with initial equity",
            });
        }

        Ok(Self {
            input: MetricInputState::new(input_kind, nan_policy)?,
            order_statistics: ExactOrderStatistics::new(),
        })
    }

    /// Append one chronological observation and return the current ratio.
    pub fn append(&mut self, value: f64) -> MetricResult<Option<f64>> {
        if let Some(simple_return) = self.input.append(value)? {
            self.order_statistics.append(simple_return);
        }
        Ok(self.value())
    }

    /// Append a chronological slice and refresh exact quantiles only once.
    pub fn extend(&mut self, values: &[f64]) -> MetricResult<Option<f64>> {
        for &value in values {
            if let Some(simple_return) = self.input.append(value)? {
                self.order_statistics.append(simple_return);
            }
        }
        Ok(self.value())
    }

    /// Return the current ratio, or `None` when empty or the lower tail is zero.
    pub fn value(&mut self) -> Option<f64> {
        let lower = self
            .order_statistics
            .quantile(0.05)
            .expect("TailRatio uses a valid fixed lower quantile")?;
        let upper = self
            .order_statistics
            .quantile(0.95)
            .expect("TailRatio uses a valid fixed upper quantile")?;
        let lower_magnitude = lower.abs();
        if lower_magnitude == 0.0 {
            None
        } else {
            Some(upper.abs() / lower_magnitude)
        }
    }

    /// Return the current exact scalar without replaying observations.
    pub fn compute(&mut self) -> Option<f64> {
        self.value()
    }

    /// Restore fresh-state behavior while preserving input configuration.
    pub fn reset(&mut self) {
        self.input.reset();
        self.order_statistics.reset();
    }

    /// Return the number of usable normalized returns retained.
    pub fn len(&self) -> usize {
        self.input.len()
    }

    /// Return whether no usable normalized returns have been retained.
    pub fn is_empty(&self) -> bool {
        self.input.is_empty()
    }
}
