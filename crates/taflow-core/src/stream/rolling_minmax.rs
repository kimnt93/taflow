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
