use crate::{
    primitives::OnlineMoments, MetricError, MetricInputKind, MetricResult, NanPolicy,
    PairedMetricInputState,
};

/// Sample standard deviation of aligned active returns.
#[derive(Debug, Clone)]
pub struct TrackingError {
    input: PairedMetricInputState,
    active_return_moments: OnlineMoments,
    annualization_scale: f64,
}

impl TrackingError {
    /// Construct an empty state for aligned primary and benchmark input domains.
    pub fn new(
        periods_per_year: f64,
        annualized: bool,
        nan_policy: NanPolicy,
    ) -> MetricResult<Self> {
        if !periods_per_year.is_finite() || periods_per_year <= 0.0 {
            return Err(MetricError::InvalidParameter {
                name: "periods_per_year",
                value: periods_per_year.to_string(),
                reason: "must be finite and greater than zero",
            });
        }
        Ok(Self {
            input: PairedMetricInputState::unbound(nan_policy),
            active_return_moments: OnlineMoments::new(),
            annualization_scale: if annualized {
                periods_per_year.sqrt()
            } else {
                1.0
            },
        })
    }

    /// Append one aligned pair and return the current tracking error.
    pub fn append(&mut self, primary: f64, benchmark: f64) -> MetricResult<Option<f64>> {
        if let Some((primary_return, benchmark_return)) = self.input.append(primary, benchmark)? {
            self.active_return_moments
                .append(primary_return - benchmark_return);
        }
        Ok(self.value())
    }

    /// Append equal-length aligned slices through the same persistent state.
    pub fn extend(&mut self, primary: &[f64], benchmark: &[f64]) -> MetricResult<Option<f64>> {
        let input = &mut self.input;
        let active_return_moments = &mut self.active_return_moments;
        input.extend_slices(primary, benchmark, |primary_return, benchmark_return| {
            active_return_moments.append(primary_return - benchmark_return);
        })?;
        Ok(self.value())
    }

    /// Return sample active-return deviation, or `None` with fewer than two pairs.
    pub fn value(&self) -> Option<f64> {
        self.active_return_moments
            .standard_deviation(1)
            .map(|standard_deviation| standard_deviation * self.annualization_scale)
    }

    /// Return the current result without replaying prior observations.
    pub fn compute(&self) -> Option<f64> {
        self.value()
    }

    /// Restore fresh-state behavior while preserving input and annualization settings.
    pub fn reset(&mut self) {
        self.input.reset();
        self.active_return_moments.reset();
    }

    /// Return the number of usable aligned return pairs processed.
    pub fn len(&self) -> usize {
        self.input.len()
    }

    /// Return whether no usable aligned return pairs have been processed.
    pub fn is_empty(&self) -> bool {
        self.input.is_empty()
    }
}

crate::impl_paired_return_metric_lifecycle!(TrackingError);
