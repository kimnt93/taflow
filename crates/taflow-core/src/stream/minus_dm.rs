//! Incremental Minus Directional Movement (-DM).
use crate::error::{TaError, TaResult};

/// Compute the minus directional movement result for the supplied aligned series.
///
/// # Parameters
///
/// * `high` - Input series or configuration value.
/// * `low` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn minus_directional_movement(high: &[f64], low: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    let len = high.len();
    if len != low.len() { return Err(TaError::LengthMismatch { expected: len, got: low.len() }); }
    if timeperiod < 1 || len < timeperiod { return Err(TaError::InsufficientData { need: timeperiod.max(1), got: len }); }
    let mut output = vec![0.0; len];
    if timeperiod > 1 { output[..timeperiod - 1].fill(f64::NAN); }
    let mut sum = 0.0;
    for index in 1..timeperiod {
        let up = high[index] - high[index - 1];
        let down = low[index - 1] - low[index];
        if down > up && down > 0.0 { sum += down; }
    }
    output[timeperiod - 1] = sum;
    let period = timeperiod as f64;
    for index in timeperiod..len {
        let up = high[index] - high[index - 1];
        let down = low[index - 1] - low[index];
        sum = sum - sum / period + if down > up && down > 0.0 { down } else { 0.0 };
        output[index] = sum;
    }
    Ok(output)
}
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_batch() {
        let high: Vec<f64> = (0..40).map(|i| 100.0 + i as f64 * 0.1).collect();
        let low: Vec<f64> = (0..40).map(|i| 98.0 - i as f64 * 0.3).collect();
        let expected = crate::stream::minus_directional_movement(&high, &low, 14).unwrap();
        let mut state = MinusDirectionalMovement::new(14).unwrap();

        for ((&high, &low), expected) in high.iter().zip(&low).zip(&expected) {
            match state.append(high, low) {
                Some(actual) => assert!((actual - expected).abs() < 1e-12),
                None => assert!(expected.is_nan()),
            }
        }
    }
}
