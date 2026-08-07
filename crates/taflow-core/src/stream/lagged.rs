//! Lagged momentum and rate-of-change streaming states.

use std::collections::VecDeque;

use crate::error::TaResult;

use super::{invalid_period, StreamingIndicator};

#[derive(Debug, Clone)]
struct LaggedValue {
    period: usize,
    values: VecDeque<f64>,
}

impl LaggedValue {
    fn new(period: usize) -> TaResult<Self> {
        if period == 0 {
            return Err(invalid_period("timeperiod", period, 1));
        }
        Ok(Self {
            period,
            values: VecDeque::with_capacity(period),
        })
    }

    fn append(&mut self, input: f64) -> Option<(f64, f64)> {
        if self.values.len() < self.period {
            self.values.push_back(input);
            return None;
        }
        let previous = self.values.pop_front().expect("lag window is full");
        self.values.push_back(input);
        Some((input, previous))
    }

    fn reset(&mut self) {
        self.values.clear();
    }
}

macro_rules! lagged_indicator {
    ($name:ident, $calculate:expr) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            lag: LaggedValue,
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
                    lag: LaggedValue::new(period)?,
                    value: None,
                })
            }
        }

        impl StreamingIndicator for $name {
            type Output = f64;

            fn append(&mut self, input: f64) -> Option<f64> {
                self.value = self
                    .lag
                    .append(input)
                    .map(|(current, previous)| $calculate(current, previous));
                self.value
            }

            fn value(&self) -> Option<f64> {
                self.value
            }

            fn reset(&mut self) {
                self.lag.reset();
                self.value = None;
            }
        }
    };
}

lagged_indicator!(Mom, |current: f64, previous: f64| current - previous);
lagged_indicator!(Roc, |current: f64, previous: f64| if previous != 0.0 {
    (current - previous) / previous * 100.0
} else {
    0.0
});
lagged_indicator!(Rocp, |current: f64, previous: f64| if previous != 0.0 {
    (current - previous) / previous
} else {
    0.0
});
lagged_indicator!(Rocr, |current: f64, previous: f64| if previous != 0.0 {
    current / previous
} else {
    0.0
});
lagged_indicator!(Rocr100, |current: f64, previous: f64| if previous != 0.0 {
    current / previous * 100.0
} else {
    0.0
});
