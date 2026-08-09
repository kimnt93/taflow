//! Persistent rolling midpoint state.

use super::vhgw;
use super::StreamingIndicator;
use crate::error::TaResult;
use crate::stream::RollingExtrema;

/// Rolling midpoint of the highest and lowest values over `period` bars.
#[derive(Debug, Clone)]
pub struct RollingMidpoint {
    extrema: RollingExtrema,
    value: Option<f64>,
}

impl RollingMidpoint {
    /// Creates a rolling midpoint state with the validated lookback period.
    pub fn new(period: usize) -> TaResult<Self> {
        Ok(Self {
            extrema: RollingExtrema::new(period)?,
            value: None,
        })
    }

    /// Extends the state with one aligned slice and appends warm-up NaNs.
    pub fn extend_slice_into(&mut self, inputs: &[f64], output: &mut Vec<f64>) {
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
        Self::extend_slice_into(self, inputs, output);
    }
}
