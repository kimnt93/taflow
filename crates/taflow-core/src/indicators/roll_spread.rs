use crate::error::TaResult;
use crate::stream::operator_states::*;

pub struct RollSpread {
    previous_price: Option<f64>,
    delta_previous: Option<f64>,
    moments: RollingPairMoments,
    value: Option<f64>,
}

impl RollSpread {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(timeperiod: usize) -> TaResult<Self> {
        Ok(Self {
            previous_price: None,
            delta_previous: None,
            moments: RollingPairMoments::new(timeperiod)?,
            value: None,
        })
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, price: f64) -> Option<f64> {
        let delta = if let Some(previous_price) = self.previous_price.replace(price) {
            price - previous_price
        } else {
            0.0
        };
        if let Some(delta_previous) = self.delta_previous {
            let _ = self.moments.append(delta, delta_previous);
        }
        self.delta_previous = Some(delta);
        self.value = self
            .moments
            .value()
            .map(|cov| 2.0 * (0.0f64 - cov).max(0.0).sqrt());
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
        self.previous_price = None;
        self.delta_previous = None;
        self.moments.reset();
        self.value = None;
    }
}
