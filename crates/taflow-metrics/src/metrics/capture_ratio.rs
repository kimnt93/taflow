use crate::{
    primitives::CompoundedGrowth, MetricError, MetricInputKind, MetricResult, NanPolicy,
    PairedMetricInputState,
};

/// Ratio of portfolio geometric annual return to benchmark geometric annual return.
#[derive(Debug, Clone)]
pub struct CaptureRatio {
    input: PairedMetricInputState,
    primary_growth: CompoundedGrowth,
    benchmark_growth: CompoundedGrowth,
    periods_per_year: f64,
}

impl CaptureRatio {
    /// Construct an empty capture-ratio state for aligned input domains.
    pub fn new(periods_per_year: f64, nan_policy: NanPolicy) -> MetricResult<Self> {
        if !periods_per_year.is_finite() || periods_per_year <= 0.0 {
            return Err(MetricError::InvalidParameter {
                name: "periods_per_year",
                value: periods_per_year.to_string(),
                reason: "must be finite and greater than zero",
            });
        }
        Ok(Self {
            input: PairedMetricInputState::unbound(nan_policy),
            primary_growth: CompoundedGrowth::new(),
            benchmark_growth: CompoundedGrowth::new(),
            periods_per_year,
        })
    }

    /// Append one aligned pair and return the current capture ratio.
    pub fn append(&mut self, primary: f64, benchmark: f64) -> MetricResult<Option<f64>> {
        if let Some((primary_return, benchmark_return)) = self.input.append(primary, benchmark)? {
            self.primary_growth.append(primary_return)?;
            self.benchmark_growth.append(benchmark_return)?;
        }
        Ok(self.value())
    }

    /// Append equal-length aligned slices through the same persistent state.
    pub fn extend(&mut self, primary: &[f64], benchmark: &[f64]) -> MetricResult<Option<f64>> {
        let input = &mut self.input;
        let primary_growth = &mut self.primary_growth;
        let benchmark_growth = &mut self.benchmark_growth;
        input.extend_slices(primary, benchmark, |primary_return, benchmark_return| {
            // Input conversion has already validated both normalized returns.
            primary_growth
                .append(primary_return)
                .expect("validated normalized primary return must compound");
            benchmark_growth
                .append(benchmark_return)
                .expect("validated normalized benchmark return must compound");
        })?;
        Ok(self.value())
    }

    /// Return annualized primary growth divided by annualized benchmark growth.
    ///
    /// At least one usable aligned pair and a non-zero benchmark annual return
    /// are required.
    pub fn value(&self) -> Option<f64> {
        let count = self.input.len();
        if count == 0 {
            return None;
        }
        let exponent = self.periods_per_year / count as f64;
        let primary = (self.primary_growth.logarithmic_growth()? * exponent).exp_m1();
        let benchmark = (self.benchmark_growth.logarithmic_growth()? * exponent).exp_m1();
        if benchmark == 0.0 {
            return None;
        }
        Some(primary / benchmark)
    }

    /// Return the current result without replaying prior observations.
    pub fn compute(&self) -> Option<f64> {
        self.value()
    }

    /// Restore fresh-state behavior while preserving configuration.
    pub fn reset(&mut self) {
        self.input.reset();
        self.primary_growth.reset();
        self.benchmark_growth.reset();
    }

    /// Return the number of usable aligned return pairs processed.
    pub fn len(&self) -> usize {
        self.input.len()
    }

    /// Return whether no usable aligned return pairs have been processed.
    pub fn is_empty(&self) -> bool {
        self.input.is_empty()
    }

    /// Return the configured annualization frequency.
    pub fn periods_per_year(&self) -> f64 {
        self.periods_per_year
    }
}

crate::impl_paired_return_metric_lifecycle!(CaptureRatio);
