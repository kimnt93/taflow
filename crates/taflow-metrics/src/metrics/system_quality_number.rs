use crate::{
    primitives::OnlineMoments, MetricError, MetricInputKind, MetricInputState, MetricResult,
    NanPolicy,
};

/// Square root of trade count times mean trade P&L divided by sample deviation.
#[derive(Debug, Clone)]
pub struct SystemQualityNumber {
    input: MetricInputState,
    moments: OnlineMoments,
    trade_sum: f64,
    trade_sum_compensation: f64,
}

impl SystemQualityNumber {
    /// Construct an empty state for realized closed-trade P&L.
    pub fn new(nan_policy: NanPolicy) -> MetricResult<Self> {
        Ok(Self {
            input: MetricInputState::unbound(nan_policy),
            moments: OnlineMoments::new(),
            trade_sum: 0.0,
            trade_sum_compensation: 0.0,
        })
    }

    /// Append one chronological closed-trade P&L and return the current SQN.
    pub fn append(&mut self, value: f64) -> MetricResult<Option<f64>> {
        if let Some(trade_pnl) = self.input.append(value)? {
            self.moments.append(trade_pnl);
            let updated_sum = self.trade_sum + trade_pnl;
            if self.trade_sum.abs() >= trade_pnl.abs() {
                self.trade_sum_compensation += (self.trade_sum - updated_sum) + trade_pnl;
            } else {
                self.trade_sum_compensation += (trade_pnl - updated_sum) + self.trade_sum;
            }
            self.trade_sum = updated_sum;
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

    /// Return SQN, or `None` with fewer than two trades or zero sample deviation.
    pub fn value(&self) -> Option<f64> {
        let standard_deviation = self.moments.standard_deviation(1)?;
        if standard_deviation == 0.0 || !standard_deviation.is_finite() {
            return None;
        }
        let mean = (self.trade_sum + self.trade_sum_compensation) / self.moments.len() as f64;
        let result = (self.moments.len() as f64).sqrt() * mean / standard_deviation;
        result.is_finite().then_some(result)
    }

    /// Return the current result without replaying prior trades.
    pub fn compute(&self) -> Option<f64> {
        self.value()
    }

    /// Restore fresh-state behavior while preserving the closed-trade domain.
    pub fn reset(&mut self) {
        self.input.reset();
        self.moments.reset();
        self.trade_sum = 0.0;
        self.trade_sum_compensation = 0.0;
    }

    /// Return the number of usable closed trades processed.
    pub fn len(&self) -> usize {
        self.input.len()
    }

    /// Return whether no usable closed trades have been processed.
    pub fn is_empty(&self) -> bool {
        self.input.is_empty()
    }
}

crate::impl_trades_only_metric_lifecycle!(SystemQualityNumber);
