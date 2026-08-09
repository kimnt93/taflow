//! Batch implementation for `williams_percent_r`.

use super::{invalid_period, vhgw};
use crate::error::TaError;
use crate::error::TaResult;

/// Compute the williams r result for the supplied aligned series.
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
pub fn williams_percent_r(
    high: &[f64],
    low: &[f64],
    close: &[f64],
    timeperiod: usize,
) -> TaResult<Vec<f64>> {
    if high.len() != low.len() || high.len() != close.len() {
        return Err(crate::TaError::LengthMismatch {
            expected: high.len(),
            got: low.len().min(close.len()),
        });
    }
    if timeperiod < 2 {
        return Err(invalid_period("timeperiod", timeperiod, 2));
    }
    let len = high.len();
    // Historical behavior: too-short inputs yield an all-NaN series, not an
    // error (the streaming state simply never warms up).
    if len < timeperiod {
        return Ok(vec![f64::NAN; len]);
    }

    let lookback = timeperiod - 1;
    let window_count = len - lookback;
    let mut highest = vec![0.0_f64; window_count];
    let mut lowest = vec![0.0_f64; window_count];
    vhgw::sliding_max_into(high, timeperiod, &mut highest);
    vhgw::sliding_min_into(low, timeperiod, &mut lowest);

    let mut output = vec![f64::NAN; len];
    for (offset, slot) in output[lookback..].iter_mut().enumerate() {
        let maximum = highest[offset];
        let minimum = lowest[offset];
        let range = maximum - minimum;
        *slot = if range > 0.0 {
            -100.0 * (maximum - close[lookback + offset]) / range
        } else {
            0.0
        };
    }
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream::tests_extrema_support::{datasets, periods_and_lengths};

    /// Original streaming-driven batch, kept as oracle (the streaming state
    /// itself is verified against the historical two-deque state in
    /// `rolling_extrema::tests`).
    fn reference_williams_percent_r(
        high: &[f64],
        low: &[f64],
        close: &[f64],
        timeperiod: usize,
    ) -> TaResult<Vec<f64>> {
        if high.len() != low.len() || high.len() != close.len() {
            return Err(crate::TaError::LengthMismatch {
                expected: high.len(),
                got: low.len().min(close.len()),
            });
        }
        let mut state = crate::stream::WilliamsPercentR::new(timeperiod)?;
        Ok(high
            .iter()
            .zip(low)
            .zip(close)
            .map(|((&high, &low), &close)| state.append(high, low, close).unwrap_or(f64::NAN))
            .collect())
    }

    #[test]
    fn matches_reference_bitwise() {
        for (period, len) in periods_and_lengths() {
            for data in datasets(len) {
                let high: Vec<f64> = data.iter().map(|v| v + 1.5).collect();
                let low: Vec<f64> = data.iter().map(|v| v - 1.5).collect();
                let close = data.clone();
                let expected = reference_williams_percent_r(&high, &low, &close, period);
                let actual = williams_percent_r(&high, &low, &close, period);
                match (expected, actual) {
                    (Ok(expected), Ok(actual)) => {
                        assert_eq!(expected.len(), actual.len());
                        for (e, a) in expected.iter().zip(&actual) {
                            assert_eq!(e.to_bits(), a.to_bits(), "p={period} len={len}");
                        }
                    }
                    (Err(_), Err(_)) => {}
                    _ => panic!("error parity mismatch p={period} len={len}"),
                }
            }
        }
    }

    #[test]
    fn flat_window_emits_zero_and_short_input_is_all_nan() {
        let flat = vec![5.0; 8];
        let out = williams_percent_r(&flat, &flat, &flat, 4).unwrap();
        assert!(out[..3].iter().all(|v| v.is_nan()));
        assert!(out[3..].iter().all(|&v| v == 0.0));

        let short = williams_percent_r(&flat[..2], &flat[..2], &flat[..2], 4).unwrap();
        assert_eq!(short.len(), 2);
        assert!(short.iter().all(|v| v.is_nan()));
    }
}
use super::volume_states::*;
use super::*;

/// Stateful Williams %R.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `WilliamsPercentR`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct WilliamsPercentR {
    highs: MonotonicMax,
    lows: MonotonicMin,
    value: Option<f64>,
}

impl WilliamsPercentR {
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
            highs: MonotonicMax::new(period)?,
            lows: MonotonicMin::new(period)?,
            value: None,
        })
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, high: f64, low: f64, close: f64) -> Option<f64> {
        let maximum = self.highs.append(high);
        let minimum = self.lows.append(low);
        self.value = maximum.zip(minimum).map(|(maximum, minimum)| {
            let range = maximum - minimum;
            if range > 0.0 {
                -100.0 * (maximum - close) / range
            } else {
                0.0
            }
        });
        self.value
    }

    /// Bulk kernel: one vHGW pass over `high` and one over `low`, then the
    /// per-bar %R arithmetic in a flat loop. The trailing `period` inputs are
    /// replayed to rebuild the monotonic deques, so outputs and post-run state
    /// are bit-identical to per-bar [`Self::append`]; warm-up bars are NaN.
    pub fn extend_slices_into(
        &mut self,
        high: &[f64],
        low: &[f64],
        close: &[f64],
        output: &mut Vec<f64>,
    ) -> TaResult<()> {
        if high.len() != low.len() || high.len() != close.len() {
            return Err(TaError::LengthMismatch {
                expected: high.len(),
                got: low.len().min(close.len()),
            });
        }
        let n = high.len();
        let period = self.highs.period();
        if self.highs.count() != 0 || n < period {
            output.reserve(n);
            for index in 0..n {
                output.push(
                    self.append(high[index], low[index], close[index])
                        .unwrap_or(f64::NAN),
                );
            }
            return Ok(());
        }
        let start = output.len();
        output.resize(start + n, f64::NAN);
        let warm = start + period - 1;
        let mut lowest = vec![0.0_f64; n - (period - 1)];
        vhgw::sliding_max_into(high, period, &mut output[warm..]);
        vhgw::sliding_min_into(low, period, &mut lowest);
        for (offset, (slot, &minimum)) in output[warm..].iter_mut().zip(&lowest).enumerate() {
            let maximum = *slot;
            let range = maximum - minimum;
            *slot = if range > 0.0 {
                -100.0 * (maximum - close[period - 1 + offset]) / range
            } else {
                0.0
            };
        }
        self.highs.rebuild_from_full_run(high);
        self.lows.rebuild_from_full_run(low);
        self.value = output.last().copied();
        Ok(())
    }

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
        self.highs.reset();
        self.lows.reset();
        self.value = None;
    }
}
