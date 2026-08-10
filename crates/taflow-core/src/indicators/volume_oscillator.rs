use crate::error::TaResult;
use crate::stream::StreamingIndicator;

#[derive(Debug, Clone)]
pub struct VolumeOscillator {
    fast: usize,
    slow: usize,
    count: usize,
    fast_value: f64,
    slow_value: f64,
    value: Option<f64>,
}
impl VolumeOscillator {
    pub fn new(fast: usize, slow: usize) -> TaResult<Self> {
        if fast == 0 || slow == 0 {
            return Err(crate::error::TaError::InvalidParameter {
                name: "period",
                value: format!("{fast}/{slow}"),
                reason: "must be >= 1",
            });
        }
        Ok(Self {
            fast,
            slow,
            count: 0,
            fast_value: 0.0,
            slow_value: 0.0,
            value: None,
        })
    }
    pub fn append(&mut self, input: f64) -> Option<f64> {
        self.count += 1;
        let af = 2.0 / (self.fast as f64 + 1.0);
        let aslow = 2.0 / (self.slow as f64 + 1.0);
        if self.count == 1 {
            self.fast_value = input;
            self.slow_value = input;
        } else {
            self.fast_value += af * (input - self.fast_value);
            self.slow_value += aslow * (input - self.slow_value);
        }
        self.value =
            (self.count >= self.slow.max(self.fast)).then_some(self.fast_value - self.slow_value);
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
impl StreamingIndicator for VolumeOscillator {
    type Output = f64;
    fn append(&mut self, x: f64) -> Option<f64> {
        Self::append(self, x)
    }
    fn value(&self) -> Option<f64> {
        Self::value(self)
    }
    fn reset(&mut self) {
        Self::reset(self);
    }
}
