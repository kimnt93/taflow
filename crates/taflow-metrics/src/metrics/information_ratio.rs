use crate::{
    primitives::OnlineMoments, MetricError, MetricInputKind, MetricResult, NanPolicy,
    PairedMetricInputState,
};

/// Mean aligned active return divided by its sample standard deviation.
#[derive(Debug, Clone)]
pub struct InformationRatio {
    input: PairedMetricInputState,
    active_return_moments: OnlineMoments,
    annualization_scale: f64,
}

impl InformationRatio {
    /// Construct an empty state for aligned primary and benchmark input domains.
    pub fn new(
        primary_input_kind: MetricInputKind,
        benchmark_input_kind: MetricInputKind,
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
        if matches!(
            primary_input_kind,
            MetricInputKind::RawPnl | MetricInputKind::Trades
        ) || matches!(
            benchmark_input_kind,
            MetricInputKind::RawPnl | MetricInputKind::Trades
        ) {
            return Err(MetricError::InvalidParameter {
                name: "input_kind",
                value: format!("{primary_input_kind:?}/{benchmark_input_kind:?}"),
                reason: "information ratio requires returns, log returns, equity, or period P&L with initial equity",
            });
        }

        Ok(Self {
            input: PairedMetricInputState::new(
                primary_input_kind,
                benchmark_input_kind,
                nan_policy,
            )?,
            active_return_moments: OnlineMoments::new(),
            annualization_scale: if annualized {
                periods_per_year.sqrt()
            } else {
                1.0
            },
        })
    }

    /// Append one aligned pair and return the current information ratio.
    pub fn append(&mut self, primary: f64, benchmark: f64) -> MetricResult<Option<f64>> {
        if let Some((primary_return, benchmark_return)) = self.input.append(primary, benchmark)? {
            self.active_return_moments
                .append(primary_return - benchmark_return);
        }
        Ok(self.value())
    }

    /// Append equal-length aligned slices through the same persistent state.
    pub fn extend(&mut self, primary: &[f64], benchmark: &[f64]) -> MetricResult<Option<f64>> {
        self.input
            .extend_slices(primary, benchmark, |primary_return, benchmark_return| {
                self.active_return_moments
                    .append(primary_return - benchmark_return);
            })?;
        Ok(self.value())
    }

    /// Return the ratio, or `None` before two pairs or at zero tracking error.
    pub fn value(&self) -> Option<f64> {
        let tracking_error = self.active_return_moments.standard_deviation(1)?;
        if tracking_error == 0.0 {
            return None;
        }
        Some(self.active_return_moments.mean()? / tracking_error * self.annualization_scale)
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
