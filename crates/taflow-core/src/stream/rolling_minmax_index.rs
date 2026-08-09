//! Batch implementation for `rolling_minmax_index`.

use std::collections::VecDeque;

use super::math_operator::*;
use crate::error::{TaError, TaResult};

/// MINMAXINDEX with TA-Lib tie semantics (see `rolling_argmax`): `>=`/`<=`
/// newest-wins fast paths, strict earliest-wins rescans, replicated exactly
/// on strict-pop monotonic deques with amortized O(1) eviction.
///
/// Compute the rolling minmax index result for the supplied aligned series.
///
/// # Parameters
///
/// * `input` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn rolling_minmax_index(input: &[f64], timeperiod: usize) -> TaResult<(Vec<f64>, Vec<f64>)> {
    validate_period(input, timeperiod)?;
    let len = input.len();
    let lookback = timeperiod - 1;
    let mut out_minidx = vec![0.0_f64; len]; // C fills lookback with 0, not NaN
    let mut out_maxidx = vec![0.0_f64; len];

    let mut max_deque: VecDeque<usize> = VecDeque::with_capacity(timeperiod);
    let mut min_deque: VecDeque<usize> = VecDeque::with_capacity(timeperiod);
    let mut highest_idx = 0usize;
    let mut highest = f64::NAN;
    let mut lowest_idx = 0usize;
    let mut lowest = f64::NAN;
    for (i, &value) in input.iter().enumerate() {
        while max_deque.back().is_some_and(|&j| input[j] < value) {
            max_deque.pop_back();
        }
        max_deque.push_back(i);
        while min_deque.back().is_some_and(|&j| input[j] > value) {
            min_deque.pop_back();
        }
        min_deque.push_back(i);
        if i < lookback {
            continue;
        }
        let first_valid = i + 1 - timeperiod;
        while max_deque.front().is_some_and(|&j| j < first_valid) {
            max_deque.pop_front();
        }
        while min_deque.front().is_some_and(|&j| j < first_valid) {
            min_deque.pop_front();
        }
        if i == lookback || highest_idx < first_valid {
            let front = *max_deque.front().expect("window is populated");
            highest_idx = front;
            highest = input[front];
        } else if value >= highest {
            highest_idx = i;
            highest = value;
        }
        if i == lookback || lowest_idx < first_valid {
            let front = *min_deque.front().expect("window is populated");
            lowest_idx = front;
            lowest = input[front];
        } else if value <= lowest {
            lowest_idx = i;
            lowest = value;
        }
        out_maxidx[i] = highest_idx as f64;
        out_minidx[i] = lowest_idx as f64;
    }

    Ok((out_minidx, out_maxidx))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream::tests_extrema_support::{datasets, periods_and_lengths};

    /// Original track-and-rescan implementation, kept verbatim as oracle.
    fn reference_rolling_minmax_index(
        input: &[f64],
        timeperiod: usize,
    ) -> TaResult<(Vec<f64>, Vec<f64>)> {
        validate_period(input, timeperiod)?;
        let len = input.len();
        let lookback = timeperiod - 1;
        let mut out_minidx = vec![0.0_f64; len];
        let mut out_maxidx = vec![0.0_f64; len];

        let mut highest = input[0];
        let mut highest_idx: usize = 0;
        let mut lowest = input[0];
        let mut lowest_idx: usize = 0;
        for j in 1..timeperiod {
            if input[j] > highest {
                highest = input[j];
                highest_idx = j;
            }
            if input[j] < lowest {
                lowest = input[j];
                lowest_idx = j;
            }
        }
        out_maxidx[lookback] = highest_idx as f64;
        out_minidx[lookback] = lowest_idx as f64;

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

            out_maxidx[today] = highest_idx as f64;
            out_minidx[today] = lowest_idx as f64;
            trailing_idx += 1;
            today += 1;
        }

        Ok((out_minidx, out_maxidx))
    }

    #[test]
    fn matches_reference_bitwise() {
        for (period, len) in periods_and_lengths() {
            for data in datasets(len) {
                let expected = reference_rolling_minmax_index(&data, period);
                let actual = rolling_minmax_index(&data, period);
                match (expected, actual) {
                    (Ok(expected), Ok(actual)) => {
                        for (e, a) in expected.0.iter().zip(&actual.0) {
                            assert_eq!(e.to_bits(), a.to_bits(), "minidx p={period} len={len}");
                        }
                        for (e, a) in expected.1.iter().zip(&actual.1) {
                            assert_eq!(e.to_bits(), a.to_bits(), "maxidx p={period} len={len}");
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
/// Persistent Rust state or aligned output type for `RollingMinmaxIndexValue`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct RollingMinmaxIndexValue {
    pub minimum: usize,
    pub maximum: usize,
}

#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `RollingMinmaxIndex`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct RollingMinmaxIndex {
    maximum: MonotonicArgmax,
    minimum: MonotonicArgmin,
    value: Option<RollingMinmaxIndexValue>,
}

impl RollingMinmaxIndex {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(period: usize) -> TaResult<Self> {
        Ok(Self {
            maximum: MonotonicArgmax::new(period)?,
            minimum: MonotonicArgmin::new(period)?,
            value: None,
        })
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, input: f64) -> RollingMinmaxIndexValue {
        let maximum = self.maximum.append(input).unwrap_or(0);
        let minimum = self.minimum.append(input).unwrap_or(0);
        let value = RollingMinmaxIndexValue { minimum, maximum };
        self.value = Some(value);
        value
    }

    /// Bulk kernel: drives the two TA-Lib-exact tracked-candidate machines in
    /// one tight loop, writing directly into the output caches.
    ///
    /// No vHGW shortcut here: TA-Lib's index tie rule is path dependent (a
    /// newest-wins fast path plus an earliest-wins rescan on eviction), so the
    /// only bit-exact route is the state machine itself. Warm-up emits `0.0`.
    ///
    /// From an empty state each side runs one [`tracked_index_rescan_into`]
    /// pass straight into its output cache — no per-bar deque traffic, no
    /// `Option`, no intermediate value struct.
    pub fn extend_slices_into(
        &mut self,
        inputs: &[f64],
        min_out: &mut Vec<f64>,
        max_out: &mut Vec<f64>,
    ) {
        let period = self.maximum.period();
        if self.maximum.count() != 0 || inputs.len() < period {
            min_out.reserve(inputs.len());
            max_out.reserve(inputs.len());
            for &input in inputs {
                let value = self.append(input);
                min_out.push(value.minimum as f64);
                max_out.push(value.maximum as f64);
            }
            return;
        }
        let min_start = min_out.len();
        let max_start = max_out.len();
        min_out.resize(min_start + inputs.len(), 0.0);
        max_out.resize(max_start + inputs.len(), 0.0);
        let maximum = tracked_index_rescan_into::<true>(inputs, period, &mut max_out[max_start..]);
        let minimum = tracked_index_rescan_into::<false>(inputs, period, &mut min_out[min_start..]);
        self.maximum.rebuild_from_full_run(inputs, maximum);
        self.minimum.rebuild_from_full_run(inputs, minimum);
        self.value = Some(RollingMinmaxIndexValue { minimum, maximum });
    }

    /// Returns the computed value, aligned history, or a validation error.
    pub fn value(&self) -> Option<RollingMinmaxIndexValue> {
        self.value
    }

    /// Computes or updates `reset` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn reset(&mut self) {
        self.maximum.reset();
        self.minimum.reset();
        self.value = None;
    }
}
