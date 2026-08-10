use crate::error::{TaError, TaResult};
use crate::stream::validate_period;

#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `McGinleyDynamic`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct McGinleyDynamic {
    length: usize,
    c: f64,
    seed_count: usize,
    seed_sum: f64,
    value: Option<f64>,
}

impl McGinleyDynamic {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(length: usize, c: f64) -> TaResult<Self> {
        validate_period(length)?;
        if !(0.0 < c && c <= 1.0) {
            return Err(TaError::InvalidParameter {
                name: "c",
                value: c.to_string(),
                reason: "must be in (0, 1]",
            });
        }
        Ok(Self {
            length,
            c,
            seed_count: 0,
            seed_sum: 0.0,
            value: None,
        })
    }
    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, close: f64) -> Option<f64> {
        if !close.is_finite() {
            return self.value;
        }
        if self.value.is_none() {
            self.seed_count += 1;
            self.seed_sum += close;
            if self.seed_count == self.length {
                self.value = Some(self.seed_sum / self.length as f64);
            }
            return self.value;
        }

        let previous = self.value.expect("McGinley state is seeded");
        if previous > 0.0 && close > 0.0 {
            let denominator = self.c * self.length as f64 * (close / previous).powi(4);
            self.value = Some(previous + (close - previous) / denominator);
        }
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
        self.value = None;
        self.seed_count = 0;
        self.seed_sum = 0.0;
    }
}
