use crate::error::TaResult;
/// Percentage of constituents above their caller-selected moving average.
#[derive(Debug, Clone, Default)]
pub struct PercentAboveMovingAverage {
    count: usize,
    value: Option<f64>,
}
impl PercentAboveMovingAverage {
    /// Create an empty percentage state.
    pub fn new() -> TaResult<Self> {
        Ok(Self::default())
    }
    /// Append above-average and universe counts for one market tick.
    pub fn append(&mut self, above_moving_average_count: f64, universe_size: f64) -> Option<f64> {
        self.count += 1;
        self.value = Some(100.0 * above_moving_average_count / universe_size.max(1.0));
        self.value
    }
    /// Return the number of processed market ticks.
    pub fn len(&self) -> usize {
        self.count
    }
    /// Return the latest percentage above the moving average.
    pub fn value(&self) -> Option<f64> {
        self.value
    }
    /// Restore fresh-state behaviour.
    pub fn reset(&mut self) {
        self.count = 0;
        self.value = None;
    }
}
