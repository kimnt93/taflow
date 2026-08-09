//! Persistent Fisher Transform state.

use crate::error::{TaError, TaResult};
use crate::stream::{MonotonicMax, MonotonicMin};

#[derive(Debug, Clone)]
pub struct FisherTransform {
    highs: MonotonicMax,
    lows: MonotonicMin,
    previous_position: f64,
    previous_fisher: f64,
    seeded: bool,
    value: Option<f64>,
}

impl FisherTransform {
    pub fn new(timeperiod: usize) -> TaResult<Self> {
        if timeperiod == 0 {
            return Err(TaError::InvalidParameter {
                name: "timeperiod",
                value: timeperiod.to_string(),
                reason: "must be >= 1",
            });
        }
        Ok(Self {
            highs: MonotonicMax::new(timeperiod)?,
            lows: MonotonicMin::new(timeperiod)?,
            previous_position: 0.0,
            previous_fisher: 0.0,
            seeded: false,
            value: None,
        })
    }

    pub fn append(&mut self, high: f64, low: f64) -> Option<f64> {
        let midpoint = (high + low) * 0.5;
        let maximum = self.highs.append(midpoint);
        let minimum = self.lows.append(midpoint);
        self.value = maximum.zip(minimum).map(|(high, low)| {
            if !self.seeded {
                self.seeded = true;
                self.previous_position = 0.0;
                self.previous_fisher = 0.0;
                return 0.0;
            }
            let position = if high != low {
                (midpoint - low) / (high - low) - 0.5
            } else {
                0.0
            };
            let raw = 0.66 * position + 0.67 * self.previous_position;
            let bounded = raw.clamp(-0.999, 0.999);
            let fisher = 0.5 * (((1.0 + bounded) / (1.0 - bounded)).ln() + self.previous_fisher);
            self.previous_position = bounded;
            self.previous_fisher = fisher;
            fisher
        });
        self.value
    }

    pub fn value(&self) -> Option<f64> {
        self.value
    }

    pub fn reset(&mut self) {
        self.highs.reset();
        self.lows.reset();
        self.previous_position = 0.0;
        self.previous_fisher = 0.0;
        self.seeded = false;
        self.value = None;
    }
}
