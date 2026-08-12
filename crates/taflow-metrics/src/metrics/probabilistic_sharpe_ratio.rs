use crate::{MetricError, MetricInputKind, MetricInputState, MetricResult, NanPolicy};

/// Probability that the strategy Sharpe ratio exceeds a configured benchmark.
#[derive(Debug, Clone)]
pub struct ProbabilisticSharpeRatio {
    input: MetricInputState,
    count: usize,
    mean: f64,
    second_central_moment: f64,
    third_central_moment: f64,
    fourth_central_moment: f64,
    period_risk_free_rate: f64,
    period_benchmark_sharpe_ratio: f64,
}

impl ProbabilisticSharpeRatio {
    /// Construct an empty probabilistic-Sharpe state.
    pub fn new(
        periods_per_year: f64,
        annual_risk_free_rate: f64,
        annual_benchmark_sharpe_ratio: f64,
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
        if !annual_benchmark_sharpe_ratio.is_finite() {
            return Err(MetricError::InvalidParameter {
                name: "annual_benchmark_sharpe_ratio",
                value: annual_benchmark_sharpe_ratio.to_string(),
                reason: "must be finite",
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
            period_risk_free_rate,
            period_benchmark_sharpe_ratio: annual_benchmark_sharpe_ratio / periods_per_year.sqrt(),
        })
    }

    /// Append one chronological observation and return the probability to date.
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

    /// Return the probability that Sharpe exceeds the configured benchmark.
    pub fn value(&self) -> Option<f64> {
        // The bias-corrected sample excess-kurtosis estimator needs four
        // observations. This also avoids silently switching estimators during
        // warm-up.
        if self.count < 4 {
            return None;
        }

        let count = self.count as f64;
        let sample_variance = self.second_central_moment / (count - 1.0);
        if !sample_variance.is_finite() || sample_variance <= 0.0 {
            return None;
        }
        let period_sharpe_ratio = self.mean / sample_variance.sqrt();
        if !period_sharpe_ratio.is_finite() {
            return None;
        }

        let population_variance = self.second_central_moment / count;
        if !population_variance.is_finite() || population_variance <= 0.0 {
            return None;
        }
        let biased_skewness = (self.third_central_moment / count) / population_variance.powf(1.5);
        let sample_skewness = (count * (count - 1.0)).sqrt() / (count - 2.0) * biased_skewness;
        let biased_excess_kurtosis =
            (self.fourth_central_moment / count) / population_variance.powi(2) - 3.0;
        let sample_excess_kurtosis = (count - 1.0) / ((count - 2.0) * (count - 3.0))
            * ((count + 1.0) * biased_excess_kurtosis + 6.0);
        let sample_pearson_kurtosis = sample_excess_kurtosis + 3.0;
        if !sample_skewness.is_finite() || !sample_pearson_kurtosis.is_finite() {
            return None;
        }

        // Bailey and Lopez de Prado's PSR, matching vectorbt 0.28.5's DSR
        // kernel after replacing its estimated maximum Sharpe with the
        // explicitly configured benchmark. Both Sharpe values are per-period.
        let variance_adjustment = 1.0 - sample_skewness * period_sharpe_ratio
            + (sample_pearson_kurtosis - 1.0) * period_sharpe_ratio.powi(2) / 4.0;
        if !variance_adjustment.is_finite() || variance_adjustment <= 0.0 {
            return None;
        }
        let test_statistic = (period_sharpe_ratio - self.period_benchmark_sharpe_ratio)
            * (count - 1.0).sqrt()
            / variance_adjustment.sqrt();
        if !test_statistic.is_finite() {
            return None;
        }
        Some(Self::standard_normal_cdf(test_statistic))
    }

    /// Return the current result without replaying prior observations.
    pub fn compute(&self) -> Option<f64> {
        self.value()
    }

    /// Restore fresh-state behavior while preserving all configuration.
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

    fn append_excess_return(&mut self, value: f64) {
        // Pébay's one-pass recurrences: O(1) time and memory per observation,
        // with no allocation after construction.
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

    fn standard_normal_cdf(value: f64) -> f64 {
        // Hart's rational tail approximation. Relative error is below 1e-15
        // across the central range and tails saturate safely to zero or one.
        let absolute = value.abs();
        if absolute > 37.0 {
            return if value < 0.0 { 0.0 } else { 1.0 };
        }
        let exponential = (-0.5 * absolute * absolute).exp();
        let tail = if absolute < 7.071_067_811_865_47 {
            const NUMERATOR: [f64; 7] = [
                0.035_262_496_599_891_1,
                0.700_383_064_443_688,
                6.373_962_203_531_65,
                33.912_866_078_383,
                112.079_291_497_871,
                221.213_596_169_931,
                220.206_867_912_376,
            ];
            const DENOMINATOR: [f64; 8] = [
                0.088_388_347_648_318_4,
                1.755_667_163_182_64,
                16.064_177_579_207,
                86.780_732_202_946_1,
                296.564_248_779_674,
                637.333_633_378_831,
                793.826_512_519_948,
                440.413_735_824_752,
            ];
            let numerator = NUMERATOR
                .iter()
                .skip(1)
                .fold(NUMERATOR[0], |accumulator, coefficient| {
                    accumulator * absolute + coefficient
                })
                * exponential;
            let denominator = DENOMINATOR
                .iter()
                .skip(1)
                .fold(DENOMINATOR[0], |accumulator, coefficient| {
                    accumulator * absolute + coefficient
                });
            numerator / denominator
        } else {
            exponential
                / (absolute
                    + 1.0
                        / (absolute
                            + 2.0 / (absolute + 3.0 / (absolute + 4.0 / (absolute + 0.65)))))
                / (2.0 * std::f64::consts::PI).sqrt()
        };
        if value < 0.0 {
            tail
        } else {
            1.0 - tail
        }
    }
}

crate::impl_return_metric_lifecycle!(ProbabilisticSharpeRatio);
