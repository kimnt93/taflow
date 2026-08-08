//! Stateful moving average with variable period.
//!
//! A newly requested period is initialized by replaying retained input once;
//! initialized periods then advance lazily — a period's state is only caught
//! up when that period is selected again, replaying the missed bars from the
//! retained history in order, which is bit-identical to advancing every state
//! on every bar but touches one state instead of all of them.

use crate::error::{TaError, TaResult};
use crate::ma_type::MaType;

use super::moving_average::MovingAverageDispatcher;

/// Computes an aligned variable-period moving-average vector.
///
/// # Parameters
///
/// * `input`, `periods` - Equal-length chronological series.
/// * `minperiod`, `maxperiod`, `matype` - Per-bar period bounds and moving-average type.
///
/// # Returns
///
/// An aligned vector with NaN warm-up values and validated parameters.
pub fn variable_period_moving_average(
    input: &[f64],
    periods: &[f64],
    minperiod: usize,
    maxperiod: usize,
    matype: MaType,
) -> TaResult<Vec<f64>> {
    if input.len() != periods.len() {
        return Err(crate::TaError::LengthMismatch {
            expected: input.len(),
            got: periods.len(),
        });
    }
    let mut state = VariablePeriodMovingAverage::new(minperiod, maxperiod, matype)?;
    let mut output = Vec::with_capacity(input.len());
    state.extend_slices_into(input, periods, &mut output);
    Ok(output)
}

/// One materialized per-period moving-average state.
struct PeriodState {
    state: MovingAverageDispatcher,
    /// Global bar index this state must consume next.
    next_index: usize,
}

/// Incremental MAVP with TA-Lib-compatible truncation and clamping.
/// Persistent Rust state or aligned output type for `VariablePeriodMovingAverage`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct VariablePeriodMovingAverage {
    minperiod: usize,
    maxperiod: usize,
    matype: MaType,
    lookback: usize,
    /// Number of bars consumed so far.
    count: usize,
    /// Retained tail of the input; `history[0]` is global bar `history_start`.
    history: Vec<f64>,
    history_start: usize,
    /// Retained length that triggers the next amortized trim.
    trim_at: usize,
    /// Materialized states indexed by `period - minperiod`, grown on demand.
    states: Vec<Option<Box<PeriodState>>>,
    /// Largest period in `[minperiod, maxperiod]` that has never been selected.
    /// While one exists, history must be retained back to its seed index.
    largest_unmaterialized: Option<usize>,
    value: Option<f64>,
}

impl VariablePeriodMovingAverage {
    /// Creates a variable-period moving-average state.
    pub fn new(minperiod: usize, maxperiod: usize, matype: MaType) -> TaResult<Self> {
        if minperiod == 0 || maxperiod < minperiod {
            return Err(TaError::InvalidParameter {
                name: "minperiod/maxperiod",
                value: format!("{minperiod}/{maxperiod}"),
                reason: "minperiod >= 1 and maxperiod >= minperiod required",
            });
        }
        Ok(Self {
            minperiod,
            maxperiod,
            matype,
            lookback: matype.lookback(maxperiod),
            count: 0,
            history: Vec::new(),
            history_start: 0,
            trim_at: 64,
            states: Vec::new(),
            largest_unmaterialized: Some(maxperiod),
            value: None,
        })
    }

    /// The first global bar index a fresh state for `period` replays from.
    #[inline]
    fn seed_index(&self, period: usize) -> usize {
        self.lookback - self.matype.lookback(period)
    }

    /// Recomputes the largest never-selected period after `period` was created.
    fn on_materialized(&mut self, period: usize) {
        if self.largest_unmaterialized != Some(period) {
            return;
        }
        // `lookback` is non-decreasing in period, so the largest unmaterialized
        // period is the one pinning history the furthest back.
        let mut candidate = period;
        loop {
            if candidate == self.minperiod {
                self.largest_unmaterialized = None;
                return;
            }
            candidate -= 1;
            let materialized = self
                .states
                .get(candidate - self.minperiod)
                .is_some_and(Option::is_some);
            if !materialized {
                self.largest_unmaterialized = Some(candidate);
                return;
            }
        }
    }

    /// Drops the history prefix no state can ever need again.
    fn trim_history(&mut self) {
        let mut retain_from = self.count;
        if let Some(period) = self.largest_unmaterialized {
            retain_from = retain_from.min(self.seed_index(period));
        }
        for entry in self.states.iter().flatten() {
            retain_from = retain_from.min(entry.next_index);
        }
        let drop = retain_from.saturating_sub(self.history_start);
        if drop > 0 {
            self.history.drain(..drop);
            self.history_start = retain_from;
        }
        self.trim_at = self.history.len().saturating_mul(2).max(64);
    }

