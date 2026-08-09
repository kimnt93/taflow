use super::swing_high_low::SwingHighLow;
use crate::error::TaResult;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RetracementsValue {
    pub direction: f64,
    pub current_retracement_pct: f64,
    pub deepest_retracement_pct: f64,
}

#[derive(Debug, Clone)]
/// Causal swing-leg retracement tracking.
pub struct Retracements {
    swing: SwingHighLow,
    last_high: Option<f64>,
    last_low: Option<f64>,
    leg_high: Option<f64>,
    leg_low: Option<f64>,
    direction: Option<f64>,
    deepest: f64,
    value: Option<RetracementsValue>,
}

impl Retracements {
    pub fn new(swing_length: usize) -> TaResult<Self> {
        Ok(Self {
            swing: SwingHighLow::new(swing_length)?,
            last_high: None,
            last_low: None,
            leg_high: None,
            leg_low: None,
            direction: None,
            deepest: 0.0,
            value: None,
        })
    }

    pub fn append(&mut self, high: f64, low: f64, close: f64) -> RetracementsValue {
        if let Some(swing) = self.swing.append(high, low) {
            if swing.signal > 0.0 {
                self.last_high = Some(swing.level);
                if let Some(last_low) = self.last_low {
                    self.leg_high = Some(swing.level);
                    self.leg_low = Some(last_low);
                    self.direction = Some(1.0);
                    self.deepest = 0.0;
                }
            } else if swing.signal < 0.0 {
                self.last_low = Some(swing.level);
                if let Some(last_high) = self.last_high {
                    self.leg_high = Some(last_high);
                    self.leg_low = Some(swing.level);
                    self.direction = Some(-1.0);
                    self.deepest = 0.0;
                }
            }
        }
        let mut current = f64::NAN;
        let mut deepest = f64::NAN;
        if let (Some(leg_high), Some(leg_low), Some(direction)) =
            (self.leg_high, self.leg_low, self.direction)
        {
            let range = leg_high - leg_low;
            if range > 0.0 {
                let pct = if direction > 0.0 {
                    (leg_high - close) / range * 100.0
                } else {
                    (close - leg_low) / range * 100.0
                };
                current = pct.max(0.0);
                self.deepest = self.deepest.max(current);
                deepest = self.deepest;
            }
        }
        let value = RetracementsValue {
            direction: self.direction.unwrap_or(f64::NAN),
            current_retracement_pct: current,
            deepest_retracement_pct: deepest,
        };
        self.value = Some(value);
        value
    }

    pub fn value(&self) -> Option<RetracementsValue> {
        self.value
    }

    pub fn reset(&mut self) {
        self.swing.reset();
        self.last_high = None;
        self.last_low = None;
        self.leg_high = None;
        self.leg_low = None;
        self.direction = None;
        self.deepest = 0.0;
        self.value = None;
    }
}
