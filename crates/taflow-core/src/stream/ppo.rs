//! Stateful Percentage Price Oscillator.
//!
//! PPO normalizes the distance between fast and slow moving averages by the
//! slow average and supports all nine TA-Lib moving-average types.

use multiversion::multiversion;

use crate::error::TaResult;
use crate::ma_type::MaType;

use super::{moving_average_dispatcher::MovingAverageDispatcher, StreamingIndicator};

/// Steady-state kernel for the fused fast/slow EMA legs.
///
/// Extracted from [`PercentagePriceOscillator::extend_slice_into`] so it can
/// carry `#[multiversion]`; a portable build without runtime dispatch lowers
/// each `mul_add` to a libm `fma()` call. `mul_add` is explicitly fused in both
/// cases, so the dispatched variants are bit-identical.
#[allow(unexpected_cfgs)]
#[multiversion(targets("x86_64+avx2+fma", "x86_64+avx", "x86_64+sse4.2"))]
fn ppo_ema_steady_loop(
    inputs: &[f64],
    fast_k: f64,
    slow_k: f64,
    state: &mut (f64, f64),
    output: &mut Vec<f64>,
) -> Option<f64> {
    let (mut fast, mut slow) = *state;
    let mut last = None;
    for &input in inputs {
        fast = fast_k.mul_add(input - fast, fast);
        slow = slow_k.mul_add(input - slow, slow);
        last = (slow != 0.0).then_some((fast - slow) / slow * 100.0);
        output.push(last.unwrap_or(f64::NAN));
    }
    *state = (fast, slow);
    last
}

/// Compute the percentage price oscillator result for the supplied aligned series.
///
/// # Parameters
///
/// * `input` - Input series or configuration value.
/// * `fastperiod` - Input series or configuration value.
/// * `slowperiod` - Input series or configuration value.
/// * `matype` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn percentage_price_oscillator(
    input: &[f64],
    fastperiod: usize,
    slowperiod: usize,
    matype: MaType,
) -> TaResult<Vec<f64>> {
    let mut state = PercentagePriceOscillator::new(fastperiod, slowperiod, matype)?;
    Ok(input
        .iter()
        .map(|&value| state.append(value).unwrap_or(f64::NAN))
        .collect())
}

/// Incremental PPO driven by two selected moving-average states.
/// Persistent Rust state or aligned output type for `PercentagePriceOscillator`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct PercentagePriceOscillator {
    fast: MovingAverageDispatcher,
    slow: MovingAverageDispatcher,
    value: Option<f64>,
}

impl PercentagePriceOscillator {
    /// Creates a PPO state; period order is normalized to fast then slow.
    pub fn new(fast_period: usize, slow_period: usize, ma_type: MaType) -> TaResult<Self> {
        let (fast_period, slow_period) = if fast_period < slow_period {
            (fast_period, slow_period)
        } else {
            (slow_period, fast_period)
        };
        Ok(Self {
            fast: MovingAverageDispatcher::new(fast_period, ma_type)?,
            slow: MovingAverageDispatcher::new(slow_period, ma_type)?,
            value: None,
        })
    }
}

impl StreamingIndicator for PercentagePriceOscillator {
    type Output = f64;