    /// Appends one value and its requested moving-average period.
    pub fn append(&mut self, input: f64, period: f64) -> Option<f64> {
        let today = self.count;
        self.history.push(input);
        self.count += 1;
        let selected = (period as usize).clamp(self.minperiod, self.maxperiod);
        let slot = selected - self.minperiod;
        if slot >= self.states.len() {
            self.states.resize_with(slot + 1, || None);
        }

        let history_start = self.history_start;
        let mut selected_value = None;
        let mut created = false;
        let Self {
            history, states, ..
        } = self;
        if let Some(entry) = states[slot].as_mut() {
            // Catch-up: replay every bar missed since this period was last
            // selected, in order — bit-identical to advancing every bar.
            for &sample in &history[entry.next_index - history_start..=today - history_start] {
                selected_value = entry.state.append(sample);
            }
            entry.next_index = today + 1;
        } else {
            let seed_index = self.seed_index(selected);
            if self.count > seed_index {
                let mut state = MovingAverageDispatcher::new(selected, self.matype)
                    .expect("MAVP constructor validates the complete period range");
                for index in seed_index..=today {
                    selected_value = state.append(self.history[index - history_start]);
                }
                self.states[slot] = Some(Box::new(PeriodState {
                    state,
                    next_index: today + 1,
                }));
                created = true;
            }
        }
        if created {
            self.on_materialized(selected);
        }
        if self.history.len() >= self.trim_at {
            self.trim_history();
        }

        self.value = if self.count > self.lookback {
            selected_value
        } else {
            None
        };
        self.value
    }

    /// Bulk kernel over aligned value and period slices.
    pub fn extend_slices_into(&mut self, input: &[f64], periods: &[f64], output: &mut Vec<f64>) {
        let len = input.len().min(periods.len());
        output.reserve(len);
        for index in 0..len {
            output.push(
                self.append(input[index], periods[index])
                    .unwrap_or(f64::NAN),
            );
        }
    }

    /// Returns the latest warmed output.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Restores the post-construction state.
    pub fn reset(&mut self) {
        self.count = 0;
        self.history.clear();
        self.history_start = 0;
        self.trim_at = 64;
        for entry in &mut self.states {
            *entry = None;
        }
        self.largest_unmaterialized = Some(self.maxperiod);
        self.value = None;
    }

