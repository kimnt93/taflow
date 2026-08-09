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
        output.reserve(input.len());
        for (&value, &period) in input.iter().zip(periods) {
            output.push(self.append(value, period).unwrap_or(f64::NAN));
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
        self.value = None;
    }
}
