use crate::{
    primitives::DownsideMomentState, MetricError, MetricInputKind, MetricInputState, MetricResult,
    NanPolicy,
};

/// Annualized lower-partial second moment of normalized simple returns.
#[derive(Debug, Clone)]
pub struct DownsideDeviation {
    input: MetricInputState,
    downside_moment: DownsideMomentState,
    annualization_scale: f64,
}

impl DownsideDeviation {
    /// Construct an empty state from annual effective target and frequency settings.
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
                reason: "downside deviation requires returns, log returns, equity, or period P&L with initial equity",
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
            downside_moment: DownsideMomentState::new(period_required_return),
            annualization_scale: periods_per_year.sqrt(),
        })
    }

    /// Append one chronological observation and return current downside deviation.
    pub fn append(&mut self, value: f64) -> MetricResult<Option<f64>> {
        if let Some(simple_return) = self.input.append(value)? {
            self.downside_moment.append(simple_return);
        }
        Ok(self.value())
    }

    /// Append a chronological slice through the same persistent state.
    pub fn extend(&mut self, values: &[f64]) -> MetricResult<Option<f64>> {
        self.input.extend(values, |simple_return| {
            self.downside_moment.append(simple_return);
            Ok(())
        })?;
        Ok(self.value())
    }

    pub(crate) fn extend_normalized(&mut self, values: &[f64]) -> MetricResult<()> {
        self.input.extend_normalized_returns(values, |value| {
            self.downside_moment.append(value);
            Ok(())
        })
    }

    /// Return annualized downside deviation, or `None` without a usable return.
    pub fn value(&self) -> Option<f64> {
        self.downside_moment
            .downside_deviation()
            .map(|deviation| deviation * self.annualization_scale)
    }

    /// Return the current result without replaying prior observations.
    pub fn compute(&self) -> Option<f64> {
        self.value()
    }

    /// Restore fresh-state behavior while preserving input and target settings.
    pub fn reset(&mut self) {
        self.input.reset();
        self.downside_moment.reset();
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
