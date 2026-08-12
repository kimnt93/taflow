use crate::{
    primitives::OnlineMoments, MetricError, MetricInputKind, MetricInputState, MetricResult,
    NanPolicy,
};

/// Annualized mean excess return divided by sample excess-return deviation.
#[derive(Debug, Clone)]
pub struct SharpeRatio {
    input: MetricInputState,
    excess_return_moments: OnlineMoments,
    period_risk_free_rate: f64,
    annualization_scale: f64,
}

impl SharpeRatio {
    /// Construct an empty state with an explicit input mode and annual settings.
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
        let period_risk_free_rate = (annual_risk_free_rate.ln_1p() / periods_per_year).exp_m1();
        Ok(Self {
            input: MetricInputState::unbound(nan_policy),
            excess_return_moments: OnlineMoments::new(),
            period_risk_free_rate,
            annualization_scale: periods_per_year.sqrt(),
        })
    }

    /// Select decimal simple returns and append a chronological slice.
    pub fn from_returns(&mut self, returns: &[f64]) -> MetricResult<&mut Self> {
        self.bind_and_extend(MetricInputKind::Returns, returns)
    }

    /// Select logarithmic returns and append a chronological slice.
    pub fn from_log_returns(&mut self, log_returns: &[f64]) -> MetricResult<&mut Self> {
        self.bind_and_extend(MetricInputKind::LogReturns, log_returns)
    }

    /// Select positive equity levels and append a chronological slice.
    pub fn from_equity(&mut self, equity: &[f64]) -> MetricResult<&mut Self> {
        self.bind_and_extend(MetricInputKind::Equity, equity)
    }

    /// Select period P&L and append it using positive starting capital.
    pub fn from_pnl(&mut self, pnl: &[f64], initial_capital: f64) -> MetricResult<&mut Self> {
        self.bind_and_extend(
            MetricInputKind::PeriodPnl {
                initial_capital: initial_capital,
            },
            pnl,
        )
    }

    fn bind_and_extend(
        &mut self,
        input_kind: MetricInputKind,
        values: &[f64],
    ) -> MetricResult<&mut Self> {
        self.input.bind(input_kind)?;
        self.extend(values)?;
        Ok(self)
    }

    /// Append one chronological observation and return the current ratio.
    pub fn append(&mut self, value: f64) -> MetricResult<Option<f64>> {
        if let Some(simple_return) = self.input.append(value)? {
            self.excess_return_moments
                .append(simple_return - self.period_risk_free_rate);
        }
        Ok(self.value())
    }

    /// Append a chronological slice through the same persistent state.
    pub fn extend(&mut self, values: &[f64]) -> MetricResult<Option<f64>> {
        let period_risk_free_rate = self.period_risk_free_rate;
        self.input.extend(values, |simple_return| {
            self.excess_return_moments
                .append(simple_return - period_risk_free_rate);
            Ok(())
        })?;
        Ok(self.value())
    }

    pub(crate) fn extend_normalized(&mut self, values: &[f64]) -> MetricResult<()> {
        let risk_free = self.period_risk_free_rate;
        self.input.extend_normalized_returns(values, |value| {
            self.excess_return_moments.append(value - risk_free);
            Ok(())
        })
    }

    /// Return the annualized Sharpe ratio, or `None` when it is undefined.
    pub fn value(&self) -> Option<f64> {
        let standard_deviation = self.excess_return_moments.standard_deviation(1)?;
        if standard_deviation == 0.0 {
            return None;
        }
        Some(self.excess_return_moments.mean()? / standard_deviation * self.annualization_scale)
    }

    /// Return the current result without replaying prior observations.
    pub fn compute(&self) -> Option<f64> {
        self.value()
    }

    /// Restore fresh-state behavior while preserving all configuration.
    pub fn reset(&mut self) {
        self.input.reset();
        self.excess_return_moments.reset();
    }

    /// Return the number of usable normalized returns processed.
    pub fn len(&self) -> usize {
        self.input.len()
    }

    /// Return whether no usable normalized returns have been processed.
    pub fn is_empty(&self) -> bool {
        self.input.is_empty()
    }
}
