use crate::{
    primitives::GainLossState, MetricError, MetricInputKind, MetricInputState, MetricResult,
    NanPolicy,
};

/// Gain above a required-return threshold divided by absolute loss below it.
#[derive(Debug, Clone)]
pub struct OmegaRatio {
    input: MetricInputState,
    excess_returns: GainLossState,
    periods_per_year: f64,
    annual_required_return: f64,
    period_required_return: f64,
}

impl OmegaRatio {
    /// Construct an empty Omega-ratio state.
    pub fn new(
        periods_per_year: f64,
        annual_required_return: f64,
        nan_policy: NanPolicy,
    ) -> MetricResult<Self> {
        if !periods_per_year.is_finite() || periods_per_year <= 0.0 {
            return Err(MetricError::InvalidParameter {
                name: "periods_per_year",
                value: periods_per_year.to_string(),
                reason: "must be finite and greater than zero",
            });
        }
        if !annual_required_return.is_finite() || annual_required_return <= -1.0 {
            return Err(MetricError::InvalidParameter {
                name: "annual_required_return",
                value: annual_required_return.to_string(),
                reason: "must be finite and greater than -1",
            });
        }

        let period_required_return = (annual_required_return.ln_1p() / periods_per_year).exp_m1();
        if !period_required_return.is_finite() {
            return Err(MetricError::InvalidParameter {
                name: "annual_required_return",
                value: annual_required_return.to_string(),
                reason: "annual effective rate conversion must produce a finite per-period rate",
            });
        }

        Ok(Self {
            input: MetricInputState::unbound(nan_policy),
            excess_returns: GainLossState::new(),
            periods_per_year,
            annual_required_return,
            period_required_return,
        })
    }

    /// Append one chronological observation and return the current ratio.
    pub fn append(&mut self, value: f64) -> MetricResult<Option<f64>> {
        if let Some(simple_return) = self.input.append(value)? {
            self.excess_returns
                .append(simple_return - self.period_required_return);
        }
        Ok(self.value())
    }

    /// Append a chronological slice through the same persistent state.
    pub fn extend(&mut self, values: &[f64]) -> MetricResult<Option<f64>> {
        let required_return = self.period_required_return;
        self.input.extend(values, |simple_return| {
            self.excess_returns.append(simple_return - required_return);
            Ok(())
        })?;
        Ok(self.value())
    }

    /// Return the Omega ratio, or `None` before two returns or without a loss.
    pub fn value(&self) -> Option<f64> {
        if self.excess_returns.len() < 2 {
            return None;
        }
        let denominator = -self.excess_returns.gross_loss();
        if denominator > 0.0 {
            Some(self.excess_returns.gross_gain() / denominator)
        } else {
            None
        }
    }

    /// Return the current result without replaying prior observations.
    pub fn compute(&self) -> Option<f64> {
        self.value()
    }

    /// Restore fresh-state behavior while preserving all configuration.
    pub fn reset(&mut self) {
        self.input.reset();
        self.excess_returns.reset();
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

    /// Return the configured annual effective required return.
    pub fn annual_required_return(&self) -> f64 {
        self.annual_required_return
    }

    /// Return the converted per-period threshold used by the ratio.
    pub fn period_required_return(&self) -> f64 {
        self.period_required_return
    }
}

crate::impl_return_metric_lifecycle!(OmegaRatio);
