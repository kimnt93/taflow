//! Persistent outside-bar relation.

use crate::stream::bar_relation::BarRelation;

/// Emit `1` when the current range strictly contains the previous range.
#[derive(Debug, Clone, Default)]
pub struct OutsideBar {
    relation: BarRelation,
}

impl OutsideBar {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn append(&mut self, high: f64, low: f64) -> Option<f64> {
        self.relation
            .append(high, low, |h, l, previous_h, previous_l| {
                h > previous_h && l < previous_l
            })
    }

    pub fn value(&self) -> Option<f64> {
        self.relation.value()
    }

    pub fn reset(&mut self) {
        self.relation.reset();
    }
}