    /// Bulk kernel. For the EMA and SMA MA types (SMA is TA-Lib's `PPO`
    /// default) the warm steady state advances both recurrences in one loop
    /// with the scalar states held in locals; other MA types fall back to a
    /// per-bar loop with no per-bar allocation. Bit-identical to per-bar
    /// [`Self::append`] in outputs and post-run state.
    fn extend_slice_into(&mut self, inputs: &[f64], output: &mut Vec<f64>) {
        output.reserve(inputs.len());
        let mut index = 0;
        if self.fast.is_sma() && self.slow.is_sma() {
            let fast_period = self.fast.as_sma_mut().expect("SMA fast state").period();
            let slow_period = self.slow.as_sma_mut().expect("SMA slow state").period();
            // `new` normalizes the periods, so `fast_period <= slow_period` and
            // `slow_period` per-bar appends leave both rings holding nothing
            // but bars of this slice - after which the evicted element of each
            // is just `inputs[i - period]`.
            let n = inputs.len();
            let prologue = n.min(slow_period);
            for &input in &inputs[..prologue] {
                output.push(self.append(input).unwrap_or(f64::NAN));
            }
            if n <= slow_period {
                return;
            }
            let mut fast_sum = self.fast.as_sma_mut().expect("SMA fast state").raw_sum();
            let mut slow_sum = self.slow.as_sma_mut().expect("SMA slow state").raw_sum();
            let fast_len = fast_period as f64;
            let slow_len = slow_period as f64;
            let mut last = None;
            for i in slow_period..n {
                // Same statement order as `SimpleMovingAverage::append`
                // (`sum -= old` then `sum += input`) on both legs.
                fast_sum -= inputs[i - fast_period];
                fast_sum += inputs[i];
                slow_sum -= inputs[i - slow_period];
                slow_sum += inputs[i];
                let fast = fast_sum / fast_len;
                let slow = slow_sum / slow_len;
                last = (slow != 0.0).then_some((fast - slow) / slow * 100.0);
                output.push(last.unwrap_or(f64::NAN));
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
            self.value = last;
            return;
        }
        if self.fast.is_ema() && self.slow.is_ema() {
            // Warm-up prologue: per-bar appends until the slow EMA is seeded.
            // (`self.value` is not a warm gate here: it stays `None` whenever
            // the slow average is exactly zero.)
            while index < inputs.len()
                && self
                    .slow
                    .as_ema_mut()
                    .expect("EMA slow state")
                    .current()
                    .is_none()
            {
                output.push(self.append(inputs[index]).unwrap_or(f64::NAN));
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
                let mut ema_state = (fast, slow);
                let last =
                    ppo_ema_steady_loop(&inputs[index..], fast_k, slow_k, &mut ema_state, output);
                (fast, slow) = ema_state;
                let appended = inputs.len() - index;
                self.fast
                    .as_ema_mut()
                    .expect("EMA fast state")
                    .store_bulk_state(fast, appended);
                self.slow
                    .as_ema_mut()
                    .expect("EMA slow state")
                    .store_bulk_state(slow, appended);
                self.value = last;
            }
            return;
        }
        for &input in &inputs[index..] {
            output.push(self.append(input).unwrap_or(f64::NAN));
        }
    }

    fn append(&mut self, input: f64) -> Option<f64> {
        let fast = self.fast.append(input);
        let slow = self.slow.append(input);
        self.value = fast
            .zip(slow)
            .and_then(|(fast, slow)| (slow != 0.0).then_some((fast - slow) / slow * 100.0));
        self.value
    }

    fn value(&self) -> Option<f64> {
        self.value
    }

    fn reset(&mut self) {
        self.fast.reset();
        self.slow.reset();
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
        let input = lcg_series(5_000, 0x5EED_9905);
        let tail = lcg_series(256, 0x7A11_9905);
        let combos = [
            (12usize, 26usize, MaType::ExponentialMovingAverage),
            (2, 2, MaType::ExponentialMovingAverage),
            (1, 5, MaType::ExponentialMovingAverage),
            (3, 10, MaType::ExponentialMovingAverage),
            // SMA legs exercise the fused SMA path (TA-Lib's PPO default).
            (7, 13, MaType::SimpleMovingAverage),
            (12, 26, MaType::SimpleMovingAverage),
            (5, 5, MaType::SimpleMovingAverage),
            (1, 9, MaType::SimpleMovingAverage),
            (26, 12, MaType::SimpleMovingAverage),
            // A non-fusable type still has to round-trip through the fallback.
            (7, 13, MaType::WeightedMovingAverage),
        ];
        for (fast, slow, ma_type) in combos {
            let mut per_bar = PercentagePriceOscillator::new(fast, slow, ma_type).unwrap();
            let reference: Vec<f64> = input
                .iter()
                .map(|&x| per_bar.append(x).unwrap_or(f64::NAN))
                .collect();
            let tail_reference: Vec<f64> = tail
                .iter()
                .map(|&x| per_bar.append(x).unwrap_or(f64::NAN))
                .collect();

            for chunk in [usize::MAX, 1, 7, 10, 97, 1_000] {
                let mut state = PercentagePriceOscillator::new(fast, slow, ma_type).unwrap();
                let mut out = Vec::new();
                for piece in input.chunks(chunk.min(input.len())) {
                    state.extend_slice_into(piece, &mut out);
                }
                let label = format!("{fast}/{slow} chunk {chunk}");
                assert_same_bits(&out, &reference, &label);
                let tail_out: Vec<f64> = tail
                    .iter()
                    .map(|&x| state.append(x).unwrap_or(f64::NAN))
                    .collect();
                assert_same_bits(&tail_out, &tail_reference, &format!("{label} tail"));
            }
        }
    }

    #[test]
    fn matches_batch_for_all_moving_average_types() {
        let input: Vec<f64> = (0..200)
            .map(|index| 100.0 + (index as f64 * 0.19).sin() * 7.0 + index as f64 * 0.03)
            .collect();
        for code in 0..=8 {
            let ma_type = MaType::try_from(code).unwrap();
            let expected = percentage_price_oscillator(&input, 7, 13, ma_type).unwrap();
            let mut state = PercentagePriceOscillator::new(7, 13, ma_type).unwrap();
            for (&input, expected) in input.iter().zip(expected) {
                let actual = state.append(input);
                if expected.is_nan() {
                    assert_eq!(actual, None, "MA type {code}");
                } else {
                    let actual = actual.unwrap();
                    assert!(
                        (actual - expected).abs() < 1e-9,
                        "MA type {code}: actual={actual}, expected={expected}"
                    );
                }
            }
        }
    }
}
