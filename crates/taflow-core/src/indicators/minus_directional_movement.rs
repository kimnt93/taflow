//! Incremental Minus Directional Movement (-DM).
use crate::error::{TaError, TaResult};

/// Persistent Rust state or aligned output type for `MinusDirectionalMovement`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct MinusDirectionalMovement {
    p: f64,
    n: usize,
    prev: Option<(f64, f64)>,
    sum: f64,
    v: Option<f64>,
}
impl MinusDirectionalMovement {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(p: usize) -> TaResult<Self> {
        if p == 0 {
            return Err(TaError::InvalidParameter {
                name: "timeperiod",
                value: "0".into(),
                reason: "must be >= 1",
            });
        }
        Ok(Self {
            p: p as f64,
            n: 0,
            prev: None,
            sum: 0.0,
            v: None,
        })
    }
    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, h: f64, l: f64) -> Option<f64> {
        let Some((ph, pl)) = self.prev.replace((h, l)) else {
            return if self.p == 1.0 {
                self.v = Some(0.0);
                self.v
            } else {
                None
            };
        };
        let up = h - ph;
        let down = pl - l;
        let dm = if down > up && down > 0.0 { down } else { 0.0 };
        self.n += 1;
        let p = self.p as usize;
        if self.n < p - 1 {
            self.sum += dm;
            return None;
        }
        if self.n == p - 1 {
            self.sum += dm
        } else {
            self.sum = self.sum - self.sum / self.p + dm
        }
        self.v = Some(self.sum);
        self.v
    }
    /// Computes or updates `value` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn value(&self) -> Option<f64> {
        self.v
    }
    /// Reset the persistent state and clear the latest value.
    pub fn reset(&mut self) {
        self.n = 0;
        self.prev = None;
        self.sum = 0.0;
        self.v = None
    }
}
