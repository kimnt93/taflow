use super::operator_states::ContiguousWindow;
use crate::error::TaResult;

/// Rolling information ratio of the input-minus-benchmark differential.
#[derive(Debug, Clone)]
pub struct RollingInformationRatio {
    values: ContiguousWindow,
    period: usize,
    value: Option<f64>,
}

impl RollingInformationRatio {
    /// Create a validated empty rolling information-ratio state.
    pub fn new(period: usize) -> TaResult<Self> {
        super::operator_states::validate_period(period)?;
        Ok(Self {
            values: ContiguousWindow::new(period),
            period,
            value: None,
        })
    }

    /// Append one input/benchmark pair and return the latest ratio.
    pub fn append(&mut self, input: f64, benchmark: f64) -> Option<f64> {
        self.values.push(input - benchmark);
        self.value = self.values.is_full().then(|| {
            let window = self.values.window();
            let n = self.period as f64;
            let mean = window.iter().sum::<f64>() / n;
            let variance = window
                .iter()
                .map(|&value| (value - mean).powi(2))
                .sum::<f64>()
                / n;
            if variance > 0.0 {
                mean / variance.sqrt()
            } else {
                0.0
            }
        });
        self.value
    }

    /// Return the latest ratio, or `None` until the window is full.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Reset the bounded rolling state without reallocating it.
    pub fn reset(&mut self) {
        self.values.clear();
        self.value = None;
    }
}
