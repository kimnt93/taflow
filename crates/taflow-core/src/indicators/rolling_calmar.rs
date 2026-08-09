//! Persistent rolling Calmar ratio state.

use crate::error::TaResult;
use crate::stream::operator_states::{validate_period, ContiguousWindow};

#[derive(Debug, Clone)]
pub struct RollingCalmar {
    values: ContiguousWindow,
    timeperiod: usize,
    value: Option<f64>,
}

impl RollingCalmar {
    pub fn new(timeperiod: usize) -> TaResult<Self> {
        validate_period(timeperiod)?;
        Ok(Self {
            values: ContiguousWindow::new(timeperiod),
            timeperiod,
            value: None,
        })
    }

    pub fn append(&mut self, input: f64) -> Option<f64> {
        self.values.push(input);
        self.value = self.values.is_full().then(|| {
            let window = self.values.window();
            let mut sum = 0.0;
            let mut peak = window[0];
            let mut drawdown: f64 = 0.0;
            for &value in window {
                sum += value;
                peak = peak.max(value);
                drawdown = drawdown.min(if peak != 0.0 { value / peak - 1.0 } else { 0.0 });
            }
            let average = sum / self.timeperiod as f64;
            if drawdown < 0.0 {
                average / -drawdown
            } else {
                0.0
            }
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
