//! Incremental linear-regression indicator states.

use crate::error::TaResult;

use super::{invalid_period, StreamingIndicator, Window};

#[derive(Debug, Clone, Copy)]
struct RegressionValue {
    slope: f64,
    intercept: f64,
}

#[derive(Debug, Clone)]
struct RegressionCore {
    period: usize,
    period_f: f64,
    sum_x: f64,
    denominator: f64,
    window: Window,
    seeded: bool,
}

impl RegressionCore {
    fn new(period: usize) -> TaResult<Self> {
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

    fn append(&mut self, input: f64) -> Option<RegressionValue> {
        if !self.seeded {
            self.window.push(input);
            if !self.window.is_full() {
                return None;
            }
            self.seeded = true;
        } else {
            self.window.push(input).expect("regression window is full");
        }
        let mut sum_y = 0.0;
        let mut weighted_sum = 0.0;
        for (index, &value) in self.window.iter().enumerate() {
            sum_y += value;
            weighted_sum += index as f64 * value;
        }
        let slope = (self.period_f * weighted_sum - self.sum_x * sum_y) / self.denominator;
        let intercept = (sum_y - slope * self.sum_x) / self.period_f;
        Some(RegressionValue { slope, intercept })
    }

    fn reset(&mut self) {
        self.window.clear();
        self.seeded = false;
    }
}

macro_rules! regression_indicator {
    ($name:ident, $calculate:expr) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            core: RegressionCore,
            value: Option<f64>,
        }

        impl $name {
            /// Computes or updates `new` through the native Rust kernel.
            ///
            /// Parameters are the typed series and configuration values in the signature.
            ///
            /// Returns the computed value, aligned history, or a validation error.
            pub fn new(period: usize) -> TaResult<Self> {
                Ok(Self {
                    core: RegressionCore::new(period)?,
                    value: None,
                })
            }
        }

        impl StreamingIndicator for $name {
            type Output = f64;

            fn append(&mut self, input: f64) -> Option<f64> {
                let period = self.core.period;
                self.value = self
                    .core
                    .append(input)
                    .map(|regression| $calculate(regression, period));
                self.value
            }

            fn value(&self) -> Option<f64> {
                self.value
            }

            fn reset(&mut self) {
                self.core.reset();
                self.value = None;
            }
        }
    };
}

regression_indicator!(Linearreg, |value: RegressionValue, period: usize| value
    .intercept
    + value.slope * (period - 1) as f64);
regression_indicator!(LinearregSlope, |value: RegressionValue, _| value.slope);
regression_indicator!(LinearregIntercept, |value: RegressionValue, _| value
    .intercept);
regression_indicator!(LinearregAngle, |value: RegressionValue, _| value
    .slope
    .atan()
    .to_degrees());
regression_indicator!(Tsf, |value: RegressionValue, period: usize| value.intercept
    + value.slope * period as f64);
