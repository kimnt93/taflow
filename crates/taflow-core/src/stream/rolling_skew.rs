//! Stateful rolling rollingskew indicator.

use crate::error::TaResult;

use super::operator_states::validate_period;
use super::Window;

/// Persistent trailing rollingskew computed from a fixed-size moment window.
#[derive(Debug, Clone)]
pub struct RollingSkew {
    values: Window,
    timeperiod: usize,
    nobs: usize,
    mean: f64,
    m2: f64,
    m3: f64,
    m4: f64,
    value: Option<f64>,
}

impl RollingSkew {
    /// Create a state with a positive trailing period.
    pub fn new(timeperiod: usize) -> TaResult<Self> {
        validate_period(timeperiod)?;
        Ok(Self {
            values: Window::new(timeperiod)?,
            timeperiod,
            nobs: 0,
            mean: 0.0,
            m2: 0.0,
            m3: 0.0,
            m4: 0.0,
            value: None,
        })
    }

    /// Append one value and return the statistic after warm-up.
    pub fn append(&mut self, input: f64) -> Option<f64> {
        if let Some(old) = self.values.push(input) {
            let n = (self.nobs - 1) as f64;
            let delta = old - self.mean;
            let delta_n = delta / n;
            let term1 = delta_n * delta * (n + 1.0);
            let old_m2 = self.m2;
            let old_m3 = self.m3;
            self.m4 += delta_n
                * (4.0 * old_m3 + delta_n * (6.0 * old_m2 - term1 * (n * n + 3.0 * n + 3.0)));
            self.m3 = old_m3 - delta_n * (term1 * (n + 2.0) - 3.0 * old_m2);
            self.m2 = old_m2 - term1;
            self.mean -= delta_n;
            self.nobs -= 1;
        }
        let n_old = self.nobs as f64;
        let n = n_old + 1.0;
        let delta = input - self.mean;
        let delta_n = delta / n;
        let term1 = delta * delta_n * n_old;
        let old_m2 = self.m2;
        let old_m3 = self.m3;
        self.m4 +=
            delta_n * (-4.0 * old_m3 + delta_n * (6.0 * old_m2 + term1 * (n * n - 3.0 * n + 3.0)));
        self.m3 += delta_n * (term1 * (n - 2.0) - 3.0 * old_m2);
        self.m2 = old_m2 + term1;
        self.mean += delta_n;
        self.nobs += 1;
        self.value = if self.nobs == self.timeperiod {
            Some(if self.m2 > 0.0 {
                (self.nobs as f64).sqrt() * self.m3 / self.m2.powf(1.5)
            } else {
                0.0
            })
        } else {
            None
        };
        self.value
    }

    /// Extend a chronological slice and append aligned NaN warm-up values.
    pub fn extend_slice_into(&mut self, input: &[f64], output: &mut Vec<f64>) {
        output.extend(
            input
                .iter()
                .copied()
                .map(|value| self.append(value).unwrap_or(f64::NAN)),
        );
    }

    /// Return the latest value, or None during warm-up.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Reset the state without reallocating its window.
    pub fn reset(&mut self) {
        self.values.clear();
        self.nobs = 0;
        self.mean = 0.0;
        self.m2 = 0.0;
        self.m3 = 0.0;
        self.m4 = 0.0;
        self.value = None;
    }
}
