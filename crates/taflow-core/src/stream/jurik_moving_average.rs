//! Stateful pandas-ta-classic Jurik moving average reconstruction.

use super::{invalid_period, StreamingIndicator};
use crate::error::TaResult;
use std::collections::VecDeque;

/// Computes the documented adaptive Jurik-like moving-average recurrence.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `JurikMovingAverage`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct JurikMovingAverage {
    period: usize,
    phase_ratio: f64,
    length1: f64,
    pow1: f64,
    bet: f64,
    beta: f64,
    index: usize,
    upper_band: f64,
    lower_band: f64,
    ma1: f64,
    det0: f64,
    det1: f64,
    jma: f64,
    volatility: VecDeque<f64>,
    volatility_sum: f64,
    volatility_sums: VecDeque<f64>,
    volatility_sums_total: f64,
    value: Option<f64>,
}

impl JurikMovingAverage {
    /// Creates the adaptive average from a positive length and phase value.
    pub fn new(period: usize, phase: f64) -> TaResult<Self> {
        if period < 1 {
            return Err(invalid_period("length", period, 1));
        }
        let half_length = 0.5 * (period as f64 - 1.0);
        let length1 = ((half_length.sqrt().ln() / 2.0_f64.ln()) + 2.0).max(0.0);
        let pow1 = (length1 - 2.0).max(0.5);
        let length2 = length1 * half_length.sqrt();
        let bet = length2 / (length2 + 1.0);
        let beta = 0.45 * (period as f64 - 1.0) / (0.45 * (period as f64 - 1.0) + 2.0);
        let phase_ratio = if phase < -100.0 {
            0.5
        } else if phase > 100.0 {
            2.5
        } else {
            phase / 100.0 + 1.5
        };
        Ok(Self {
            period,
            phase_ratio,
            length1,
            pow1,
            bet,
            beta,
            index: 0,
            upper_band: 0.0,
            lower_band: 0.0,
            ma1: 0.0,
            det0: 0.0,
            det1: 0.0,
            jma: 0.0,
            volatility: VecDeque::with_capacity(11),
            volatility_sum: 0.0,
            volatility_sums: VecDeque::with_capacity(67),
            volatility_sums_total: 0.0,
            value: None,
        })
    }
}

impl StreamingIndicator for JurikMovingAverage {
    type Output = f64;

    fn append(&mut self, input: f64) -> Option<f64> {
        if self.index == 0 {
            self.index = 1;
            self.upper_band = input;
            self.lower_band = input;
            self.ma1 = input;
            self.jma = input;
            self.volatility.push_back(0.0);
            self.volatility_sums.push_back(0.0);
            self.value = (self.period == 1).then_some(input);
            return self.value;
        }

        let del1 = input - self.upper_band;
        let del2 = input - self.lower_band;
        let volatility = if del1.abs() == del2.abs() {
            0.0
        } else {
            del1.abs().max(del2.abs())
        };
        let old_volatility = if self.index >= 10 {
            self.volatility.front().copied().unwrap_or(0.0)
        } else {
            0.0
        };
        self.volatility.push_back(volatility);
        if self.volatility.len() > 10 {
            self.volatility.pop_front();
        }
        self.volatility_sum += (volatility - old_volatility) / 10.0;
        self.volatility_sums.push_back(self.volatility_sum);
        self.volatility_sums_total += self.volatility_sum;
        if self.volatility_sums.len() > 66 {
            self.volatility_sums_total -= self.volatility_sums.pop_front().unwrap_or(0.0);
        }
        let average_volatility = self.volatility_sums_total / self.volatility_sums.len() as f64;
        let dynamic_volatility = if average_volatility == 0.0 {
            0.0
        } else {
            volatility / average_volatility
        };
        let relative_volatility = dynamic_volatility
            .min(self.length1.powf(1.0 / self.pow1))
            .max(1.0);
        let power = relative_volatility.powf(self.pow1);
        let kv = self.bet.powf(power.sqrt());
        self.upper_band = if del1 > 0.0 { input } else { input - kv * del1 };
        self.lower_band = if del2 < 0.0 { input } else { input - kv * del2 };
        let alpha = self.beta.powf(power);
        self.ma1 = (1.0 - alpha) * input + alpha * self.ma1;
        self.det0 = (input - self.ma1) * (1.0 - self.beta) + self.beta * self.det0;
        let ma2 = self.ma1 + self.phase_ratio * self.det0;
        self.det1 = (ma2 - self.jma) * (1.0 - alpha).powi(2) + alpha.powi(2) * self.det1;
        self.jma += self.det1;
        self.index += 1;
        self.value = (self.index >= self.period).then_some(self.jma);
        self.value
    }

    fn value(&self) -> Option<f64> {
        self.value
    }
    fn reset(&mut self) {
        self.index = 0;
        self.upper_band = 0.0;
        self.lower_band = 0.0;
        self.ma1 = 0.0;
        self.det0 = 0.0;
        self.det1 = 0.0;
        self.jma = 0.0;
        self.volatility.clear();
        self.volatility_sum = 0.0;
        self.volatility_sums.clear();
        self.volatility_sums_total = 0.0;
        self.value = None;
    }
}
