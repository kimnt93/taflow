use crate::error::{TaError, TaResult};
use crate::stream::validate_period;
use std::collections::VecDeque;

/// Simple-moving-average of range-scaled midpoint movement per unit of volume.
#[derive(Debug, Clone)]
pub struct EaseOfMovement {
    period: usize,
    divisor: f64,
    previous_midpoint: Option<f64>,
    movements: VecDeque<f64>,
    sum: f64,
    count: usize,
    value: Option<f64>,
}

impl EaseOfMovement {
    /// Create an indicator with a period and conventional volume divisor.
    pub fn new(period: usize, divisor: f64) -> TaResult<Self> {
        validate_period(period)?;
        if !divisor.is_finite() || divisor <= 0.0 {
            return Err(TaError::InvalidParameter {
                name: "divisor",
                value: divisor.to_string(),
                reason: "must be finite and > 0",
            });
        }
        Ok(Self {
            period,
            divisor,
            previous_midpoint: None,
            movements: VecDeque::with_capacity(period),
            sum: 0.0,
            count: 0,
            value: None,
        })
    }

    /// Append one high/low/volume bar and return the averaged movement.
    pub fn append(&mut self, high: f64, low: f64, volume: f64) -> Option<f64> {
        self.count += 1;
        let midpoint = f64::midpoint(high, low);
        let Some(previous) = self.previous_midpoint.replace(midpoint) else {
            return None;
        };
        let movement = if volume == 0.0 {
            0.0
        } else {
            (midpoint - previous) * (high - low) * self.divisor / volume
        };
        if self.movements.len() == self.period {
            self.sum -= self.movements.pop_front().expect("full movement window");
        }
        self.movements.push_back(movement);
        self.sum += movement;
        self.value = (self.movements.len() == self.period).then(|| self.sum / self.period as f64);
        self.value
    }

    /// Return the latest averaged movement, or `None` during warm-up.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Return the number of processed bars.
    pub fn len(&self) -> usize {
        self.count
    }

    /// Return whether no bars have been processed.
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    /// Clear midpoint, rolling mean, count, and latest-value state.
    pub fn reset(&mut self) {
        self.previous_midpoint = None;
        self.movements.clear();
        self.sum = 0.0;
        self.count = 0;
        self.value = None;
    }
}
