use crate::error::TaResult;

/// Aggregate advancing volume divided by aggregate declining volume.
#[derive(Debug, Clone, Default)]
pub struct UpDownVolumeRatio {
    count: usize,
    value: Option<f64>,
}

impl UpDownVolumeRatio {
    /// Create an empty ratio state.
    pub fn new() -> TaResult<Self> {
        Ok(Self::default())
    }
    /// Append one pair of pre-aggregated market-wide volumes.
    pub fn append(&mut self, advancing_volume: f64, declining_volume: f64) -> Option<f64> {
        self.count += 1;
        self.value = Some(advancing_volume / declining_volume.max(1.0));
        self.value
    }
    /// Return the latest ratio.
    pub fn value(&self) -> Option<f64> {
        self.value
    }
    /// Return the number of processed market ticks.
    pub fn len(&self) -> usize {
        self.count
    }
    /// Return whether no market ticks were processed.
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }
    /// Restore fresh-state behaviour.
    pub fn reset(&mut self) {
        self.count = 0;
        self.value = None;
    }
}
