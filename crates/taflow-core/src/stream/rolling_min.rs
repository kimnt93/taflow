//! Batch implementation for `rolling_min`.

use super::math_operator::*;
use super::vhgw;
use crate::error::{TaError, TaResult};

/// Compute the rolling min result for the supplied aligned series.
///
/// # Parameters
///
/// * `input` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn rolling_min(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    validate_period(input, timeperiod)?;
    let len = input.len();
    let lookback = timeperiod - 1;
    let mut output = vec![0.0_f64; len];
    output[..lookback].fill(f64::NAN);
    vhgw::sliding_min_into(input, timeperiod, &mut output[lookback..]);
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream::tests_extrema_support::{datasets, periods_and_lengths};

    /// Original track-and-rescan implementation, kept verbatim as oracle.
    fn reference_rolling_min(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
        validate_period(input, timeperiod)?;
        let len = input.len();
        let lookback = timeperiod - 1;
        let mut output = vec![0.0_f64; len];
        output[..lookback].fill(f64::NAN);

        let mut lowest = input[0];
        let mut lowest_idx: usize = 0;
        for j in 1..timeperiod {
            if input[j] <= lowest {
                lowest = input[j];
                lowest_idx = j;
            }
        }
        output[lookback] = lowest;

        let mut trailing_idx = 1;
        let mut today = timeperiod;

        while today < len {
            let v = input[today];
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
            output[today] = lowest;
            trailing_idx += 1;
            today += 1;
        }
        Ok(output)
    }

    #[test]
    fn matches_reference_bitwise() {
        for (period, len) in periods_and_lengths() {
            for data in datasets(len) {
                let expected = reference_rolling_min(&data, period);
                let actual = rolling_min(&data, period);
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
