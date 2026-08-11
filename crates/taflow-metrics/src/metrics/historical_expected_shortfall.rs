use crate::{
    primitives::ExactOrderStatistics, MetricError, MetricInputKind, MetricInputState, MetricResult,
    NanPolicy,
};

/// Signed mean of the exact lower tail of normalized simple returns.
#[derive(Debug, Clone)]
pub struct HistoricalExpectedShortfall {
    input: MetricInputState,
    order_statistics: ExactOrderStatistics,
    cutoff: f64,
}

impl HistoricalExpectedShortfall {
    /// Construct an empty exact historical expected-shortfall state.
    pub fn new(
        input_kind: MetricInputKind,
        cutoff: f64,
        nan_policy: NanPolicy,
    ) -> MetricResult<Self> {
        if !cutoff.is_finite() || cutoff <= 0.0 || cutoff >= 1.0 {
            return Err(MetricError::InvalidParameter {
                name: "cutoff",
                value: cutoff.to_string(),
                reason: "must be finite and strictly between zero and one",
            });
        }
        if matches!(
            input_kind,
            MetricInputKind::RawPnl | MetricInputKind::Trades
        ) {
            return Err(MetricError::InvalidParameter {
                name: "input_kind",
                value: format!("{input_kind:?}"),
                reason: "historical expected shortfall requires returns, log returns, equity, or period P&L with initial equity",
            });
        }

        Ok(Self {
            input: MetricInputState::new(input_kind, nan_policy)?,
            order_statistics: ExactOrderStatistics::new(),
            cutoff,
        })
    }

    /// Append one chronological observation and return the current signed tail mean.
    pub fn append(&mut self, value: f64) -> MetricResult<Option<f64>> {
        if let Some(simple_return) = self.input.append(value)? {
            self.order_statistics.append(simple_return);
        }
        Ok(self.value())
    }

    /// Append a chronological slice and refresh the exact result once at the end.
    pub fn extend(&mut self, values: &[f64]) -> MetricResult<Option<f64>> {
        for &value in values {
            if let Some(simple_return) = self.input.append(value)? {
                self.order_statistics.append(simple_return);
            }
        }
        Ok(self.value())
    }

    /// Return the signed mean of the selected lower tail, or `None` when empty.
    ///
    /// The selected count is `floor((n - 1) * cutoff) + 1`, matching
    /// Empyrical Reloaded 0.5.12 `conditional_value_at_risk`. The retained
    /// observations are sorted only when this result is requested after input.
    pub fn value(&mut self) -> Option<f64> {
        self.order_statistics
            .lower_tail_mean(self.cutoff)
            .expect("HistoricalExpectedShortfall validates cutoff during construction")
    }

    /// Return the current exact scalar without replaying prior observations.
    pub fn compute(&mut self) -> Option<f64> {
        self.value()
    }

    /// Restore fresh-state behavior while preserving input mode and cutoff.
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
