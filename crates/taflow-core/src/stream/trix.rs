//! Incremental Triple Exponential Average Rate of Change (TRIX).

use multiversion::multiversion;

use crate::error::TaResult;

use super::{invalid_period, ExponentialMovingAverage, StreamingIndicator};

/// Steady-state kernel for the triple EMA cascade plus the ROC step.
///
/// Extracted from [`TripleExponentialRateOfChange::extend_slice_into`] so it
/// can carry `#[multiversion]`; without runtime dispatch a portable build
/// lowers each `mul_add` to a libm `fma()` call. `mul_add` is explicitly fused
/// either way, so the dispatched variants are bit-identical.
#[allow(unexpected_cfgs)]
#[multiversion(targets("x86_64+avx2+fma", "x86_64+avx", "x86_64+sse4.2"))]
fn trix_steady_loop(
    inputs: &[f64],
    k: [f64; 3],
    state: &mut [f64; 3],
    output: &mut Vec<f64>,
) -> Option<f64> {
    let [k1, k2, k3] = k;
    let [mut e1, mut e2, mut e3] = *state;
    let mut last = None;
    for &input in inputs {
        e1 = k1.mul_add(input - e1, e1);
        e2 = k2.mul_add(e1 - e2, e2);
        let previous = e3;
        e3 = k3.mul_add(e2 - e3, e3);
        let value = if previous != 0.0 {
            (e3 - previous) / previous * 100.0
        } else {
            0.0
        };
        output.push(value);
        last = Some(value);
    }
    *state = [e1, e2, e3];
    last
}

/// Persistent TRIX with a triple TA-Lib-seeded EMA cascade and O(1) updates.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `TripleExponentialRateOfChange`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct TripleExponentialRateOfChange {
    ema1: ExponentialMovingAverage,
    ema2: ExponentialMovingAverage,
    ema3: ExponentialMovingAverage,
    previous_ema3: Option<f64>,
    value: Option<f64>,
}

impl TripleExponentialRateOfChange {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(period: usize) -> TaResult<Self> {
        if period < 2 {
            return Err(invalid_period("timeperiod", period, 2));
        }
        Ok(Self {
            ema1: ExponentialMovingAverage::new(period)?,
            ema2: ExponentialMovingAverage::new(period)?,
            ema3: ExponentialMovingAverage::new(period)?,
            previous_ema3: None,
            value: None,
        })
    }
}

impl StreamingIndicator for TripleExponentialRateOfChange {
    type Output = f64;

    /// Bulk kernel: advances the triple EMA cascade and the ROC step in one
    /// loop with the scalar states held in locals. Bit-identical to per-bar
    /// [`Self::append`] in outputs and post-run streaming state.
    fn extend_slice_into(&mut self, inputs: &[f64], output: &mut Vec<f64>) {
        output.reserve(inputs.len());
        let mut index = 0;
        // Warm-up prologue: per-bar appends until EMA3 has produced a value
        // and it has been latched as the previous ROC reference.
        while index < inputs.len() && self.previous_ema3.is_none() {
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
        ];
        let mut state = [
            self.ema1.current().expect("warm EMA1"),
            self.ema2.current().expect("warm EMA2"),
            self.ema3.current().expect("warm EMA3"),
        ];
        let last = trix_steady_loop(&inputs[index..], k, &mut state, output).or(self.value);

        let appended = inputs.len() - index;
        self.ema1.store_bulk_state(state[0], appended);
        self.ema2.store_bulk_state(state[1], appended);
        self.ema3.store_bulk_state(state[2], appended);
        self.previous_ema3 = Some(state[2]);
        self.value = last;
    }

    fn append(&mut self, input: f64) -> Option<f64> {
        let e1 = self.ema1.append(input)?;
        let e2 = self.ema2.append(e1)?;
        let e3 = self.ema3.append(e2)?;
        let previous = self.previous_ema3.replace(e3)?;
        self.value = Some(if previous != 0.0 {
            (e3 - previous) / previous * 100.0
        } else {
            0.0
        });
        self.value
    }

    fn value(&self) -> Option<f64> {
        self.value
    }

