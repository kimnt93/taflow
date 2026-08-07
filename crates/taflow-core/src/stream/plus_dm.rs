//! Incremental Plus Directional Movement (+DM).
use crate::error::{TaError, TaResult};
pub struct PlusDirectionalMovement {
    period: f64,
    seen: usize,
    previous: Option<(f64, f64)>,
    sum: f64,
    value: Option<f64>,
}
impl PlusDirectionalMovement {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(period: usize) -> TaResult<Self> {
        if period == 0 {
            return Err(TaError::InvalidParameter {
                name: "timeperiod",
                value: "0".into(),
                reason: "must be >= 1",
            });
        }
        Ok(Self {
            period: period as f64,
            seen: 0,
            previous: None,
            sum: 0.0,
            value: None,
        })
    }
    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, h: f64, l: f64) -> Option<f64> {
        let Some((ph, pl)) = self.previous.replace((h, l)) else {
            return if self.period == 1.0 {
                self.value = Some(0.0);
                self.value
            } else {
                None
            };
        };
        let up = h - ph;
        let down = pl - l;
        let dm = if up > down && up > 0.0 { up } else { 0.0 };
        self.seen += 1;
        let p = self.period as usize;
        if self.seen < p - 1 {
            self.sum += dm;
            return None;
        }
        if self.seen == p - 1 {
            self.sum += dm
        } else {
            self.sum = self.sum - self.sum / self.period + dm
        }
        self.value = Some(self.sum);
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
        self.seen = 0;
        self.previous = None;
        self.sum = 0.0;
        self.value = None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn matches_batch() {
        let high: Vec<f64> = (0..40).map(|i| 100.0 + i as f64 * 0.3).collect();
        let low: Vec<f64> = (0..40).map(|i| 98.0 + i as f64 * 0.1).collect();
        let expected = crate::momentum::plus_directional_movement(&high, &low, 14).unwrap();
        let mut state = PlusDirectionalMovement::new(14).unwrap();
        for ((&h, &l), expected) in high.iter().zip(&low).zip(&expected) {
            match state.append(h, l) {
                Some(actual) => assert!((actual - expected).abs() < 1e-12),
                None => assert!(expected.is_nan()),
            }
        }
    }
}
