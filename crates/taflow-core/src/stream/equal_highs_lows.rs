//! Batch implementation for `equal_highs_lows`.

use super::operator_states::*;
use super::*;
use crate::error::{TaError, TaResult};

/// Computes or updates `equal_highs_lows` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn equal_highs_lows(
    high: &[f64],
    low: &[f64],
    close: &[f64],
    eq_len: usize,
    atr_period: usize,
    eq_threshold: f64,
) -> TaResult<(Vec<f64>, Vec<f64>, Vec<f64>)> {
    if high.len() != low.len() || low.len() != close.len() {
        return Err(TaError::LengthMismatch {
            expected: high.len(),
            got: low.len().max(close.len()),
        });
    }
    let mut state = EqualHighsLows::new(eq_len, atr_period, eq_threshold)?;
    let mut eqh = Vec::with_capacity(high.len());
    let mut eql = Vec::with_capacity(high.len());
    let mut level = Vec::with_capacity(high.len());
    for ((&high, &low), &close) in high.iter().zip(low).zip(close) {
        let value = state.append(high, low, close);
        eqh.push(value.eqh);
        eql.push(value.eql);
        level.push(value.level);
    }
    Ok((eqh, eql, level))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream::swing_highs_lows::tests::{lcg_series, ReferenceSwing};
    use crate::stream::AverageTrueRange;

    /// `EqualHighsLows` driven by the pre-optimization swing state, so any
    /// behaviour change in the rewritten `SwingHighLow` feed shows up here.
    struct Reference {
        atr: AverageTrueRange,
        swing: ReferenceSwing,
        previous_high: Option<f64>,
        previous_low: Option<f64>,
        eq_threshold: f64,
    }

    impl Reference {
        fn new(eq_len: usize, atr_period: usize, eq_threshold: f64) -> Self {
            Self {
                atr: AverageTrueRange::new(atr_period).unwrap(),
                swing: ReferenceSwing::new(eq_len),
                previous_high: None,
                previous_low: None,
                eq_threshold,
            }
        }

        fn append(&mut self, high: f64, low: f64, close: f64) -> [f64; 3] {
            let atr = self.atr.append(high, low, close);
            let mut eqh = f64::NAN;
            let mut eql = f64::NAN;
            let mut level = f64::NAN;
            if let Some(swing) = self.swing.append(high, low) {
                if swing.signal > 0.0 {
                    if let (Some(previous), Some(atr)) = (self.previous_high, atr) {
                        if (swing.level - previous).abs() < atr * self.eq_threshold {
                            eqh = 1.0;
                            level = swing.level;
                        }
                    }
                    self.previous_high = Some(swing.level);
                } else if swing.signal < 0.0 {
                    if let (Some(previous), Some(atr)) = (self.previous_low, atr) {
                        if (swing.level - previous).abs() < atr * self.eq_threshold {
                            eql = 1.0;
                            level = swing.level;
                        }
                    }
                    self.previous_low = Some(swing.level);
                }
            }
            [eqh, eql, level]
        }
    }

    #[test]
    fn matches_reference_bitwise_and_survives_chunking() {
        let base = lcg_series(5_000, 0x11_5EED_D1);
        let high: Vec<f64> = base.iter().map(|v| v + 0.7).collect();
        let low: Vec<f64> = base.iter().map(|v| v - 0.7).collect();
        for (eq_len, atr_period, threshold) in [(1usize, 14usize, 0.1), (5, 14, 0.5), (20, 30, 1.0)]
        {
            let mut reference = Reference::new(eq_len, atr_period, threshold);
            let mut state = EqualHighsLows::new(eq_len, atr_period, threshold).unwrap();
            for i in 0..base.len() {
                let want = reference.append(high[i], low[i], base[i]);
                let got = state.append(high[i], low[i], base[i]);
                let got = [got.eqh, got.eql, got.level];
                for (k, (w, g)) in want.iter().zip(&got).enumerate() {
                    assert_eq!(w.to_bits(), g.to_bits(), "l={eq_len} bar {i} output {k}");
                }
            }
            state.reset();
            let mut fresh = Reference::new(eq_len, atr_period, threshold);
            for i in 0..512 {
                let want = fresh.append(high[i], low[i], base[i]);
                let got = state.append(high[i], low[i], base[i]);
                let got = [got.eqh, got.eql, got.level];
                for (w, g) in want.iter().zip(&got) {
                    assert_eq!(w.to_bits(), g.to_bits(), "post-reset l={eq_len}");
                }
            }
        }
    }

    #[test]
    fn batch_matches_streaming() {
        let base = lcg_series(1_000, 0x12_5EED_D2);
        let high: Vec<f64> = base.iter().map(|v| v + 0.7).collect();
        let low: Vec<f64> = base.iter().map(|v| v - 0.7).collect();
        let (eqh, eql, level) = equal_highs_lows(&high, &low, &base, 5, 14, 0.5).unwrap();
        let mut state = EqualHighsLows::new(5, 14, 0.5).unwrap();
        for i in 0..base.len() {
            let got = state.append(high[i], low[i], base[i]);
            assert_eq!(eqh[i].to_bits(), got.eqh.to_bits());
            assert_eq!(eql[i].to_bits(), got.eql.to_bits());
            assert_eq!(level[i].to_bits(), got.level.to_bits());
        }
    }
}
use super::operator_states::*;
use super::*;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq)]
/// Persistent Rust state or aligned output type for `EqualHighsLowsValue`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct EqualHighsLowsValue {
    pub eqh: f64,
    pub eql: f64,
    pub level: f64,
}

