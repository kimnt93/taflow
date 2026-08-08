//! Stateful Stochastic Relative Strength Index.
//!
//! STOCHRSI pipelines each warmed Wilder RSI value through the persistent
//! Fast Stochastic state, preserving adaptive-MA seed and rounding semantics.

use crate::error::TaResult;
use crate::ma_type::MaType;

use super::{FastStochasticOscillator, RelativeStrengthIndex, StreamingIndicator};

/// One aligned stochastic-RSI fast %K and fast %D observation.
#[derive(Debug, Clone, Copy, PartialEq)]
/// Persistent Rust state or aligned output type for `StochasticRelativeStrengthIndexValue`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct StochasticRelativeStrengthIndexValue {
    pub fastk: f64,
    pub fastd: f64,
}

/// Incremental STOCHRSI state.
/// Persistent Rust state or aligned output type for `StochasticRelativeStrengthIndex`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct StochasticRelativeStrengthIndex {
    rsi: RelativeStrengthIndex,
    stochastic: FastStochasticOscillator,
    value: Option<StochasticRelativeStrengthIndexValue>,
}

impl StochasticRelativeStrengthIndex {
    /// Creates STOCHRSI with a selectable fast-%D moving-average type.
    pub fn new(
        timeperiod: usize,
        fastk_period: usize,
        fastd_period: usize,
        fastd_matype: MaType,
    ) -> TaResult<Self> {
        Ok(Self {
            rsi: RelativeStrengthIndex::new(timeperiod)?,
            stochastic: FastStochasticOscillator::new(fastk_period, fastd_period, fastd_matype)?,
            value: None,
        })
    }

    /// Appends one close value.
    pub fn append(&mut self, input: f64) -> Option<StochasticRelativeStrengthIndexValue> {
        self.value = self.rsi.append(input).and_then(|rsi| {
            self.stochastic.append(rsi, rsi, rsi).map(|value| {
                StochasticRelativeStrengthIndexValue {
                    fastk: value.fastk,
                    fastd: value.fastd,
                }
            })
        });
        self.value
    }

    /// Bulk kernel: the Wilder RSI recurrence runs per bar (it is a two-FLOP
    /// serial recurrence, not the bottleneck), the warmed RSI values are then
    /// handed to [`FastStochasticOscillator::extend_slices_into`], whose vHGW
    /// extrema pass removes the O(n * fastk_period) work. Outputs and post-run
    /// state are bit-identical to per-bar [`Self::append`]; warm-up bars are
    /// NaN.
    pub fn extend_slices_into(
        &mut self,
        inputs: &[f64],
        fastk_out: &mut Vec<f64>,
        fastd_out: &mut Vec<f64>,
    ) {
        fastk_out.reserve(inputs.len());
        fastd_out.reserve(inputs.len());
        let mut warmed = Vec::with_capacity(inputs.len());
        for &input in inputs {
            if let Some(rsi) = self.rsi.append(input) {
                warmed.push(rsi);
            }
        }
        // RSI warm-up is a strict prefix, so the NaN bars are exactly the
        // leading `inputs.len() - warmed.len()` positions.
        for _ in 0..(inputs.len() - warmed.len()) {
            fastk_out.push(f64::NAN);
            fastd_out.push(f64::NAN);
        }
        self.stochastic
            .extend_slices_into(&warmed, &warmed, &warmed, fastk_out, fastd_out)
            .expect("identical slice lengths");
        self.value = self
            .stochastic
            .value()
            .map(|value| StochasticRelativeStrengthIndexValue {
                fastk: value.fastk,
                fastd: value.fastd,
            });
    }

    /// Returns the latest warmed output.
    pub fn value(&self) -> Option<StochasticRelativeStrengthIndexValue> {
        self.value
    }

