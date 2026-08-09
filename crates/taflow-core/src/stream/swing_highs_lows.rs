//! Batch implementation for `swing_highs_lows`.

use super::operator_states::*;
use super::*;
use crate::error::{TaError, TaResult};

/// Causal swing-point confirmation.
///
/// The center bar of a `2 * swing_length + 1` window is confirmed at the
/// current bar. A signal is emitted only after the required future bars have
/// arrived, so no output uses lookahead when it is observed.
pub fn swing_highs_lows(
    high: &[f64],
    low: &[f64],
    swing_length: usize,
) -> TaResult<(Vec<f64>, Vec<f64>, Vec<f64>)> {
    if high.len() != low.len() {
        return Err(TaError::LengthMismatch {
            expected: high.len(),
            got: low.len(),
        });
    }
    let mut state = SwingHighLow::new(swing_length)?;
    let mut signal = Vec::with_capacity(high.len());
    let mut level = Vec::with_capacity(high.len());
    let mut bars_since = Vec::with_capacity(high.len());
    for (&high, &low) in high.iter().zip(low) {
        let value = state.append(high, low);
        signal.push(value.map_or(f64::NAN, |value| value.signal));
        level.push(value.map_or(f64::NAN, |value| value.level));
        bars_since.push(value.map_or(f64::NAN, |value| value.bars_since));
    }
    Ok((signal, level, bars_since))
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use std::collections::VecDeque;

    /// Pre-optimization `SwingHighLow::append`, kept verbatim as the oracle
    /// for the monotonic-extrema rewrite (also reused by the equal-highs/lows
    /// and retracement tests).
    #[derive(Debug, Clone)]
    pub(crate) struct ReferenceSwing {
        highs: VecDeque<f64>,
        lows: VecDeque<f64>,
        length: usize,
        bars_since: Option<usize>,
    }

    /// Mirror of `SwingValue` for the reference implementation.
    #[derive(Debug, Clone, Copy)]
    pub(crate) struct ReferenceSwingValue {
        pub signal: f64,
        pub level: f64,
        pub bars_since: f64,
    }

    impl ReferenceSwing {
        pub(crate) fn new(length: usize) -> Self {
            let capacity = length * 2 + 1;
            Self {
                highs: VecDeque::with_capacity(capacity),
                lows: VecDeque::with_capacity(capacity),
                length,
                bars_since: None,
            }
        }

        pub(crate) fn append(&mut self, high: f64, low: f64) -> Option<ReferenceSwingValue> {
            let capacity = self.length * 2 + 1;
            if self.highs.len() == capacity {
                self.highs.pop_front();
                self.lows.pop_front();
            }
            self.highs.push_back(high);
            self.lows.push_back(low);
            if self.highs.len() < capacity {
                return None;
            }
            let center_high = self.highs[self.length];
            let center_low = self.lows[self.length];
            let is_high =
                center_high >= self.highs.iter().copied().fold(f64::NEG_INFINITY, f64::max);
            let is_low = center_low <= self.lows.iter().copied().fold(f64::INFINITY, f64::min);
            let (signal, level) = match (is_high, is_low) {
                (true, false) => (1.0, center_high),
                (false, true) => (-1.0, center_low),
                _ => (f64::NAN, f64::NAN),
            };
            self.bars_since = if signal.is_nan() {
                self.bars_since.map(|bars| bars + 1)
            } else {
                Some(0)
            };
            Some(ReferenceSwingValue {
                signal,
                level,
                bars_since: self.bars_since.map_or(f64::NAN, |bars| bars as f64),
            })
        }
    }

    pub(crate) fn lcg_series(len: usize, seed: u64) -> Vec<f64> {
        let mut state = seed;
        (0..len)
            .map(|_| {
                state = state
                    .wrapping_mul(6364136223846793005)
                    .wrapping_add(1442695040888963407);
                90.0 + ((state >> 11) as f64 / (1u64 << 53) as f64) * 20.0
            })
            .collect()
    }

    #[test]
    fn matches_reference_bitwise_and_survives_chunking() {
        let base = lcg_series(5_000, 0xF1_5EED_C1);
        let mut high: Vec<f64> = base.iter().map(|v| v + 0.7).collect();
        let mut low: Vec<f64> = base.iter().map(|v| v - 0.7).collect();
        // A plateau exercises the equal-value tie handling on both sides.
        for i in 3_000..3_040 {
            high[i] = 105.0;
            low[i] = 95.0;
        }
        for length in [1usize, 2, 5, 20, 100] {
            let mut reference = ReferenceSwing::new(length);
            let mut state = SwingHighLow::new(length).unwrap();
            for i in 0..base.len() {
                let want = reference.append(high[i], low[i]);
                let got = state.append(high[i], low[i]);
                match (want, got) {
                    (Some(want), Some(got)) => {
                        assert_eq!(
                            want.signal.to_bits(),
                            got.signal.to_bits(),
                            "l={length} bar {i} signal"
                        );
                        assert_eq!(
                            want.level.to_bits(),
                            got.level.to_bits(),
                            "l={length} bar {i} level"
                        );
                        assert_eq!(
                            want.bars_since.to_bits(),
                            got.bars_since.to_bits(),
                            "l={length} bar {i} bars_since"
                        );
                    }
                    (None, None) => {}
                    _ => panic!("warm-up mismatch l={length} bar {i}"),
                }
            }
            state.reset();
            let mut fresh = ReferenceSwing::new(length);
            for i in 0..512 {
                let want = fresh.append(high[i], low[i]);
                let got = state.append(high[i], low[i]);
                assert_eq!(want.is_some(), got.is_some(), "post-reset l={length}");
                if let (Some(want), Some(got)) = (want, got) {
                    assert_eq!(want.signal.to_bits(), got.signal.to_bits());
                    assert_eq!(want.level.to_bits(), got.level.to_bits());
                    assert_eq!(want.bars_since.to_bits(), got.bars_since.to_bits());
                }
            }
        }
    }

    #[test]
    fn batch_matches_streaming() {
        let base = lcg_series(1_000, 0xF2_5EED_C2);
        let high: Vec<f64> = base.iter().map(|v| v + 0.7).collect();
        let low: Vec<f64> = base.iter().map(|v| v - 0.7).collect();
        let (signal, level, bars_since) = swing_highs_lows(&high, &low, 5).unwrap();
        let mut state = SwingHighLow::new(5).unwrap();
        for i in 0..base.len() {
            let value = state.append(high[i], low[i]);
            let (s, l, b) = value.map_or((f64::NAN, f64::NAN, f64::NAN), |v| {
                (v.signal, v.level, v.bars_since)
            });
            assert_eq!(signal[i].to_bits(), s.to_bits());
            assert_eq!(level[i].to_bits(), l.to_bits());
            assert_eq!(bars_since[i].to_bits(), b.to_bits());
        }
    }
}
use super::operator_states::*;
use super::*;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq)]
/// Persistent Rust state or aligned output type for `SwingValue`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct SwingValue {
    pub signal: f64,
    pub level: f64,
    pub bars_since: f64,
}

