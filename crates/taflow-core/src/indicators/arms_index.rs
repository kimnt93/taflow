use crate::error::TaResult;
#[derive(Debug, Clone, Default)]
pub struct ArmsIndex {
    advances: f64,
    declines: f64,
    up_volume: f64,
    down_volume: f64,
    count: usize,
    value: Option<f64>,
}
impl ArmsIndex {
    pub fn new() -> TaResult<Self> {
        Ok(Self::default())
    }
    pub fn append(
        &mut self,
        change: f64,
        volume: f64,
        _new_high: f64,
        _new_low: f64,
    ) -> Option<f64> {
        self.count += 1;
        if change > 0.0 {
            self.advances += change;
            self.up_volume += volume
        } else if change < 0.0 {
            self.declines -= change;
            self.down_volume += volume
        }
        self.value = (self.declines != 0.0 && self.up_volume != 0.0 && self.down_volume != 0.0)
            .then(|| (self.advances / self.declines) / (self.up_volume / self.down_volume));
        self.value
    }
    pub fn value(&self) -> Option<f64> {
        self.value
    }
    pub fn len(&self) -> usize {
        self.count
    }
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}
