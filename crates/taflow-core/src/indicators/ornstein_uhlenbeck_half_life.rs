use crate::error::TaResult;
use crate::stream::operator_states::*;

pub struct OrnsteinUhlenbeckHalfLife {
    moments: RollingPairMoments,
    previous_price: Option<f64>,
    value: Option<f64>,
}

impl OrnsteinUhlenbeckHalfLife {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(timeperiod: usize) -> TaResult<Self> {
        Ok(Self {
            moments: RollingPairMoments::new(timeperiod)?,
            previous_price: None,
            value: None,
        })
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, price: f64) -> Option<f64> {
        if let Some(previous_price) = self.previous_price.replace(price) {
            let delta = price - previous_price;
            let _ = self.moments.append(delta, previous_price);
        }
        self.value = if let Some(cov) = self.moments.value() {
            // `var_y` is computed inside `RollingPairMoments::append` from the
            // same window with the same summation order as the scans this
            // replaced, so the result is bit-identical.
            let var_y = self.moments.var_y;
            if var_y > 0.0 {
                let lambda = -cov / var_y;
                (lambda > 0.0).then_some(2.0f64.ln() / lambda)
            } else {
                None
            }
        } else {
            None
        };
        self.value
    }

    /// Computes or updates `value` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Reset the persistent state and clear the latest value.
    pub fn reset(&mut self) {
        self.moments.reset();
        self.previous_price = None;
        self.value = None;
    }
}
