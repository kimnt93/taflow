//! Incremental Plus Directional Indicator (+DI).
use super::directional::DirectionalMovement;
use crate::error::TaResult;

/// Persistent Rust state or aligned output type for `PlusDirectionalIndicator`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct PlusDirectionalIndicator {
    directional: DirectionalMovement,
    value: Option<f64>,
}
impl PlusDirectionalIndicator {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(period: usize) -> TaResult<Self> {
        Ok(Self {
            directional: DirectionalMovement::new(period)?,
            value: None,
        })
    }
    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, high: f64, low: f64, close: f64) -> Option<f64> {
        self.value = self.directional.append(high, low, close).map(|v| v.plus_di);
        self.value
    }
    /// Bulk kernel: once warm, advances the Wilder-smoothed TR/+DM/-DM
    /// recurrences in one loop with the scalar states held in locals, writing
    /// NaN during warm-up. Bit-identical to per-bar [`Self::append`] in
    /// outputs and post-run streaming state.
    pub fn extend_slices_into(
        &mut self,
        high: &[f64],
        low: &[f64],
        close: &[f64],
        output: &mut Vec<f64>,
    ) {
        let len = high.len().min(low.len()).min(close.len());
        output.reserve(len);
        let mut index = 0;
        // Warm-up prologue: per-bar appends until the Wilder sums are seeded.
        while index < len && self.value.is_none() {
            output.push(
                self.append(high[index], low[index], close[index])
                    .unwrap_or(f64::NAN),
            );
            index += 1;
        }
        if index == len {
            return;
        }

        let pf = self.directional.period_f;
        let (mut previous_high, mut previous_low, mut previous_close) = self
            .directional
            .previous
            .expect("warm directional state has a previous bar");
        let mut smoothed_tr = self.directional.true_range;
        let mut smoothed_pdm = self.directional.plus_dm;
        let mut smoothed_mdm = self.directional.minus_dm;
        let mut last = f64::NAN;
        for bar in index..len {
            let (high, low, close) = (high[bar], low[bar], close[bar]);
            let true_range = (high - low)
                .max((high - previous_close).abs())
                .max((low - previous_close).abs());
            let up = high - previous_high;
            let down = previous_low - low;
            let plus_dm = if up > down && up > 0.0 { up } else { 0.0 };
            let minus_dm = if down > up && down > 0.0 { down } else { 0.0 };
            previous_high = high;
            previous_low = low;
            previous_close = close;

            smoothed_tr = smoothed_tr - smoothed_tr / pf + true_range;
            smoothed_pdm = smoothed_pdm - smoothed_pdm / pf + plus_dm;
            smoothed_mdm = smoothed_mdm - smoothed_mdm / pf + minus_dm;
            last = if smoothed_tr > 0.0 {
                100.0 * smoothed_pdm / smoothed_tr
            } else {
                0.0
            };
            output.push(last);
        }

        self.directional.previous = Some((previous_high, previous_low, previous_close));
        self.directional.true_range = smoothed_tr;
        self.directional.plus_dm = smoothed_pdm;
        self.directional.minus_dm = smoothed_mdm;
        self.directional.index += len - index;
        self.value = Some(last);
    }

    /// Computes or updates `value` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn value(&self) -> Option<f64> {
        self.value
    }
    /// Reset the persistent state and clear the latest value.
    pub fn reset(&mut self) {
        self.directional.reset();
        self.value = None;
    }
}
