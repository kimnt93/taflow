//! Stateful extended Moving Average Convergence/Divergence.
//!
//! MACDEXT delays each fast/slow input stream by the difference between its
//! own lookback and the shared largest lookback, then feeds synchronized
//! differences through the selected signal moving average.

use crate::error::{TaError, TaResult};
use crate::ma_type::MaType;

use super::macd::macd_ema_steady_loop;
use super::{
    moving_average::MovingAverageDispatcher, MovingAverageConvergenceDivergenceValue,
    StreamingIndicator,
};

/// Incremental MACDEXT with aligned fast/slow seeds.
/// Persistent Rust state or aligned output type for `MovingAverageConvergenceDivergenceExtended`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct MovingAverageConvergenceDivergenceExtended {
    fast: MovingAverageDispatcher,
    slow: MovingAverageDispatcher,
    signal: MovingAverageDispatcher,
    fast_start: usize,
    slow_start: usize,
    index: usize,
    value: Option<MovingAverageConvergenceDivergenceValue>,
}

impl MovingAverageConvergenceDivergenceExtended {
    /// Creates a MACDEXT state with independently selected MA types.
    pub fn new(
        fastperiod: usize,
        fastmatype: MaType,
        slowperiod: usize,
        slowmatype: MaType,
        signalperiod: usize,
        signalmatype: MaType,
    ) -> TaResult<Self> {
        if fastperiod < 2 || slowperiod < 2 || signalperiod == 0 {
            return Err(TaError::InvalidParameter {
                name: "fastperiod/slowperiod/signalperiod",
                value: format!("{fastperiod}/{slowperiod}/{signalperiod}"),
                reason: "fastperiod >= 2, slowperiod >= 2, signalperiod >= 1",
            });
        }
        let (fastperiod, fastmatype, slowperiod, slowmatype) = if fastperiod < slowperiod {
            (fastperiod, fastmatype, slowperiod, slowmatype)
        } else {
            (slowperiod, slowmatype, fastperiod, fastmatype)
        };
        let fast_lookback = fastmatype.lookback(fastperiod);
        let slow_lookback = slowmatype.lookback(slowperiod);
        let largest_lookback = fast_lookback.max(slow_lookback);
        Ok(Self {
            fast: MovingAverageDispatcher::new(fastperiod, fastmatype)?,
            slow: MovingAverageDispatcher::new(slowperiod, slowmatype)?,
            signal: MovingAverageDispatcher::new(signalperiod, signalmatype)?,
            fast_start: largest_lookback - fast_lookback,
            slow_start: largest_lookback - slow_lookback,
            index: 0,
            value: None,
        })
    }

    /// Appends one close value.
    pub fn append(&mut self, input: f64) -> Option<MovingAverageConvergenceDivergenceValue> {
        let index = self.index;
        self.index += 1;
        let fast = if index >= self.fast_start {
            self.fast.append(input)
        } else {
            None
        };
        let slow = if index >= self.slow_start {
            self.slow.append(input)
        } else {
            None
        };
        self.value = fast.zip(slow).and_then(|(fast, slow)| {
            let macd = fast - slow;
            self.signal
                .append(macd)
                .map(|signal| MovingAverageConvergenceDivergenceValue {
                    macd,
                    signal,
                    histogram: macd - signal,
                })
        });
        self.value
    }

