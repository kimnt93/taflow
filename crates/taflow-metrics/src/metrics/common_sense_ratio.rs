use crate::{
    primitives::{ExactOrderStatistics, GainLossState},
    MetricError, MetricInputKind, MetricInputState, MetricResult, NanPolicy,
};

/// Profit factor multiplied by the exact 95th-to-5th percentile tail ratio.
#[derive(Debug, Clone)]
pub struct CommonSenseRatio {
    input: MetricInputState,
    observations: GainLossState,
    order_statistics: ExactOrderStatistics,
}

impl CommonSenseRatio {
    /// Construct an empty state for decimal simple returns.
    pub fn new(input_kind: MetricInputKind, nan_policy: NanPolicy) -> MetricResult<Self> {
        if input_kind != MetricInputKind::Returns {
            return Err(MetricError::InvalidParameter {
                name: "input_kind",
                value: format!("{input_kind:?}"),
                reason: "common sense ratio requires decimal simple returns",
            });
        }
        Ok(Self {
            input: MetricInputState::new(input_kind, nan_policy)?,
            observations: GainLossState::new(),
            order_statistics: ExactOrderStatistics::new(),
        })
    }

    /// Append one return and refresh the current composite ratio.
    pub fn append(&mut self, value: f64) -> MetricResult<Option<f64>> {
        if let Some(simple_return) = self.input.append(value)? {
            self.observations.append(simple_return);
            self.order_statistics.append(simple_return);
        }
        Ok(self.value())
    }

    /// Append a chronological slice and sort retained values only once.
    pub fn extend(&mut self, values: &[f64]) -> MetricResult<Option<f64>> {
        for &value in values {
            if let Some(simple_return) = self.input.append(value)? {
                self.observations.append(simple_return);
                self.order_statistics.append(simple_return);
            }
        }
        Ok(self.value())
    }

    /// Return profit factor multiplied by the absolute tail ratio.
    ///
    /// Empty input, zero gross loss, or a zero-magnitude lower percentile is
    /// undefined. A loss-only sample can validly return zero.
    pub fn value(&mut self) -> Option<f64> {
        let absolute_gross_loss = -self.observations.gross_loss();
        if absolute_gross_loss == 0.0 {
            return None;
        }
        let lower = self
            .order_statistics
            .quantile(0.05)
            .expect("CommonSenseRatio uses a valid fixed lower quantile")?;
        let upper = self
            .order_statistics
            .quantile(0.95)
            .expect("CommonSenseRatio uses a valid fixed upper quantile")?;
        let lower_magnitude = lower.abs();
        if lower_magnitude == 0.0 {
            return None;
        }
        let profit_factor = self.observations.gross_gain() / absolute_gross_loss;
        Some(profit_factor * upper.abs() / lower_magnitude)
    }

    /// Return the current exact scalar without replaying observations.
    pub fn compute(&mut self) -> Option<f64> {
        self.value()
    }

    /// Restore fresh-state behavior while retaining allocated buffers.
    pub fn reset(&mut self) {
        self.input.reset();
        self.observations.reset();
        self.order_statistics.reset();
    }

    /// Return the number of usable simple returns retained.
    pub fn len(&self) -> usize {
        self.input.len()
    }

    /// Return whether no usable return has been retained.
    pub fn is_empty(&self) -> bool {
        self.input.is_empty()
    }
}
