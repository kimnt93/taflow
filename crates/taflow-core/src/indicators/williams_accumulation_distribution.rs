use crate::error::TaResult;

/// Larry Williams' cumulative price-only accumulation/distribution line.
#[derive(Debug, Clone)]
pub struct WilliamsAccumulationDistribution {
    previous_close: Option<f64>,
    total: f64,
    value: Option<f64>,
}

impl WilliamsAccumulationDistribution {
    /// Create a fresh parameter-free WAD state.
    pub fn new() -> TaResult<Self> {
        Ok(Self {
            previous_close: None,
            total: 0.0,
            value: None,
        })
    }

    /// Append one high/low/close bar and return WAD after the seed bar.
    pub fn append(&mut self, high: f64, low: f64, close: f64) -> Option<f64> {
        let Some(previous) = self.previous_close.replace(close) else {
            self.value = None;
            return None;
        };
        self.total += if close > previous {
            close - low.min(previous)
        } else if close < previous {
            close - high.max(previous)
        } else {
            0.0
        };
        self.value = Some(self.total);
        self.value
    }

    /// Return the latest WAD, or `None` before the second bar.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Restore fresh-state cumulative behavior.
    pub fn reset(&mut self) {
        self.previous_close = None;
        self.total = 0.0;
        self.value = None;
    }
}
