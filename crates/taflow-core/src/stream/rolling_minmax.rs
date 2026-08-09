//! Batch implementation for `rolling_minmax`.

use super::math_operator::*;
use super::vhgw;
use crate::error::{TaError, TaResult};

/// MINMAX — two vHGW passes (O(n), period independent).
///
/// Compute the rolling minmax result for the supplied aligned series.
///
/// # Parameters
///
/// * `input` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn rolling_minmax(input: &[f64], timeperiod: usize) -> TaResult<(Vec<f64>, Vec<f64>)> {
    validate_period(input, timeperiod)?;
    let len = input.len();
    let lookback = timeperiod - 1;
    let mut out_min = vec![0.0_f64; len];
    out_min[..lookback].fill(f64::NAN);
    let mut out_max = vec![0.0_f64; len];
    out_max[..lookback].fill(f64::NAN);

    vhgw::sliding_max_into(input, timeperiod, &mut out_max[lookback..]);
    vhgw::sliding_min_into(input, timeperiod, &mut out_min[lookback..]);

    Ok((out_min, out_max))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream::tests_extrema_support::{datasets, periods_and_lengths};

    /// Original fused track-and-rescan implementation, kept verbatim as oracle.
    fn reference_rolling_minmax(
        input: &[f64],
        timeperiod: usize,
    ) -> TaResult<(Vec<f64>, Vec<f64>)> {
        validate_period(input, timeperiod)?;
        let len = input.len();
        let lookback = timeperiod - 1;
        let mut out_min = vec![0.0_f64; len];
        out_min[..lookback].fill(f64::NAN);
        let mut out_max = vec![0.0_f64; len];
        out_max[..lookback].fill(f64::NAN);

        let mut highest = input[0];
        let mut highest_idx: usize = 0;
        let mut lowest = input[0];
        let mut lowest_idx: usize = 0;
        for j in 1..timeperiod {
            if input[j] >= highest {
                highest = input[j];
                highest_idx = j;
            }
            if input[j] <= lowest {
                lowest = input[j];
                lowest_idx = j;
            }
        }
        out_max[lookback] = highest;
        out_min[lookback] = lowest;

        let mut trailing_idx = 1;
        let mut today = timeperiod;

        while today < len {
            let v = input[today];

            if highest_idx < trailing_idx {
                highest_idx = trailing_idx;
                highest = input[trailing_idx];
                for (j, &val) in input[trailing_idx + 1..=today].iter().enumerate() {
                    if val >= highest {
                        highest = val;
                        highest_idx = trailing_idx + 1 + j;
                    }
                }
            } else if v >= highest {
                highest_idx = today;
                highest = v;
            }

            if lowest_idx < trailing_idx {
                lowest_idx = trailing_idx;
                lowest = input[trailing_idx];
                for (j, &val) in input[trailing_idx + 1..=today].iter().enumerate() {
                    if val <= lowest {
                        lowest = val;
                        lowest_idx = trailing_idx + 1 + j;
                    }
                }
            } else if v <= lowest {
                lowest_idx = today;
                lowest = v;
            }

            out_max[today] = highest;
            out_min[today] = lowest;
            trailing_idx += 1;
            today += 1;
        }

        Ok((out_min, out_max))
    }

    #[test]
    fn matches_reference_bitwise() {
        for (period, len) in periods_and_lengths() {
            for data in datasets(len) {
                let expected = reference_rolling_minmax(&data, period);
                let actual = rolling_minmax(&data, period);
                match (expected, actual) {
                    (Ok(expected), Ok(actual)) => {
                        for (e, a) in expected.0.iter().zip(&actual.0) {
                            assert_eq!(e.to_bits(), a.to_bits(), "min p={period} len={len}");
                        }
                        for (e, a) in expected.1.iter().zip(&actual.1) {
                            assert_eq!(e.to_bits(), a.to_bits(), "max p={period} len={len}");
                        }
                    }
                    (Err(_), Err(_)) => {}
                    _ => panic!("error parity mismatch p={period} len={len}"),
                }
            }
        }
    }
}
use super::rolling_extrema::*;
use super::*;

#[derive(Debug, Clone, Copy, PartialEq)]
/// Persistent Rust state or aligned output type for `RollingMinmaxValue`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct RollingMinmaxValue {
    pub minimum: f64,
    pub maximum: f64,
}

#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `RollingMinmax`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct RollingMinmax {
    extrema: RollingExtrema,
    value: Option<RollingMinmaxValue>,
}

impl RollingMinmax {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(period: usize) -> TaResult<Self> {
        Ok(Self {
            extrema: RollingExtrema::new(period)?,
            value: None,
        })
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, input: f64) -> Option<RollingMinmaxValue> {
        self.value = self
            .extrema
            .append(input)
            .map(|(maximum, minimum)| RollingMinmaxValue { minimum, maximum });
        self.value
    }

    /// Bulk kernel: two vHGW comparison-only passes over the whole slice when
    /// the state is still empty, then the trailing `period` inputs are replayed
    /// to rebuild the monotonic deques. Outputs and post-run state are
    /// bit-identical to per-bar [`Self::append`]; warm-up bars are NaN.
    pub fn extend_slices_into(
        &mut self,
        inputs: &[f64],
        min_out: &mut Vec<f64>,
        max_out: &mut Vec<f64>,
    ) {
        let period = self.extrema.period();
        if self.extrema.count() != 0 || inputs.len() < period {
            min_out.reserve(inputs.len());
            max_out.reserve(inputs.len());
            for &input in inputs {
                match self.append(input) {
                    Some(value) => {
                        min_out.push(value.minimum);
                        max_out.push(value.maximum);
                    }
                    None => {
                        min_out.push(f64::NAN);
                        max_out.push(f64::NAN);
                    }
                }
            }
            return;
        }
        let min_start = min_out.len();
        let max_start = max_out.len();
        min_out.resize(min_start + inputs.len(), f64::NAN);
        max_out.resize(max_start + inputs.len(), f64::NAN);
        vhgw::sliding_max_into(inputs, period, &mut max_out[max_start + period - 1..]);
        vhgw::sliding_min_into(inputs, period, &mut min_out[min_start + period - 1..]);
        self.extrema.rebuild_from_full_run(inputs);
        self.value = Some(RollingMinmaxValue {
            minimum: *min_out.last().expect("at least one warmed bar"),
            maximum: *max_out.last().expect("at least one warmed bar"),
        });
    }

    /// Computes or updates `value` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn value(&self) -> Option<RollingMinmaxValue> {
        self.value
    }

    /// Computes or updates `reset` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn reset(&mut self) {
        self.extrema.reset();
        self.value = None;
    }
}
