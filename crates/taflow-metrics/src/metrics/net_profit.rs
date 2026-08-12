use crate::{MetricError, MetricInputKind, MetricInputState, MetricResult, NanPolicy};

/// Net sum of raw period or closed-trade profit and loss observations.
#[derive(Debug, Clone)]
pub struct NetProfit {
    input: MetricInputState,
    sum: f64,
    compensation: f64,
}
impl NetProfit {
    /// Construct an empty raw P&L or closed-trade state.
    pub fn new(nan_policy: NanPolicy) -> MetricResult<Self> {
        Ok(Self {
            input: MetricInputState::unbound(nan_policy),
            sum: 0.0,
            compensation: 0.0,
        })
    }
    /// Append one P&L observation and return net profit.
    pub fn append(&mut self, value: f64) -> MetricResult<Option<f64>> {
        if let Some(observation) = self.input.append(value)? {
            let adjusted = observation - self.compensation;
            let next = self.sum + adjusted;
            self.compensation = (next - self.sum) - adjusted;
            self.sum = next;
        }
        Ok(self.value())
    }
    /// Append observations through the same persistent state.
    pub fn extend(&mut self, values: &[f64]) -> MetricResult<Option<f64>> {
        self.input.extend(values, |observation| {
            let adjusted = observation - self.compensation;
            let next = self.sum + adjusted;
            self.compensation = (next - self.sum) - adjusted;
            self.sum = next;
            Ok(())
        })?;
        Ok(self.value())
    }
    /// Return gross profit plus signed gross loss.
    pub fn value(&self) -> Option<f64> {
        (!self.input.is_empty()).then_some(self.sum)
    }
    /// Return current scalar without replay.
    pub fn compute(&self) -> Option<f64> {
        self.value()
    }
    /// Reset while preserving input domain.
    pub fn reset(&mut self) {
        self.input.reset();
        self.sum = 0.0;
        self.compensation = 0.0;
    }
    /// Return valid observation count.
    pub fn len(&self) -> usize {
        self.input.len()
    }
    /// Return whether no observations were processed.
    pub fn is_empty(&self) -> bool {
        self.input.is_empty()
    }
}

crate::impl_pnl_trade_metric_lifecycle!(NetProfit);
