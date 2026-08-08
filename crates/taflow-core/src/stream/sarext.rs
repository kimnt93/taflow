//! Stateful Parabolic SAR Extended.
//!
//! SAREXT preserves TA-Lib's signed output, optional starting direction,
//! reversal offset, and independent long/short acceleration schedules.

use crate::TaResult;

/// Computes an aligned extended Parabolic SAR vector from high and low slices.
///
/// # Parameters
///
/// * `high`, `low` - Equal-length chronological price series.
/// * Remaining arguments configure starting value, reversal offset, and acceleration schedules.
///
/// # Returns
///
/// An aligned vector of signed extended Parabolic SAR values.
#[allow(clippy::too_many_arguments)]
pub fn parabolic_sar_extended(
    high: &[f64],
    low: &[f64],
    startvalue: f64,
    offsetonreverse: f64,
    accelerationinitlong: f64,
    accelerationlong: f64,
    accelerationmaxlong: f64,
    accelerationinitshort: f64,
    accelerationshort: f64,
    accelerationmaxshort: f64,
) -> TaResult<Vec<f64>> {
    if high.len() != low.len() {
        return Err(crate::TaError::LengthMismatch {
            expected: high.len(),
            got: low.len(),
        });
    }
    let mut state = ParabolicSarExtended::new(
        startvalue,
        offsetonreverse,
        accelerationinitlong,
        accelerationlong,
        accelerationmaxlong,
        accelerationinitshort,
        accelerationshort,
        accelerationmaxshort,
    );
    let mut output = Vec::with_capacity(high.len());
    state.extend_slice_into(high, low, &mut output);
    Ok(output)
}

/// Incremental extended Parabolic SAR with a one-bar lookback.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `ParabolicSarExtended`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct ParabolicSarExtended {
    start_value: f64,
    offset_on_reverse: f64,
    acceleration_init_long: f64,
    acceleration_long: f64,
    acceleration_max_long: f64,
    acceleration_init_short: f64,
    acceleration_short: f64,
    acceleration_max_short: f64,
    first_bar: Option<(f64, f64)>,
    initialized: bool,
    is_long: bool,
    sar: f64,
    extreme: f64,
    factor_long: f64,
    factor_short: f64,
    previous_high: f64,
    previous_low: f64,
    value: Option<f64>,
}

