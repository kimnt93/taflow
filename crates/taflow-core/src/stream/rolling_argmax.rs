//! Batch implementation for `rolling_argmax`.

use std::collections::VecDeque;

use super::math_operator::*;
use crate::error::{TaError, TaResult};

/// Index of the rolling maximum with TA-Lib MAXINDEX tie semantics.
///
/// TA-Lib's tie behavior is path dependent: while the tracked maximizer is
/// still inside the window an incoming equal value steals the index (newest
/// wins, `>=` fast path); once the tracked index falls out of the window the
/// rescan picks the EARLIEST maximizer (`>` scan). This kernel replicates
/// that machine exactly, replacing the O(period) rescan with an amortized
/// O(1) strict-pop monotonic deque whose front is always the earliest
/// maximizer of the current window.
///
/// Compute the rolling argmax result for the supplied aligned series.
///
/// # Parameters
///
/// * `input` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn rolling_argmax(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    validate_period(input, timeperiod)?;
    let len = input.len();
    let lookback = timeperiod - 1;
    let mut output = vec![0.0_f64; len]; // C fills lookback with 0, not NaN

    let mut deque: VecDeque<usize> = VecDeque::with_capacity(timeperiod);
    let mut tracked_idx = 0usize;
    let mut tracked_val = f64::NAN;
    for (i, &value) in input.iter().enumerate() {
        // Strict pop keeps equal values, so the front stays the earliest
        // maximizer (matches the `>` rescan).
        while deque.back().is_some_and(|&j| input[j] < value) {
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
        } else if value >= tracked_val {
            // Fast path matches C: `>=`, newest wins the tie.
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
    fn reference_rolling_argmax(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
        validate_period(input, timeperiod)?;
        let len = input.len();
        let lookback = timeperiod - 1;
        let mut output = vec![0.0_f64; len];

        let mut highest = input[0];
        let mut highest_idx: usize = 0;
        for j in 1..timeperiod {
            if input[j] > highest {
                highest = input[j];
                highest_idx = j;
            }
        }
        output[lookback] = highest_idx as f64;

        let mut trailing_idx = 1;
        let mut today = timeperiod;

        while today < len {
            let v = input[today];
            if highest_idx < trailing_idx {
                highest_idx = trailing_idx;
                highest = input[trailing_idx];
                for (j, &val) in input[trailing_idx + 1..=today].iter().enumerate() {
                    if val > highest {
                        highest = val;
                        highest_idx = trailing_idx + 1 + j;
                    }
                }
            } else if v >= highest {
                highest_idx = today;
                highest = v;
            }
            output[today] = highest_idx as f64;
            trailing_idx += 1;
            today += 1;
        }
        Ok(output)
    }

    #[test]
    fn matches_reference_bitwise() {
        for (period, len) in periods_and_lengths() {
            for data in datasets(len) {
                let expected = reference_rolling_argmax(&data, period);
                let actual = rolling_argmax(&data, period);
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
        // Fast-path tie: the old max (5 @ 1) is still in the window when the
        // equal 5 @ 3 arrives, so the newest index wins.
        assert_eq!(rolling_argmax(&[3.0, 5.0, 4.0, 5.0], 3).unwrap()[3], 3.0);
        // Rescan tie: the tracked 9 @ 0 dies the same bar, and the rescan
        // picks the EARLIEST maximizer of [5, 4, 5].
        assert_eq!(rolling_argmax(&[9.0, 5.0, 4.0, 5.0], 3).unwrap()[3], 1.0);
        // Constant data walks the trailing edge.
        let constant = rolling_argmax(&[7.0; 6], 3).unwrap();
        assert_eq!(constant, vec![0.0, 0.0, 0.0, 1.0, 2.0, 3.0]);
    }
}
