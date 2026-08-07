//! Pointwise math, binary math, and price-transform streaming states.

use super::StreamingIndicator;

macro_rules! unary_indicator {
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
        }

        impl StreamingIndicator for $name {
            type Output = f64;

            fn append(&mut self, input: f64) -> Option<f64> {
                self.value = Some($operation(input));
                self.value
            }

            fn value(&self) -> Option<f64> {
                self.value
            }

            fn reset(&mut self) {
                self.value = None;
            }
        }
    };
}

unary_indicator!(MathAbs, f64::abs);
unary_indicator!(MathAcos, f64::acos);
unary_indicator!(MathAcosh, f64::acosh);
unary_indicator!(MathAsin, f64::asin);
unary_indicator!(MathAsinh, f64::asinh);
unary_indicator!(MathAtan, f64::atan);
unary_indicator!(MathAtanh, f64::atanh);
unary_indicator!(MathCbrt, f64::cbrt);
unary_indicator!(MathCeil, f64::ceil);
unary_indicator!(MathCos, f64::cos);
unary_indicator!(MathCosh, f64::cosh);
unary_indicator!(MathCot, |input: f64| input.tan().recip());
unary_indicator!(MathDegrees, f64::to_degrees);
unary_indicator!(MathExp, f64::exp);
unary_indicator!(MathFloor, f64::floor);
unary_indicator!(MathLn, f64::ln);
unary_indicator!(MathLog10, f64::log10);
unary_indicator!(MathLog1p, f64::ln_1p);
unary_indicator!(MathRadians, f64::to_radians);
unary_indicator!(MathSin, f64::sin);
unary_indicator!(MathSinh, f64::sinh);
unary_indicator!(MathSqrt, f64::sqrt);
unary_indicator!(MathTan, f64::tan);
unary_indicator!(MathTanh, f64::tanh);

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

binary_indicator!(MathAdd, |left: f64, right: f64| left + right);
binary_indicator!(MathSubtract, |left: f64, right: f64| left - right);
binary_indicator!(MathMultiply, |left: f64, right: f64| left * right);
binary_indicator!(MathDivide, |left: f64, right: f64| left / right);

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
