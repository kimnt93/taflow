//! Stateful Average Directional Movement Index Rating.

use std::collections::VecDeque;

use crate::error::TaResult;

use super::AverageDirectionalIndex;

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

#[cfg(test)]
mod tests {
    use super::*;

    fn lcg_bars(n: usize, mut state: u64) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
        let mut next = move || {
            state = state
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            (state >> 11) as f64 / (1u64 << 53) as f64
        };
        let mut high = Vec::with_capacity(n);
        let mut low = Vec::with_capacity(n);
        let mut close = Vec::with_capacity(n);
        for _ in 0..n {
            let base = 90.0 + next() * 20.0;
            let up = next() * 2.0;
            let down = next() * 2.0;
            high.push(base + up);
            low.push(base - down);
            close.push(base + (up - down) * next());
        }
        (high, low, close)
    }

    fn assert_same_bits(actual: &[f64], expected: &[f64], label: &str) {
        assert_eq!(actual.len(), expected.len(), "{label}: length");
        for (i, (a, b)) in actual.iter().zip(expected).enumerate() {
            assert_eq!(a.to_bits(), b.to_bits(), "{label}: bar {i}");
        }
    }

    #[test]
    fn bulk_is_bitwise_identical_to_per_bar_append() {
        let (high, low, close) = lcg_bars(5_000, 0x5EED_AD2A);
        let (th, tl, tc) = lcg_bars(128, 0x7A11_AD2A);
        for period in [1usize, 2, 14, 30] {
            let mut per_bar = AverageDirectionalIndexRating::new(period).unwrap();
            let reference: Vec<f64> = (0..high.len())
                .map(|i| {
                    per_bar
                        .append(high[i], low[i], close[i])
                        .unwrap_or(f64::NAN)
                })
                .collect();
            let tail_reference: Vec<f64> = (0..th.len())
                .map(|i| per_bar.append(th[i], tl[i], tc[i]).unwrap_or(f64::NAN))
                .collect();

            for chunk in [usize::MAX, 1, 7, 97] {
                let mut state = AverageDirectionalIndexRating::new(period).unwrap();
                let mut out = Vec::new();
                let mut start = 0;
                while start < high.len() {
                    let end = (start + chunk.min(high.len())).min(high.len());
                    state.extend_slices_into(
                        &high[start..end],
                        &low[start..end],
                        &close[start..end],
                        &mut out,
                    );
                    start = end;
                }
                let label = format!("p{period} chunk {chunk}");
                assert_same_bits(&out, &reference, &label);
                let tail_out: Vec<f64> = (0..th.len())
                    .map(|i| state.append(th[i], tl[i], tc[i]).unwrap_or(f64::NAN))
                    .collect();
                assert_same_bits(&tail_out, &tail_reference, &format!("{label} tail"));
            }
        }
    }

    #[test]
    fn matches_batch_and_reset_replay() {
        let close: Vec<f64> = (0..700)
            .map(|index| 100.0 + (index as f64 * 0.17).sin() * 8.0 + index as f64 * 0.01)
            .collect();
        let high: Vec<f64> = close.iter().map(|value| value + 1.3).collect();
        let low: Vec<f64> = close.iter().map(|value| value - 1.1).collect();
        for period in [2, 3, 14, 30] {
            let expected =
                crate::stream::average_directional_index_rating(&high, &low, &close, period)
                    .unwrap();
            let mut state = AverageDirectionalIndexRating::new(period).unwrap();
            for index in 0..close.len() {
                match state.append(high[index], low[index], close[index]) {
                    Some(actual) => assert!((actual - expected[index]).abs() < 1e-12),
                    None => assert!(expected[index].is_nan()),
                }
            }
            let final_value = state.value();
            state.reset();
            for index in 0..close.len() {
                state.append(high[index], low[index], close[index]);
            }
            assert_eq!(state.value(), final_value);
        }
    }
}
// Batch Average Directional Movement Index Rating.

/// Compute the average directional index rating result for the supplied aligned series.
///
/// # Parameters
///
/// * `high` - Input series or configuration value.
/// * `low` - Input series or configuration value.
/// * `close` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn average_directional_index_rating(
    high: &[f64],
    low: &[f64],
    close: &[f64],
    timeperiod: usize,
) -> TaResult<Vec<f64>> {
    let adx_values = crate::stream::average_directional_index(high, low, close, timeperiod)?;
    let len = adx_values.len();
    let lookback = 3 * timeperiod - 2;
    let mut output = vec![f64::NAN; len];
    for index in lookback..len {
        output[index] = (adx_values[index] + adx_values[index - timeperiod + 1]) / 2.0;
    }
    Ok(output)
}
