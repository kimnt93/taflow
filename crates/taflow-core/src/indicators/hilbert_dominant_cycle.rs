use crate::error::TaResult;
use crate::indicators::HilbertTransformDominantCyclePeriod;
use crate::stream::StreamingIndicator;

/// Consolidated dominant-cycle period estimated by the Hilbert transform.
pub struct HilbertDominantCycle {
    period: HilbertTransformDominantCyclePeriod,
}

impl HilbertDominantCycle {
    pub fn new() -> TaResult<Self> {
        Ok(Self {
            period: HilbertTransformDominantCyclePeriod::new(),
        })
    }
    pub fn append(&mut self, price: f64) -> Option<f64> {
        self.period.append(price)
    }
    pub fn value(&self) -> Option<f64> {
        self.period.value()
    }
    pub fn reset(&mut self) {
        self.period.reset();
    }
}

impl StreamingIndicator for HilbertDominantCycle {
    type Output = f64;
    fn append(&mut self, value: f64) -> Option<f64> {
        Self::append(self, value)
    }
    fn value(&self) -> Option<f64> {
        Self::value(self)
    }
    fn reset(&mut self) {
        Self::reset(self)
    }
}
