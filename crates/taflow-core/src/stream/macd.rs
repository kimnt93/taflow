//! Stateful Moving Average Convergence/Divergence.
//!
//! MACD aligns the fast EMA seed to the end of the slow EMA seed window, then
//! seeds the signal EMA from the first `signal_period` MACD observations.

use multiversion::multiversion;

use crate::error::{TaError, TaResult};

/// The three values produced by a warmed MACD state machine.
#[derive(Debug, Clone, Copy, PartialEq)]
/// Persistent Rust state or aligned output type for `MovingAverageConvergenceDivergenceValue`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct MovingAverageConvergenceDivergenceValue {
    pub macd: f64,
    pub signal: f64,
    pub histogram: f64,
}

/// Steady-state kernel shared by the MACD, MACDFIX and MACDEXT bulk paths:
/// the fast, slow and signal EMA recurrences advanced in one loop.
///
/// It lives in a free function so it can carry `#[multiversion]` (a trait or
/// inherent method cannot). Without runtime dispatch a portable build lowers
/// every `mul_add` to a libm `fma()` call; `mul_add` is an explicitly fused
/// operation either way, so the dispatched FMA variant is bit-identical.
#[allow(unexpected_cfgs)]
#[multiversion(targets("x86_64+avx2+fma", "x86_64+avx", "x86_64+sse4.2"))]
pub(crate) fn macd_ema_steady_loop(
    inputs: &[f64],
    k: [f64; 3],
    state: &mut [f64; 3],
    macd_out: &mut Vec<f64>,
    signal_out: &mut Vec<f64>,
    histogram_out: &mut Vec<f64>,
) -> Option<MovingAverageConvergenceDivergenceValue> {
    let [fast_k, slow_k, signal_k] = k;
    let [mut fast, mut slow, mut signal] = *state;
    let mut last = None;
    for &input in inputs {
        fast = fast_k.mul_add(input - fast, fast);
        slow = slow_k.mul_add(input - slow, slow);
        let macd = fast - slow;
        signal = signal_k.mul_add(macd - signal, signal);
        let histogram = macd - signal;
        macd_out.push(macd);
        signal_out.push(signal);
        histogram_out.push(histogram);
        last = Some(MovingAverageConvergenceDivergenceValue {
            macd,
            signal,
            histogram,
        });
    }
    *state = [fast, slow, signal];
    last
}

/// Stateful MACD matching the batch function's aligned EMA seeds.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `MovingAverageConvergenceDivergence`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct MovingAverageConvergenceDivergence {
    fast_period: usize,
    slow_period: usize,
    signal_period: usize,
    warmup: Vec<f64>,
    fast_k: f64,
    slow_k: f64,
    signal_k: f64,
    fast_ema: Option<f64>,
    slow_ema: Option<f64>,
    signal_count: usize,
    signal_sum: f64,
    signal_ema: Option<f64>,
    value: Option<MovingAverageConvergenceDivergenceValue>,
}

impl MovingAverageConvergenceDivergence {
    /// Creates a MACD state with TA-Lib-compatible periods.
    pub fn new(fast_period: usize, slow_period: usize, signal_period: usize) -> TaResult<Self> {
        if fast_period < 2 || slow_period < 2 || signal_period == 0 {
            return Err(TaError::InvalidParameter {
                name: "fastperiod/slowperiod/signalperiod",
                value: format!("{fast_period}/{slow_period}/{signal_period}"),
                reason: "fastperiod >= 2, slowperiod >= 2, signalperiod >= 1",
            });
        }
        let (fast_period, slow_period) = if fast_period < slow_period {
            (fast_period, slow_period)
        } else {
            (slow_period, fast_period)
        };
        Ok(Self {
            fast_period,
            slow_period,
            signal_period,
            warmup: Vec::with_capacity(slow_period),
            fast_k: 2.0 / (fast_period as f64 + 1.0),
            slow_k: 2.0 / (slow_period as f64 + 1.0),
            signal_k: 2.0 / (signal_period as f64 + 1.0),
            fast_ema: None,
            slow_ema: None,
            signal_count: 0,
            signal_sum: 0.0,
            signal_ema: None,
            value: None,
        })
    }

