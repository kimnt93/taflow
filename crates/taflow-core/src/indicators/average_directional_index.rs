//! Stateful Average Directional Index.
//!
//! ADX advances Wilder-smoothed true range and directional movement, seeds
//! from the first full period of DX values, and then Wilder-smooths later DX.

use crate::error::TaResult;

use crate::stream::directional::DirectionalMovement;

/// Incremental ADX with TA-Lib-compatible seeding and lookback.
/// Persistent Rust state or aligned output type for `AverageDirectionalIndex`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct AverageDirectionalIndex {
    // `pub(super)` so the ADXR bulk kernel can advance the shared recurrence.
    pub(super) period: usize,
    pub(super) period_f: f64,
    pub(super) directional: DirectionalMovement,
    pub(super) dx_sum: f64,
    pub(super) dx_count: usize,
    pub(super) value: Option<f64>,
}

impl AverageDirectionalIndex {
    /// Creates an ADX state with a period of at least two bars.
    pub fn new(period: usize) -> TaResult<Self> {
        Ok(Self {
            period,
            period_f: period as f64,
            directional: DirectionalMovement::new(period)?,
            dx_sum: 0.0,
            dx_count: 0,
            value: None,
        })
    }

    /// Appends one high, low, and close observation.
    pub fn append(&mut self, high: f64, low: f64, close: f64) -> Option<f64> {
        let directional = self.directional.append(high, low, close)?;
        self.value = if self.dx_count < self.period {
            self.dx_sum += directional.dx;
            self.dx_count += 1;
            (self.dx_count == self.period).then_some(self.dx_sum / self.period_f)
        } else {
            Some(
                (self.value.expect("ADX is seeded") * (self.period_f - 1.0) + directional.dx)
                    / self.period_f,
            )
        };
        self.value
    }

    /// Bulk kernel: once the ADX seed exists, advances the Wilder-smoothed
    /// TR/+DM/-DM recurrences and the ADX recurrence in one loop with all
    /// scalar states held in locals, writing NaN during warm-up. Bit-identical
    /// to per-bar [`Self::append`] in outputs and post-run streaming state.
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
        // Warm-up prologue: per-bar appends until the ADX seed is emitted.
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

        let pf = self.period_f;
        let (mut previous_high, mut previous_low, mut previous_close) = self
            .directional
            .previous
            .expect("warm directional state has a previous bar");
        let mut smoothed_tr = self.directional.true_range;
        let mut smoothed_pdm = self.directional.plus_dm;
        let mut smoothed_mdm = self.directional.minus_dm;
        let mut adx = self.value.expect("ADX is seeded");
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
            let dx = if smoothed_tr > 0.0 {
                let plus_di = 100.0 * smoothed_pdm / smoothed_tr;
                let minus_di = 100.0 * smoothed_mdm / smoothed_tr;
                let sum = plus_di + minus_di;
                if sum > 0.0 {
                    100.0 * (plus_di - minus_di).abs() / sum
                } else {
                    0.0
                }
            } else {
                0.0
            };
            adx = (adx * (pf - 1.0) + dx) / pf;
            output.push(adx);
        }

        self.directional.previous = Some((previous_high, previous_low, previous_close));
        self.directional.true_range = smoothed_tr;
        self.directional.plus_dm = smoothed_pdm;
        self.directional.minus_dm = smoothed_mdm;
        self.directional.index += len - index;
        self.value = Some(adx);
    }

    /// Returns the latest warmed output.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Restores the post-construction state.
    pub fn reset(&mut self) {
        self.directional.reset();
        self.dx_sum = 0.0;
        self.dx_count = 0;
        self.value = None;
    }
}
