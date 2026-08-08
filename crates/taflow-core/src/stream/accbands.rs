//! Stateful Acceleration Bands.
//!
//! ACCBANDS applies TA-Lib's high/low acceleration transform and advances
//! three aligned simple moving averages for upper, middle, and lower bands.

use crate::error::TaResult;

use super::{invalid_period, SimpleMovingAverage, StreamingIndicator};

/// Compute the acceleration bands result for the supplied aligned series.
///
/// # Parameters
///
/// * `high` - Input series or configuration value.
/// * `low` - Input series or configuration value.
/// * `close` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn acceleration_bands(
    high: &[f64],
    low: &[f64],
    close: &[f64],
    timeperiod: usize,
) -> TaResult<(Vec<f64>, Vec<f64>, Vec<f64>)> {
    if high.len() != low.len() || high.len() != close.len() {
        return Err(crate::TaError::LengthMismatch {
            expected: high.len(),
            got: low.len().min(close.len()),
        });
    }
    let mut state = AccelerationBands::new(timeperiod)?;
    let mut upper = Vec::new();
    let mut middle = Vec::new();
    let mut lower = Vec::new();
    state.extend_slices_into(high, low, close, &mut upper, &mut middle, &mut lower)?;
    Ok((upper, middle, lower))
}

/// One aligned upper, middle, and lower Acceleration Bands observation.
#[derive(Debug, Clone, Copy, PartialEq)]
/// Persistent Rust state or aligned output type for `AccelerationBandsValue`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct AccelerationBandsValue {
    pub upper: f64,
    pub middle: f64,
    pub lower: f64,
}

/// Incremental Acceleration Bands with constant per-bar work.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `AccelerationBands`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct AccelerationBands {
    upper: SimpleMovingAverage,
    middle: SimpleMovingAverage,
    lower: SimpleMovingAverage,
    value: Option<AccelerationBandsValue>,
}

impl AccelerationBands {
    /// Creates an ACCBANDS state for a period of at least two bars.
    pub fn new(period: usize) -> TaResult<Self> {
        if period < 2 {
            return Err(invalid_period("timeperiod", period, 2));
        }
        Ok(Self {
            upper: SimpleMovingAverage::new(period)?,
            middle: SimpleMovingAverage::new(period)?,
            lower: SimpleMovingAverage::new(period)?,
            value: None,
        })
    }

    /// Appends one high, low, and close bar.
    pub fn append(&mut self, high: f64, low: f64, close: f64) -> Option<AccelerationBandsValue> {
        let denominator = high + low;
        let (upper_input, lower_input) = if denominator == 0.0 {
            (high, low)
        } else {
            let adjustment = 4.0 * (high - low) / denominator;
            (high * (1.0 + adjustment), low * (1.0 - adjustment))
        };
        let upper = self.upper.append(upper_input);
        let middle = self.middle.append(close);
        let lower = self.lower.append(lower_input);
        self.value = upper
            .zip(middle)
            .zip(lower)
            .map(|((upper, middle), lower)| AccelerationBandsValue {
                upper,
                middle,
                lower,
            });
        self.value
    }

    /// Bulk kernel: materializes the deterministic high/low acceleration
    /// transforms once, then advances each band through the SMA bulk path
    /// (O(1) add/evict sliding sums over the transformed slices).
    /// Bit-identical to per-bar [`Self::append`] in outputs and state.
    pub fn extend_slices_into(
        &mut self,
        high: &[f64],
        low: &[f64],
        close: &[f64],
        upper_out: &mut Vec<f64>,
        middle_out: &mut Vec<f64>,
        lower_out: &mut Vec<f64>,
    ) -> TaResult<()> {
        if high.len() != low.len() || high.len() != close.len() {
            return Err(crate::TaError::LengthMismatch {
                expected: high.len(),
                got: low.len().min(close.len()),
            });
        }
        let n = high.len();
        if n == 0 {
            return Ok(());
        }
        let mut upper_inputs = Vec::with_capacity(n);
        let mut lower_inputs = Vec::with_capacity(n);
        for i in 0..n {
            let high = high[i];
            let low = low[i];
            let denominator = high + low;
            let (upper_input, lower_input) = if denominator == 0.0 {
                (high, low)
            } else {
                let adjustment = 4.0 * (high - low) / denominator;
                (high * (1.0 + adjustment), low * (1.0 - adjustment))
            };
            upper_inputs.push(upper_input);
            lower_inputs.push(lower_input);
        }
        self.upper.extend_slice_into(&upper_inputs, upper_out);
        self.middle.extend_slice_into(close, middle_out);
        self.lower.extend_slice_into(&lower_inputs, lower_out);
        self.value = self
            .upper
            .value()
            .zip(self.middle.value())
            .zip(self.lower.value())
            .map(|((upper, middle), lower)| AccelerationBandsValue {
                upper,
                middle,
                lower,
            });
        Ok(())
    }

