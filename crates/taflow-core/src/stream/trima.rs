//! Incremental Triangular Moving Average (TRIMA).

use crate::error::TaResult;

use super::{invalid_period, SimpleMovingAverage, StreamingIndicator};

/// Compute the triangular moving average result for the supplied aligned series.
///
/// # Parameters
///
/// * `input` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn triangular_moving_average(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    let mut state = TriangularMovingAverage::new(timeperiod)?;
    let mut output = Vec::new();
    state.extend_slice_into(input, &mut output);
    Ok(output)
}

/// Stateful triangular moving average as two cascaded SMA windows.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `TriangularMovingAverage`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct TriangularMovingAverage {
    sma1: SimpleMovingAverage,
    sma2: SimpleMovingAverage,
    value: Option<f64>,
}

impl TriangularMovingAverage {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(period: usize) -> TaResult<Self> {
        if period == 0 {
            return Err(invalid_period("timeperiod", period, 1));
        }
        let (p1, p2) = if period % 2 == 1 {
            let half = (period + 1) / 2;
            (half, half)
        } else {
            (period / 2 + 1, period / 2)
        };
        Ok(Self {
            sma1: SimpleMovingAverage::new(p1)?,
            sma2: SimpleMovingAverage::new(p2)?,
            value: None,
        })
    }
}

impl StreamingIndicator for TriangularMovingAverage {
    type Output = f64;

    /// Bulk kernel: runs the first SMA's bulk path into a scratch buffer and
    /// feeds the emitted suffix through the second SMA's bulk path, exactly
    /// mirroring the `sma1.append(..).and_then(|v| sma2.append(v))` chain.
    /// Bit-identical to per-bar [`Self::append`] in outputs and state.
    fn extend_slice_into(&mut self, inputs: &[f64], output: &mut Vec<f64>) {
        if inputs.is_empty() {
            return;
        }
        let n = inputs.len();
        output.reserve(n);
        // Bars before `sma1` warms up never reach `sma2` (matches and_then).
        // `sma1` emits its first value on the append that fills its window,
        // i.e. at index `warmup_remaining - 1` (0 when already warm).
        let first_valid = self.sma1.warmup_remaining().saturating_sub(1).min(n);
        let mut stage1 = Vec::with_capacity(n);
        self.sma1.extend_slice_into(inputs, &mut stage1);
        for _ in 0..first_valid {
            output.push(f64::NAN);
        }
        if first_valid == n {
            self.value = None;
            return;
        }
        self.sma2.extend_slice_into(&stage1[first_valid..], output);
        self.value = self.sma2.value();
    }

    fn append(&mut self, input: f64) -> Option<f64> {
        self.value = self
            .sma1
            .append(input)
            .and_then(|first| self.sma2.append(first));
        self.value
    }

    fn value(&self) -> Option<f64> {
        self.value
    }

    fn reset(&mut self) {
        self.sma1.reset();
        self.sma2.reset();
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
    fn trima_bulk_is_bitwise_identical_to_per_bar_append() {
        let input = lcg_series(5_000, 0x5EED_0071);
        let tail = lcg_series(256, 0x7A11_0071);
        for period in [2usize, 5, 14, 30, 200] {
            let mut per_bar = TriangularMovingAverage::new(period).unwrap();
            let reference: Vec<f64> = input
                .iter()
                .map(|&x| per_bar.append(x).unwrap_or(f64::NAN))
                .collect();
            let tail_reference: Vec<f64> = tail
                .iter()
                .map(|&x| per_bar.append(x).unwrap_or(f64::NAN))
                .collect();

            for chunk in [usize::MAX, 1, 7, 97] {
                let mut state = TriangularMovingAverage::new(period).unwrap();
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
