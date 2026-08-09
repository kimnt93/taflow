//! Persistent gap-up relation.

use crate::stream::bar_relation::BarRelation;

/// Emit `1` when the current low is above the previous high.
#[derive(Debug, Clone, Default)]
pub struct GapUp {
    relation: BarRelation,
}

impl GapUp {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn append(&mut self, high: f64, low: f64) -> Option<f64> {
        self.relation
            .append(high, low, |_, l, previous_h, _| l > previous_h)
    }

    pub fn value(&self) -> Option<f64> {
        self.relation.value()
    }

    pub fn reset(&mut self) {
        self.relation.reset();
    }
}
