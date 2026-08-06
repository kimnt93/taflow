//! Incremental Triple Exponential Moving Average (TEMA).

use crate::error::TaResult;

use super::{invalid_period, Ema, StreamingIndicator};

/// Stateful triple EMA composed from the shared EMA primitive.
#[derive(Debug, Clone)]
pub struct Tema {
    ema1: Ema,
    ema2: Ema,
    ema3: Ema,
    value: Option<f64>,
}

impl Tema {
    pub fn new(period: usize) -> TaResult<Self> {
        if period < 2 {
            return Err(invalid_period("timeperiod", period, 2));
        }
        Ok(Self {
            ema1: Ema::new(period)?,
            ema2: Ema::new(period)?,
            ema3: Ema::new(period)?,
            value: None,
        })
    }
}

impl StreamingIndicator for Tema {
    type Output = f64;

    fn append(&mut self, input: f64) -> Option<f64> {
        self.value = self.ema1.append(input).and_then(|ema1| {
            self.ema2.append(ema1).and_then(|ema2| {
                self.ema3
                    .append(ema2)
                    .map(|ema3| 3.0 * ema1 - 3.0 * ema2 + ema3)
            })
        });
        self.value
    }

    fn value(&self) -> Option<f64> {
        self.value
    }

    fn reset(&mut self) {
        self.ema1.reset();
        self.ema2.reset();
        self.ema3.reset();
        self.value = None;
    }
}
