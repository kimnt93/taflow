//! Incremental Double Exponential Moving Average (DEMA).

use crate::error::TaResult;

use super::{invalid_period, Ema, StreamingIndicator};

/// Stateful double EMA composed from the shared EMA primitive.
#[derive(Debug, Clone)]
pub struct Dema {
    ema1: Ema,
    ema2: Ema,
    value: Option<f64>,
}

impl Dema {
    pub fn new(period: usize) -> TaResult<Self> {
        if period < 2 {
            return Err(invalid_period("timeperiod", period, 2));
        }
        Ok(Self {
            ema1: Ema::new(period)?,
            ema2: Ema::new(period)?,
            value: None,
        })
    }
}

impl StreamingIndicator for Dema {
    type Output = f64;

    fn append(&mut self, input: f64) -> Option<f64> {
        self.value = self
            .ema1
            .append(input)
            .and_then(|ema1| self.ema2.append(ema1).map(|ema2| 2.0 * ema1 - ema2));
        self.value
    }

    fn value(&self) -> Option<f64> {
        self.value
    }

    fn reset(&mut self) {
        self.ema1.reset();
        self.ema2.reset();
        self.value = None;
    }
}
