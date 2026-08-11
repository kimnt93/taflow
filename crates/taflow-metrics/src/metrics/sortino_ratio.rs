use crate::{
    primitives::DownsideMomentState, MetricError, MetricInputKind, MetricInputState, MetricResult,
    NanPolicy,
};

/// Annualized excess return divided by annualized downside deviation.
#[derive(Debug, Clone)]
pub struct SortinoRatio {
    input: MetricInputState,
    downside: DownsideMomentState,
    excess_return_sum: f64,
    periods_per_year: f64,
    annual_required_return: f64,
}

impl SortinoRatio {
    /// Construct an empty Sortino-ratio state.
    pub fn new(
        input_kind: MetricInputKind,
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
        if matches!(
            input_kind,
            MetricInputKind::RawPnl | MetricInputKind::Trades
        ) {
            return Err(MetricError::InvalidParameter {
                name: "input_kind",
                value: format!("{input_kind:?}"),
                reason: "Sortino ratio requires returns, log returns, equity, or period P&L with initial equity",
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
            input: MetricInputState::new(input_kind, nan_policy)?,
            downside: DownsideMomentState::new(period_required_return),
            excess_return_sum: 0.0,
            periods_per_year,
            annual_required_return,
        })
    }

    /// Append one chronological observation and return the current ratio.
    pub fn append(&mut self, value: f64) -> MetricResult<Option<f64>> {
        if let Some(simple_return) = self.input.append(value)? {
            self.excess_return_sum += simple_return - self.downside.required_return();
            self.downside.append(simple_return);
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

    /// Return the Sortino ratio, or `None` before two returns or at zero downside.
    pub fn value(&self) -> Option<f64> {
        if self.downside.len() < 2 {
            return None;
        }
        let annualized_downside =
            self.downside.downside_deviation()? * self.periods_per_year.sqrt();
        if annualized_downside == 0.0 {
            return None;
        }
        let annualized_excess_return =
            self.excess_return_sum / self.downside.len() as f64 * self.periods_per_year;
        Some(annualized_excess_return / annualized_downside)
    }

    /// Return the current result without replaying prior observations.
    pub fn compute(&self) -> Option<f64> {
        self.value()
    }

    /// Restore fresh-state behavior while preserving all configuration.
    pub fn reset(&mut self) {
        self.input.reset();
        self.downside.reset();
        self.excess_return_sum = 0.0;
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

    /// Return the configured annual effective minimum acceptable return.
    pub fn annual_required_return(&self) -> f64 {
        self.annual_required_return
    }

    /// Return the per-period rate used by the Empyrical-compatible formula.
    pub fn period_required_return(&self) -> f64 {
        self.downside.required_return()
    }
}
