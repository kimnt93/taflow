use std::collections::VecDeque;
use std::f64::consts::PI;

use crate::error::{TaError, TaResult};
use crate::stream::StreamingIndicator;

use super::SuperSmoother;

/// Ehlers empirical-mode line built from a resonant bandpass and envelopes.
#[derive(Debug, Clone)]
pub struct EmpiricalModeDecomposition {
    period: usize,
    fraction: f64,
    bandpass: f64,
    previous_bandpass_1: f64,
    previous_bandpass_2: f64,
    previous_input_1: Option<f64>,
    previous_input_2: Option<f64>,
    beta: f64,
    alpha: f64,
    smoother: SuperSmoother,
    peak_smoother: SuperSmoother,
    valley_smoother: SuperSmoother,
    bandpass_history: VecDeque<f64>,
    history_length: usize,
    value: Option<f64>,
}

impl EmpiricalModeDecomposition {
    /// Create an EMD state with a centre period and envelope-window fraction.
    pub fn new(period: usize, fraction: f64) -> TaResult<Self> {
        if period == 0 {
            return Err(TaError::InvalidParameter {
                name: "period",
                value: period.to_string(),
                reason: "must be positive",
            });
        }
        if !fraction.is_finite() || fraction <= 0.0 || fraction > 1.0 {
            return Err(TaError::InvalidParameter {
                name: "fraction",
                value: fraction.to_string(),
                reason: "must be finite and in (0, 1]",
            });
        }

        let beta = (2.0 * PI / period as f64).cos();
        let gamma = 1.0 / (2.0 * PI * 0.25 / period as f64).cos();
        let alpha = gamma - (gamma * gamma - 1.0).sqrt();
        let history_length = (period as f64 * fraction).round().max(1.0) as usize;
        Ok(Self {
            period,
            fraction,
            bandpass: 0.0,
            previous_bandpass_1: 0.0,
            previous_bandpass_2: 0.0,
            previous_input_1: None,
            previous_input_2: None,
            beta,
            alpha,
            smoother: SuperSmoother::new(period.max(2))?,
            peak_smoother: SuperSmoother::new(period.max(2))?,
            valley_smoother: SuperSmoother::new(period.max(2))?,
            bandpass_history: VecDeque::with_capacity(history_length),
            history_length,
            value: None,
        })
    }

    /// Append one chronological price and return the current EMD line.
    pub fn append(&mut self, input: f64) -> Option<f64> {
        // Wickra treats a non-finite sample as a missing update and preserves
        // the latest result without advancing any recursive state.
        if !input.is_finite() {
            return self.value;
        }

        let bandpass =
            if let (Some(_), Some(input_2)) = (self.previous_input_1, self.previous_input_2) {
                0.5 * (1.0 - self.alpha) * (input - input_2)
                    + self.beta * (1.0 + self.alpha) * self.previous_bandpass_1
                    - self.alpha * self.previous_bandpass_2
            } else {
                0.0
            };
        self.previous_bandpass_2 = self.previous_bandpass_1;
        self.previous_bandpass_1 = bandpass;
        self.bandpass = bandpass;
        self.previous_input_2 = self.previous_input_1;
        self.previous_input_1 = Some(input);

        if self.bandpass_history.len() == self.history_length {
            self.bandpass_history.pop_front();
        }
        self.bandpass_history.push_back(bandpass);
        if self.bandpass_history.len() < self.history_length {
            return None;
        }

        let peak = self
            .bandpass_history
            .iter()
            .copied()
            .fold(f64::NEG_INFINITY, f64::max);
        let valley = self
            .bandpass_history
            .iter()
            .copied()
            .fold(f64::INFINITY, f64::min);
        let average_peak = self.peak_smoother.append(peak)?;
        let average_valley = self.valley_smoother.append(valley)?;
        let envelope_mean = 0.5 * (average_peak + average_valley);
        self.value = self.smoother.append(bandpass - envelope_mean);
        self.value
    }

    /// Return the latest EMD line, or `None` during warm-up.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Restore fresh-state behaviour while retaining allocated storage.
    pub fn reset(&mut self) {
        self.bandpass = 0.0;
        self.previous_bandpass_1 = 0.0;
        self.previous_bandpass_2 = 0.0;
        self.previous_input_1 = None;
        self.previous_input_2 = None;
        self.smoother.reset();
        self.peak_smoother.reset();
        self.valley_smoother.reset();
        self.bandpass_history.clear();
        self.value = None;
    }

    /// Return the configured centre period.
    pub const fn period(&self) -> usize {
        self.period
    }

    /// Return the configured peak/valley history fraction.
    pub const fn fraction(&self) -> f64 {
        self.fraction
    }
}

impl StreamingIndicator for EmpiricalModeDecomposition {
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
