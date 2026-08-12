use crate::{
    primitives::PairedMoments, MetricError, MetricInputKind, MetricResult, NanPolicy,
    PairedMetricInputState,
};

/// Market beta from aligned primary and benchmark simple returns.
#[derive(Debug, Clone)]
pub struct Beta {
    input: PairedMetricInputState,
    moments: PairedMoments,
}

impl Beta {
    /// Construct an empty state for aligned primary and benchmark input domains.
    pub fn new(nan_policy: NanPolicy) -> MetricResult<Self> {
        Ok(Self {
            input: PairedMetricInputState::unbound(nan_policy),
            moments: PairedMoments::new(),
        })
    }

    /// Append one aligned pair and return the current beta when defined.
    pub fn append(&mut self, primary: f64, benchmark: f64) -> MetricResult<Option<f64>> {
        if let Some((primary_return, benchmark_return)) = self.input.append(primary, benchmark)? {
            self.moments.append(primary_return, benchmark_return);
        }
        Ok(self.value())
    }

    /// Append equal-length aligned slices through the same persistent state.
    pub fn extend(&mut self, primary: &[f64], benchmark: &[f64]) -> MetricResult<Option<f64>> {
        let input = &mut self.input;
        let moments = &mut self.moments;
        input.extend_slices(primary, benchmark, |primary_return, benchmark_return| {
            moments.append(primary_return, benchmark_return);
        })?;
        Ok(self.value())
    }

    /// Return sample covariance divided by sample benchmark variance.
    ///
    /// At least two usable pairs and non-zero benchmark variance are required.
    pub fn value(&self) -> Option<f64> {
        let benchmark_variance = self.moments.benchmark_variance(1)?;
        if benchmark_variance == 0.0 {
            return None;
        }
        self.moments
            .covariance(1)
            .map(|covariance| covariance / benchmark_variance)
    }

    /// Return the current result without replaying prior observations.
    pub fn compute(&self) -> Option<f64> {
        self.value()
    }

    /// Restore fresh-state behavior while preserving the selected input modes.
    pub fn reset(&mut self) {
        self.input.reset();
        self.moments.reset();
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

crate::impl_paired_return_metric_lifecycle!(Beta);
