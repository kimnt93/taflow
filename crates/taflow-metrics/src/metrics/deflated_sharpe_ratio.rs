use crate::{MetricError, MetricInputKind, MetricInputState, MetricResult, NanPolicy};

/// Probability that observed Sharpe exceeds the expected maximum across trials.
#[derive(Debug, Clone)]
pub struct DeflatedSharpeRatio {
    input: MetricInputState,
    count: usize,
    mean: f64,
    second_central_moment: f64,
    third_central_moment: f64,
    fourth_central_moment: f64,
    period_risk_free_rate: f64,
    expected_maximum_period_sharpe_ratio: f64,
}

impl DeflatedSharpeRatio {
    /// Construct a state using explicit independent-trial configuration.
    pub fn new(
        input_kind: MetricInputKind,
        periods_per_year: f64,
        annual_risk_free_rate: f64,
        number_of_trials: usize,
        annual_sharpe_ratio_variance: f64,
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
        if number_of_trials < 2 {
            return Err(MetricError::InvalidParameter {
                name: "number_of_trials",
                value: number_of_trials.to_string(),
                reason: "must be at least two",
            });
        }
        if !annual_sharpe_ratio_variance.is_finite() || annual_sharpe_ratio_variance < 0.0 {
            return Err(MetricError::InvalidParameter {
                name: "annual_sharpe_ratio_variance",
                value: annual_sharpe_ratio_variance.to_string(),
                reason: "must be finite and non-negative",
            });
        }
        if matches!(
            input_kind,
            MetricInputKind::RawPnl | MetricInputKind::Trades
        ) {
            return Err(MetricError::InvalidParameter {
                name: "input_kind",
                value: format!("{input_kind:?}"),
                reason: "deflated Sharpe ratio requires returns, log returns, equity, or period P&L with initial equity",
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
        let trial_count = number_of_trials as f64;
        let first_quantile = Self::inverse_standard_normal(1.0 - 1.0 / trial_count);
        let second_quantile =
            Self::inverse_standard_normal(1.0 - 1.0 / (trial_count * std::f64::consts::E));
        let period_sharpe_ratio_variance = annual_sharpe_ratio_variance / periods_per_year;
        const EULER_MASCHERONI: f64 = 0.577_215_664_901_532_9;
        let expected_maximum_period_sharpe_ratio = period_sharpe_ratio_variance.sqrt()
            * ((1.0 - EULER_MASCHERONI) * first_quantile + EULER_MASCHERONI * second_quantile);

        Ok(Self {
            input: MetricInputState::new(input_kind, nan_policy)?,
            count: 0,
            mean: 0.0,
            second_central_moment: 0.0,
            third_central_moment: 0.0,
            fourth_central_moment: 0.0,
            period_risk_free_rate,
            expected_maximum_period_sharpe_ratio,
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

    /// Return the deflated-Sharpe probability when four moments are defined.
    pub fn value(&self) -> Option<f64> {
        if self.count < 4 {
            return None;
        }
        let count = self.count as f64;
        let sample_variance = self.second_central_moment / (count - 1.0);
        if !sample_variance.is_finite() || sample_variance <= 0.0 {
            return None;
        }
        let period_sharpe_ratio = self.mean / sample_variance.sqrt();
        let population_variance = self.second_central_moment / count;
        let biased_skewness = (self.third_central_moment / count) / population_variance.powf(1.5);
        let sample_skewness = (count * (count - 1.0)).sqrt() / (count - 2.0) * biased_skewness;
        let biased_excess_kurtosis =
            (self.fourth_central_moment / count) / population_variance.powi(2) - 3.0;
        let sample_pearson_kurtosis = (count - 1.0) / ((count - 2.0) * (count - 3.0))
            * ((count + 1.0) * biased_excess_kurtosis + 6.0)
            + 3.0;
        if !period_sharpe_ratio.is_finite()
            || !sample_skewness.is_finite()
            || !sample_pearson_kurtosis.is_finite()
        {
            return None;
        }
        let variance_adjustment = 1.0 - sample_skewness * period_sharpe_ratio
            + (sample_pearson_kurtosis - 1.0) * period_sharpe_ratio.powi(2) / 4.0;
        if !variance_adjustment.is_finite() || variance_adjustment <= 0.0 {
            return None;
        }
        let statistic = (period_sharpe_ratio - self.expected_maximum_period_sharpe_ratio)
            * (count - 1.0).sqrt()
            / variance_adjustment.sqrt();
        statistic
            .is_finite()
            .then(|| Self::standard_normal_cdf(statistic))
    }

    /// Return the cached scalar without replaying prior observations.
    pub fn compute(&self) -> Option<f64> {
        self.value()
    }

    /// Reset observations while preserving input and trial configuration.
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
        const A: [f64; 6] = [
            -39.696_830_286_653_76,
            220.946_098_424_520_5,
            -275.928_510_446_968_7,
            138.357_751_867_269,
            -30.664_798_066_147_16,
            2.506_628_277_459_239,
        ];
        const B: [f64; 5] = [
            -54.476_098_798_224_06,
            161.585_836_858_040_9,
            -155.698_979_859_886_6,
            66.801_311_887_719_72,
            -13.280_681_552_885_72,
        ];
        const C: [f64; 6] = [
            -0.007_784_894_002_430_293,
            -0.322_396_458_041_136_5,
            -2.400_758_277_161_838,
            -2.549_732_539_343_734,
            4.374_664_141_464_968,
            2.938_163_982_698_783,
        ];
        const D: [f64; 4] = [
            0.007_784_695_709_041_462,
            0.322_467_129_070_039_8,
            2.445_134_137_142_996,
            3.754_408_661_907_416,
        ];
        if probability < 0.024_25 {
            let q = (-2.0 * probability.ln()).sqrt();
            return (((((C[0] * q + C[1]) * q + C[2]) * q + C[3]) * q + C[4]) * q + C[5])
                / ((((D[0] * q + D[1]) * q + D[2]) * q + D[3]) * q + 1.0);
        }
        if probability > 0.975_75 {
            return -Self::inverse_standard_normal(1.0 - probability);
        }
        let q = probability - 0.5;
        let r = q * q;
        (((((A[0] * r + A[1]) * r + A[2]) * r + A[3]) * r + A[4]) * r + A[5]) * q
            / (((((B[0] * r + B[1]) * r + B[2]) * r + B[3]) * r + B[4]) * r + 1.0)
    }

    fn standard_normal_cdf(value: f64) -> f64 {
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
