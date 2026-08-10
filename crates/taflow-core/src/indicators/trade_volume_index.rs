use crate::error::{TaError, TaResult};

/// Cumulative volume signed by a persistent minimum-tick direction.
#[derive(Debug, Clone)]
pub struct TradeVolumeIndex {
    min_tick: f64,
    previous_close: Option<f64>,
    direction: f64,
    total: f64,
    value: Option<f64>,
}

impl TradeVolumeIndex {
    /// Create a TVI state with a finite, non-negative minimum tick.
    pub fn new(min_tick: f64) -> TaResult<Self> {
        if !min_tick.is_finite() || min_tick < 0.0 {
            return Err(TaError::InvalidParameter {
                name: "min_tick",
                value: min_tick.to_string(),
                reason: "must be finite and non-negative",
            });
        }
        Ok(Self {
            min_tick,
            previous_close: None,
            direction: 0.0,
            total: 0.0,
            value: None,
        })
    }

    /// Append one close/volume sample and return the cumulative TVI.
    pub fn append(&mut self, close: f64, volume: f64) -> Option<f64> {
        let Some(previous) = self.previous_close.replace(close) else {
            self.value = None;
            return None;
        };
        let change = close - previous;
        if change > self.min_tick {
            self.direction = 1.0;
        } else if change < -self.min_tick {
            self.direction = -1.0;
        }
        self.total += self.direction * volume;
        self.value = Some(self.total);
        self.value
    }

    /// Return the latest TVI, or `None` before the second sample.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Clear the close reference, direction, cumulative total, and latest value.
    pub fn reset(&mut self) {
        self.previous_close = None;
        self.direction = 0.0;
        self.total = 0.0;
        self.value = None;
    }
}
