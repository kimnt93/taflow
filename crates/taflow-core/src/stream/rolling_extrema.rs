//! Rolling extrema and rolling extremum-index streaming states.

use std::collections::VecDeque;

use crate::error::TaResult;

use super::{invalid_period, StreamingIndicator};

#[cfg(test)]
use crate::indicators::{RollingMinMax, RollingMinMaxIndex, RollingMinMaxIndexValue};

/// Single-sided monotonic deque tracking the rolling maximum.
///
/// Newest-wins on equal values (`<=` pop), matching the max side of the
/// historical `RollingExtrema` state bit for bit.
#[derive(Debug, Clone)]
pub(crate) struct MonotonicMax {
    period: usize,
    index: usize,
    deque: VecDeque<(usize, f64)>,
}

impl MonotonicMax {
    pub(crate) fn new(period: usize) -> TaResult<Self> {
        if period == 0 {
            return Err(invalid_period("timeperiod", period, 1));
        }
        Ok(Self {
            period,
            index: 0,
            deque: VecDeque::with_capacity(period),
        })
    }

    pub(crate) fn period(&self) -> usize {
        self.period
    }

    /// Number of observations consumed since construction/reset.
    pub(crate) fn count(&self) -> usize {
        self.index
    }

    pub(crate) fn append_indexed(&mut self, input: f64) -> Option<(usize, f64)> {
        let index = self.index;
        self.index += 1;
        while self.deque.back().is_some_and(|&(_, value)| value <= input) {
            self.deque.pop_back();
        }
        self.deque.push_back((index, input));
        let first_valid = index.saturating_add(1).saturating_sub(self.period);
        while self.deque.front().is_some_and(|&(i, _)| i < first_valid) {
            self.deque.pop_front();
        }
        (index + 1 >= self.period).then(|| *self.deque.front().expect("maximum queue is populated"))
    }

    pub(crate) fn append(&mut self, input: f64) -> Option<f64> {
        self.append_indexed(input).map(|(_, value)| value)
    }

    /// Rebuilds the exact post-run deque state from a full from-empty input.
    ///
    /// The deque contents after consuming `inputs` depend only on the last
    /// `period` observations, so replaying that tail reproduces the state a
    /// pure `append` run would have left, bit for bit.
    pub(crate) fn rebuild_from_full_run(&mut self, inputs: &[f64]) {
        debug_assert_eq!(self.index, 0);
        debug_assert!(self.deque.is_empty());
        let start = inputs.len().saturating_sub(self.period);
        for (offset, &value) in inputs[start..].iter().enumerate() {
            let index = start + offset;
            while self.deque.back().is_some_and(|&(_, v)| v <= value) {
                self.deque.pop_back();
            }
            self.deque.push_back((index, value));
        }
        self.index = inputs.len();
    }

    pub(crate) fn reset(&mut self) {
        self.index = 0;
        self.deque.clear();
    }
}

/// Single-sided monotonic deque tracking the rolling minimum.
///
/// Newest-wins on equal values (`>=` pop), matching the min side of the
/// historical `RollingExtrema` state bit for bit.
#[derive(Debug, Clone)]
pub(crate) struct MonotonicMin {
    period: usize,
    index: usize,
    deque: VecDeque<(usize, f64)>,
}

impl MonotonicMin {
    pub(crate) fn new(period: usize) -> TaResult<Self> {
        if period == 0 {
            return Err(invalid_period("timeperiod", period, 1));
        }
        Ok(Self {
            period,
            index: 0,
            deque: VecDeque::with_capacity(period),
        })
    }

    pub(crate) fn period(&self) -> usize {
        self.period
    }

    pub(crate) fn count(&self) -> usize {
        self.index
    }

    pub(crate) fn append_indexed(&mut self, input: f64) -> Option<(usize, f64)> {
        let index = self.index;
        self.index += 1;
        while self.deque.back().is_some_and(|&(_, value)| value >= input) {
            self.deque.pop_back();
        }
        self.deque.push_back((index, input));
        let first_valid = index.saturating_add(1).saturating_sub(self.period);
        while self.deque.front().is_some_and(|&(i, _)| i < first_valid) {
            self.deque.pop_front();
        }
        (index + 1 >= self.period).then(|| *self.deque.front().expect("minimum queue is populated"))
    }

