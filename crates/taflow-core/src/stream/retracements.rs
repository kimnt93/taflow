//! Batch implementation for `retracements`.

use super::operator_states::*;
use super::*;
use crate::error::{TaError, TaResult};

/// Computes or updates `retracements` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn retracements(
    high: &[f64],
    low: &[f64],
    close: &[f64],
    swing_length: usize,
) -> TaResult<(Vec<f64>, Vec<f64>, Vec<f64>)> {
    if high.len() != low.len() || low.len() != close.len() {
        return Err(TaError::LengthMismatch {
            expected: high.len(),
            got: low.len().max(close.len()),
        });
    }
    let mut state = Retracements::new(swing_length)?;
    let mut direction = Vec::with_capacity(high.len());
    let mut current_retracement_pct = Vec::with_capacity(high.len());
    let mut deepest_retracement_pct = Vec::with_capacity(high.len());
    for ((&high, &low), &close) in high.iter().zip(low).zip(close) {
        let value = state.append(high, low, close);
        direction.push(value.direction);
        current_retracement_pct.push(value.current_retracement_pct);
        deepest_retracement_pct.push(value.deepest_retracement_pct);
    }
    Ok((direction, current_retracement_pct, deepest_retracement_pct))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream::swing_highs_lows::tests::{lcg_series, ReferenceSwing};

    /// `Retracements` driven by the pre-optimization swing state.
    struct Reference {
        swing: ReferenceSwing,
        last_high: Option<f64>,
        last_low: Option<f64>,
        leg_high: Option<f64>,
        leg_low: Option<f64>,
        direction: Option<f64>,
        deepest: f64,
    }

    impl Reference {
        fn new(swing_length: usize) -> Self {
            Self {
                swing: ReferenceSwing::new(swing_length),
                last_high: None,
                last_low: None,
                leg_high: None,
                leg_low: None,
                direction: None,
                deepest: 0.0,
            }
        }

        fn append(&mut self, high: f64, low: f64, close: f64) -> [f64; 3] {
            if let Some(swing) = self.swing.append(high, low) {
                if swing.signal > 0.0 {
                    self.last_high = Some(swing.level);
                    if let Some(last_low) = self.last_low {
                        self.leg_high = Some(swing.level);
                        self.leg_low = Some(last_low);
                        self.direction = Some(1.0);
                        self.deepest = 0.0;
                    }
                } else if swing.signal < 0.0 {
                    self.last_low = Some(swing.level);
                    if let Some(last_high) = self.last_high {
                        self.leg_high = Some(last_high);
                        self.leg_low = Some(swing.level);
                        self.direction = Some(-1.0);
                        self.deepest = 0.0;
                    }
                }
            }
            let mut current = f64::NAN;
            let mut deepest = f64::NAN;
            if let (Some(leg_high), Some(leg_low), Some(direction)) =
                (self.leg_high, self.leg_low, self.direction)
            {
                let range = leg_high - leg_low;
                if range > 0.0 {
                    let pct = if direction > 0.0 {
                        (leg_high - close) / range * 100.0
                    } else {
                        (close - leg_low) / range * 100.0
                    };
                    current = pct.max(0.0);
                    self.deepest = self.deepest.max(current);
                    deepest = self.deepest;
                }
            }
            [self.direction.unwrap_or(f64::NAN), current, deepest]
        }
    }

    #[test]
    fn matches_reference_bitwise_and_survives_chunking() {
        let base = lcg_series(5_000, 0x21_5EED_E1);
        let high: Vec<f64> = base.iter().map(|v| v + 0.7).collect();
        let low: Vec<f64> = base.iter().map(|v| v - 0.7).collect();
        for swing_length in [1usize, 2, 5, 20] {
            let mut reference = Reference::new(swing_length);
            let mut state = Retracements::new(swing_length).unwrap();
            for i in 0..base.len() {
                let want = reference.append(high[i], low[i], base[i]);
                let value = state.append(high[i], low[i], base[i]);
                let got = [
                    value.direction,
                    value.current_retracement_pct,
                    value.deepest_retracement_pct,
                ];
                for (k, (w, g)) in want.iter().zip(&got).enumerate() {
                    assert_eq!(
                        w.to_bits(),
                        g.to_bits(),
                        "l={swing_length} bar {i} output {k}"
                    );
                }
            }
            state.reset();
            let mut fresh = Reference::new(swing_length);
            for i in 0..512 {
                let want = fresh.append(high[i], low[i], base[i]);
                let value = state.append(high[i], low[i], base[i]);
                let got = [
                    value.direction,
                    value.current_retracement_pct,
                    value.deepest_retracement_pct,
                ];
                for (w, g) in want.iter().zip(&got) {
                    assert_eq!(w.to_bits(), g.to_bits(), "post-reset l={swing_length}");
                }
            }
        }
    }

    #[test]
    fn batch_matches_streaming() {
        let base = lcg_series(1_000, 0x22_5EED_E2);
        let high: Vec<f64> = base.iter().map(|v| v + 0.7).collect();
        let low: Vec<f64> = base.iter().map(|v| v - 0.7).collect();
        let (direction, current, deepest) = retracements(&high, &low, &base, 5).unwrap();
        let mut state = Retracements::new(5).unwrap();
        for i in 0..base.len() {
            let value = state.append(high[i], low[i], base[i]);
            assert_eq!(direction[i].to_bits(), value.direction.to_bits());
            assert_eq!(
                current[i].to_bits(),
                value.current_retracement_pct.to_bits()
            );
            assert_eq!(
                deepest[i].to_bits(),
                value.deepest_retracement_pct.to_bits()
            );
        }
    }
}
use super::operator_states::*;
use super::*;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq)]
/// Persistent Rust state or aligned output type for `RetracementsValue`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct RetracementsValue {
    pub direction: f64,
    pub current_retracement_pct: f64,
    pub deepest_retracement_pct: f64,
}