#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `SwingHighLow`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct SwingHighLow {
    /// Rolling extrema over the confirmation window (`2 * length + 1`).
    high_extrema: MonotonicMax,
    low_extrema: MonotonicMin,
    /// Delay lines of `length + 1` bars: their oldest slot is the center bar
    /// under test once the confirmation window is full.
    center_highs: ContiguousWindow,
    center_lows: ContiguousWindow,
    bars_since: Option<usize>,
    value: Option<SwingValue>,
}

impl SwingHighLow {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(length: usize) -> TaResult<Self> {
        validate_period(length)?;
        let capacity = length.saturating_mul(2).saturating_add(1);
        Ok(Self {
            high_extrema: MonotonicMax::new(capacity)?,
            low_extrema: MonotonicMin::new(capacity)?,
            center_highs: ContiguousWindow::new(length + 1),
            center_lows: ContiguousWindow::new(length + 1),
            bars_since: None,
            value: None,
        })
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    ///
    /// M1: the two O(2·length) window rescans become amortized-O(1) monotonic
    /// deques, and the center bar comes from a fixed delay ring instead of
    /// indexing a `VecDeque`. Extrema are comparison-only, so the confirmed
    /// signals and levels are bit-identical to the rescan version.
    pub fn append(&mut self, high: f64, low: f64) -> Option<SwingValue> {
        let window_high = self.high_extrema.append(high);
        let window_low = self.low_extrema.append(low);
        self.center_highs.push(high);
        self.center_lows.push(low);

        let (Some(window_high), Some(window_low)) = (window_high, window_low) else {
            self.value = None;
            return None;
        };
        let center_high = self.center_highs.window()[0];
        let center_low = self.center_lows.window()[0];
        let is_high = center_high >= window_high;
        let is_low = center_low <= window_low;
        let (signal, level) = match (is_high, is_low) {
            (true, false) => (1.0, center_high),
            (false, true) => (-1.0, center_low),
            _ => (f64::NAN, f64::NAN),
        };
        self.bars_since = if signal.is_nan() {
            self.bars_since.map(|bars| bars + 1)
        } else {
            Some(0)
        };
        let value = SwingValue {
            signal,
            level,
            bars_since: self.bars_since.map_or(f64::NAN, |bars| bars as f64),
        };
        self.value = Some(value);
        Some(value)
    }

    /// Computes or updates `value` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn value(&self) -> Option<SwingValue> {
        self.value
    }

    /// Return the current bars-since result, if available.
    ///
    pub fn bars_since(&self) -> Option<f64> {
        self.bars_since.map(|bars| bars as f64)
    }

    /// Computes or updates `reset` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn reset(&mut self) {
        self.high_extrema.reset();
        self.low_extrema.reset();
        self.center_highs.clear();
        self.center_lows.clear();
        self.bars_since = None;
        self.value = None;
    }
}