    pub(crate) fn append(&mut self, input: f64) -> Option<f64> {
        self.append_indexed(input).map(|(_, value)| value)
    }

    /// See [`MonotonicMax::rebuild_from_full_run`].
    pub(crate) fn rebuild_from_full_run(&mut self, inputs: &[f64]) {
        debug_assert_eq!(self.index, 0);
        debug_assert!(self.deque.is_empty());
        let start = inputs.len().saturating_sub(self.period);
        for (offset, &value) in inputs[start..].iter().enumerate() {
            let index = start + offset;
            while self.deque.back().is_some_and(|&(_, v)| v >= value) {
                self.deque.pop_back();
            }
            self.deque.push_back((index, value));
        }
        self.index = inputs.len();
    }

    pub(crate) fn reset(&mut self) {
        self.index = 0;
        self.deque.clear();
    }
}

/// Two-sided rolling extrema used by the MINMAX/MIDPOINT-style consumers
/// that genuinely need both sides per bar.
#[derive(Debug, Clone)]
pub(crate) struct RollingExtrema {
    maximum: MonotonicMax,
    minimum: MonotonicMin,
}

impl RollingExtrema {
    pub(crate) fn new(period: usize) -> TaResult<Self> {
        Ok(Self {
            maximum: MonotonicMax::new(period)?,
            minimum: MonotonicMin::new(period)?,
        })
    }

    pub(crate) fn period(&self) -> usize {
        self.maximum.period()
    }

    pub(crate) fn count(&self) -> usize {
        self.maximum.count()
    }

    pub(crate) fn append_indexed(&mut self, input: f64) -> Option<((usize, f64), (usize, f64))> {
        let maximum = self.maximum.append_indexed(input);
        let minimum = self.minimum.append_indexed(input);
        maximum.zip(minimum)
    }

    pub(crate) fn append(&mut self, input: f64) -> Option<(f64, f64)> {
        self.append_indexed(input)
            .map(|(maximum, minimum)| (maximum.1, minimum.1))
    }

    /// See [`MonotonicMax::rebuild_from_full_run`].
    pub(crate) fn rebuild_from_full_run(&mut self, inputs: &[f64]) {
        self.maximum.rebuild_from_full_run(inputs);
        self.minimum.rebuild_from_full_run(inputs);
    }

    pub(crate) fn reset(&mut self) {
        self.maximum.reset();
        self.minimum.reset();
    }
}

/// TA-Lib-exact rolling argmax tracker.
///
/// TA-Lib's MAXINDEX tie behavior is path dependent: an incoming value equal
/// to the live tracked maximum steals the index (newest wins), but once the
/// tracked index leaves the window a rescan picks the EARLIEST maximizer.
/// This state replicates that machine exactly while replacing the O(period)
/// eviction rescan with an amortized O(1) strict-pop monotonic deque (equal
/// values are kept, so the deque front is always the earliest maximizer).
#[derive(Debug, Clone)]
pub(crate) struct MonotonicArgmax {
    period: usize,
    index: usize,
    deque: VecDeque<(usize, f64)>,
    tracked: Option<(usize, f64)>,
}

impl MonotonicArgmax {
    pub(crate) fn new(period: usize) -> TaResult<Self> {
        if period == 0 {
            return Err(invalid_period("timeperiod", period, 1));
        }
        Ok(Self {
            period,
            index: 0,
            deque: VecDeque::with_capacity(period),
            tracked: None,
        })
    }

    pub(crate) fn period(&self) -> usize {
        self.period
    }

    /// Observations consumed since construction/reset.
    pub(crate) fn count(&self) -> usize {
        self.index
    }

    /// Restores the exact post-run state a full from-empty `append` run would
    /// have left, given the tracked index the bulk kernel finished on.
    ///
    /// The strict-pop deque contents depend only on the last `period`
    /// observations (an entry survives iff no later in-window value is
    /// strictly greater), so replaying that tail reproduces them bit for bit.
    pub(crate) fn rebuild_from_full_run(&mut self, inputs: &[f64], tracked_index: usize) {
        debug_assert_eq!(self.index, 0);
        debug_assert!(self.deque.is_empty());
        let start = inputs.len().saturating_sub(self.period);
        for (offset, &value) in inputs[start..].iter().enumerate() {
            let index = start + offset;
            while self.deque.back().is_some_and(|&(_, v)| v < value) {
                self.deque.pop_back();
            }
            self.deque.push_back((index, value));
        }
        self.index = inputs.len();
        self.tracked = Some((tracked_index, inputs[tracked_index]));
    }

