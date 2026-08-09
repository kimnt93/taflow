//! Persistent lower-low relation.

use super::bar_relation::BarRelation;

/// Emit `1` when the current low is below the previous low, otherwise `0`.
#[derive(Debug, Clone, Default)]
pub struct LowerLow {
    relation: BarRelation,
}

impl LowerLow {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn append(&mut self, high: f64, low: f64) -> Option<f64> {
        self.relation
            .append(high, low, |_, l, _, previous_l| l < previous_l)
    }

    pub fn value(&self) -> Option<f64> {
        self.relation.value()
    }

    pub fn reset(&mut self) {
        self.relation.reset();
    }
}
