//! Batch implementation for `rolling_argmin`.

use std::collections::VecDeque;

use super::math_operator::*;
use crate::error::{TaError, TaResult};

/// Index of the rolling minimum with TA-Lib MININDEX tie semantics.
///
/// Mirror image of `rolling_argmax`: `<=` newest-wins fast path, `<`
/// earliest-wins rescan, replicated exactly on a strict-pop monotonic deque.
///
/// Compute the rolling argmin result for the supplied aligned series.
///
/// # Parameters
///
/// * `input` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn rolling_argmin(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    validate_period(input, timeperiod)?;
    let len = input.len();
    let lookback = timeperiod - 1;
    let mut output = vec![0.0_f64; len]; // C fills lookback with 0, not NaN

    let mut deque: VecDeque<usize> = VecDeque::with_capacity(timeperiod);
    let mut tracked_idx = 0usize;
    let mut tracked_val = f64::NAN;
    for (i, &value) in input.iter().enumerate() {
        while deque.back().is_some_and(|&j| input[j] > value) {
            deque.pop_back();
        }
        deque.push_back(i);
        if i < lookback {
            continue;
        }
        let first_valid = i + 1 - timeperiod;
        while deque.front().is_some_and(|&j| j < first_valid) {
            deque.pop_front();
        }
        if i == lookback || tracked_idx < first_valid {
            let front = *deque.front().expect("window is populated");
            tracked_idx = front;
            tracked_val = input[front];
        } else if value <= tracked_val {
            tracked_idx = i;
            tracked_val = value;
        }
        output[i] = tracked_idx as f64;
    }
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream::tests_extrema_support::{datasets, periods_and_lengths};

    /// Original track-and-rescan implementation, kept verbatim as oracle.
    fn reference_rolling_argmin(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
        validate_period(input, timeperiod)?;
        let len = input.len();
        let lookback = timeperiod - 1;
        let mut output = vec![0.0_f64; len];

        let mut lowest = input[0];
        let mut lowest_idx: usize = 0;
        for j in 1..timeperiod {
            if input[j] < lowest {
                lowest = input[j];
                lowest_idx = j;
            }
        }
        output[lookback] = lowest_idx as f64;

        let mut trailing_idx = 1;
        let mut today = timeperiod;

        while today < len {
            let v = input[today];
            if lowest_idx < trailing_idx {
                lowest_idx = trailing_idx;
                lowest = input[trailing_idx];
                for (j, &val) in input[trailing_idx + 1..=today].iter().enumerate() {
                    if val < lowest {
                        lowest = val;
                        lowest_idx = trailing_idx + 1 + j;
                    }
                }
            } else if v <= lowest {
                lowest_idx = today;
                lowest = v;
            }
            output[today] = lowest_idx as f64;
            trailing_idx += 1;
            today += 1;
        }
        Ok(output)
    }

    #[test]
    fn matches_reference_bitwise() {
        for (period, len) in periods_and_lengths() {
            for data in datasets(len) {
                let expected = reference_rolling_argmin(&data, period);
                let actual = rolling_argmin(&data, period);
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

    #[test]
    fn path_dependent_tie_cases_match_c_semantics() {
        assert_eq!(rolling_argmin(&[9.0, 2.0, 4.0, 2.0], 3).unwrap()[3], 3.0);
        assert_eq!(rolling_argmin(&[1.0, 2.0, 4.0, 2.0], 3).unwrap()[3], 1.0);
        let constant = rolling_argmin(&[7.0; 6], 3).unwrap();
        assert_eq!(constant, vec![0.0, 0.0, 0.0, 1.0, 2.0, 3.0]);
    }
}
