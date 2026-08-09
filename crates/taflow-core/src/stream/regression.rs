//! Shared state for the rolling linear-regression indicator classes.

use crate::error::TaResult;

use super::{invalid_period, Window};

#[derive(Debug, Clone, Copy)]
pub(crate) struct RegressionValue {
    pub(crate) slope: f64,
    pub(crate) intercept: f64,
}

#[derive(Debug, Clone)]
pub(crate) struct RegressionCore {
    pub(crate) period: usize,
    period_f: f64,
    sum_x: f64,
    denominator: f64,
    pub(crate) window: Window,
    seeded: bool,
}

impl RegressionCore {
    pub(crate) fn new(period: usize) -> TaResult<Self> {
        if period < 2 {
            return Err(invalid_period("timeperiod", period, 2));
        }
        let period_f = period as f64;
        let sum_x = period_f * (period_f - 1.0) / 2.0;
        let sum_x2 = period_f * (period_f - 1.0) * (2.0 * period_f - 1.0) / 6.0;
        Ok(Self {
            period,
            period_f,
            sum_x,
            denominator: period_f * sum_x2 - sum_x * sum_x,
            window: Window::new(period)?,
            seeded: false,
        })
    }

    pub(crate) fn append(&mut self, input: f64) -> Option<RegressionValue> {
        if !self.seeded {
            self.window.push(input);
            if !self.window.is_full() {
                return None;
            }
            self.seeded = true;
        } else {
            self.window.push(input).expect("regression window is full");
        }
        Some(self.compute_window())
    }

    pub(crate) fn compute_window(&self) -> RegressionValue {
        let mut sum_y = 0.0;
        let mut weighted_sum = 0.0;
        for (index, &value) in self.window.iter().enumerate() {
            sum_y += value;
            weighted_sum += index as f64 * value;
        }
        let slope = (self.period_f * weighted_sum - self.sum_x * sum_y) / self.denominator;
        let intercept = (sum_y - slope * self.sum_x) / self.period_f;
        RegressionValue { slope, intercept }
    }

    pub(crate) fn reset(&mut self) {
        self.window.clear();
        self.seeded = false;
    }

    pub(crate) fn extend_map_into(
        &mut self,
        inputs: &[f64],
        output: &mut Vec<f64>,
        mut map: impl FnMut(RegressionValue) -> f64,
    ) -> Option<f64> {
        let period = self.period;
        let n = inputs.len();
        output.reserve(n);
        let prologue = n.min(period - 1);
        let mut last = None;
        for &input in &inputs[..prologue] {
            last = self.append(input).map(&mut map);
            output.push(last.unwrap_or(f64::NAN));
        }
        if n < period {
            return last;
        }
        for i in (period - 1)..n {
            let window = &inputs[i + 1 - period..=i];
            let mut sum_y = 0.0;
            let mut weighted_sum = 0.0;
            for (index, &value) in window.iter().enumerate() {
                sum_y += value;
                weighted_sum += index as f64 * value;
            }
            let slope = (self.period_f * weighted_sum - self.sum_x * sum_y) / self.denominator;
            let intercept = (sum_y - slope * self.sum_x) / self.period_f;
            let mapped = map(RegressionValue { slope, intercept });
            output.push(mapped);
            last = Some(mapped);
        }
        self.window.clear();
        for &input in &inputs[n - period..] {
            self.window.push(input);
        }
        self.seeded = true;
        last
    }
}