    /// Appends one value; `Some(index)` once the window is full.
    pub(crate) fn append(&mut self, input: f64) -> Option<usize> {
        let index = self.index;
        self.index += 1;
        // Strict pop: equal values stay, so the front is the earliest
        // maximizer (TA-Lib rescan uses `>`).
        while self.deque.back().is_some_and(|&(_, value)| value < input) {
            self.deque.pop_back();
        }
        self.deque.push_back((index, input));
        let first_valid = index.saturating_add(1).saturating_sub(self.period);
        while self.deque.front().is_some_and(|&(i, _)| i < first_valid) {
            self.deque.pop_front();
        }
        if index + 1 < self.period {
            return None;
        }
        match self.tracked {
            Some((tracked_index, _)) if tracked_index >= first_valid => {
                if let Some((_, tracked_value)) = self.tracked {
                    // Fast path matches TA-Lib: `>=`, newest wins the tie.
                    if input >= tracked_value {
                        self.tracked = Some((index, input));
                    }
                }
            }
            _ => {
                // Rescan path: earliest maximizer of the current window.
                self.tracked = self.deque.front().copied();
            }
        }
        self.tracked.map(|(i, _)| i)
    }

    pub(crate) fn reset(&mut self) {
        self.index = 0;
        self.deque.clear();
        self.tracked = None;
    }
}

/// TA-Lib-exact rolling argmin tracker; mirror image of [`MonotonicArgmax`].
#[derive(Debug, Clone)]
pub(crate) struct MonotonicArgmin {
    period: usize,
    index: usize,
    deque: VecDeque<(usize, f64)>,
    tracked: Option<(usize, f64)>,
}

impl MonotonicArgmin {
    pub(crate) fn new(period: usize) -> TaResult<Self> {
        if period == 0 {
            return Err(invalid_period("timeperiod", period, 1));
        }
        Ok(Self {
            period,
            index: 0,
            deque: VecDeque::with_capacity(period),
            tracked: None,
        })
    }

    pub(crate) fn period(&self) -> usize {
        self.period
    }

    /// Observations consumed since construction/reset.
    pub(crate) fn count(&self) -> usize {
        self.index
    }

    /// See [`MonotonicArgmax::rebuild_from_full_run`].
    pub(crate) fn rebuild_from_full_run(&mut self, inputs: &[f64], tracked_index: usize) {
        debug_assert_eq!(self.index, 0);
        debug_assert!(self.deque.is_empty());
        let start = inputs.len().saturating_sub(self.period);
        for (offset, &value) in inputs[start..].iter().enumerate() {
            let index = start + offset;
            while self.deque.back().is_some_and(|&(_, v)| v > value) {
                self.deque.pop_back();
            }
            self.deque.push_back((index, value));
        }
        self.index = inputs.len();
        self.tracked = Some((tracked_index, inputs[tracked_index]));
    }

    /// Appends one value; `Some(index)` once the window is full.
    pub(crate) fn append(&mut self, input: f64) -> Option<usize> {
        let index = self.index;
        self.index += 1;
        while self.deque.back().is_some_and(|&(_, value)| value > input) {
            self.deque.pop_back();
        }
        self.deque.push_back((index, input));
        let first_valid = index.saturating_add(1).saturating_sub(self.period);
        while self.deque.front().is_some_and(|&(i, _)| i < first_valid) {
            self.deque.pop_front();
        }
        if index + 1 < self.period {
            return None;
        }
        match self.tracked {
            Some((tracked_index, _)) if tracked_index >= first_valid => {
                if let Some((_, tracked_value)) = self.tracked {
                    if input <= tracked_value {
                        self.tracked = Some((index, input));
                    }
                }
            }
            _ => {
                self.tracked = self.deque.front().copied();
            }
        }
        self.tracked.map(|(i, _)| i)
    }

