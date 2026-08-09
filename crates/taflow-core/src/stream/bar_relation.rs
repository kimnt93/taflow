//! Shared bounded storage for two-bar high/low relations.

#[derive(Debug, Clone, Default)]
pub(crate) struct BarRelation {
    previous: Option<(f64, f64)>,
    value: Option<f64>,
}

impl BarRelation {
    pub(crate) fn append(
        &mut self,
        high: f64,
        low: f64,
        predicate: impl FnOnce(f64, f64, f64, f64) -> bool,
    ) -> Option<f64> {
        self.value = self.previous.map(|(previous_high, previous_low)| {
            f64::from(predicate(high, low, previous_high, previous_low))
        });
        self.previous = Some((high, low));
        self.value
    }

    pub(crate) fn value(&self) -> Option<f64> {
        self.value
    }

    pub(crate) fn reset(&mut self) {
        self.previous = None;
        self.value = None;
    }
}
