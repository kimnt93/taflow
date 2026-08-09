//! Batch implementation for `rolling_midpoint`.

use super::vhgw;
use crate::error::{TaError, TaResult};

/// MIDPOINT — vHGW sliding extrema (O(n), period independent)
///
/// MIDPOINT = (highest + lowest) / 2
/// Compute the midpoint result for the supplied aligned series.
///
/// # Parameters
///
/// * `input` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn rolling_midpoint(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    if timeperiod == 0 {
        return Err(TaError::InvalidParameter {
            name: "timeperiod",
            value: "0".to_string(),
            reason: "must be >= 1",
        });
    }
    let len = input.len();
    if len < timeperiod {
        return Err(TaError::InsufficientData {
            need: timeperiod,
            got: len,
        });
    }

    let lookback = timeperiod - 1;
    let mut output = vec![0.0_f64; len];
    output[..lookback].fill(f64::NAN);

    let window_count = len - lookback;
    let mut lowest = vec![0.0_f64; window_count];
    vhgw::sliding_max_into(input, timeperiod, &mut output[lookback..]);
    vhgw::sliding_min_into(input, timeperiod, &mut lowest);
    for (slot, &low) in output[lookback..].iter_mut().zip(&lowest) {
        *slot = (*slot + low) / 2.0;
    }
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream::tests_extrema_support::{datasets, periods_and_lengths};

    /// Original track-and-rescan implementation, kept verbatim as oracle.
    fn reference_rolling_midpoint(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
        if timeperiod == 0 {
            return Err(TaError::InvalidParameter {
                name: "timeperiod",
                value: "0".to_string(),
                reason: "must be >= 1",
            });
        }
        let len = input.len();
        if len < timeperiod {
            return Err(TaError::InsufficientData {
                need: timeperiod,
                got: len,
            });
        }

        let lookback = timeperiod - 1;
        let mut output = vec![0.0_f64; len];
        output[..lookback].fill(f64::NAN);

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
        output[lookback] = (highest + lowest) / 2.0;

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

            output[today] = (highest + lowest) / 2.0;
            trailing_idx += 1;
            today += 1;
        }
        Ok(output)
    }

    #[test]
    fn matches_reference_bitwise() {
        for (period, len) in periods_and_lengths() {
            for data in datasets(len) {
                let expected = reference_rolling_midpoint(&data, period);
                let actual = rolling_midpoint(&data, period);
                match (expected, actual) {
                    (Ok(expected), Ok(actual)) => {
                        assert_eq!(expected.len(), actual.len());
                        for (e, a) in expected.iter().zip(&actual) {
                            assert_eq!(e.to_bits(), a.to_bits(), "p={period} len={len}");
                        }
                    }
                    (Err(_), Err(_)) => {}
                    _ => panic!("error parity mismatch p={period} len={len}"),
                }
            }
        }
    }
}
use super::rolling_price::*;
use super::*;

/// Stateful midpoint of the rolling highest and lowest input values.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `RollingMidpoint`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct RollingMidpoint {
    extrema: RollingExtrema,
    value: Option<f64>,
}

impl RollingMidpoint {
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
}

impl StreamingIndicator for RollingMidpoint {
    type Output = f64;

    fn append(&mut self, input: f64) -> Option<f64> {
        self.value = self
            .extrema
            .append(input)
            .map(|(maximum, minimum)| (maximum + minimum) * 0.5);
        self.value
    }

    fn value(&self) -> Option<f64> {
        self.value
    }

    fn reset(&mut self) {
        self.extrema.reset();
        self.value = None;
    }

    fn extend_slice_into(&mut self, inputs: &[f64], output: &mut Vec<f64>) {
        let period = self.extrema.period();
        if self.extrema.count() != 0 || inputs.len() < period {
            output.reserve(inputs.len());
            output.extend(
                inputs
                    .iter()
                    .copied()
                    .map(|input| self.append(input).unwrap_or(f64::NAN)),
            );
            return;
        }
        let start = output.len();
        output.resize(start + inputs.len(), f64::NAN);
        let warm = start + period - 1;
        let mut lowest = vec![0.0_f64; inputs.len() - (period - 1)];
        vhgw::sliding_max_into(inputs, period, &mut output[warm..]);
        vhgw::sliding_min_into(inputs, period, &mut lowest);
        for (slot, &minimum) in output[warm..].iter_mut().zip(&lowest) {
            *slot = (*slot + minimum) * 0.5;
        }
        self.extrema.rebuild_from_full_run(inputs);
        self.value = output.last().copied();
    }
}