    pub(crate) fn reset(&mut self) {
        self.index = 0;
        self.deque.clear();
        self.tracked = None;
    }
}

/// TA-Lib-exact rolling extremum-index kernel over a whole slice.
///
/// Replicates `TA_MAXINDEX`/`TA_MININDEX` statement for statement: a tracked
/// `(index, value)` candidate advanced by the newest-wins (`>=`/`<=`) fast
/// path, plus an earliest-wins (`>`/`<`) rescan of the live window whenever
/// the candidate ages past the trailing edge. That path dependence is the
/// whole reason no window-determined algorithm (vHGW included) can be used
/// here — `[3,5,4,5]` and `[9,5,4,5]` at `period=3` share a final window but
/// emit different indices.
///
/// The rescan is amortized O(1) on non-degenerate data (the candidate ages out
/// roughly once per `period` bars, and each rescan is one contiguous forward
/// pass), and measures several times faster than carrying a monotonic deque
/// through every bar.
///
/// `out` must be the full-length output slice; warm-up entries are left
/// untouched, so callers pre-fill them with TA-Lib's `0.0`. Returns the final
/// tracked index, which seeds the streaming state's candidate.
#[inline]
pub(crate) fn tracked_index_rescan_into<const MAXIMUM: bool>(
    input: &[f64],
    period: usize,
    out: &mut [f64],
) -> usize {
    debug_assert!(period >= 1);
    debug_assert!(input.len() >= period);
    debug_assert_eq!(out.len(), input.len());
    // Strict form drives the rescan (earliest extremum wins); the non-strict
    // form drives the fast path (newest wins the tie).
    let replaces = |candidate: f64, best: f64| {
        if MAXIMUM {
            candidate > best
        } else {
            candidate < best
        }
    };
    let wins = |candidate: f64, best: f64| {
        if MAXIMUM {
            candidate >= best
        } else {
            candidate <= best
        }
    };

    let len = input.len();
    let lookback = period - 1;
    let mut best = input[0];
    let mut best_index = 0usize;
    for (offset, &value) in input[1..period].iter().enumerate() {
        if replaces(value, best) {
            best = value;
            best_index = offset + 1;
        }
    }
    out[lookback] = best_index as f64;

    let mut trailing = 1usize;
    for today in period..len {
        let value = input[today];
        if best_index < trailing {
            best = input[trailing];
            best_index = trailing;
            for (offset, &candidate) in input[trailing + 1..=today].iter().enumerate() {
                if replaces(candidate, best) {
                    best = candidate;
                    best_index = trailing + 1 + offset;
                }
            }
        } else if wins(value, best) {
            best = value;
            best_index = today;
        }
        out[today] = best_index as f64;
        trailing += 1;
    }
    best_index
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream::{RollingArgmax, RollingArgmin, RollingMax, RollingMin};

    /// Original two-deque implementation, kept verbatim as the reference
    /// oracle for the split monotonic states.
    #[derive(Debug, Clone)]
    struct ReferenceRollingExtrema {
        period: usize,
        index: usize,
        maximum: VecDeque<(usize, f64)>,
        minimum: VecDeque<(usize, f64)>,
    }

    impl ReferenceRollingExtrema {
        fn new(period: usize) -> Self {
            Self {
                period,
                index: 0,
                maximum: VecDeque::with_capacity(period),
                minimum: VecDeque::with_capacity(period),
            }
        }

        fn append_indexed(&mut self, input: f64) -> Option<((usize, f64), (usize, f64))> {
            let index = self.index;
            self.index += 1;
            while self
                .maximum
                .back()
                .is_some_and(|&(_, value)| value <= input)
            {
                self.maximum.pop_back();
            }
            while self
                .minimum
                .back()
                .is_some_and(|&(_, value)| value >= input)
            {
                self.minimum.pop_back();
            }
            self.maximum.push_back((index, input));
            self.minimum.push_back((index, input));
            let first_valid = index.saturating_add(1).saturating_sub(self.period);
            while self.maximum.front().is_some_and(|&(i, _)| i < first_valid) {
                self.maximum.pop_front();
            }
            while self.minimum.front().is_some_and(|&(i, _)| i < first_valid) {
                self.minimum.pop_front();
            }
            (index + 1 >= self.period).then(|| {
                (
                    *self.maximum.front().expect("maximum queue is populated"),
                    *self.minimum.front().expect("minimum queue is populated"),
                )
            })
        }
    }

    /// Original tracked-candidate index implementation with the O(period)
    /// eviction rescan, kept verbatim as the reference oracle.
    #[derive(Debug, Clone)]
    struct ReferenceRollingIndexExtrema {
        period: usize,
        index: usize,
        window: VecDeque<(usize, f64)>,
        maximum: Option<(usize, f64)>,
        minimum: Option<(usize, f64)>,
    }

    impl ReferenceRollingIndexExtrema {
        fn new(period: usize) -> Self {
            Self {
                period,
                index: 0,
                window: VecDeque::with_capacity(period),
                maximum: None,
                minimum: None,
            }
        }

        fn append(&mut self, input: f64) -> RollingMinMaxIndexValue {
            let index = self.index;
            self.index += 1;
            if self.window.len() == self.period {
                self.window.pop_front();
            }
            self.window.push_back((index, input));
            if self.window.len() < self.period {
                return RollingMinMaxIndexValue {
                    minimum: 0,
                    maximum: 0,
                };
            }

            let first_valid = index + 1 - self.period;
            if self.maximum.is_none() || self.maximum.is_some_and(|(i, _)| i < first_valid) {
                self.maximum = self.window.iter().copied().reduce(|best, current| {
                    if current.1 > best.1 {
                        current
                    } else {
                        best
                    }
                });
            } else if self.maximum.is_some_and(|(_, value)| input >= value) {
                self.maximum = Some((index, input));
            }
            if self.minimum.is_none() || self.minimum.is_some_and(|(i, _)| i < first_valid) {
                self.minimum = self.window.iter().copied().reduce(|best, current| {
                    if current.1 < best.1 {
                        current
                    } else {
                        best
                    }
                });
            } else if self.minimum.is_some_and(|(_, value)| input <= value) {
                self.minimum = Some((index, input));
            }
            RollingMinMaxIndexValue {
                minimum: self.minimum.expect("full window has a minimum").0,
                maximum: self.maximum.expect("full window has a maximum").0,
            }
        }
    }

    fn datasets(len: usize) -> Vec<Vec<f64>> {
        let mut state = 0x9E3779B97F4A7C15_u64;
        let random: Vec<f64> = (0..len)
            .map(|_| {
                state = state
                    .wrapping_mul(6364136223846793005)
                    .wrapping_add(1442695040888963407);
                ((state >> 33) % 997) as f64 / 7.0
            })
            .collect();
        let increasing: Vec<f64> = (0..len).map(|i| i as f64 * 0.5).collect();
        let decreasing: Vec<f64> = (0..len).map(|i| (len as f64) - i as f64 * 0.5).collect();
        let constant = vec![13.25_f64; len];
        let quantized: Vec<f64> = (0..len).map(|i| ((i * 7) % 5) as f64).collect();
        vec![random, increasing, decreasing, constant, quantized]
    }

    fn periods_and_lengths() -> Vec<(usize, usize)> {
        let mut cases = Vec::new();
        for &period in &[1usize, 2, 5, 30, 200] {
            for &len in &[0usize, 1, period - 1, period, period + 1, 10_000] {
                cases.push((period, len));
            }
        }
        cases
    }

    #[test]
    fn monotonic_split_matches_reference_two_deque_state() {
        for (period, len) in periods_and_lengths() {
            for data in datasets(len) {
                let mut reference = ReferenceRollingExtrema::new(period);
                let mut maximum = MonotonicMax::new(period).unwrap();
                let mut minimum = MonotonicMin::new(period).unwrap();
                for &value in &data {
                    let expected = reference.append_indexed(value);
                    let actual = maximum
                        .append_indexed(value)
                        .zip(minimum.append_indexed(value));
                    match (expected, actual) {
                        (Some((emax, emin)), Some((amax, amin))) => {
                            assert_eq!(emax.0, amax.0, "p={period} len={len}");
                            assert_eq!(emax.1.to_bits(), amax.1.to_bits());
                            assert_eq!(emin.0, amin.0);
                            assert_eq!(emin.1.to_bits(), amin.1.to_bits());
                        }
                        (None, None) => {}
                        _ => panic!("warm-up mismatch p={period} len={len}"),
                    }
                }
            }
        }
    }

    #[test]
    fn argmax_argmin_match_reference_index_extrema() {
        for (period, len) in periods_and_lengths() {
            for data in datasets(len) {
                let mut reference = ReferenceRollingIndexExtrema::new(period);
                let mut argmax = MonotonicArgmax::new(period).unwrap();
                let mut argmin = MonotonicArgmin::new(period).unwrap();
                for &value in &data {
                    let expected = reference.append(value);
                    let actual = RollingMinMaxIndexValue {
                        maximum: argmax.append(value).unwrap_or(0),
                        minimum: argmin.append(value).unwrap_or(0),
                    };
                    assert_eq!(expected, actual, "p={period} len={len}");
                }
            }
        }
    }

    #[test]
    fn rolling_minmax_index_matches_reference() {
        for (period, len) in periods_and_lengths() {
            for data in datasets(len) {
                let mut reference = ReferenceRollingIndexExtrema::new(period);
                let mut state = RollingMinMaxIndex::new(period).unwrap();
                for &value in &data {
                    assert_eq!(reference.append(value), state.append(value));
                }
                state.reset();
                let mut fresh = ReferenceRollingIndexExtrema::new(period);
                for &value in &data {
                    assert_eq!(fresh.append(value), state.append(value));
                }
            }
        }
    }

    /// 5,000-bar LCG series shared by the P1b bulk-kernel tests.
    pub(crate) fn lcg_series(len: usize, seed: u64) -> Vec<f64> {
        let mut state = seed;
        (0..len)
            .map(|_| {
                state = state
                    .wrapping_mul(6364136223846793005)
                    .wrapping_add(1442695040888963407);
                ((state >> 33) % 100_003) as f64 / 101.0
            })
            .collect()
    }

    pub(crate) const BULK_PERIODS: [usize; 5] = [2, 5, 14, 30, 200];
    pub(crate) const BULK_CHUNKS: [usize; 3] = [1, 7, 97];

    #[test]
    fn rolling_minmax_bulk_matches_append_bitwise() {
        let data = lcg_series(5_000, 0x1234_5678_9ABC_DEF0);
        for period in BULK_PERIODS {
            let mut reference = RollingMinMax::new(period).unwrap();
            let expected: Vec<(f64, f64)> = data
                .iter()
                .map(|&v| match reference.append(v) {
                    Some(value) => (value.minimum, value.maximum),
                    None => (f64::NAN, f64::NAN),
                })
                .collect();
            for chunk in [BULK_CHUNKS[0], BULK_CHUNKS[1], BULK_CHUNKS[2], data.len()] {
                let mut state = RollingMinMax::new(period).unwrap();
                let (mut min_out, mut max_out) = (Vec::new(), Vec::new());
                for piece in data.chunks(chunk) {
                    state.extend_slices_into(piece, &mut min_out, &mut max_out);
                }
                assert_eq!(min_out.len(), data.len());
                for (i, (e_min, e_max)) in expected.iter().enumerate() {
                    assert_eq!(
                        e_min.to_bits(),
                        min_out[i].to_bits(),
                        "min p={period} c={chunk} i={i}"
                    );
                    assert_eq!(
                        e_max.to_bits(),
                        max_out[i].to_bits(),
                        "max p={period} c={chunk} i={i}"
                    );
                }
                // 256-bar continuation from the post-bulk state.
                let mut follow = reference.clone();
                for &value in data.iter().take(256) {
                    assert_eq!(
                        follow.append(value),
                        state.append(value),
                        "continue p={period} c={chunk}"
                    );
                }
            }
        }
    }

    #[test]
    fn rolling_minmax_index_bulk_matches_append_bitwise() {
        let data = lcg_series(5_000, 0x0FED_CBA9_8765_4321);
        for period in BULK_PERIODS {
            let mut reference = RollingMinMaxIndex::new(period).unwrap();
            let expected: Vec<RollingMinMaxIndexValue> =
                data.iter().map(|&v| reference.append(v)).collect();
            for chunk in [BULK_CHUNKS[0], BULK_CHUNKS[1], BULK_CHUNKS[2], data.len()] {
                let mut state = RollingMinMaxIndex::new(period).unwrap();
                let (mut min_out, mut max_out) = (Vec::new(), Vec::new());
                for piece in data.chunks(chunk) {
                    state.extend_slices_into(piece, &mut min_out, &mut max_out);
                }
                for (i, expected) in expected.iter().enumerate() {
                    assert_eq!(
                        expected.minimum as f64, min_out[i],
                        "minidx p={period} c={chunk} i={i}"
                    );
                    assert_eq!(
                        expected.maximum as f64, max_out[i],
                        "maxidx p={period} c={chunk} i={i}"
                    );
                }
                let mut follow = reference.clone();
                for &value in data.iter().take(256) {
                    assert_eq!(
                        follow.append(value),
                        state.append(value),
                        "continue p={period} c={chunk}"
                    );
                }
            }
        }
    }

    /// Inputs that exercise every branch of the path-dependent index machine:
    /// random (rescans and fast-path ties interleaved), constant (every bar a
    /// tie), monotone up/down (one side never rescans, the other rescans every
    /// bar) and a coarse quantized series (dense ties).
    fn index_bulk_datasets(len: usize) -> Vec<Vec<f64>> {
        vec![
            lcg_series(len, 0x51ED_2701_C0FF_EE11),
            vec![13.25_f64; len],
            (0..len).map(|i| i as f64 * 0.5).collect(),
            (0..len).map(|i| (len as f64) - i as f64 * 0.5).collect(),
            (0..len).map(|i| ((i * 7) % 5) as f64).collect(),
        ]
    }

    const INDEX_BULK_CHUNKS: [usize; 5] = [1, 7, 10, 97, 1000];
    /// `BULK_PERIODS` plus the degenerate single-bar window.
    const INDEX_BULK_PERIODS: [usize; 6] = [1, 2, 5, 14, 30, 200];

    #[test]
    fn rolling_argmax_argmin_bulk_matches_append_bitwise() {
        for data in index_bulk_datasets(5_000) {
            for period in INDEX_BULK_PERIODS {
                let mut reference_max = RollingArgmax::new(period).unwrap();
                let mut reference_min = RollingArgmin::new(period).unwrap();
                let expected_max: Vec<f64> = data
                    .iter()
                    .map(|&v| reference_max.append(v).unwrap_or(f64::NAN))
                    .collect();
                let expected_min: Vec<f64> = data
                    .iter()
                    .map(|&v| reference_min.append(v).unwrap_or(f64::NAN))
                    .collect();
                for chunk in INDEX_BULK_CHUNKS.iter().copied().chain([data.len()]) {
                    let mut max_state = RollingArgmax::new(period).unwrap();
                    let mut min_state = RollingArgmin::new(period).unwrap();
                    let (mut max_out, mut min_out) = (Vec::new(), Vec::new());
                    for piece in data.chunks(chunk) {
                        max_state.extend_slice_into(piece, &mut max_out);
                        min_state.extend_slice_into(piece, &mut min_out);
                    }
                    assert_eq!(max_out.len(), data.len());
                    for i in 0..data.len() {
                        assert_eq!(
                            expected_max[i].to_bits(),
                            max_out[i].to_bits(),
                            "maxindex p={period} c={chunk} i={i}"
                        );
                        assert_eq!(
                            expected_min[i].to_bits(),
                            min_out[i].to_bits(),
                            "minindex p={period} c={chunk} i={i}"
                        );
                    }
                    // The state left behind must continue identically.
                    let mut follow_max = reference_max.clone();
                    let mut follow_min = reference_min.clone();
                    for &value in data.iter().take(256) {
                        assert_eq!(
                            follow_max.append(value),
                            max_state.append(value),
                            "continue maxindex p={period} c={chunk}"
                        );
                        assert_eq!(
                            follow_min.append(value),
                            min_state.append(value),
                            "continue minindex p={period} c={chunk}"
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn rolling_minmax_index_bulk_covers_tie_paths() {
        for data in index_bulk_datasets(5_000) {
            for period in INDEX_BULK_PERIODS {
                let mut reference = RollingMinMaxIndex::new(period).unwrap();
                let expected: Vec<RollingMinMaxIndexValue> =
                    data.iter().map(|&v| reference.append(v)).collect();
                for chunk in INDEX_BULK_CHUNKS.iter().copied().chain([data.len()]) {
                    let mut state = RollingMinMaxIndex::new(period).unwrap();
                    let (mut min_out, mut max_out) = (Vec::new(), Vec::new());
                    for piece in data.chunks(chunk) {
                        state.extend_slices_into(piece, &mut min_out, &mut max_out);
                    }
                    for (i, expected) in expected.iter().enumerate() {
                        assert_eq!(
                            expected.minimum as f64, min_out[i],
                            "minidx p={period} c={chunk} i={i}"
                        );
                        assert_eq!(
                            expected.maximum as f64, max_out[i],
                            "maxidx p={period} c={chunk} i={i}"
                        );
                    }
                    let mut follow = reference.clone();
                    for &value in data.iter().take(256) {
                        assert_eq!(
                            follow.append(value),
                            state.append(value),
                            "continue p={period} c={chunk}"
                        );
                    }
                }
            }
        }
    }

    /// The bulk kernel must reproduce TA-Lib's path dependence, not just the
    /// window-determined answer: the same final window emits a different index
    /// depending on whether the fast path or the rescan produced it.
    #[test]
    fn bulk_index_kernel_is_path_dependent_like_c() {
        let fast_path = [3.0, 5.0, 4.0, 5.0];
        let rescan = [9.0, 5.0, 4.0, 5.0];
        let mut out = Vec::new();
        RollingArgmax::new(3)
            .unwrap()
            .extend_slice_into(&fast_path, &mut out);
        assert_eq!(out, vec![0.0, 0.0, 1.0, 3.0]);
        out.clear();
        RollingArgmax::new(3)
            .unwrap()
            .extend_slice_into(&rescan, &mut out);
        assert_eq!(out, vec![0.0, 0.0, 0.0, 1.0]);
        out.clear();
        RollingArgmin::new(3)
            .unwrap()
            .extend_slice_into(&[9.0, 2.0, 4.0, 2.0], &mut out);
        assert_eq!(out[3], 3.0);
        out.clear();
        RollingArgmin::new(3)
            .unwrap()
            .extend_slice_into(&[1.0, 2.0, 4.0, 2.0], &mut out);
        assert_eq!(out[3], 1.0);
    }

    #[test]
    fn rolling_max_min_extend_slice_into_is_chunk_invariant() {
        for (period, len) in periods_and_lengths() {
            for data in datasets(len) {
                for chunk in [1usize, 7, data.len().max(1)] {
                    // Fresh pure-append reference run per chunking.
                    let mut append_max = RollingMax::new(period).unwrap();
                    let mut append_min = RollingMin::new(period).unwrap();
                    let expected_max: Vec<f64> = data
                        .iter()
                        .map(|&v| append_max.append(v).unwrap_or(f64::NAN))
                        .collect();
                    let expected_min: Vec<f64> = data
                        .iter()
                        .map(|&v| append_min.append(v).unwrap_or(f64::NAN))
                        .collect();

                    let mut max_state = RollingMax::new(period).unwrap();
                    let mut min_state = RollingMin::new(period).unwrap();
                    let mut max_out = Vec::new();
                    let mut min_out = Vec::new();
                    for piece in data.chunks(chunk) {
                        max_state.extend_slice_into(piece, &mut max_out);
                        min_state.extend_slice_into(piece, &mut min_out);
                    }
                    assert_eq!(expected_max.len(), max_out.len());
                    for (e, a) in expected_max.iter().zip(&max_out) {
                        assert_eq!(e.to_bits(), a.to_bits(), "max p={period} chunk={chunk}");
                    }
                    for (e, a) in expected_min.iter().zip(&min_out) {
                        assert_eq!(e.to_bits(), a.to_bits(), "min p={period} chunk={chunk}");
                    }
                    // The state left behind must continue identically.
                    for &value in data.iter().take(64) {
                        assert_eq!(
                            append_max.append(value),
                            max_state.append(value),
                            "post-bulk continuation p={period} chunk={chunk}"
                        );
                        assert_eq!(append_min.append(value), min_state.append(value));
                    }
                }
            }
        }
    }
}
