//! Persistent variable-period moving average.

use crate::error::{TaError, TaResult};
use crate::ma_type::MaType;

use crate::stream::moving_average_dispatcher::MovingAverageDispatcher;

/// One configured moving-average state and the global bar where it starts.
struct PeriodState {
    start_index: usize,
    state: MovingAverageDispatcher,
}

/// Variable-period moving average with TA-Lib-compatible period coercion.
///
/// One moving-average state is constructed for every permitted period. Each
/// state advances chronologically after its TA-Lib alignment offset, so
/// `append` retains bounded state, performs no allocation, and can switch to
/// any configured period without replaying historical inputs.
pub struct VariablePeriodMovingAverage {
    min_period: usize,
    max_period: usize,
    lookback: usize,
    count: usize,
    states: Vec<PeriodState>,
    scratch: Vec<f64>,
    selected_scratch: Vec<usize>,
    value: Option<f64>,
}

impl VariablePeriodMovingAverage {
    /// Create a variable-period moving average.
    ///
    /// `min_period` must be positive and no greater than `max_period`. The
    /// moving-average type controls both the recurrence and global warm-up.
    pub fn new(min_period: usize, max_period: usize, average_type: MaType) -> TaResult<Self> {
        if min_period == 0 || max_period < min_period {
            return Err(TaError::InvalidParameter {
                name: "min_period/max_period",
                value: format!("{min_period}/{max_period}"),
                reason: "min_period >= 1 and max_period >= min_period required",
            });
        }

        let lookback = average_type.lookback(max_period);
        let mut states = Vec::with_capacity(max_period - min_period + 1);
        for period in min_period..=max_period {
            states.push(PeriodState {
                start_index: lookback - average_type.lookback(period),
                state: MovingAverageDispatcher::new(period, average_type)?,
            });
        }

        Ok(Self {
            min_period,
            max_period,
            lookback,
            count: 0,
            states,
            scratch: Vec::new(),
            selected_scratch: Vec::new(),
            value: None,
        })
    }

    /// Append one value and its requested period.
    ///
    /// TA-Lib truncates the floating-point period to an integer and clamps it
    /// to the configured inclusive range. `None` is returned during the global
    /// warm-up determined by `max_period` and the selected average type.
    pub fn append(&mut self, input: f64, period: f64) -> Option<f64> {
        let index = self.count;
        let selected = (period as usize).clamp(self.min_period, self.max_period);
        let selected_slot = selected - self.min_period;
        let mut selected_value = None;

        for (slot, period_state) in self.states.iter_mut().enumerate() {
            if index >= period_state.start_index {
                let current = period_state.state.append(input);
                if slot == selected_slot {
                    selected_value = current;
                }
            }
        }

        self.count += 1;
        self.value = if self.count > self.lookback {
            selected_value
        } else {
            None
        };
        self.value
    }

    /// Append aligned value and period slices into an aligned output history.
    ///
    /// Lengths are validated before state mutation. Warm-up positions are
    /// represented by `f64::NAN`, and exit state is identical to scalar replay.
    pub fn extend_slices_into(
        &mut self,
        input: &[f64],
        periods: &[f64],
        output: &mut Vec<f64>,
    ) -> TaResult<()> {
        if input.len() != periods.len() {
            return Err(TaError::LengthMismatch {
                expected: input.len(),
                got: periods.len(),
            });
        }
        if input.is_empty() {
            return Ok(());
        }

        let output_start = output.len();
        output.resize(output_start + input.len(), f64::NAN);
        let initial_count = self.count;
        self.selected_scratch.clear();
        self.selected_scratch.reserve(periods.len());
        self.selected_scratch.extend(periods.iter().map(|&period| {
            (period as usize).clamp(self.min_period, self.max_period) - self.min_period
        }));

        // Advance one configured MA at a time. This keeps the exact state that
        // scalar replay would leave behind while moving dispatcher selection
        // out of the per-bar hot loop and allowing every concrete MA to use
        // its own slice kernel.
        for (slot, period_state) in self.states.iter_mut().enumerate() {
            let input_offset = period_state.start_index.saturating_sub(initial_count);
            if input_offset >= input.len() {
                continue;
            }

            self.scratch.clear();
            period_state
                .state
                .extend_slice_into(&input[input_offset..], &mut self.scratch);

            for local_index in input_offset..input.len() {
                let global_index = initial_count + local_index;
                if global_index < self.lookback {
                    continue;
                }
                if self.selected_scratch[local_index] == slot {
                    output[output_start + local_index] = self.scratch[local_index - input_offset];
                }
            }
        }

        self.count += input.len();
        if self.count > self.lookback {
            self.value = Some(output[output_start + input.len() - 1]);
        } else {
            self.value = None;
        }
        Ok(())
    }

    /// Return the latest warmed output, or `None` during warm-up.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Restore fresh-state behavior while retaining all allocated storage.
    pub fn reset(&mut self) {
        self.count = 0;
        for period_state in &mut self.states {
            period_state.state.reset();
        }
        self.scratch.clear();
        self.selected_scratch.clear();
        self.value = None;
    }
}
