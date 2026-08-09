//! Persistent inside-bar relation.

use crate::stream::bar_relation::BarRelation;

/// Emit `1` when the current range is strictly inside the previous range.
#[derive(Debug, Clone, Default)]
pub struct InsideBar {
    relation: BarRelation,
}

impl InsideBar {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn append(&mut self, high: f64, low: f64) -> Option<f64> {
        self.relation
            .append(high, low, |h, l, previous_h, previous_l| {
                h < previous_h && l > previous_l
            })
    }

    pub fn value(&self) -> Option<f64> {
        self.relation.value()
    }

    pub fn reset(&mut self) {
        self.relation.reset();
    }
}
