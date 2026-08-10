use crate::error::TaResult;
/// Magnitude of the difference between advancing and declining issue counts.
#[derive(Debug, Clone, Default)]
pub struct AbsoluteBreadthIndex {
    count: usize,
    value: Option<f64>,
}
impl AbsoluteBreadthIndex {
    /// Create an empty breadth state.
    pub fn new() -> TaResult<Self> {
        Ok(Self::default())
    }
    /// Append one pair of pre-aggregated cross-sectional issue counts.
    pub fn append(&mut self, advancers: f64, decliners: f64) -> Option<f64> {
        self.count += 1;
        self.value = Some((advancers - decliners).abs());
        self.value
    }
    /// Return the number of processed market ticks.
    pub fn len(&self) -> usize {
        self.count
    }
    /// Return the most recent breadth magnitude.
    pub fn value(&self) -> Option<f64> {
        self.value
    }
    /// Restore fresh-state behaviour without reallocating.
    pub fn reset(&mut self) {
        self.count = 0;
        self.value = None;
    }
}