    /// Appends one close value.
    pub fn append(&mut self, input: f64) -> Option<MovingAverageConvergenceDivergenceValue> {
        let macd = match (self.fast_ema, self.slow_ema) {
            (Some(fast), Some(slow)) => {
                let fast = self.fast_k.mul_add(input - fast, fast);
                let slow = self.slow_k.mul_add(input - slow, slow);
                self.fast_ema = Some(fast);
                self.slow_ema = Some(slow);
                fast - slow
            }
            _ => {
                self.warmup.push(input);
                if self.warmup.len() < self.slow_period {
                    return None;
                }
                let slow = self.warmup.iter().sum::<f64>() / self.slow_period as f64;
                let fast = self.warmup[self.slow_period - self.fast_period..]
                    .iter()
                    .sum::<f64>()
                    / self.fast_period as f64;
                self.fast_ema = Some(fast);
                self.slow_ema = Some(slow);
                fast - slow
            }
        };

        self.signal_count += 1;
        let signal = if self.signal_count < self.signal_period {
            self.signal_sum += macd;
            return None;
        } else if self.signal_count == self.signal_period {
            let seed = (self.signal_sum + macd) / self.signal_period as f64;
            self.signal_ema = Some(seed);
            seed
        } else {
            let previous = self.signal_ema.expect("signal EMA is seeded before use");
            let next = self.signal_k.mul_add(macd - previous, previous);
            self.signal_ema = Some(next);
            next
        };
        self.value = Some(MovingAverageConvergenceDivergenceValue {
            macd,
            signal,
            histogram: macd - signal,
        });
        self.value
    }

    /// Bulk kernel: advances the fast, slow, and signal EMA recurrences in one
    /// loop with the scalar states held in locals, writing NaN during warm-up.
    /// Bit-identical to per-bar [`Self::append`] in outputs and post-run state.
    pub fn extend_slices_into(
        &mut self,
        inputs: &[f64],
        macd_out: &mut Vec<f64>,
        signal_out: &mut Vec<f64>,
        histogram_out: &mut Vec<f64>,
    ) {
        macd_out.reserve(inputs.len());
        signal_out.reserve(inputs.len());
        histogram_out.reserve(inputs.len());
        let mut index = 0;
        // Warm-up prologue: per-bar appends until the signal EMA is seeded.
        while index < inputs.len() && self.signal_ema.is_none() {
            match self.append(inputs[index]) {
                Some(value) => {
                    macd_out.push(value.macd);
                    signal_out.push(value.signal);
                    histogram_out.push(value.histogram);
                }
                None => {
                    macd_out.push(f64::NAN);
                    signal_out.push(f64::NAN);
                    histogram_out.push(f64::NAN);
                }
            }
            index += 1;
        }
        if index == inputs.len() {
            return;
        }

        let k = [self.fast_k, self.slow_k, self.signal_k];
        let mut state = [
            self.fast_ema.expect("warm fast EMA"),
            self.slow_ema.expect("warm slow EMA"),
            self.signal_ema.expect("warm signal EMA"),
        ];
        let last = macd_ema_steady_loop(
            &inputs[index..],
            k,
            &mut state,
            macd_out,
            signal_out,
            histogram_out,
        )
        .or(self.value);

        self.fast_ema = Some(state[0]);
        self.slow_ema = Some(state[1]);
        self.signal_ema = Some(state[2]);
        self.signal_count += inputs.len() - index;
        self.value = last;
    }

    /// Returns the latest warmed output.
    pub fn value(&self) -> Option<MovingAverageConvergenceDivergenceValue> {
        self.value
    }

