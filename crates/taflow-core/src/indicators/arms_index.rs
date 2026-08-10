use crate::error::TaResult;
/// TRIN ratio of issue breadth to volume breadth.
#[derive(Debug, Clone, Default)]
pub struct ArmsIndex {
    count: usize,
    value: Option<f64>,
}
impl ArmsIndex {
    /// Create an empty Arms Index state.
    pub fn new() -> TaResult<Self> {
        Ok(Self::default())
    }
    /// Append pre-aggregated issue and volume breadth for one market tick.
    pub fn append(
        &mut self,
        advancers: f64,
        decliners: f64,
        advancing_volume: f64,
        declining_volume: f64,
    ) -> Option<f64> {
        self.count += 1;
        let advance_decline_ratio = advancers / decliners.max(1.0);
        let volume_ratio = advancing_volume.max(1.0) / declining_volume.max(1.0);
        self.value = Some(advance_decline_ratio / volume_ratio);
        self.value
    }
    /// Return the latest TRIN value.
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
        *self = Self::default();
    }
}
