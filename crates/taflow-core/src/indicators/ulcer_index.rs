use crate::error::TaResult;
use crate::stream::{validate_period, MonotonicMax};
use std::collections::VecDeque;

/// Downside-risk RMS computed from trailing-maximum percentage drawdowns.
#[derive(Debug, Clone)]
pub struct UlcerIndex {
    period: usize,
    maximum: MonotonicMax,
    squared_drawdowns: VecDeque<f64>,
    squared_sum: f64,
    value: Option<f64>,
}

impl UlcerIndex {
    /// Create an Ulcer Index with the requested trailing period.
    pub fn new(period: usize) -> TaResult<Self> {
        validate_period(period)?;
        Ok(Self {
            period,
            maximum: MonotonicMax::new(period)?,
            squared_drawdowns: VecDeque::with_capacity(period),
            squared_sum: 0.0,
            value: None,
        })
    }

    /// Append one price and return the RMS after both rolling windows warm.
    pub fn append(&mut self, input: f64) -> Option<f64> {
        let maximum = self.maximum.append(input)?;
        let drawdown = if maximum == 0.0 {
            0.0
        } else {
            100.0 * (input - maximum) / maximum
        };
        let squared = drawdown * drawdown;
        if self.squared_drawdowns.len() == self.period {
            self.squared_sum -= self
                .squared_drawdowns
                .pop_front()
                .expect("full drawdown window");
        }
        self.squared_drawdowns.push_back(squared);
        self.squared_sum += squared;
        self.value = (self.squared_drawdowns.len() == self.period)
            .then(|| (self.squared_sum / self.period as f64).sqrt());
        self.value
    }

    /// Return the latest index, or `None` during warm-up.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Clear both rolling windows and the latest value.
    pub fn reset(&mut self) {
        self.maximum.reset();
        self.squared_drawdowns.clear();
        self.squared_sum = 0.0;
        self.value = None;
    }
}