    /// Computes or updates `value` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn value(&self) -> Option<AccelerationBandsValue> {
        self.value
    }

    /// Computes or updates `reset` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn reset(&mut self) {
        self.upper.reset();
        self.middle.reset();
        self.lower.reset();
        self.value = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_batch_and_reset_replay() {
        let close: Vec<f64> = (0..200)
            .map(|index| 100.0 + (index as f64 * 0.31).sin() * 8.0 + index as f64 * 0.02)
            .collect();
        let high: Vec<f64> = close
            .iter()
            .enumerate()
            .map(|(index, close)| close + 1.0 + (index as f64 * 0.17).sin().abs())
            .collect();
        let low: Vec<f64> = close
            .iter()
            .enumerate()
            .map(|(index, close)| close - 1.0 - (index as f64 * 0.13).cos().abs())
            .collect();
        let (upper, middle, lower) = acceleration_bands(&high, &low, &close, 13).unwrap();
        let mut state = AccelerationBands::new(13).unwrap();
        for index in 0..close.len() {
            match state.append(high[index], low[index], close[index]) {
                Some(actual) => {
                    assert!((actual.upper - upper[index]).abs() < 1e-10);
                    assert!((actual.middle - middle[index]).abs() < 1e-10);
                    assert!((actual.lower - lower[index]).abs() < 1e-10);
                }
                None => {
                    assert!(upper[index].is_nan());
                    assert!(middle[index].is_nan());
                    assert!(lower[index].is_nan());
                }
            }
        }
        let expected_final = state.value();
        state.reset();
        for index in 0..close.len() {
            state.append(high[index], low[index], close[index]);
        }
        assert_eq!(state.value(), expected_final);
    }

    fn lcg_series(n: usize, mut state: u64) -> Vec<f64> {
        (0..n)
            .map(|_| {
                state = state
                    .wrapping_mul(6364136223846793005)
                    .wrapping_add(1442695040888963407);
                90.0 + (state >> 11) as f64 / (1u64 << 53) as f64 * 20.0
            })
            .collect()
    }

    fn assert_same_bits(actual: &[f64], expected: &[f64], label: &str) {
        assert_eq!(actual.len(), expected.len(), "{label}: length");
        for (i, (a, b)) in actual.iter().zip(expected).enumerate() {
            assert_eq!(a.to_bits(), b.to_bits(), "{label}: bar {i}");
        }
    }

    fn hlc(n: usize, seed: u64) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
        let close = lcg_series(n, seed);
        let spread_hi = lcg_series(n, seed ^ 0xDEAD_BEEF);
        let spread_lo = lcg_series(n, seed ^ 0x1234_5678);
        let high: Vec<f64> = close
            .iter()
            .zip(&spread_hi)
            .map(|(c, s)| c + (s - 89.0).abs() * 0.1)
            .collect();
        let low: Vec<f64> = close
            .iter()
            .zip(&spread_lo)
            .map(|(c, s)| c - (s - 89.0).abs() * 0.1)
            .collect();
        (high, low, close)
    }

    #[test]
    fn accbands_bulk_is_bitwise_identical_to_per_bar_append() {
        let (high, low, close) = hlc(5_000, 0x5EED_00AB);
        let (th, tl, tc) = hlc(256, 0x7A11_00AB);
        for period in [2usize, 5, 14, 30, 200] {
            let mut per_bar = AccelerationBands::new(period).unwrap();
            let mut ref_u = Vec::new();
            let mut ref_m = Vec::new();
            let mut ref_l = Vec::new();
            for i in 0..close.len() {
                match per_bar.append(high[i], low[i], close[i]) {
                    Some(v) => {
                        ref_u.push(v.upper);
                        ref_m.push(v.middle);
                        ref_l.push(v.lower);
                    }
                    None => {
                        ref_u.push(f64::NAN);
                        ref_m.push(f64::NAN);
                        ref_l.push(f64::NAN);
                    }
                }
            }
            let tail_ref: Vec<Option<AccelerationBandsValue>> = (0..tc.len())
                .map(|i| per_bar.append(th[i], tl[i], tc[i]))
                .collect();

            for chunk in [usize::MAX, 1, 7, 97] {
                let mut state = AccelerationBands::new(period).unwrap();
                let mut u = Vec::new();
                let mut m = Vec::new();
                let mut l = Vec::new();
                let mut start = 0;
                while start < close.len() {
                    let end = (start + chunk.min(close.len())).min(close.len());
                    state
                        .extend_slices_into(
                            &high[start..end],
                            &low[start..end],
                            &close[start..end],
                            &mut u,
                            &mut m,
                            &mut l,
                        )
                        .unwrap();
                    start = end;
                }
                let label = format!("ACCBANDS p{period} chunk {chunk}");
                assert_same_bits(&u, &ref_u, &format!("{label} upper"));
                assert_same_bits(&m, &ref_m, &format!("{label} middle"));
                assert_same_bits(&l, &ref_l, &format!("{label} lower"));
                for (i, expected) in tail_ref.iter().enumerate() {
                    let actual = state.append(th[i], tl[i], tc[i]);
                    match (actual, expected) {
                        (Some(a), Some(e)) => {
                            assert_eq!(a.upper.to_bits(), e.upper.to_bits(), "{label} tail {i}");
                            assert_eq!(a.middle.to_bits(), e.middle.to_bits(), "{label} tail {i}");
                            assert_eq!(a.lower.to_bits(), e.lower.to_bits(), "{label} tail {i}");
                        }
                        (None, None) => {}
                        _ => panic!("{label} tail {i}: warm-up mismatch"),
                    }
                }
            }
        }
    }
}
