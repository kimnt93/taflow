use crate::{
    primitives::{CompoundedGrowth, DrawdownState},
    MetricError, MetricInputKind, MetricInputState, MetricResult, NanPolicy,
};

/// Geometric annualized excess return divided by mean absolute drawdown.
#[derive(Debug, Clone)]
pub struct PainRatio {
    input: MetricInputState,
    growth: CompoundedGrowth,
    drawdown: DrawdownState,
    absolute_drawdown_sum: f64,
    periods_per_year: f64,
    annual_risk_free_rate: f64,
}

impl PainRatio {
    /// Construct an empty state with explicit annualization and input semantics.
    pub fn new(
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

        Ok(Self {
            input: MetricInputState::unbound(nan_policy),
            growth: CompoundedGrowth::new(),
            drawdown: DrawdownState::new(),
            absolute_drawdown_sum: 0.0,
            periods_per_year,
            annual_risk_free_rate,
        })
    }

    /// Append one chronological observation and return the ratio to date.
    pub fn append(&mut self, value: f64) -> MetricResult<Option<f64>> {
        if let Some(simple_return) = self.input.append(value)? {
            self.growth.append(simple_return)?;
            self.drawdown.append(simple_return)?;
            let current_drawdown = self
                .drawdown
                .current_drawdown()
                .expect("an appended return always produces a drawdown");
            self.absolute_drawdown_sum += current_drawdown.abs();
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

    /// Return the Pain ratio, or `None` while empty or without drawdown pain.
    pub fn value(&self) -> Option<f64> {
        let count = self.input.len();
        if count == 0 {
            return None;
        }
        let pain_index = self.absolute_drawdown_sum / count as f64;
        if pain_index == 0.0 {
            return None;
        }
        let logarithmic_growth = self.growth.logarithmic_growth()?;
        let annualized_return =
            (logarithmic_growth * self.periods_per_year / count as f64).exp_m1();
        let ratio = (annualized_return - self.annual_risk_free_rate) / pain_index;
        ratio.is_finite().then_some(ratio)
    }

    /// Return the current result without replaying prior observations.
    pub fn compute(&self) -> Option<f64> {
        self.value()
    }

    /// Restore fresh-state behavior while preserving configuration.
    pub fn reset(&mut self) {
        self.input.reset();
        self.growth.reset();
        self.drawdown.reset();
        self.absolute_drawdown_sum = 0.0;
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

    /// Return the annual effective risk-free rate subtracted from CAGR.
    pub fn annual_risk_free_rate(&self) -> f64 {
        self.annual_risk_free_rate
    }
}

crate::impl_return_metric_lifecycle!(PainRatio);
