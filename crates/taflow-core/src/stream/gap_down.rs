//! Persistent gap-down relation.

use super::bar_relation::BarRelation;

/// Emit `1` when the current high is below the previous low.
#[derive(Debug, Clone, Default)]
pub struct GapDown {
    relation: BarRelation,
}

impl GapDown {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn append(&mut self, high: f64, low: f64) -> Option<f64> {
        self.relation
            .append(high, low, |h, _, _, previous_l| h < previous_l)
    }

    pub fn value(&self) -> Option<f64> {
        self.relation.value()
    }

    pub fn reset(&mut self) {
        self.relation.reset();
    }
}
