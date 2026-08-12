use crate::{
    primitives::ExactOrderStatistics, MetricError, MetricInputKind, MetricInputState, MetricResult,
    NanPolicy,
};

/// Signed lower-tail linear quantile of normalized simple returns.
#[derive(Debug, Clone)]
pub struct HistoricalValueAtRisk {
    input: MetricInputState,
    order_statistics: ExactOrderStatistics,
    cutoff: f64,
}

impl HistoricalValueAtRisk {
    /// Construct an empty exact historical value-at-risk state.
    pub fn new(cutoff: f64, nan_policy: NanPolicy) -> MetricResult<Self> {
        if !cutoff.is_finite() || cutoff <= 0.0 || cutoff >= 1.0 {
            return Err(MetricError::InvalidParameter {
                name: "cutoff",
                value: cutoff.to_string(),
                reason: "must be finite and strictly between zero and one",
            });
        }

        Ok(Self {
            input: MetricInputState::unbound(nan_policy),
            order_statistics: ExactOrderStatistics::new(),
            cutoff,
        })
    }

    /// Append one chronological observation and return the current signed quantile.
    pub fn append(&mut self, value: f64) -> MetricResult<Option<f64>> {
        if let Some(simple_return) = self.input.append(value)? {
            self.order_statistics.append(simple_return);
        }
        Ok(self.value())
    }

    /// Append a chronological slice through the same persistent state.
    pub fn extend(&mut self, values: &[f64]) -> MetricResult<Option<f64>> {
        self.input.extend(values, |simple_return| {
            self.order_statistics.append(simple_return);
            Ok(())
        })?;
        Ok(self.value())
    }

    /// Return the current signed lower-tail quantile, or `None` when empty.
    ///
    /// Exact order statistics lazily refresh their sorted cache when new input
    /// has made it dirty. Already processed observations are never replayed.
    pub fn value(&mut self) -> Option<f64> {
        self.order_statistics
            .quantile(self.cutoff)
            .expect("HistoricalValueAtRisk validates cutoff during construction")
    }

    /// Return the current result without replaying prior observations.
    pub fn compute(&mut self) -> Option<f64> {
        self.value()
    }

    /// Restore fresh-state behavior while preserving mode and cutoff.
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

crate::impl_return_metric_lifecycle!(HistoricalValueAtRisk);
