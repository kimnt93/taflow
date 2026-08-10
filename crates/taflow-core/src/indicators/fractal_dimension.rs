use crate::error::TaResult;
use crate::indicators::hurst::Hurst;

#[derive(Debug, Clone)]
/// Causal rolling fractal dimension defined as `2 - Hurst`.
pub struct FractalDimension {
    hurst: Hurst,
    value: Option<f64>,
}

impl FractalDimension {
    pub fn new(period: usize) -> TaResult<Self> {
        Ok(Self {
            hurst: Hurst::new(period, 2)?,
            value: None,
        })
    }

    pub fn append(&mut self, input: f64) -> Option<f64> {
        self.value = self.hurst.append(input).map(|hurst| 2.0 - hurst);
        self.value
    }

    pub fn value(&self) -> Option<f64> {
        self.value
    }

    pub fn reset(&mut self) {
        self.hurst.reset();
        self.value = None;
    }
}