/// Causal swing-leg retracement tracking. On each confirmed swing a leg is
/// established from the opposite prior pivot; the retracement percentage is
/// the fraction of that leg already given back by the current close, with
/// the deepest value tracked since the leg began.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `Retracements`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct Retracements {
    swing: SwingHighLow,
    last_high: Option<f64>,
    last_low: Option<f64>,
    leg_high: Option<f64>,
    leg_low: Option<f64>,
    direction: Option<f64>,
    deepest: f64,
    value: Option<RetracementsValue>,
}

impl Retracements {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(swing_length: usize) -> TaResult<Self> {
        Ok(Self {
            swing: SwingHighLow::new(swing_length)?,
            last_high: None,
            last_low: None,
            leg_high: None,
            leg_low: None,
            direction: None,
            deepest: 0.0,
            value: None,
        })
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, high: f64, low: f64, close: f64) -> RetracementsValue {
        if let Some(swing) = self.swing.append(high, low) {
            if swing.signal > 0.0 {
                self.last_high = Some(swing.level);
                if let Some(last_low) = self.last_low {
                    self.leg_high = Some(swing.level);
                    self.leg_low = Some(last_low);
                    self.direction = Some(1.0);
                    self.deepest = 0.0;
                }
            } else if swing.signal < 0.0 {
                self.last_low = Some(swing.level);
                if let Some(last_high) = self.last_high {
                    self.leg_high = Some(last_high);
                    self.leg_low = Some(swing.level);
                    self.direction = Some(-1.0);
                    self.deepest = 0.0;
                }
            }
        }

        let mut current_retracement_pct = f64::NAN;
        let mut deepest_retracement_pct = f64::NAN;
        if let (Some(leg_high), Some(leg_low), Some(direction)) =
            (self.leg_high, self.leg_low, self.direction)
        {
            let range = leg_high - leg_low;
            if range > 0.0 {
                let pct = if direction > 0.0 {
                    (leg_high - close) / range * 100.0
                } else {
                    (close - leg_low) / range * 100.0
                };
                current_retracement_pct = pct.max(0.0);
                self.deepest = self.deepest.max(current_retracement_pct);
                deepest_retracement_pct = self.deepest;
            }
        }

        let value = RetracementsValue {
            direction: self.direction.unwrap_or(f64::NAN),
            current_retracement_pct,
            deepest_retracement_pct,
        };
        self.value = Some(value);
        value
    }

    /// Computes or updates `value` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn value(&self) -> Option<RetracementsValue> {
        self.value
    }

    /// Reset the persistent state and clear the latest value.
    pub fn reset(&mut self) {
        self.swing.reset();
        self.last_high = None;
        self.last_low = None;
        self.leg_high = None;
        self.leg_low = None;
        self.direction = None;
        self.deepest = 0.0;
        self.value = None;
    }
}
