//! Stateful Average Directional Movement Index Rating.

use std::collections::VecDeque;

use crate::error::TaResult;

use crate::indicators::AverageDirectionalIndex;

/// Incremental ADXR using the current and `period - 1` lagged ADX values.
/// Persistent Rust state or aligned output type for `AverageDirectionalIndexRating`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct AverageDirectionalIndexRating {
    period: usize,
    adx: AverageDirectionalIndex,
    values: VecDeque<f64>,
    value: Option<f64>,
}

impl AverageDirectionalIndexRating {
    /// Creates an ADXR state with a period of at least two bars.
    pub fn new(period: usize) -> TaResult<Self> {
        Ok(Self {
            period,
            adx: AverageDirectionalIndex::new(period)?,
            values: VecDeque::with_capacity(period),
            value: None,
        })
    }

    /// Appends one high, low, and close observation.
    pub fn append(&mut self, high: f64, low: f64, close: f64) -> Option<f64> {
        let current = self.adx.append(high, low, close)?;
        self.values.push_back(current);
        self.value = if self.values.len() == self.period {
            let lagged = self.values.pop_front().expect("full ADXR lag window");
            Some((current + lagged) / 2.0)
        } else {
            None
        };
        self.value
    }

    /// Bulk kernel: once warm, advances the shared Wilder/ADX recurrences in
    /// one loop with all scalar states held in locals while maintaining the
    /// ADX lag window, writing NaN during warm-up. Bit-identical to per-bar
    /// [`Self::append`] in outputs and post-run streaming state.
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
        // Warm-up prologue: per-bar appends until the lag window is full.
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

        let pf = self.adx.period_f;
        let (mut previous_high, mut previous_low, mut previous_close) = self
            .adx
            .directional
            .previous
            .expect("warm directional state has a previous bar");
        let mut smoothed_tr = self.adx.directional.true_range;
        let mut smoothed_pdm = self.adx.directional.plus_dm;
        let mut smoothed_mdm = self.adx.directional.minus_dm;
        let mut adx = self.adx.value.expect("ADX is seeded");
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

            self.values.push_back(adx);
            let lagged = self.values.pop_front().expect("full ADXR lag window");
            last = (adx + lagged) / 2.0;
            output.push(last);
        }

        self.adx.directional.previous = Some((previous_high, previous_low, previous_close));
        self.adx.directional.true_range = smoothed_tr;
        self.adx.directional.plus_dm = smoothed_pdm;
        self.adx.directional.minus_dm = smoothed_mdm;
        self.adx.directional.index += len - index;
        self.adx.value = Some(adx);
        self.value = Some(last);
    }

    /// Returns the latest warmed output.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Restores the post-construction state.
    pub fn reset(&mut self) {
        self.adx.reset();
        self.values.clear();
        self.value = None;
    }
}
