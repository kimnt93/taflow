use crate::{MetricError, MetricInputKind, MetricInputState, MetricResult, NanPolicy};

/// Mean per-period excess return divided by Cornish-Fisher modified value at risk.
#[derive(Debug, Clone)]
pub struct ModifiedSharpeRatio {
    input: MetricInputState,
    count: usize,
    mean: f64,
    second_central_moment: f64,
    third_central_moment: f64,
    fourth_central_moment: f64,
    periods_per_year: f64,
    annual_risk_free_rate: f64,
    period_risk_free_rate: f64,
    confidence_level: f64,
    lower_tail_quantile: f64,
}

impl ModifiedSharpeRatio {
    /// Construct an empty modified-Sharpe state.
    pub fn new(
        periods_per_year: f64,
        annual_risk_free_rate: f64,
        confidence_level: f64,
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
        if !confidence_level.is_finite() || confidence_level <= 0.5 || confidence_level >= 1.0 {
            return Err(MetricError::InvalidParameter {
                name: "confidence_level",
                value: confidence_level.to_string(),
                reason: "must be finite and strictly between 0.5 and 1",
            });
        }

        let period_risk_free_rate = (annual_risk_free_rate.ln_1p() / periods_per_year).exp_m1();
        if !period_risk_free_rate.is_finite() {
            return Err(MetricError::InvalidParameter {
                name: "annual_risk_free_rate",
                value: annual_risk_free_rate.to_string(),
                reason: "annual effective rate conversion must produce a finite per-period rate",
            });
        }

        Ok(Self {
            input: MetricInputState::unbound(nan_policy),
            count: 0,
            mean: 0.0,
            second_central_moment: 0.0,
            third_central_moment: 0.0,
            fourth_central_moment: 0.0,
            periods_per_year,
            annual_risk_free_rate,
            period_risk_free_rate,
            confidence_level,
            lower_tail_quantile: Self::inverse_standard_normal(1.0 - confidence_level),
        })
    }

