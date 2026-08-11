use crate::{
    primitives::PairedMoments, MetricError, MetricInputKind, MetricResult, NanPolicy,
    PairedMetricInputState,
};

/// Annualized single-factor regression intercept for aligned simple returns.
#[derive(Debug, Clone)]
pub struct Alpha {
    input: PairedMetricInputState,
    moments: PairedMoments,
    periods_per_year: f64,
    period_risk_free_rate: f64,
}

impl Alpha {
    /// Construct an empty state for aligned primary and benchmark input domains.
    pub fn new(
        primary_input_kind: MetricInputKind,
        benchmark_input_kind: MetricInputKind,
        periods_per_year: f64,
        annual_risk_free_rate: f64,
        nan_policy: NanPolicy,
    ) -> MetricResult<Self> {
        if !periods_per_year.is_finite() || periods_per_year <= 0.0 {
            return Err(MetricError::InvalidParameter {
                name: "periods_per_year",
                value: periods_per_year.to_string(),
                reason: "must be finite and greater than zero",
            });
        }
        if !annual_risk_free_rate.is_finite() || annual_risk_free_rate <= -1.0 {
            return Err(MetricError::InvalidParameter {
                name: "annual_risk_free_rate",
                value: annual_risk_free_rate.to_string(),
                reason: "must be finite and greater than -1",
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
                reason:
                    "alpha requires returns, log returns, equity, or period P&L with initial equity",
            });
        }

        let period_risk_free_rate = (annual_risk_free_rate.ln_1p() / periods_per_year).exp_m1();
        Ok(Self {
            input: PairedMetricInputState::new(
                primary_input_kind,
                benchmark_input_kind,
                nan_policy,
            )?,
            moments: PairedMoments::new(),
            periods_per_year,
            period_risk_free_rate,
        })
    }

    /// Append one aligned pair and return current annualized alpha when defined.
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

    /// Return Empyrical-compatible annualized regression alpha.
    ///
    /// At least two usable pairs and benchmark population variance of at least
    /// `1e-30` are required. The per-period intercept is compounded as
    /// `(1 + intercept).powf(periods_per_year) - 1`.
    pub fn value(&self) -> Option<f64> {
        let benchmark_variance = self.moments.benchmark_variance(0)?;
        if self.moments.len() < 2 || benchmark_variance < 1.0e-30 {
            return None;
        }
        let beta = self.moments.covariance(0)? / benchmark_variance;
        let (primary_mean, benchmark_mean) = self.moments.means()?;
        let intercept = (primary_mean - self.period_risk_free_rate)
            - beta * (benchmark_mean - self.period_risk_free_rate);
        let annualized = (1.0 + intercept).powf(self.periods_per_year) - 1.0;
        annualized.is_finite().then_some(annualized)
    }

    /// Return the current result without replaying prior observations.
    pub fn compute(&self) -> Option<f64> {
        self.value()
    }

    /// Restore fresh-state behavior while preserving input and annual settings.
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
