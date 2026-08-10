use std::collections::VecDeque;

use crate::error::TaResult;
use crate::stream::invalid_period;

/// Worden Time Segmented Volume over close-change-weighted volume flows.
#[derive(Debug, Clone)]
pub struct TimeSegmentedVolume {
    period: usize,
    previous_close: Option<f64>,
    flows: VecDeque<f64>,
    sum: f64,
    value: Option<f64>,
}

impl TimeSegmentedVolume {
    /// Create a TSV state with a non-zero rolling period.
    pub fn new(period: usize) -> TaResult<Self> {
        if period == 0 {
            return Err(invalid_period("period", period, 1));
        }
        Ok(Self {
            period,
            previous_close: None,
            flows: VecDeque::with_capacity(period),
            sum: 0.0,
            value: None,
        })
    }

    /// Append one close/volume sample and return the rolling flow sum.
    pub fn append(&mut self, close: f64, volume: f64) -> Option<f64> {
        let Some(previous) = self.previous_close.replace(close) else {
            self.value = None;
            return None;
        };
        let flow = (close - previous) * volume;
        if self.flows.len() == self.period {
            self.sum -= self.flows.pop_front().expect("full flow window");
        }
        self.flows.push_back(flow);
        self.sum += flow;
        self.value = (self.flows.len() == self.period).then_some(self.sum);
        self.value
    }

    /// Return the latest TSV, or `None` during warm-up.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Clear the previous close, rolling flows, sum, and latest value.
    pub fn reset(&mut self) {
        self.previous_close = None;
        self.flows.clear();
        self.sum = 0.0;
        self.value = None;
    }
}