    /// Restores the post-construction state.
    pub fn reset(&mut self) {
        self.rsi.reset();
        self.stochastic.reset();
        self.value = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_batch_for_all_moving_average_types() {
        let input: Vec<f64> = (0..500)
            .map(|index| 100.0 + (index as f64 * 0.17).sin() * 8.0 + index as f64 * 0.01)
            .collect();
        for code in 0..=8 {
            let ma_type = MaType::try_from(code).unwrap();
            let expected =
                crate::stream::stochastic_relative_strength_index(&input, 14, 5, 13, ma_type)
                    .unwrap();
            let mut state = StochasticRelativeStrengthIndex::new(14, 5, 13, ma_type).unwrap();
            for (index, input) in input.iter().copied().enumerate() {
                match state.append(input) {
                    Some(actual) => {
                        assert!((actual.fastk - expected.0[index]).abs() < 1e-8);
                        assert!((actual.fastd - expected.1[index]).abs() < 1e-8);
                    }
                    None => assert!(expected.0[index].is_nan()),
                }
            }
            let final_value = state.value();
            state.reset();
            for input in input.iter().copied() {
                state.append(input);
            }
            assert_eq!(state.value(), final_value);
        }
    }
}
// Batch Stochastic Relative Strength Index.
//
// STOCHRSI applies a rolling stochastic range to RSI values, then smooths
// fast %K with a selectable TA-Lib moving average to produce fast %D.

/// Computes aligned stochastic-RSI fast %K and fast %D arrays.
///
/// # Parameters
///
/// * `input` - Chronological close-price series.
/// * Period and moving-average parameters configure RSI and stochastic windows.
///
/// # Returns
///
/// A pair of same-length fast %K and fast %D arrays with warm-up NaNs.
pub fn stochastic_relative_strength_index(
    input: &[f64],
    timeperiod: usize,
    fastk_period: usize,
    fastd_period: usize,
    fastd_matype: MaType,
) -> TaResult<(Vec<f64>, Vec<f64>)> {
    let rsi_values = crate::stream::relative_strength_index(input, timeperiod)?;
    let rsi_valid = &rsi_values[timeperiod..];
    let (stochastic_k, stochastic_d) = crate::stream::fast_stochastic_oscillator(
        rsi_valid,
        rsi_valid,
        rsi_valid,
        fastk_period,
        fastd_period,
        fastd_matype,
    )?;
    let len = input.len();
    let mut fastk_out = vec![f64::NAN; len];
    let mut fastd_out = vec![f64::NAN; len];
    fastk_out[timeperiod..].copy_from_slice(&stochastic_k);
    fastd_out[timeperiod..].copy_from_slice(&stochastic_d);
    Ok((fastk_out, fastd_out))
}

#[cfg(test)]
mod stochrsi_bulk_tests {
    use super::*;

    fn lcg_series(len: usize, seed: u64) -> Vec<f64> {
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

    #[test]
    fn stochrsi_bulk_matches_append_bitwise() {
        let data = lcg_series(5_000, 0x4D59_5A5F_1122_3344);
        for period in [2usize, 5, 14, 30, 200] {
            for (fastk, fastd, ma) in [
                (5usize, 3usize, MaType::SimpleMovingAverage),
                (14, 4, MaType::ExponentialMovingAverage),
            ] {
                let mut reference =
                    StochasticRelativeStrengthIndex::new(period, fastk, fastd, ma).unwrap();
                let expected: Vec<(f64, f64)> = data
                    .iter()
                    .map(|&v| match reference.append(v) {
                        Some(value) => (value.fastk, value.fastd),
                        None => (f64::NAN, f64::NAN),
                    })
                    .collect();
                for chunk in [1usize, 7, 97, data.len()] {
                    let mut state =
                        StochasticRelativeStrengthIndex::new(period, fastk, fastd, ma).unwrap();
                    let (mut k_out, mut d_out) = (Vec::new(), Vec::new());
                    for piece in data.chunks(chunk) {
                        state.extend_slices_into(piece, &mut k_out, &mut d_out);
                    }
                    assert_eq!(k_out.len(), data.len());
                    for (i, (ek, ed)) in expected.iter().enumerate() {
                        assert_eq!(
                            ek.to_bits(),
                            k_out[i].to_bits(),
                            "fastk p={period} c={chunk} i={i}"
                        );
                        assert_eq!(
                            ed.to_bits(),
                            d_out[i].to_bits(),
                            "fastd p={period} c={chunk} i={i}"
                        );
                    }
                    for &value in data.iter().take(256) {
                        assert_eq!(
                            reference.append(value),
                            state.append(value),
                            "continue p={period} c={chunk}"
                        );
                    }
                    reference.reset();
                    for &value in &data {
                        reference.append(value);
                    }
                }
            }
        }
    }
}
