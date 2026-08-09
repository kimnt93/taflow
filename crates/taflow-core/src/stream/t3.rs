//! Stateful Tillson TripleExponentialAverage moving average.
//!
//! TripleExponentialAverage cascades six TA-Lib-seeded exponential moving averages and combines the
//! final four layers with coefficients derived from the volume factor.

use multiversion::multiversion;

use crate::error::{TaError, TaResult};

use super::{ExponentialMovingAverage, StreamingIndicator};

/// Steady-state kernel for the six fused EMA recurrences.
///
/// Split out of [`TripleExponentialAverage::extend_slice_into`] so it can carry
/// `#[multiversion]`: the loop is `mul_add`-bound and a portable build without
/// runtime dispatch lowers every `mul_add` to a libm `fma()` call. `mul_add` is
/// an explicitly fused operation in both cases, so the dispatched FMA variant
/// returns bit-identical results — only faster.
#[allow(clippy::too_many_arguments)]
#[allow(unexpected_cfgs)]
#[multiversion(targets("x86_64+avx2+fma", "x86_64+avx", "x86_64+sse4.2"))]
fn t3_steady_loop(
    inputs: &[f64],
    k: [f64; 6],
    state: &mut [f64; 6],
    coefficients: [f64; 4],
    output: &mut Vec<f64>,
) -> f64 {
    let [k1, k2, k3, k4, k5, k6] = k;
    let [c1, c2, c3, c4] = coefficients;
    let [mut e1, mut e2, mut e3, mut e4, mut e5, mut e6] = *state;
    let mut last = f64::NAN;
    for &input in inputs {
        e1 = k1.mul_add(input - e1, e1);
        e2 = k2.mul_add(e1 - e2, e2);
        e3 = k3.mul_add(e2 - e3, e3);
        e4 = k4.mul_add(e3 - e4, e4);
        e5 = k5.mul_add(e4 - e5, e5);
        e6 = k6.mul_add(e5 - e6, e6);
        last = c1 * e6 + c2 * e5 + c3 * e4 + c4 * e3;
        output.push(last);
    }
    *state = [e1, e2, e3, e4, e5, e6];
    last
}

/// Compute the triple exponential average result for the supplied aligned series.
///
/// # Parameters
///
/// * `input` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
/// * `v_factor` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn triple_exponential_average(
    input: &[f64],
    timeperiod: usize,
    v_factor: f64,
) -> TaResult<Vec<f64>> {
    let mut state = TripleExponentialAverage::new(timeperiod, v_factor)?;
    Ok(input
        .iter()
        .map(|&value| state.append(value).unwrap_or(f64::NAN))
        .collect())
}

/// Incremental TripleExponentialAverage with constant work and storage per appended bar.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `TripleExponentialAverage`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct TripleExponentialAverage {
    ema1: ExponentialMovingAverage,
    ema2: ExponentialMovingAverage,
    ema3: ExponentialMovingAverage,
    ema4: ExponentialMovingAverage,
    ema5: ExponentialMovingAverage,
    ema6: ExponentialMovingAverage,
    c1: f64,
    c2: f64,
    c3: f64,
    c4: f64,
    value: Option<f64>,
}

impl TripleExponentialAverage {
    /// Creates a TripleExponentialAverage state with a period of at least two bars.
    pub fn new(period: usize, v_factor: f64) -> TaResult<Self> {
        if period < 2 {
            return Err(TaError::InvalidParameter {
                name: "timeperiod",
                value: period.to_string(),
                reason: "must be >= 2 for TripleExponentialAverage",
            });
        }
        let v2 = v_factor * v_factor;
        let v3 = v2 * v_factor;
        Ok(Self {
            ema1: ExponentialMovingAverage::new(period)?,
            ema2: ExponentialMovingAverage::new(period)?,
            ema3: ExponentialMovingAverage::new(period)?,
            ema4: ExponentialMovingAverage::new(period)?,
            ema5: ExponentialMovingAverage::new(period)?,
            ema6: ExponentialMovingAverage::new(period)?,
            c1: -v3,
            c2: 3.0 * v2 + 3.0 * v3,
            c3: -6.0 * v2 - 3.0 * v_factor - 3.0 * v3,
            c4: 1.0 + 3.0 * v_factor + v3 + 3.0 * v2,
            value: None,
        })
    }
}

impl StreamingIndicator for TripleExponentialAverage {
    type Output = f64;

