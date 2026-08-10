use crate::error::TaResult;
/// Difference between cross-sectional new-high and new-low counts.
#[derive(Debug, Clone, Default)]
pub struct NewHighsNewLows {
    count: usize,
    value: Option<f64>,
}
impl NewHighsNewLows {
    /// Create an empty net-extremes state.
    pub fn new() -> TaResult<Self> {
        Ok(Self::default())
    }
    /// Append aggregate new-high and new-low counts for one market tick.
    pub fn append(&mut self, new_highs: f64, new_lows: f64) -> Option<f64> {
        self.count += 1;
        self.value = Some(new_highs - new_lows);
        self.value
    }
    /// Return the latest net new-extremes count.
    pub fn value(&self) -> Option<f64> {
        self.value
    }
    /// Return the number of processed market ticks.
    pub fn len(&self) -> usize {
        self.count
    }
    /// Return whether no market ticks have been processed.
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }
    /// Restore fresh-state behaviour.
    pub fn reset(&mut self) {
        self.count = 0;
        self.value = None;
    }
}