    /// Append one chronological observation and return the ratio to date.
    pub fn append(&mut self, value: f64) -> MetricResult<Option<f64>> {
        if let Some(simple_return) = self.input.append(value)? {
            self.append_excess_return(simple_return - self.period_risk_free_rate);
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

    /// Return mean excess return divided by modified VaR when defined.
    pub fn value(&self) -> Option<f64> {
        if self.count < 2 {
            return None;
        }
        let count = self.count as f64;
        let population_variance = self.second_central_moment / count;
        if !population_variance.is_finite() || population_variance < 0.0 {
            return None;
        }

        // PerformanceAnalytics 2.1.0 uses all.equal(m2, 0.0) inside its
        // skewness and kurtosis estimators. Its default tolerance is sqrt(eps).
        let effectively_zero = population_variance.abs() <= f64::EPSILON.sqrt();
        let (skewness, excess_kurtosis) = if effectively_zero {
            (0.0, 0.0)
        } else {
            let population_third = self.third_central_moment / count;
            let population_fourth = self.fourth_central_moment / count;
            (
                population_third / population_variance.powf(1.5),
                population_fourth / population_variance.powi(2) - 3.0,
            )
        };
        if !skewness.is_finite() || !excess_kurtosis.is_finite() {
            return None;
        }

        let z = self.lower_tail_quantile;
        let adjusted_quantile =
            z + (z * z - 1.0) * skewness / 6.0 + (z.powi(3) - 3.0 * z) * excess_kurtosis / 24.0
                - (2.0 * z.powi(3) - 5.0 * z) * skewness * skewness / 36.0;
        let mut modified_value_at_risk =
            -self.mean - adjusted_quantile * population_variance.sqrt();

        // VaR(..., invert=FALSE) rejects inverse risk and caps risk at 100%.
        if !modified_value_at_risk.is_finite() || modified_value_at_risk < 0.0 {
            return None;
        }
        modified_value_at_risk = modified_value_at_risk.min(1.0);
        if modified_value_at_risk == 0.0 {
            return None;
        }
        let ratio = self.mean / modified_value_at_risk;
        ratio.is_finite().then_some(ratio)
    }

    /// Return the current result without replaying prior observations.
    pub fn compute(&self) -> Option<f64> {
        self.value()
    }

    /// Restore fresh-state behavior while preserving configuration.
    pub fn reset(&mut self) {
        self.input.reset();
        self.count = 0;
        self.mean = 0.0;
        self.second_central_moment = 0.0;
        self.third_central_moment = 0.0;
        self.fourth_central_moment = 0.0;
    }

    /// Return the number of usable normalized returns processed.
    pub fn len(&self) -> usize {
        self.input.len()
    }

    /// Return whether no usable normalized returns have been processed.
    pub fn is_empty(&self) -> bool {
        self.input.is_empty()
    }

    /// Return the configured observations per year.
    pub fn periods_per_year(&self) -> f64 {
        self.periods_per_year
    }

    /// Return the configured annual effective risk-free rate.
    pub fn annual_risk_free_rate(&self) -> f64 {
        self.annual_risk_free_rate
    }

    /// Return the configured confidence level.
    pub fn confidence_level(&self) -> f64 {
        self.confidence_level
    }

    fn append_excess_return(&mut self, value: f64) {
        let previous_count = self.count as f64;
        self.count += 1;
        let count = self.count as f64;
        let delta = value - self.mean;
        let delta_over_count = delta / count;
        let delta_over_count_squared = delta_over_count * delta_over_count;
        let term = delta * delta_over_count * previous_count;

        self.fourth_central_moment +=
            term * delta_over_count_squared * (count * count - 3.0 * count + 3.0)
                + 6.0 * delta_over_count_squared * self.second_central_moment
                - 4.0 * delta_over_count * self.third_central_moment;
        self.third_central_moment += term * delta_over_count * (count - 2.0)
            - 3.0 * delta_over_count * self.second_central_moment;
        self.second_central_moment += term;
        self.mean += delta_over_count;
    }

    fn inverse_standard_normal(probability: f64) -> f64 {
        // Peter J. Acklam's rational approximation. The validated public
        // confidence range keeps probability strictly inside (0, 0.5).
        const A: [f64; 6] = [
            -3.969_683_028_665_376e1,
            2.209_460_984_245_205e2,
            -2.759_285_104_469_687e2,
            1.383_577_518_672_69e2,
            -3.066_479_806_614_716e1,
            2.506_628_277_459_239,
        ];
        const B: [f64; 5] = [
            -5.447_609_879_822_406e1,
            1.615_858_368_580_409e2,
            -1.556_989_798_598_866e2,
            6.680_131_188_771_972e1,
            -1.328_068_155_288_572e1,
        ];
        const C: [f64; 6] = [
            -7.784_894_002_430_293e-3,
            -3.223_964_580_411_365e-1,
            -2.400_758_277_161_838,
            -2.549_732_539_343_734,
            4.374_664_141_464_968,
            2.938_163_982_698_783,
        ];
        const D: [f64; 4] = [
            7.784_695_709_041_462e-3,
            3.224_671_290_700_398e-1,
            2.445_134_137_142_996,
            3.754_408_661_907_416,
        ];
        const LOWER_BREAKPOINT: f64 = 0.024_25;

        if probability < LOWER_BREAKPOINT {
            let q = (-2.0 * probability.ln()).sqrt();
            return (((((C[0] * q + C[1]) * q + C[2]) * q + C[3]) * q + C[4]) * q + C[5])
                / ((((D[0] * q + D[1]) * q + D[2]) * q + D[3]) * q + 1.0);
        }
        let q = probability - 0.5;
        let r = q * q;
        (((((A[0] * r + A[1]) * r + A[2]) * r + A[3]) * r + A[4]) * r + A[5]) * q
            / (((((B[0] * r + B[1]) * r + B[2]) * r + B[3]) * r + B[4]) * r + 1.0)
    }
}

crate::impl_return_metric_lifecycle!(ModifiedSharpeRatio);
