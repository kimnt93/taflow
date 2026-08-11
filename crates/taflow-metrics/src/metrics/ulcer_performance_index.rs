use crate::{
    primitives::{CompoundedGrowth, DrawdownState},
    MetricError, MetricInputKind, MetricInputState, MetricResult, NanPolicy,
};

/// Compounded whole-sample return divided by root-mean-square drawdown.
#[derive(Debug, Clone)]
pub struct UlcerPerformanceIndex {
    input: MetricInputState,
    growth: CompoundedGrowth,
    drawdown: DrawdownState,
    squared_drawdown_sum: f64,
}

impl UlcerPerformanceIndex {
    /// Construct an empty state with an explicitly selected semantic input mode.
    pub fn new(input_kind: MetricInputKind, nan_policy: NanPolicy) -> MetricResult<Self> {
        if matches!(
            input_kind,
            MetricInputKind::RawPnl | MetricInputKind::Trades
        ) {
            return Err(MetricError::InvalidParameter {
                name: "input_kind",
                value: format!("{input_kind:?}"),
                reason: "ulcer performance index requires returns, log returns, equity, or period P&L with initial equity",
            });
        }
        Ok(Self {
            input: MetricInputState::new(input_kind, nan_policy)?,
            growth: CompoundedGrowth::new(),
            drawdown: DrawdownState::new(),
            squared_drawdown_sum: 0.0,
        })
    }

    /// Append one chronological observation and return the index to date.
    pub fn append(&mut self, value: f64) -> MetricResult<Option<f64>> {
        if let Some(simple_return) = self.input.append(value)? {
            self.growth.append(simple_return)?;
            self.drawdown.append(simple_return)?;
            let current_drawdown = self
                .drawdown
                .current_drawdown()
                .expect("an appended return always produces a drawdown");
            self.squared_drawdown_sum += current_drawdown * current_drawdown;
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

    /// Return the index, or `None` before two returns or with zero ulcer index.
    pub fn value(&self) -> Option<f64> {
        let count = self.input.len();
        if count < 2 {
            return None;
        }
        let ulcer_index = (self.squared_drawdown_sum / (count - 1) as f64).sqrt();
        if ulcer_index == 0.0 {
            return None;
        }
        let total_return = self
            .growth
            .growth_factor()
            .expect("two normalized returns imply growth state")
            - 1.0;
        let index = total_return / ulcer_index;
        index.is_finite().then_some(index)
    }

    /// Return the current result without replaying prior observations.
    pub fn compute(&self) -> Option<f64> {
        self.value()
    }

    /// Restore fresh-state behavior while preserving input configuration.
    pub fn reset(&mut self) {
        self.input.reset();
        self.growth.reset();
        self.drawdown.reset();
        self.squared_drawdown_sum = 0.0;
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
