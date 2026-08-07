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
    sum_y: f64,
    weighted_sum: f64,
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
            sum_y: 0.0,
            weighted_sum: 0.0,
            seeded: false,
        })
    }

    fn append(&mut self, input: f64) -> Option<RegressionValue> {
        if !self.seeded {
            self.window.push(input);
            if !self.window.is_full() {
                return None;
            }
            let values: Vec<f64> = self.window.iter().copied().collect();
            self.sum_y = crate::simd::sum_f64(&values);
            self.weighted_sum = values
                .iter()
                .enumerate()
                .map(|(index, value)| index as f64 * value)
                .sum();
            self.seeded = true;
        } else {
            let old = self.window.push(input).expect("regression window is full");
            self.weighted_sum =
                self.weighted_sum - self.sum_y + old + (self.period_f - 1.0) * input;
            self.sum_y = self.sum_y - old + input;
        }
        let slope =
            (self.period_f * self.weighted_sum - self.sum_x * self.sum_y) / self.denominator;
        let intercept = (self.sum_y - slope * self.sum_x) / self.period_f;
        Some(RegressionValue { slope, intercept })
    }

    fn reset(&mut self) {
        self.window.clear();
        self.sum_y = 0.0;
        self.weighted_sum = 0.0;
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