    /// Number of retained history samples (test/diagnostic hook).
    #[cfg(test)]
    fn retained_history(&self) -> usize {
        self.history.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lcg_series(len: usize, mut seed: u64) -> Vec<f64> {
        (0..len)
            .map(|_| {
                seed = seed
                    .wrapping_mul(6364136223846793005)
                    .wrapping_add(1442695040888963407);
                100.0 + ((seed >> 11) as f64 / (1u64 << 53) as f64 - 0.5) * 20.0
            })
            .collect()
    }

    fn lcg_periods(len: usize, mut seed: u64, low: usize, high: usize) -> Vec<f64> {
        let span = (high - low + 1) as u64;
        (0..len)
            .map(|_| {
                seed = seed
                    .wrapping_mul(6364136223846793005)
                    .wrapping_add(1442695040888963407);
                (low as u64 + (seed >> 33) % span) as f64
            })
            .collect()
    }

    /// Verbatim copy of the pre-optimization implementation: unbounded history,
    /// every materialized state advanced on every bar, replay from full history.
    mod oracle {
        use crate::ma_type::MaType;
        use crate::stream::moving_average::MovingAverageDispatcher;
        use std::collections::HashMap;

        pub struct Mavp {
            minperiod: usize,
            maxperiod: usize,
            matype: MaType,
            lookback: usize,
            history: Vec<f64>,
            states: HashMap<usize, MovingAverageDispatcher>,
            pub value: Option<f64>,
        }

        impl Mavp {
            pub fn new(minperiod: usize, maxperiod: usize, matype: MaType) -> Self {
                Self {
                    minperiod,
                    maxperiod,
                    matype,
                    lookback: matype.lookback(maxperiod),
                    history: Vec::new(),
                    states: HashMap::new(),
                    value: None,
                }
            }

            pub fn append(&mut self, input: f64, period: f64) -> Option<f64> {
                self.history.push(input);
                let selected = (period as usize).clamp(self.minperiod, self.maxperiod);

                let mut selected_value = None;
                for (state_period, state) in &mut self.states {
                    let current = state.append(input);
                    if *state_period == selected {
                        selected_value = current;
                    }
                }

                if !self.states.contains_key(&selected) {
                    let source_start = self.lookback - self.matype.lookback(selected);
                    if self.history.len() > source_start {
                        let mut state = MovingAverageDispatcher::new(selected, self.matype)
                            .expect("MAVP constructor validates the complete period range");
                        selected_value = self.history[source_start..]
                            .iter()
                            .copied()
                            .fold(None, |_, value| state.append(value));
                        self.states.insert(selected, state);
                    }
                }

                self.value = if self.history.len() > self.lookback {
                    selected_value
                } else {
                    None
                };
                self.value
            }
        }
    }

    #[test]
    fn bitwise_matches_previous_behaviour_for_random_periods_and_every_matype() {
        let input = lcg_series(5_000, 0x4d41_5650_0000_0001);
        let periods = lcg_periods(input.len(), 0x4d41_5650_0000_0002, 2, 30);
        for code in 0..=8 {
            let matype = MaType::try_from(code).unwrap();
            let mut oracle = oracle::Mavp::new(2, 30, matype);
            let mut state = VariablePeriodMovingAverage::new(2, 30, matype).unwrap();
            for index in 0..input.len() {
                let want = oracle.append(input[index], periods[index]);
                let got = state.append(input[index], periods[index]);
                assert_eq!(
                    got.map(f64::to_bits),
                    want.map(f64::to_bits),
                    "matype {code} bar {index}"
                );
            }
            // The bulk entry point must agree with the oracle too.
            let bulk = variable_period_moving_average(&input, &periods, 2, 30, matype).unwrap();
            let mut replay = oracle::Mavp::new(2, 30, matype);
            for index in 0..input.len() {
                let want = replay
                    .append(input[index], periods[index])
                    .unwrap_or(f64::NAN);
                assert_eq!(
                    bulk[index].to_bits(),
                    want.to_bits(),
                    "bulk matype {code} bar {index}"
                );
            }
        }
    }

    #[test]
    fn bitwise_matches_previous_behaviour_for_sparse_and_clamped_periods() {
        let input = lcg_series(2_000, 0x4d41_5650_0000_0003);
        // Values outside the bounds exercise the clamp, and long runs of a
        // single period exercise the catch-up path with large gaps.
        let mut periods = Vec::with_capacity(input.len());
        for index in 0..input.len() {
            periods.push(match index % 500 {
                0..=200 => 3.0,
                201..=400 => 1.0,
                _ => 99.0,
            });
        }
        for code in 0..=8 {
            let matype = MaType::try_from(code).unwrap();
            let mut oracle = oracle::Mavp::new(2, 12, matype);
            let mut state = VariablePeriodMovingAverage::new(2, 12, matype).unwrap();
            for index in 0..input.len() {
                let want = oracle.append(input[index], periods[index]);
                let got = state.append(input[index], periods[index]);
                assert_eq!(
                    got.map(f64::to_bits),
                    want.map(f64::to_bits),
                    "matype {code} bar {index}"
                );
            }
        }
    }

    #[test]
    fn retained_history_is_bounded_once_every_period_is_materialized() {
        let input = lcg_series(50_000, 0x4d41_5650_0000_0004);
        let periods = lcg_periods(input.len(), 0x4d41_5650_0000_0005, 2, 30);
        let mut state =
            VariablePeriodMovingAverage::new(2, 30, MaType::SimpleMovingAverage).unwrap();
        let mut peak = 0;
        for index in 0..input.len() {
            state.append(input[index], periods[index]);
            peak = peak.max(state.retained_history());
        }
        assert!(
            peak < 5_000,
            "retained history should stay bounded, peaked at {peak}"
        );
    }

    #[test]
    fn reset_restores_initial_behaviour() {
        let input = lcg_series(1_500, 0x4d41_5650_0000_0006);
        let periods = lcg_periods(input.len(), 0x4d41_5650_0000_0007, 2, 12);
        let mut state =
            VariablePeriodMovingAverage::new(2, 12, MaType::ExponentialMovingAverage).unwrap();
        let first: Vec<Option<f64>> = (0..input.len())
            .map(|index| state.append(input[index], periods[index]))
            .collect();
        state.reset();
        assert!(state.value().is_none());
        for index in 0..input.len() {
            assert_eq!(
                state.append(input[index], periods[index]).map(f64::to_bits),
                first[index].map(f64::to_bits)
            );
        }
    }

    #[test]
    fn matches_batch_for_every_moving_average_type() {
        let input: Vec<f64> = (0..700)
            .map(|index| 100.0 + (index as f64 * 0.17).sin() * 8.0 + index as f64 * 0.01)
            .collect();
        let requested = [1.9, 3.8, 7.2, 11.9, 50.0];
        let periods: Vec<f64> = (0..input.len())
            .map(|index| requested[index % requested.len()])
            .collect();
        for code in 0..=8 {
            let ma_type = MaType::try_from(code).unwrap();
            let expected =
                variable_period_moving_average(&input, &periods, 2, 12, ma_type).unwrap();
            let mut state = VariablePeriodMovingAverage::new(2, 12, ma_type).unwrap();
            for index in 0..input.len() {
                match state.append(input[index], periods[index]) {
                    Some(actual) => assert!((actual - expected[index]).abs() < 1e-8),
                    None => assert!(expected[index].is_nan()),
                }
            }
            state.reset();
            assert!(state.value().is_none());
        }
    }
}
