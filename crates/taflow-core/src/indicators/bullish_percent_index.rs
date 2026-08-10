use crate::error::TaResult;
/// Percentage of a universe currently on point-and-figure buy signals.
#[derive(Debug, Clone, Default)]
pub struct BullishPercentIndex {
    count: usize,
    value: Option<f64>,
}
impl BullishPercentIndex {
    /// Create an empty percentage state.
    pub fn new() -> TaResult<Self> {
        Ok(Self::default())
    }
    /// Append buy-signal and universe counts for one market tick.
    pub fn append(&mut self, on_buy_signal_count: f64, universe_size: f64) -> Option<f64> {
        self.count += 1;
        self.value = Some(100.0 * on_buy_signal_count / universe_size.max(1.0));
        self.value
    }
    /// Return the number of processed market ticks.
    pub fn len(&self) -> usize {
        self.count
    }
    /// Return the latest bullish percentage.
    pub fn value(&self) -> Option<f64> {
        self.value
    }
    /// Restore fresh-state behaviour.
    pub fn reset(&mut self) {
        self.count = 0;
        self.value = None;
    }
}
