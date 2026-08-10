use crate::error::TaResult;
use crate::indicators::HilbertDominantCycle;
use crate::stream::StreamingIndicator;

/// Half of Ehlers' dominant cycle, rounded and clamped for adaptive oscillators.
#[derive(Debug, Clone)]
pub struct AdaptiveCycle {
    cycle: HilbertDominantCycle,
    value: Option<f64>,
}

impl AdaptiveCycle {
    /// Create an empty adaptive-cycle estimator.
    pub fn new() -> TaResult<Self> {
        Ok(Self {
            cycle: HilbertDominantCycle::new()?,
            value: None,
        })
    }

    /// Append one price and return an integer-valued period in `[3, 25]`.
    pub fn append(&mut self, input: f64) -> Option<f64> {
        let period = self.cycle.append(input)?;
        self.value = Some((period * 0.5).round().clamp(3.0, 25.0));
        self.value
    }

    /// Return the latest adaptive period, or `None` during warm-up.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Reset the composed dominant-cycle state and latest value.
    pub fn reset(&mut self) {
        self.cycle.reset();
        self.value = None;
    }
}

impl StreamingIndicator for AdaptiveCycle {
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