    /// Restores the post-construction state while retaining warm-up capacity.
    pub fn reset(&mut self) {
        self.warmup.clear();
        self.fast_ema = None;
        self.slow_ema = None;
        self.signal_count = 0;
        self.signal_sum = 0.0;
        self.signal_ema = None;
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

    fn per_bar_outputs(
        state: &mut MovingAverageConvergenceDivergence,
        input: &[f64],
    ) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
        let mut macd = Vec::new();
        let mut signal = Vec::new();
        let mut histogram = Vec::new();
        for &x in input {
            match state.append(x) {
                Some(v) => {
                    macd.push(v.macd);
                    signal.push(v.signal);
                    histogram.push(v.histogram);
                }
                None => {
                    macd.push(f64::NAN);
                    signal.push(f64::NAN);
                    histogram.push(f64::NAN);
                }
            }
        }
        (macd, signal, histogram)
    }

    #[test]
    fn bulk_is_bitwise_identical_to_per_bar_append() {
        let input = lcg_series(5_000, 0x5EED_0A0D);
        let tail = lcg_series(128, 0x7A11_0A0D);
        for (fast, slow, signal) in [(12usize, 26usize, 9usize), (2, 2, 1), (3, 10, 1), (5, 5, 4)] {
            let mut per_bar = MovingAverageConvergenceDivergence::new(fast, slow, signal).unwrap();
            let reference = per_bar_outputs(&mut per_bar, &input);
            let tail_reference = per_bar_outputs(&mut per_bar, &tail);

            for chunk in [usize::MAX, 1, 7, 97] {
                let mut state =
                    MovingAverageConvergenceDivergence::new(fast, slow, signal).unwrap();
                let (mut m, mut s, mut h) = (Vec::new(), Vec::new(), Vec::new());
                for piece in input.chunks(chunk.min(input.len())) {
                    state.extend_slices_into(piece, &mut m, &mut s, &mut h);
                }
                let label = format!("{fast}/{slow}/{signal} chunk {chunk}");
                assert_same_bits(&m, &reference.0, &label);
                assert_same_bits(&s, &reference.1, &label);
                assert_same_bits(&h, &reference.2, &label);
                let tail_out = per_bar_outputs(&mut state, &tail);
                assert_same_bits(&tail_out.0, &tail_reference.0, &format!("{label} tail"));
                assert_same_bits(&tail_out.1, &tail_reference.1, &format!("{label} tail"));
                assert_same_bits(&tail_out.2, &tail_reference.2, &format!("{label} tail"));
            }
        }
    }

    #[test]
    fn matches_batch_and_reset_replay() {
        let input: Vec<f64> = (0..300)
            .map(|index| 100.0 + (index as f64 * 0.21).sin() * 12.0 + index as f64 * 0.01)
            .collect();
        let expected =
            crate::stream::moving_average_convergence_divergence(&input, 12, 26, 9).unwrap();
        let mut state = MovingAverageConvergenceDivergence::new(12, 26, 9).unwrap();
        for (index, input) in input.iter().copied().enumerate() {
            match state.append(input) {
                Some(actual) => {
                    assert!((actual.macd - expected.0[index]).abs() < 1e-12);
                    assert!((actual.signal - expected.1[index]).abs() < 1e-12);
                    assert!((actual.histogram - expected.2[index]).abs() < 1e-12);
                }
                None => assert!(expected.0[index].is_nan()),
            }
        }
        let expected_final = state.value();
        state.reset();
        for input in input {
            state.append(input);
        }
        assert_eq!(state.value(), expected_final);
    }
}
// Batch Moving Average Convergence/Divergence.
//
// MACD aligns the fast EMA seed to the slow EMA seed window before applying
// the signal EMA, matching TA-Lib's dedicated MACD seeding convention.

