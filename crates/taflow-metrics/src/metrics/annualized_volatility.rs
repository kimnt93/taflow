use crate::{
    primitives::OnlineMoments, MetricError, MetricInputKind, MetricInputState, MetricResult,
    NanPolicy,
};

/// Annualized sample standard deviation of normalized simple returns.
#[derive(Debug, Clone)]
pub struct AnnualizedVolatility {
    input: MetricInputState,
    moments: OnlineMoments,
    annualization_scale: f64,
}

impl AnnualizedVolatility {
    /// Construct an empty state with an explicit input mode and annual frequency.
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
                reason: "annualized volatility requires returns, log returns, equity, or period P&L with initial equity",
            });
        }

        Ok(Self {
            input: MetricInputState::new(input_kind, nan_policy)?,
            moments: OnlineMoments::new(),
            annualization_scale: periods_per_year.sqrt(),
        })
    }

    /// Append one chronological observation and return current annualized volatility.
    pub fn append(&mut self, value: f64) -> MetricResult<Option<f64>> {
        if let Some(simple_return) = self.input.append(value)? {
            self.moments.append(simple_return);
        }
        Ok(self.value())
    }

    /// Append a chronological slice through the same persistent state.
    pub fn extend(&mut self, values: &[f64]) -> MetricResult<Option<f64>> {
        self.input.extend(values, |simple_return| {
            self.moments.append(simple_return);
            Ok(())
        })?;
        Ok(self.value())
    }

    pub(crate) fn extend_normalized(&mut self, values: &[f64]) -> MetricResult<()> {
        self.input.extend_normalized_returns(values, |value| {
            self.moments.append(value);
            Ok(())
        })
    }

    /// Return annualized sample volatility, or `None` with fewer than two returns.
    pub fn value(&self) -> Option<f64> {
        self.moments
            .standard_deviation(1)
            .map(|standard_deviation| standard_deviation * self.annualization_scale)
    }

    /// Return the current result without replaying prior observations.
    pub fn compute(&self) -> Option<f64> {
        self.value()
    }

    /// Restore fresh-state behavior while preserving input and annualization settings.
    pub fn reset(&mut self) {
        self.input.reset();
        self.moments.reset();
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
