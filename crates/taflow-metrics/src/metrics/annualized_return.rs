use crate::{
    primitives::CompoundedGrowth, MetricError, MetricInputKind, MetricInputState, MetricResult,
    NanPolicy,
};

/// Geometric annualized return over every usable normalized return.
#[derive(Debug, Clone)]
pub struct AnnualizedReturn {
    input: MetricInputState,
    growth: CompoundedGrowth,
    periods_per_year: f64,
}

impl AnnualizedReturn {
    /// Construct an empty annualized-return state.
    pub fn new(periods_per_year: f64, nan_policy: NanPolicy) -> MetricResult<Self> {
        if !periods_per_year.is_finite() || periods_per_year <= 0.0 {
            return Err(MetricError::InvalidParameter {
                name: "periods_per_year",
                value: periods_per_year.to_string(),
                reason: "must be finite and greater than zero",
            });
        }

        Ok(Self {
            input: MetricInputState::unbound(nan_policy),
            growth: CompoundedGrowth::new(),
            periods_per_year,
        })
    }

    /// Append one chronological observation and return the current result.
    pub fn append(&mut self, value: f64) -> MetricResult<Option<f64>> {
        if let Some(simple_return) = self.input.append(value)? {
            self.growth.append(simple_return)?;
        }
        Ok(self.value())
    }

    /// Append a chronological slice through the same persistent state.
    pub fn extend(&mut self, values: &[f64]) -> MetricResult<Option<f64>> {
        self.input
            .extend(values, |simple_return| self.growth.append(simple_return))?;
        Ok(self.value())
    }

    pub(crate) fn extend_normalized(&mut self, values: &[f64]) -> MetricResult<()> {
        self.input
            .extend_normalized_returns(values, |value| self.growth.append(value))
    }

    /// Return geometric CAGR, or `None` when no usable return exists.
    pub fn value(&self) -> Option<f64> {
        self.growth.logarithmic_growth().map(|logarithmic_growth| {
            (logarithmic_growth * self.periods_per_year / self.growth.len() as f64).exp_m1()
        })
    }

    /// Return the current result without replaying prior observations.
    pub fn compute(&self) -> Option<f64> {
        self.value()
    }

    /// Restore fresh-state behavior while preserving configuration.
    pub fn reset(&mut self) {
        self.input.reset();
        self.growth.reset();
    }

    /// Return the number of usable normalized returns processed.
    pub fn len(&self) -> usize {
        self.input.len()
    }

    /// Return whether no usable normalized returns have been processed.
    pub fn is_empty(&self) -> bool {
        self.input.is_empty()
    }

    /// Return the configured annualization frequency.
    pub fn periods_per_year(&self) -> f64 {
        self.periods_per_year
    }
}

crate::impl_return_metric_lifecycle!(AnnualizedReturn);