    /// Bulk kernel: advances all six EMA recurrences in one loop with the
    /// scalar states held in locals. Bit-identical to per-bar [`Self::append`]
    /// in both outputs and post-run streaming state.
    fn extend_slice_into(&mut self, inputs: &[f64], output: &mut Vec<f64>) {
        output.reserve(inputs.len());
        let mut index = 0;
        // Warm-up prologue: per-bar appends until the last EMA in the cascade
        // is seeded (each sub-EMA seeds from the SMA of its own input stream).
        while index < inputs.len() && self.ema6.current().is_none() {
            output.push(self.append(inputs[index]).unwrap_or(f64::NAN));
            index += 1;
        }
        if index == inputs.len() {
            return;
        }

        let k = [
            self.ema1.smoothing(),
            self.ema2.smoothing(),
            self.ema3.smoothing(),
            self.ema4.smoothing(),
            self.ema5.smoothing(),
            self.ema6.smoothing(),
        ];
        let mut state = [
            self.ema1.current().expect("warm EMA1"),
            self.ema2.current().expect("warm EMA2"),
            self.ema3.current().expect("warm EMA3"),
            self.ema4.current().expect("warm EMA4"),
            self.ema5.current().expect("warm EMA5"),
            self.ema6.current().expect("warm EMA6"),
        ];
        let coefficients = [self.c1, self.c2, self.c3, self.c4];
        let last = t3_steady_loop(&inputs[index..], k, &mut state, coefficients, output);

        let appended = inputs.len() - index;
        self.ema1.store_bulk_state(state[0], appended);
        self.ema2.store_bulk_state(state[1], appended);
        self.ema3.store_bulk_state(state[2], appended);
        self.ema4.store_bulk_state(state[3], appended);
        self.ema5.store_bulk_state(state[4], appended);
        self.ema6.store_bulk_state(state[5], appended);
        self.value = Some(last);
    }

    fn append(&mut self, input: f64) -> Option<f64> {
        let Some(e1) = self.ema1.append(input) else {
            return None;
        };
        let Some(e2) = self.ema2.append(e1) else {
            return None;
        };
        let Some(e3) = self.ema3.append(e2) else {
            return None;
        };
        let Some(e4) = self.ema4.append(e3) else {
            return None;
        };
        let Some(e5) = self.ema5.append(e4) else {
            return None;
        };
        let Some(e6) = self.ema6.append(e5) else {
            return None;
        };
        self.value = Some(self.c1 * e6 + self.c2 * e5 + self.c3 * e4 + self.c4 * e3);
        self.value
    }

    fn value(&self) -> Option<f64> {
        self.value
    }

    fn reset(&mut self) {
        self.ema1.reset();
        self.ema2.reset();
        self.ema3.reset();
        self.ema4.reset();
        self.ema5.reset();
        self.ema6.reset();
        self.value = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lcg_series(n: usize, mut state: u64) -> Vec<f64> {
        (0..n)
            .map(|_| {
                state = state
                    .wrapping_mul(6364136223846793005)
                    .wrapping_add(1442695040888963407);
                90.0 + (state >> 11) as f64 / (1u64 << 53) as f64 * 20.0
            })
            .collect()
    }

    fn assert_same_bits(actual: &[f64], expected: &[f64], label: &str) {
        assert_eq!(actual.len(), expected.len(), "{label}: length");
        for (i, (a, b)) in actual.iter().zip(expected).enumerate() {
            assert_eq!(a.to_bits(), b.to_bits(), "{label}: bar {i}");
        }
    }

    #[test]
    fn bulk_is_bitwise_identical_to_per_bar_append() {
        let input = lcg_series(5_000, 0x5EED_7301);
        let tail = lcg_series(128, 0x7A11_7301);
        for (period, v_factor) in [(2usize, 0.7), (3, 0.0), (5, 1.0), (14, 0.7), (30, 0.2)] {
            let mut per_bar = TripleExponentialAverage::new(period, v_factor).unwrap();
            let reference: Vec<f64> = input
                .iter()
                .map(|&x| per_bar.append(x).unwrap_or(f64::NAN))
                .collect();
            let tail_reference: Vec<f64> = tail
                .iter()
                .map(|&x| per_bar.append(x).unwrap_or(f64::NAN))
                .collect();

            for chunk in [usize::MAX, 1, 7, 97] {
                let mut state = TripleExponentialAverage::new(period, v_factor).unwrap();
                let mut out = Vec::new();
                for piece in input.chunks(chunk.min(input.len())) {
                    state.extend_slice_into(piece, &mut out);
                }
                assert_same_bits(&out, &reference, &format!("p{period} chunk {chunk}"));
                let tail_out: Vec<f64> = tail
                    .iter()
                    .map(|&x| state.append(x).unwrap_or(f64::NAN))
                    .collect();
                assert_same_bits(
                    &tail_out,
                    &tail_reference,
                    &format!("p{period} chunk {chunk} tail"),
                );
            }
        }
    }

    #[test]
    fn matches_batch_and_reset_replay() {
        let input: Vec<f64> = (0..200)
            .map(|index| 100.0 + (index as f64 * 0.23).sin() * 9.0 + index as f64 * 0.04)
            .collect();
        let expected = triple_exponential_average(&input, 7, 0.7).unwrap();
        let mut state = TripleExponentialAverage::new(7, 0.7).unwrap();
        for (&input, expected) in input.iter().zip(expected) {
            let actual = state.append(input);
            if expected.is_nan() {
                assert_eq!(actual, None);
            } else {
                assert!((actual.unwrap() - expected).abs() < 1e-12);
            }
        }
        let final_value = state.value();
        state.reset();
        for input in input {
            state.append(input);
        }
        assert_eq!(state.value(), final_value);
    }
}
