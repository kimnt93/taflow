//! Persistent Intraday Momentum Index state.

use super::{invalid_period, Window};
use crate::error::TaResult;

/// Incremental Intraday Momentum Index with TA-Lib-compatible warm-up.
#[derive(Debug, Clone)]
pub struct IntradayMomentumIndex {
    gains: Window,
    losses: Window,
    gain_sum: f64,
    loss_sum: f64,
    value: Option<f64>,
}

impl IntradayMomentumIndex {
    pub fn new(period: usize) -> TaResult<Self> {
        if period < 2 {
            return Err(invalid_period("timeperiod", period, 2));
        }
        Ok(Self {
            gains: Window::new(period)?,
            losses: Window::new(period)?,
            gain_sum: 0.0,
            loss_sum: 0.0,
            value: None,
        })
    }

    pub fn append(&mut self, open: f64, close: f64) -> Option<f64> {
        let movement = close - open;
        let (gain, loss) = if movement > 0.0 {
            (movement, 0.0)
        } else {
            (0.0, -movement)
        };
        if let Some(expired) = self.gains.push(gain) {
            self.gain_sum -= expired;
        }
        if let Some(expired) = self.losses.push(loss) {
            self.loss_sum -= expired;
        }
        self.gain_sum += gain;
        self.loss_sum += loss;
        self.value = self.gains.is_full().then(|| {
            let total = self.gain_sum + self.loss_sum;
            if total == 0.0 {
                50.0
            } else {
                100.0 * self.gain_sum / total
            }
        });
        self.value
    }

    pub fn extend_slice_into(
        &mut self,
        open: &[f64],
        close: &[f64],
        output: &mut Vec<f64>,
    ) -> TaResult<()> {
        if open.len() != close.len() {
            return Err(crate::error::TaError::LengthMismatch {
                expected: open.len(),
                got: close.len(),
            });
        }
        for (&open, &close) in open.iter().zip(close) {
            output.push(self.append(open, close).unwrap_or(f64::NAN));
        }
        Ok(())
    }

    pub fn value(&self) -> Option<f64> {
        self.value
    }
    pub fn reset(&mut self) {
        self.gains.clear();
        self.losses.clear();
        self.gain_sum = 0.0;
        self.loss_sum = 0.0;
        self.value = None;
    }
}