    fn reset(&mut self) {
        self.ema1.reset();
        self.ema2.reset();
        self.ema3.reset();
        self.previous_ema3 = None;
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
        let input = lcg_series(5_000, 0x5EED_7212);
        let tail = lcg_series(128, 0x7A11_7212);
        for period in [2usize, 3, 14, 30] {
            let mut per_bar = TripleExponentialRateOfChange::new(period).unwrap();
            let reference: Vec<f64> = input
                .iter()
                .map(|&x| per_bar.append(x).unwrap_or(f64::NAN))
                .collect();
            let tail_reference: Vec<f64> = tail
                .iter()
                .map(|&x| per_bar.append(x).unwrap_or(f64::NAN))
                .collect();

            for chunk in [usize::MAX, 1, 7, 97] {
                let mut state = TripleExponentialRateOfChange::new(period).unwrap();
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
    fn matches_batch_for_chunked_extend() {
        let input: Vec<f64> = (0..96)
            .map(|i| 100.0 + i as f64 * 0.2 + (i as f64 * 0.3).sin())
            .collect();
        let expected = crate::stream::triple_exponential_rate_of_change(&input, 7).unwrap();
        let mut state = TripleExponentialRateOfChange::new(7).unwrap();
        let mut actual = state.extend(input[..43].iter().copied());
        actual.extend(state.extend(input[43..].iter().copied()));
        for (actual, expected) in actual.iter().zip(&expected) {
            match actual {
                Some(actual) => assert!((actual - expected).abs() < 1e-10),
                None => assert!(expected.is_nan()),
            }
        }
    }
}
use crate::error::TaError;
use crate::simd::sum_f64;

/// TRIX — rate of change of a triple-exponentially smoothed series.
///
/// TRIX = ROC(EMA(EMA(EMA(input))))
/// lookback = 3*(timeperiod-1) + 1
///
/// Optimized version: three scalar EMA stages plus ROC, with one output Vec
/// and no intermediate allocation.
/// Compute the triple exponential rate of change result for the supplied aligned series.
///
/// # Parameters
///
/// * `input` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
/// Multiversioned: the EMA recurrences below are `mul_add`-bound and a
/// portable build without runtime dispatch lowers each one to a libm
/// `fma()` call. `mul_add` is explicitly fused either way, so the
/// dispatched variants are bit-identical.
#[allow(unexpected_cfgs)]
#[multiversion(targets("x86_64+avx2+fma", "x86_64+avx", "x86_64+sse4.2"))]
pub fn triple_exponential_rate_of_change(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    if timeperiod < 2 {
        return Err(TaError::InvalidParameter {
            name: "timeperiod",
            value: timeperiod.to_string(),
            reason: "must be >= 2",
        });
    }
    let len = input.len();
    let lookback = 3 * (timeperiod - 1) + 1;
    if len <= lookback {
        return Err(TaError::InsufficientData {
            need: lookback + 1,
            got: len,
        });
    }

    let k = 2.0 / (timeperiod as f64 + 1.0);
    let p = timeperiod - 1;
    let tp = timeperiod as f64;

    let mut output = vec![0.0_f64; len];
    output[..lookback].fill(f64::NAN);

    // Phase 1: Build EMA1, indices [p .. 2p]. Accumulate SMA for EMA2 seed.
    let seed1 = sum_f64(&input[..timeperiod]) / tp;
    let mut e1 = seed1;
    let mut sum2 = seed1;
    for i in timeperiod..(2 * p + 1) {
        e1 = k.mul_add(input[i] - e1, e1);
        sum2 += e1;
    }

    // Phase 2: Build EMA2, indices [2p .. 3p]. Accumulate SMA for EMA3 seed.
    let seed2 = sum2 / tp;
    let mut e2 = seed2;
    let mut sum3 = seed2;
    for i in (2 * p + 1)..(3 * p + 1) {
        e1 = k.mul_add(input[i] - e1, e1);
        e2 = k.mul_add(e1 - e2, e2);
        sum3 += e2;
    }

    // Phase 3: EMA3 seed ready. First EMA3 value at index 3*p.
    // ROC needs previous EMA3 value, so first output at index 3*p + 1 = lookback.
    let seed3 = sum3 / tp;
    let mut e3_prev = seed3;

    // Compute one more step to get e3 at index 3*p + 1
    let i = 3 * p + 1;
    e1 = k.mul_add(input[i] - e1, e1);
    e2 = k.mul_add(e1 - e2, e2);
    let e3_cur = k.mul_add(e2 - e3_prev, e3_prev);
    if e3_prev != 0.0 {
        output[lookback] = ((e3_cur - e3_prev) / e3_prev) * 100.0;
    }
    e3_prev = e3_cur;

    // Steady state: cascade all 3 EMA layers + ROC
    for i in (lookback + 1)..len {
        e1 = k.mul_add(input[i] - e1, e1);
        e2 = k.mul_add(e1 - e2, e2);
        let e3_cur = k.mul_add(e2 - e3_prev, e3_prev);
        if e3_prev != 0.0 {
            output[i] = ((e3_cur - e3_prev) / e3_prev) * 100.0;
        }
        e3_prev = e3_cur;
    }

    Ok(output)
}