/// Computes aligned MACD, signal, and histogram arrays.
///
/// # Parameters
///
/// * `input` - Chronological close-price series.
/// * Period parameters configure fast, slow, and signal averages.
///
/// # Returns
///
/// Three aligned arrays containing MACD, signal, and histogram values.
/// Multiversioned: the EMA recurrences below are `mul_add`-bound and a
/// portable build without runtime dispatch lowers each one to a libm
/// `fma()` call. `mul_add` is explicitly fused either way, so the
/// dispatched variants are bit-identical.
#[allow(unexpected_cfgs)]
#[multiversion(targets("x86_64+avx2+fma", "x86_64+avx", "x86_64+sse4.2"))]
pub fn moving_average_convergence_divergence(
    input: &[f64],
    fastperiod: usize,
    slowperiod: usize,
    signalperiod: usize,
) -> TaResult<(Vec<f64>, Vec<f64>, Vec<f64>)> {
    if fastperiod < 2 || slowperiod < 2 || signalperiod < 1 {
        return Err(TaError::InvalidParameter {
            name: "fastperiod/slowperiod/signalperiod",
            value: format!("{}/{}/{}", fastperiod, slowperiod, signalperiod),
            reason: "fastperiod >= 2, slowperiod >= 2, signalperiod >= 1",
        });
    }

    // Ensure slow is greater than fast.
    let (fp, sp) = if fastperiod < slowperiod {
        (fastperiod, slowperiod)
    } else {
        (slowperiod, fastperiod)
    };

    let len = input.len();
    let lookback = sp - 1 + signalperiod - 1;
    if len <= lookback {
        return Err(TaError::InsufficientData {
            need: lookback + 1,
            got: len,
        });
    }

    let k_fast = 2.0 / (fp as f64 + 1.0);
    let k_slow = 2.0 / (sp as f64 + 1.0);
    let k_signal = 2.0 / (signalperiod as f64 + 1.0);

    // C TA-Lib MACD internal EMA calculation:
    // slow seed = SMA(close[0..sp]), fast seed = SMA(close[sp-fp..sp])
    let slow_seed: f64 = input[..sp].iter().sum::<f64>() / sp as f64;
    let fast_seed: f64 = input[sp - fp..sp].iter().sum::<f64>() / fp as f64;

    // MACD line: first value (bar sp-1) = fast_seed - slow_seed.
    // Recurrence starts at bar sp.
    let mut macd_values = Vec::with_capacity(len - sp + 1);
    macd_values.push(fast_seed - slow_seed);

    let mut slow_ema = slow_seed;
    let mut fast_ema = fast_seed;
    for i in sp..len {
        slow_ema = k_slow.mul_add(input[i] - slow_ema, slow_ema);
        fast_ema = k_fast.mul_add(input[i] - fast_ema, fast_ema);
        macd_values.push(fast_ema - slow_ema);
    }

    // Signal line = EMA(macd_values, signalperiod)
    // seed = SMA(macd_values[0..signalperiod])
    let signal_seed: f64 = macd_values[..signalperiod].iter().sum::<f64>() / signalperiod as f64;

    // Build the output.
    let out_start = sp - 1 + signalperiod - 1; // = lookback
    let mut macd_line = vec![0.0_f64; len];
    macd_line[..out_start].fill(f64::NAN);
    let mut signal_line = vec![0.0_f64; len];
    signal_line[..out_start].fill(f64::NAN);
    let mut histogram = vec![0.0_f64; len];
    histogram[..out_start].fill(f64::NAN);

    // The first signal value corresponds to macd_values[signalperiod-1], bar out_start.
    let mut signal_ema = signal_seed;
    let macd_at_out_start = macd_values[signalperiod - 1];
    macd_line[out_start] = macd_at_out_start;
    signal_line[out_start] = signal_seed;
    histogram[out_start] = macd_at_out_start - signal_seed;

    for i in signalperiod..macd_values.len() {
        let bar = sp - 1 + i;
        signal_ema = k_signal.mul_add(macd_values[i] - signal_ema, signal_ema);
        macd_line[bar] = macd_values[i];
        signal_line[bar] = signal_ema;
        histogram[bar] = macd_values[i] - signal_ema;
    }

    Ok((macd_line, signal_line, histogram))
}