    /// Bulk kernel. When all three moving averages are plain EMAs (the common
    /// TA-Lib default), the warm steady state advances the three EMA
    /// recurrences in one loop with the scalar states held in locals; other
    /// MA types fall back to a per-bar loop with no per-bar allocation.
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
        if self.fast.is_sma() && self.slow.is_sma() && self.signal.is_sma() {
            self.extend_slices_fused_sma(inputs, macd_out, signal_out, histogram_out);
            return;
        }
        let fused = self.fast.is_ema() && self.slow.is_ema() && self.signal.is_ema();
        let mut index = 0;
        if fused {
            // Warm-up prologue: per-bar appends until the signal EMA emits.
            while index < inputs.len() && self.value.is_none() {
                Self::push_outputs(
                    self.append(inputs[index]),
                    macd_out,
                    signal_out,
                    histogram_out,
                );
                index += 1;
            }
            if index < inputs.len() {
                let (fast_k, mut fast) = {
                    let state = self.fast.as_ema_mut().expect("EMA fast state");
                    (state.smoothing(), state.current().expect("warm fast EMA"))
                };
                let (slow_k, mut slow) = {
                    let state = self.slow.as_ema_mut().expect("EMA slow state");
                    (state.smoothing(), state.current().expect("warm slow EMA"))
                };
                let (signal_k, mut signal) = {
                    let state = self.signal.as_ema_mut().expect("EMA signal state");
                    (state.smoothing(), state.current().expect("warm signal EMA"))
                };
                let mut ema_state = [fast, slow, signal];
                let last = macd_ema_steady_loop(
                    &inputs[index..],
                    [fast_k, slow_k, signal_k],
                    &mut ema_state,
                    macd_out,
                    signal_out,
                    histogram_out,
                )
                .or(self.value);
                [fast, slow, signal] = ema_state;
                let appended = inputs.len() - index;
                self.fast
                    .as_ema_mut()
                    .expect("EMA fast state")
                    .store_bulk_state(fast, appended);
                self.slow
                    .as_ema_mut()
                    .expect("EMA slow state")
                    .store_bulk_state(slow, appended);
                self.signal
                    .as_ema_mut()
                    .expect("EMA signal state")
                    .store_bulk_state(signal, appended);
                self.index += appended;
                self.value = last;
            }
            return;
        }
        for &input in &inputs[index..] {
            Self::push_outputs(self.append(input), macd_out, signal_out, histogram_out);
        }
    }

    /// Fused bulk kernel for the all-SMA configuration - TA-Lib's `MACDEXT`
    /// default, and therefore the path the benchmark actually exercises.
    ///
    /// The fast and slow legs become sliding sums indexed straight off the
    /// input slice, with the same `sum -= old; sum += input` statement order
    /// `SimpleMovingAverage::append` uses. The signal leg keeps its own ring,
    /// because its input is the MACD line rather than the price slice. Outputs
    /// and post-run state are bit-identical to per-bar [`Self::append`].
    fn extend_slices_fused_sma(
        &mut self,
        inputs: &[f64],
        macd_out: &mut Vec<f64>,
        signal_out: &mut Vec<f64>,
        histogram_out: &mut Vec<f64>,
    ) {
        let fast_period = self.fast.as_sma_mut().expect("SMA fast state").period();
        let slow_period = self.slow.as_sma_mut().expect("SMA slow state").period();
        // `new` normalizes the periods, so `fast_period <= slow_period`. After
        // `slow_period` per-bar appends both price rings hold nothing but bars
        // of this slice (the fast leg's `fast_start` delay is at most
        // `slow_period - fast_period`, so it too is fully seeded from here),
        // and the evicted element of each is just `inputs[i - period]`.
        let n = inputs.len();
        let prologue = n.min(slow_period);
        for &input in &inputs[..prologue] {
            Self::push_outputs(self.append(input), macd_out, signal_out, histogram_out);
        }
        if n <= slow_period {
            return;
        }
        let mut fast_sum = self.fast.as_sma_mut().expect("SMA fast state").raw_sum();
        let mut slow_sum = self.slow.as_sma_mut().expect("SMA slow state").raw_sum();
        let fast_len = fast_period as f64;
        let slow_len = slow_period as f64;
        let mut last = self.value;
        {
            let signal = self.signal.as_sma_mut().expect("SMA signal state");
            for i in slow_period..n {
                fast_sum -= inputs[i - fast_period];
                fast_sum += inputs[i];
                slow_sum -= inputs[i - slow_period];
                slow_sum += inputs[i];
                let macd = fast_sum / fast_len - slow_sum / slow_len;
                last = signal
                    .append(macd)
                    .map(|signal| MovingAverageConvergenceDivergenceValue {
                        macd,
                        signal,
                        histogram: macd - signal,
                    });
                Self::push_outputs(last, macd_out, signal_out, histogram_out);
            }
        }
        MovingAverageDispatcher::restore_sma_leg(
            self.fast.as_sma_mut().expect("SMA fast state"),
            inputs,
            fast_sum,
        );
        MovingAverageDispatcher::restore_sma_leg(
            self.slow.as_sma_mut().expect("SMA slow state"),
            inputs,
            slow_sum,
        );
        self.index += n - slow_period;
        self.value = last;
    }

    #[inline]
    fn push_outputs(
        value: Option<MovingAverageConvergenceDivergenceValue>,
        macd_out: &mut Vec<f64>,
        signal_out: &mut Vec<f64>,
        histogram_out: &mut Vec<f64>,
    ) {
        match value {
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
    }

    /// Returns the latest warmed output.
    pub fn value(&self) -> Option<MovingAverageConvergenceDivergenceValue> {
        self.value
    }

    /// Restores the post-construction state.
    pub fn reset(&mut self) {
        self.fast.reset();
        self.slow.reset();
        self.signal.reset();
        self.index = 0;
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
        state: &mut MovingAverageConvergenceDivergenceExtended,
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
        let input = lcg_series(5_000, 0x5EED_E217);
        let tail = lcg_series(256, 0x7A11_E217);
        let ema = MaType::ExponentialMovingAverage;
        let combos = [
            (12usize, ema, 26usize, ema, 9usize, ema),
            (2, ema, 2, ema, 1, ema),
            (3, ema, 10, ema, 1, ema),
            (5, ema, 5, ema, 4, ema),
            // All-SMA legs exercise the fused SMA path (TA-Lib's default).
            (
                12,
                MaType::SimpleMovingAverage,
                26,
                MaType::SimpleMovingAverage,
                9,
                MaType::SimpleMovingAverage,
            ),
            (
                5,
                MaType::SimpleMovingAverage,
                5,
                MaType::SimpleMovingAverage,
                1,
                MaType::SimpleMovingAverage,
            ),
            (
                2,
                MaType::SimpleMovingAverage,
                7,
                MaType::SimpleMovingAverage,
                1,
                MaType::SimpleMovingAverage,
            ),
            (
                26,
                MaType::SimpleMovingAverage,
                12,
                MaType::SimpleMovingAverage,
                9,
                MaType::SimpleMovingAverage,
            ),
            // Mixed / non-EMA legs exercise the per-bar fallback path.
            (
                7,
                MaType::SimpleMovingAverage,
                13,
                ema,
                5,
                MaType::WeightedMovingAverage,
            ),
        ];
        for (fp, fmt, sp, smt, gp, gmt) in combos {
            let mut per_bar =
                MovingAverageConvergenceDivergenceExtended::new(fp, fmt, sp, smt, gp, gmt).unwrap();
            let reference = per_bar_outputs(&mut per_bar, &input);
            let tail_reference = per_bar_outputs(&mut per_bar, &tail);

            for chunk in [usize::MAX, 1, 7, 10, 97, 1_000] {
                let mut state =
                    MovingAverageConvergenceDivergenceExtended::new(fp, fmt, sp, smt, gp, gmt)
                        .unwrap();
                let (mut m, mut s, mut h) = (Vec::new(), Vec::new(), Vec::new());
                for piece in input.chunks(chunk.min(input.len())) {
                    state.extend_slices_into(piece, &mut m, &mut s, &mut h);
                }
                let label = format!("{fp}/{sp}/{gp} chunk {chunk}");
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
    fn matches_batch_for_every_moving_average_combination() {
        let input: Vec<f64> = (0..700)
            .map(|index| 100.0 + (index as f64 * 0.17).sin() * 8.0 + index as f64 * 0.01)
            .collect();
        for fast_code in 0..=8 {
            for slow_code in 0..=8 {
                for signal_code in 0..=8 {
                    let fast_type = MaType::try_from(fast_code).unwrap();
                    let slow_type = MaType::try_from(slow_code).unwrap();
                    let signal_type = MaType::try_from(signal_code).unwrap();
                    let expected = crate::stream::moving_average_convergence_divergence_extended(
                        &input,
                        7,
                        fast_type,
                        13,
                        slow_type,
                        5,
                        signal_type,
                    )
                    .unwrap();
                    let mut state = MovingAverageConvergenceDivergenceExtended::new(
                        7,
                        fast_type,
                        13,
                        slow_type,
                        5,
                        signal_type,
                    )
                    .unwrap();
                    for (index, input) in input.iter().copied().enumerate() {
                        match state.append(input) {
                            Some(actual) => {
                                assert!((actual.macd - expected.0[index]).abs() < 1e-8);
                                assert!((actual.signal - expected.1[index]).abs() < 1e-8);
                                assert!((actual.histogram - expected.2[index]).abs() < 1e-8);
                            }
                            None => assert!(expected.0[index].is_nan()),
                        }
                    }
                }
            }
        }
    }
}
// Batch extended Moving Average Convergence/Divergence.
//
// MACDEXT permits independently selected fast, slow, and signal moving
// averages. Fast and slow inputs are aligned to a shared largest lookback so
// their seeds reproduce TA-Lib's internal start-index calls.

use crate::ma_type::compute_ma;

fn ma_from_aligned_start(
    input: &[f64],
    start: usize,
    period: usize,
    ma_type: MaType,
) -> TaResult<Vec<f64>> {
    let lookback = ma_type.lookback(period);
    let source_start = start - lookback;
    let values = compute_ma(&input[source_start..], period, ma_type)?;
    Ok(values[lookback..].to_vec())
}

/// Computes aligned MACDEXT, signal, and histogram arrays.
///
/// # Parameters
///
/// * `input` - Chronological close-price series.
/// * Period and moving-average parameters configure each MACD leg.
///
/// # Returns
///
/// Three aligned arrays containing MACD, signal, and histogram values.
pub fn moving_average_convergence_divergence_extended(
    input: &[f64],
    fastperiod: usize,
    fastmatype: MaType,
    slowperiod: usize,
    slowmatype: MaType,
    signalperiod: usize,
    signalmatype: MaType,
) -> TaResult<(Vec<f64>, Vec<f64>, Vec<f64>)> {
    if fastperiod < 2 || slowperiod < 2 || signalperiod == 0 {
        return Err(TaError::InvalidParameter {
            name: "fastperiod/slowperiod/signalperiod",
            value: format!("{fastperiod}/{slowperiod}/{signalperiod}"),
            reason: "fastperiod >= 2, slowperiod >= 2, signalperiod >= 1",
        });
    }
    let (fp, fmt, sp, smt) = if fastperiod < slowperiod {
        (fastperiod, fastmatype, slowperiod, slowmatype)
    } else {
        (slowperiod, slowmatype, fastperiod, fastmatype)
    };

    if fastmatype == MaType::ExponentialMovingAverage
        && slowmatype == MaType::ExponentialMovingAverage
        && signalmatype == MaType::ExponentialMovingAverage
    {
        return super::macd::moving_average_convergence_divergence(
            input,
            fastperiod,
            slowperiod,
            signalperiod,
        );
    }

    let len = input.len();
    let largest_lookback = fmt.lookback(fp).max(smt.lookback(sp));
    let signal_lookback = signalmatype.lookback(signalperiod);
    let total_lookback = largest_lookback + signal_lookback;
    if len <= total_lookback {
        return Err(TaError::InsufficientData {
            need: total_lookback + 1,
            got: len,
        });
    }

    let fast_ma = ma_from_aligned_start(input, largest_lookback, fp, fmt)?;
    let slow_ma = ma_from_aligned_start(input, largest_lookback, sp, smt)?;
    let macd_valid: Vec<f64> = fast_ma
        .iter()
        .zip(slow_ma.iter())
        .map(|(fast, slow)| fast - slow)
        .collect();
    let signal_ma = compute_ma(&macd_valid, signalperiod, signalmatype)?;

    let mut macd_line = vec![f64::NAN; len];
    let mut signal_line = vec![f64::NAN; len];
    let mut histogram = vec![f64::NAN; len];
    for index in signal_lookback..signal_ma.len() {
        let bar = largest_lookback + index;
        macd_line[bar] = macd_valid[index];
        signal_line[bar] = signal_ma[index];
        histogram[bar] = macd_valid[index] - signal_ma[index];
    }
    Ok((macd_line, signal_line, histogram))
}