impl ParabolicSarExtended {
    /// Creates a SAREXT state with TA-Lib's complete parameter surface.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        start_value: f64,
        offset_on_reverse: f64,
        acceleration_init_long: f64,
        acceleration_long: f64,
        acceleration_max_long: f64,
        acceleration_init_short: f64,
        acceleration_short: f64,
        acceleration_max_short: f64,
    ) -> Self {
        Self {
            start_value,
            offset_on_reverse,
            acceleration_init_long,
            acceleration_long,
            acceleration_max_long,
            acceleration_init_short,
            acceleration_short,
            acceleration_max_short,
            first_bar: None,
            initialized: false,
            is_long: false,
            sar: 0.0,
            extreme: 0.0,
            factor_long: acceleration_init_long,
            factor_short: acceleration_init_short,
            previous_high: 0.0,
            previous_low: 0.0,
            value: None,
        }
    }

    /// Appends one high and low bar.
    pub fn append(&mut self, high: f64, low: f64) -> Option<f64> {
        if self.first_bar.is_none() {
            self.first_bar = Some((high, low));
            return None;
        }
        if !self.initialized {
            let (first_high, first_low) = self.first_bar.expect("first SAREXT bar is stored");
            self.is_long = if self.start_value == 0.0 {
                let minus_move = first_low - low;
                let plus_move = high - first_high;
                !(minus_move > 0.0 && minus_move > plus_move)
            } else {
                self.start_value > 0.0
            };
            if self.start_value == 0.0 {
                if self.is_long {
                    self.extreme = high;
                    self.sar = first_low;
                } else {
                    self.extreme = low;
                    self.sar = first_high;
                }
            } else if self.start_value > 0.0 {
                self.extreme = high;
                self.sar = self.start_value;
            } else {
                self.extreme = low;
                self.sar = self.start_value.abs();
            }
            self.factor_long = self.acceleration_init_long;
            self.factor_short = self.acceleration_init_short;
            self.previous_high = high;
            self.previous_low = low;
            self.initialized = true;
            self.advance(high, low, high, low);
            return self.value;
        }

        let previous_high = self.previous_high;
        let previous_low = self.previous_low;
        self.previous_high = high;
        self.previous_low = low;
        self.advance(high, low, previous_high, previous_low);
        self.value
    }

    fn advance(&mut self, high: f64, low: f64, previous_high: f64, previous_low: f64) {
        if self.is_long {
            if low <= self.sar {
                self.is_long = false;
                self.sar = self.extreme.max(previous_high).max(high);
                if self.offset_on_reverse != 0.0 {
                    self.sar += self.sar * self.offset_on_reverse;
                }
                self.value = Some(-self.sar);
                self.factor_short = self.acceleration_init_short;
                self.extreme = low;
                self.sar += self.factor_short * (self.extreme - self.sar);
                self.sar = self.sar.max(previous_high).max(high);
            } else {
                self.value = Some(self.sar);
                if high > self.extreme {
                    self.extreme = high;
                    self.factor_long =
                        (self.factor_long + self.acceleration_long).min(self.acceleration_max_long);
                }
                self.sar += self.factor_long * (self.extreme - self.sar);
                self.sar = self.sar.min(previous_low).min(low);
            }
        } else if high >= self.sar {
            self.is_long = true;
            self.sar = self.extreme.min(previous_low).min(low);
            if self.offset_on_reverse != 0.0 {
                self.sar -= self.sar * self.offset_on_reverse;
            }
            self.value = Some(self.sar);
            self.factor_long = self.acceleration_init_long;
            self.extreme = high;
            self.sar += self.factor_long * (self.extreme - self.sar);
            self.sar = self.sar.min(previous_low).min(low);
        } else {
            self.value = Some(-self.sar);
            if low < self.extreme {
                self.extreme = low;
                self.factor_short =
                    (self.factor_short + self.acceleration_short).min(self.acceleration_max_short);
            }
            self.sar += self.factor_short * (self.extreme - self.sar);
            self.sar = self.sar.max(previous_high).max(high);
        }
    }

    /// Computes or updates `value` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Computes or updates `reset` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn reset(&mut self) {
        self.first_bar = None;
        self.initialized = false;
        self.is_long = false;
        self.sar = 0.0;
        self.extreme = 0.0;
        self.factor_long = self.acceleration_init_long;
        self.factor_short = self.acceleration_init_short;
        self.previous_high = 0.0;
        self.previous_low = 0.0;
        self.value = None;
    }

    /// Bulk kernel over aligned high/low slices.
    ///
    /// The recurrence is inherently serial, so the only bulk win is splitting
    /// the warm-up prologue from a branch-free steady loop that runs the very
    /// same `advance` step; outputs and exit state stay bit-identical to
    /// repeated `append`.
    pub fn extend_slice_into(&mut self, high: &[f64], low: &[f64], output: &mut Vec<f64>) {
        let len = high.len().min(low.len());
        output.reserve(len);
        let mut index = 0;
        while index < len && !self.initialized {
            output.push(self.append(high[index], low[index]).unwrap_or(f64::NAN));
            index += 1;
        }
        while index < len {
            let (high, low) = (high[index], low[index]);
            let previous_high = self.previous_high;
            let previous_low = self.previous_low;
            self.previous_high = high;
            self.previous_low = low;
            self.advance(high, low, previous_high, previous_low);
            output.push(
                self.value
                    .expect("an initialized SAREXT always has a value"),
            );
            index += 1;
        }
    }
}

