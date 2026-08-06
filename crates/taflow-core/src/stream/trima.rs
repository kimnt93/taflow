//! Incremental Triangular Moving Average (TRIMA).

use crate::error::TaResult;

use super::{invalid_period, Sma, StreamingIndicator};

/// Stateful triangular moving average as two cascaded SMA windows.
#[derive(Debug, Clone)]
pub struct Trima {
    sma1: Sma,
    sma2: Sma,
    value: Option<f64>,
}

impl Trima {
    pub fn new(period: usize) -> TaResult<Self> {
        if period == 0 {
            return Err(invalid_period("timeperiod", period, 1));
        }
        let (p1, p2) = if period % 2 == 1 {
            let half = (period + 1) / 2;
            (half, half)
        } else {
            (period / 2 + 1, period / 2)
        };
        Ok(Self {
            sma1: Sma::new(p1)?,
            sma2: Sma::new(p2)?,
            value: None,
        })
    }
}

impl StreamingIndicator for Trima {
    type Output = f64;

    fn append(&mut self, input: f64) -> Option<f64> {
        self.value = self
            .sma1
            .append(input)
            .and_then(|first| self.sma2.append(first));
        self.value
    }

    fn value(&self) -> Option<f64> {
        self.value
    }

    fn reset(&mut self) {
        self.sma1.reset();
        self.sma2.reset();
        self.value = None;
    }
}
