use crate::{
    primitives::PairedMoments, MetricError, MetricInputKind, MetricResult, NanPolicy,
    PairedMetricInputState,
};

/// Geometrically annualized excess return divided by market beta.
#[derive(Debug, Clone)]
pub struct TreynorRatio {
    input: PairedMetricInputState,
    moments: PairedMoments,
    excess_growth_product: f64,
    periods_per_year: f64,
    period_risk_free_rate: f64,
}

impl TreynorRatio {
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
                reason: "Treynor ratio requires returns, log returns, equity, or period P&L with initial equity",
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
            excess_growth_product: 1.0,
            periods_per_year,
            period_risk_free_rate,
        })
    }

    /// Append one aligned pair and return the current Treynor ratio when defined.
    pub fn append(&mut self, primary: f64, benchmark: f64) -> MetricResult<Option<f64>> {
        if let Some((primary_return, benchmark_return)) = self.input.append(primary, benchmark)? {
            let primary_excess = primary_return - self.period_risk_free_rate;
            let benchmark_excess = benchmark_return - self.period_risk_free_rate;
            self.moments.append(primary_excess, benchmark_excess);
            self.excess_growth_product *= 1.0 + primary_excess;
        }
        Ok(self.value())
    }

    /// Append equal-length aligned slices through the same persistent state.
    pub fn extend(&mut self, primary: &[f64], benchmark: &[f64]) -> MetricResult<Option<f64>> {
        let input = &mut self.input;
        let moments = &mut self.moments;
        let product = &mut self.excess_growth_product;
        let risk_free = self.period_risk_free_rate;
        input.extend_slices(primary, benchmark, |primary_return, benchmark_return| {
            let primary_excess = primary_return - risk_free;
            let benchmark_excess = benchmark_return - risk_free;
            moments.append(primary_excess, benchmark_excess);
            *product *= 1.0 + primary_excess;
        })?;
        Ok(self.value())
    }

    /// Return the PerformanceAnalytics-compatible Treynor ratio.
    ///
    /// The numerator is `prod(1 + primary_return - period_risk_free_rate)
    /// ^ (periods_per_year / n) - 1`. Beta is the covariance of primary and
    /// benchmark excess returns divided by benchmark excess-return variance.
    /// At least two usable pairs, a non-zero finite beta, and a finite real
    /// annualized excess return are required.
    pub fn value(&self) -> Option<f64> {
        let benchmark_variance = self.moments.benchmark_variance(1)?;
        if benchmark_variance == 0.0 {
            return None;
        }
        let beta = self.moments.covariance(1)? / benchmark_variance;
        if beta == 0.0 || !beta.is_finite() {
            return None;
        }
        let exponent = self.periods_per_year / self.input.len() as f64;
        let annualized_excess = self.excess_growth_product.powf(exponent) - 1.0;
        let ratio = annualized_excess / beta;
        ratio.is_finite().then_some(ratio)
    }

    /// Return the current result without replaying prior observations.
    pub fn compute(&self) -> Option<f64> {
        self.value()
    }

    /// Restore fresh-state behavior while preserving input and annual settings.
    pub fn reset(&mut self) {
        self.input.reset();
        self.moments.reset();
        self.excess_growth_product = 1.0;
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