/// Causal equal-high/equal-low detection. Two consecutive confirmed pivots
/// of the same kind are "equal" when their levels differ by less than
/// `eq_threshold * ATR(atr_period)`, matching the LuxAlgo Pine variant.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `EqualHighsLows`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct EqualHighsLows {
    atr: AverageTrueRange,
    swing: SwingHighLow,
    previous_high: Option<f64>,
    previous_low: Option<f64>,
    eq_threshold: f64,
    value: Option<EqualHighsLowsValue>,
}

impl EqualHighsLows {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(eq_len: usize, atr_period: usize, eq_threshold: f64) -> TaResult<Self> {
        validate_period(eq_len)?;
        if atr_period == 0 {
            return Err(TaError::InvalidParameter {
                name: "atr_period",
                value: atr_period.to_string(),
                reason: "must be >= 1",
            });
        }
        if !(0.0..=1.0).contains(&eq_threshold) {
            return Err(TaError::InvalidParameter {
                name: "eq_threshold",
                value: eq_threshold.to_string(),
                reason: "must be between 0 and 1",
            });
        }
        Ok(Self {
            atr: AverageTrueRange::new(atr_period)?,
            swing: SwingHighLow::new(eq_len)?,
            previous_high: None,
            previous_low: None,
            eq_threshold,
            value: None,
        })
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, high: f64, low: f64, close: f64) -> EqualHighsLowsValue {
        let atr = self.atr.append(high, low, close);
        let mut eqh = f64::NAN;
        let mut eql = f64::NAN;
        let mut level = f64::NAN;

        if let Some(swing) = self.swing.append(high, low) {
            if swing.signal > 0.0 {
                if let (Some(previous), Some(atr)) = (self.previous_high, atr) {
                    if (swing.level - previous).abs() < atr * self.eq_threshold {
                        eqh = 1.0;
                        level = swing.level;
                    }
                }
                self.previous_high = Some(swing.level);
            } else if swing.signal < 0.0 {
                if let (Some(previous), Some(atr)) = (self.previous_low, atr) {
                    if (swing.level - previous).abs() < atr * self.eq_threshold {
                        eql = 1.0;
                        level = swing.level;
                    }
                }
                self.previous_low = Some(swing.level);
            }
        }

        let value = EqualHighsLowsValue { eqh, eql, level };
        self.value = Some(value);
        value
    }

    /// Computes or updates `value` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn value(&self) -> Option<EqualHighsLowsValue> {
        self.value
    }

    /// Reset the persistent state and clear the latest value.
    pub fn reset(&mut self) {
        self.atr.reset();
        self.swing.reset();
        self.previous_high = None;
        self.previous_low = None;
        self.value = None;
    }
}
