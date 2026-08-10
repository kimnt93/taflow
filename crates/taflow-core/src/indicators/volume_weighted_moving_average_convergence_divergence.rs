use crate::error::TaResult;
#[derive(Debug, Clone)]
pub struct VolumeWeightedMovingAverageConvergenceDivergence {
    fast: usize,
    slow: usize,
    fc: f64,
    sc: f64,
    fast_value: f64,
    slow_value: f64,
    count: usize,
    value: Option<f64>,
}
impl VolumeWeightedMovingAverageConvergenceDivergence {
    pub fn new(fast: usize, slow: usize) -> TaResult<Self> {
        Ok(Self {
            fast,
            slow,
            fc: 0.0,
            sc: 0.0,
            fast_value: 0.0,
            slow_value: 0.0,
            count: 0,
            value: None,
        })
    }
    pub fn append(&mut self, close: f64, volume: f64) -> Option<f64> {
        let x = close * volume;
        self.count += 1;
        let f = 2.0 / (self.fast as f64 + 1.0);
        let s = 2.0 / (self.slow as f64 + 1.0);
        if self.count == 1 {
            self.fast_value = x;
            self.slow_value = x
        } else {
            self.fast_value += f * (x - self.fast_value);
            self.slow_value += s * (x - self.slow_value);
        }
        self.value = (self.count >= self.slow).then_some(self.fast_value - self.slow_value);
        self.value
    }
    pub fn value(&self) -> Option<f64> {
        self.value
    }
    pub fn reset(&mut self) {
        self.count = 0;
        self.fast_value = 0.0;
        self.slow_value = 0.0;
        self.value = None;
    }
}
