use crate::{
    primitives::{CompoundedGrowth, DrawdownState},
    MetricError, MetricInputKind, MetricInputState, MetricResult, NanPolicy,
};

/// Geometric annualized return divided by absolute maximum drawdown.
#[derive(Debug, Clone)]
pub struct CalmarRatio {
    input: MetricInputState,
    growth: CompoundedGrowth,
    drawdown: DrawdownState,
    periods_per_year: f64,
}

impl CalmarRatio {
    /// Construct an empty Calmar-ratio state.
    pub fn new(
        input_kind: MetricInputKind,
        periods_per_year: f64,
        nan_policy: NanPolicy,
    ) -> MetricResult<Self> {
        if !periods_per_year.is_finite() || periods_per_year <= 0.0 {
            return Err(MetricError::InvalidParameter {
                name: "periods_per_year",
                value: periods_per_year.to_string(),
                reason: "must be finite and greater than zero",
            });
        }
        if matches!(
            input_kind,
            MetricInputKind::RawPnl | MetricInputKind::Trades
        ) {
            return Err(MetricError::InvalidParameter {
                name: "input_kind",
                value: format!("{input_kind:?}"),
                reason: "Calmar ratio requires returns, log returns, equity, or period P&L with initial equity",
            });
        }

        Ok(Self {
            input: MetricInputState::new(input_kind, nan_policy)?,
            growth: CompoundedGrowth::new(),
            drawdown: DrawdownState::new(),
            periods_per_year,
        })
    }

    /// Append one chronological observation and return the current ratio.
    pub fn append(&mut self, value: f64) -> MetricResult<Option<f64>> {
        if let Some(simple_return) = self.input.append(value)? {
            self.growth.append(simple_return)?;
            self.drawdown.append(simple_return)?;
        }
        Ok(self.value())
    }

    /// Append a chronological slice through the same persistent state.
    pub fn extend(&mut self, values: &[f64]) -> MetricResult<Option<f64>> {
        self.input.extend(values, |simple_return| {
            self.growth.append(simple_return)?;
            self.drawdown.append(simple_return)
        })?;
        Ok(self.value())
    }

    pub(crate) fn extend_normalized(&mut self, values: &[f64]) -> MetricResult<()> {
        self.input.extend_normalized_returns(values, |value| {
            self.growth.append(value)?;
            self.drawdown.append(value)
        })
    }

    /// Return the Calmar ratio, or `None` while empty or without a drawdown.
    pub fn value(&self) -> Option<f64> {
        let maximum_drawdown = self.drawdown.maximum_drawdown()?;
        if maximum_drawdown >= 0.0 {
            return None;
        }
        let logarithmic_growth = self.growth.logarithmic_growth()?;
        let annualized_return =
            (logarithmic_growth * self.periods_per_year / self.growth.len() as f64).exp_m1();
        let ratio = annualized_return / maximum_drawdown.abs();
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
