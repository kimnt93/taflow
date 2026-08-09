//! Price-transform streaming states.

macro_rules! binary_indicator {
    ($name:ident, $operation:expr) => {
        #[derive(Debug, Clone, Default)]
        pub struct $name {
            value: Option<f64>,
        }

        impl $name {
            /// Computes or updates `new` through the native Rust kernel.
            ///
            /// Parameters are the typed series and configuration values in the signature.
            ///
            /// Returns the computed value, aligned history, or a validation error.
            pub fn new() -> Self {
                Self::default()
            }

            /// Computes or updates `append` through the native Rust kernel.
            ///
            /// Parameters are the typed series and configuration values in the signature.
            ///
            /// Returns the computed value, aligned history, or a validation error.
            pub fn append(&mut self, left: f64, right: f64) -> f64 {
                let value = $operation(left, right);
                self.value = Some(value);
                value
            }

            /// Computes or updates `value` through the native Rust kernel.
            ///
            /// Parameters are the typed series and configuration values in the signature.
            ///
            /// Returns the computed value, aligned history, or a validation error.
            pub fn value(&self) -> Option<f64> {
                self.value
            }

            /// Computes or updates `reset` through the native Rust kernel.
            ///
            /// Parameters are the typed series and configuration values in the signature.
            ///
            /// Returns the computed value, aligned history, or a validation error.
            pub fn reset(&mut self) {
                self.value = None;
            }
        }
    };
}

/// Stateful average price `(open + high + low + close) / 4`.
#[derive(Debug, Clone, Default)]
/// Persistent Rust state or aligned output type for `AveragePrice`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct AveragePrice {
    value: Option<f64>,
}

impl AveragePrice {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new() -> Self {
        Self::default()
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, open: f64, high: f64, low: f64, close: f64) -> f64 {
        let value = (open + high + low + close) * 0.25;
        self.value = Some(value);
        value
    }

    /// Computes or updates `value` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Computes or updates `reset` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn reset(&mut self) {
        self.value = None;
    }
}

binary_indicator!(MedianPrice, |high: f64, low: f64| (high + low) * 0.5);

macro_rules! price3_indicator {
    ($name:ident, $operation:expr) => {
        #[derive(Debug, Clone, Default)]
        pub struct $name {
            value: Option<f64>,
        }

        impl $name {
            /// Computes or updates `new` through the native Rust kernel.
            ///
            /// Parameters are the typed series and configuration values in the signature.
            ///
            /// Returns the computed value, aligned history, or a validation error.
            pub fn new() -> Self {
                Self::default()
            }

            /// Computes or updates `append` through the native Rust kernel.
            ///
            /// Parameters are the typed series and configuration values in the signature.
            ///
            /// Returns the computed value, aligned history, or a validation error.
            pub fn append(&mut self, high: f64, low: f64, close: f64) -> f64 {
                let value = $operation(high, low, close);
                self.value = Some(value);
                value
            }

            /// Computes or updates `value` through the native Rust kernel.
            ///
            /// Parameters are the typed series and configuration values in the signature.
            ///
            /// Returns the computed value, aligned history, or a validation error.
            pub fn value(&self) -> Option<f64> {
                self.value
            }

            /// Computes or updates `reset` through the native Rust kernel.
            ///
            /// Parameters are the typed series and configuration values in the signature.
            ///
            /// Returns the computed value, aligned history, or a validation error.
            pub fn reset(&mut self) {
                self.value = None;
            }
        }
    };
}

price3_indicator!(TypicalPrice, |high: f64, low: f64, close: f64| (high
    + low
    + close)
    * (1.0 / 3.0));
price3_indicator!(WeightedClose, |high: f64, low: f64, close: f64| (high
    + low
    + close
    + close)
    * 0.25);
