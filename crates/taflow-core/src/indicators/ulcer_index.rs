//! Persistent Ulcer Index state.

use crate::error::TaResult;
use crate::stream::operator_states::{validate_period, ContiguousWindow};

#[derive(Debug, Clone)]
pub struct UlcerIndex {
    values: ContiguousWindow,
    period: usize,
    value: Option<f64>,
}

impl UlcerIndex {
    pub fn new(period: usize) -> TaResult<Self> {
        validate_period(period)?;
        Ok(Self {
            values: ContiguousWindow::new(period),
            period,
            value: None,
        })
    }

    pub fn append(&mut self, input: f64) -> Option<f64> {
        self.values.push(input);
        self.value = self.values.is_full().then(|| {
            let mut peak = f64::NEG_INFINITY;
            let sum = self
                .values
                .window()
                .iter()
                .map(|&value| {
                    peak = peak.max(value);
                    let drawdown = if peak != 0.0 {
                        100.0 * (value - peak) / peak
                    } else {
                        0.0
                    };
                    drawdown * drawdown
                })
                .sum::<f64>();
            (sum / self.period as f64).sqrt()
        });
        self.value
    }

    pub fn value(&self) -> Option<f64> {
        self.value
    }

    pub fn reset(&mut self) {
        self.values.clear();
        self.value = None;
    }
}
