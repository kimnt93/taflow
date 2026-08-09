//! Persistent higher-high relation.

use crate::stream::bar_relation::BarRelation;

/// Emit `1` when the current high exceeds the previous high, otherwise `0`.
#[derive(Debug, Clone, Default)]
pub struct HigherHigh {
    relation: BarRelation,
}

impl HigherHigh {
    /// Create an empty state; the first appended bar is warm-up.
    pub fn new() -> Self {
        Self::default()
    }

    /// Append one high/low bar in chronological order.
    pub fn append(&mut self, high: f64, low: f64) -> Option<f64> {
        self.relation
            .append(high, low, |h, _, previous_h, _| h > previous_h)
    }

    /// Return the latest relation value, or `None` before two bars exist.
    pub fn value(&self) -> Option<f64> {
        self.relation.value()
    }

    /// Restore fresh-state behavior.
    pub fn reset(&mut self) {
        self.relation.reset();
    }
}