impl Default for ParabolicSarExtended {
    fn default() -> Self {
        Self::new(0.0, 0.0, 0.02, 0.02, 0.2, 0.02, 0.02, 0.2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lcg_bars(len: usize, mut seed: u64) -> (Vec<f64>, Vec<f64>) {
        let mut high = Vec::with_capacity(len);
        let mut low = Vec::with_capacity(len);
        let mut center = 100.0_f64;
        for _ in 0..len {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            let unit = (seed >> 11) as f64 / (1u64 << 53) as f64;
            center += (unit - 0.5) * 2.0;
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            let spread = 0.2 + ((seed >> 11) as f64 / (1u64 << 53) as f64) * 2.0;
            high.push(center + spread);
            low.push(center - spread);
        }
        (high, low)
    }

    const PARAMS: [f64; 8] = [0.0, 0.01, 0.03, 0.02, 0.25, 0.04, 0.03, 0.3];

    fn make_state() -> ParabolicSarExtended {
        ParabolicSarExtended::new(
            PARAMS[0], PARAMS[1], PARAMS[2], PARAMS[3], PARAMS[4], PARAMS[5], PARAMS[6], PARAMS[7],
        )
    }

    #[test]
    fn bulk_chunked_and_continuation_match_append_bitwise() {
        let (high, low) = lcg_bars(5_000, 0xb6b6_0000_0000_0001);
        let mut reference = make_state();
        let expected: Vec<Option<f64>> = (0..high.len())
            .map(|index| reference.append(high[index], low[index]))
            .collect();

        let bulk = parabolic_sar_extended(
            &high, &low, PARAMS[0], PARAMS[1], PARAMS[2], PARAMS[3], PARAMS[4], PARAMS[5],
            PARAMS[6], PARAMS[7],
        )
        .unwrap();
        for (index, (got, want)) in bulk.iter().zip(&expected).enumerate() {
            assert_eq!(
                got.to_bits(),
                want.unwrap_or(f64::NAN).to_bits(),
                "bulk @{index}"
            );
        }

        for chunk in [1_usize, 7, 97] {
            let mut state = make_state();
            let mut output = Vec::new();
            for start in (0..high.len()).step_by(chunk) {
                let end = (start + chunk).min(high.len());
                state.extend_slice_into(&high[start..end], &low[start..end], &mut output);
            }
            for (index, (got, want)) in output.iter().zip(&expected).enumerate() {
                assert_eq!(
                    got.to_bits(),
                    want.unwrap_or(f64::NAN).to_bits(),
                    "chunk {chunk} @{index}"
                );
            }
        }

        let split = 2_121;
        let mut state = make_state();
        let mut output = Vec::new();
        state.extend_slice_into(&high[..split], &low[..split], &mut output);
        for index in split..high.len() {
            assert_eq!(
                state.append(high[index], low[index]).map(f64::to_bits),
                expected[index].map(f64::to_bits),
                "continuation @{index}"
            );
        }
    }

    #[test]
    fn reset_in_place_restores_initial_behaviour() {
        let (high, low) = lcg_bars(600, 0xb6b6_0000_0000_0002);
        let mut state = make_state();
        let first: Vec<Option<f64>> = (0..high.len())
            .map(|index| state.append(high[index], low[index]))
            .collect();
        state.reset();
        assert!(state.value().is_none());
        for index in 0..high.len() {
            assert_eq!(
                state.append(high[index], low[index]).map(f64::to_bits),
                first[index].map(f64::to_bits)
            );
        }
    }

    #[test]
    fn matches_batch_with_asymmetric_parameters_and_reversals() {
        let center: Vec<f64> = (0..300)
            .map(|index| 100.0 + (index as f64 * 0.21).sin() * 12.0)
            .collect();
        let high: Vec<f64> = center.iter().map(|value| value + 1.5).collect();
        let low: Vec<f64> = center.iter().map(|value| value - 1.2).collect();
        let expected =
            parabolic_sar_extended(&high, &low, 0.0, 0.01, 0.03, 0.02, 0.25, 0.04, 0.03, 0.3)
                .unwrap();
        let mut state = ParabolicSarExtended::new(0.0, 0.01, 0.03, 0.02, 0.25, 0.04, 0.03, 0.3);
        for index in 0..center.len() {
            let actual = state.append(high[index], low[index]);
            if expected[index].is_nan() {
                assert_eq!(actual, None);
            } else {
                assert!((actual.unwrap() - expected[index]).abs() < 1e-12);
            }
        }
        let expected_final = state.value();
        state.reset();
        for index in 0..center.len() {
            state.append(high[index], low[index]);
        }
        assert_eq!(state.value(), expected_final);
    }
}
