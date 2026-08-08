//! Incremental Simple Moving Average (SMA).

use crate::error::TaResult;

use super::{StreamingIndicator, Window};

/// Compute the simple moving average result for the supplied aligned series.
///
/// # Parameters
///
/// * `input` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn simple_moving_average(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    let mut state = SimpleMovingAverage::new(timeperiod)?;
    let mut output = Vec::new();
    state.extend_slice_into(input, &mut output);
    Ok(output)
}

/// Stateful simple moving average with O(1) updates.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `SimpleMovingAverage`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct SimpleMovingAverage {
    period: usize,
    window: Window,
    sum: f64,
    value: Option<f64>,
}

impl SimpleMovingAverage {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(period: usize) -> TaResult<Self> {
        Ok(Self {
            period,
            window: Window::new(period)?,
            sum: 0.0,
            value: None,
        })
    }

    /// Number of further appends required before the state emits values.
    #[inline]
    pub(super) fn warmup_remaining(&self) -> usize {
        self.period - self.window.len()
    }

    /// The configured averaging period.
    #[inline]
    pub(crate) fn period(&self) -> usize {
        self.period
    }

    /// The current running window sum.
    #[inline]
    pub(crate) fn raw_sum(&self) -> f64 {
        self.sum
    }

    /// Mutable access to the backing window for fused bulk kernels that keep
    /// the running sum in a local while still advancing the ring in place.
    #[inline]
    pub(crate) fn window_mut(&mut self) -> &mut Window {
        &mut self.window
    }

    /// Writes back the scalar recurrence state after a fused bulk loop. The
    /// window must already have been advanced through [`Self::window_mut`].
    #[inline]
    pub(crate) fn store_bulk_state(&mut self, sum: f64, value: Option<f64>) {
        self.sum = sum;
        self.value = value;
    }
}

impl StreamingIndicator for SimpleMovingAverage {
    type Output = f64;

    /// Bulk kernel: O(1) add/evict sliding-sum recurrence indexing the input
    /// slice directly. Bit-identical to per-bar [`Self::append`] in both
    /// outputs and post-run streaming state.
    fn extend_slice_into(&mut self, inputs: &[f64], output: &mut Vec<f64>) {
        let period = self.period;
        let n = inputs.len();
        output.reserve(n);
        // Warm-up prologue: after `period` appends the ring holds exactly
        // `inputs[..period]`, regardless of any prior state.
        let prologue = n.min(period);
        for &input in &inputs[..prologue] {
            output.push(self.append(input).unwrap_or(f64::NAN));
        }
        if n <= period {
            return;
        }
        // Steady loop: same arithmetic order as `append` (`sum -= old` then
        // `sum += input`), evicted element read from the input slice.
        let mut sum = self.sum;
        let period_f = period as f64;
        let mut last = f64::NAN;
        for i in period..n {
            sum -= inputs[i - period];
            sum += inputs[i];
            last = sum / period_f;
            output.push(last);
        }
        self.sum = sum;
        self.value = Some(last);
        // Rebuild the ring so subsequent appends continue bit-identically.
        self.window.clear();
        for &input in &inputs[n - period..] {
            self.window.push(input);
        }
    }

    fn append(&mut self, input: f64) -> Option<f64> {
        if let Some(old) = self.window.push(input) {
            self.sum -= old;
        }
        self.sum += input;
        self.value = self.window.is_full().then(|| self.sum / self.period as f64);
        self.value
    }

    fn value(&self) -> Option<f64> {
        self.value
    }

    fn reset(&mut self) {
        self.window.clear();
        self.sum = 0.0;
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
    fn sma_bulk_is_bitwise_identical_to_per_bar_append() {
        let input = lcg_series(5_000, 0x5EED_0051);
        let tail = lcg_series(256, 0x7A11_0051);
        for period in [2usize, 5, 14, 30, 200] {
            let mut per_bar = SimpleMovingAverage::new(period).unwrap();
            let reference: Vec<f64> = input
                .iter()
                .map(|&x| per_bar.append(x).unwrap_or(f64::NAN))
                .collect();
            let tail_reference: Vec<f64> = tail
                .iter()
                .map(|&x| per_bar.append(x).unwrap_or(f64::NAN))
                .collect();

            for chunk in [usize::MAX, 1, 7, 97] {
                let mut state = SimpleMovingAverage::new(period).unwrap();
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
}
