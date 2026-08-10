use crate::error::TaResult;
/// Running sum of volume-normalized net advancing volume.
#[derive(Debug, Clone, Default)]
pub struct CumulativeVolumeIndex {
    total: f64,
    count: usize,
    value: Option<f64>,
}
impl CumulativeVolumeIndex {
    /// Create an empty cumulative index.
    pub fn new() -> TaResult<Self> {
        Ok(Self::default())
    }
    /// Append aggregate advancing and declining volume for one market tick.
    pub fn append(&mut self, advancing_volume: f64, declining_volume: f64) -> Option<f64> {
        self.count += 1;
        let volume = (advancing_volume + declining_volume).max(f64::MIN_POSITIVE);
        self.total += (advancing_volume - declining_volume) / volume;
        self.value = Some(self.total);
        self.value
    }
    /// Return the number of processed market ticks.
    pub fn len(&self) -> usize {
        self.count
    }
    /// Return the latest cumulative index value.
    pub fn value(&self) -> Option<f64> {
        self.value
    }
    /// Clear the cumulative total and processed count.
    pub fn reset(&mut self) {
        self.total = 0.0;
        self.count = 0;
        self.value = None;
    }
}
