use super::*;
use crate::error::TaResult;
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq)]
/// Persistent Rust state or aligned output type for `BreakOfStructureChangeOfCharacterValue`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct BreakOfStructureChangeOfCharacterValue {
    pub bos: f64,
    pub choch: f64,
    pub level: f64,
    pub broken: f64,
}

/// Causal break-of-structure and change-of-character events.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `BreakOfStructureChangeOfCharacter`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct BreakOfStructureChangeOfCharacter {
    swing: SwingHighLow,
    swings: VecDeque<(f64, f64)>,
    pending: Option<(f64, f64)>,
    trend: Option<f64>,
    value: Option<BreakOfStructureChangeOfCharacterValue>,
}

impl BreakOfStructureChangeOfCharacter {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(swing_length: usize) -> TaResult<Self> {
        Ok(Self {
            swing: SwingHighLow::new(swing_length)?,
            swings: VecDeque::with_capacity(4),
            pending: None,
            trend: None,
            value: None,
        })
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(
        &mut self,
        high: f64,
        low: f64,
        close: f64,
    ) -> BreakOfStructureChangeOfCharacterValue {
        let mut bos = f64::NAN;
        let mut choch = f64::NAN;
        let mut level = f64::NAN;
        let mut broken = f64::NAN;

        if let Some((direction, pending_level)) = self.pending {
            let crossed = (direction > 0.0 && close > pending_level)
                || (direction < 0.0 && close < pending_level);
            if crossed {
                broken = direction;
                level = pending_level;
                self.pending = None;
                self.trend = Some(direction);
            }
        }

        if let Some(swing) = self.swing.append(high, low) {
            self.swings.push_back((swing.signal, swing.level));
            if self.swings.len() > 4 {
                self.swings.pop_front();
            }
            if self.swings.len() == 4 {
                // Stack copy instead of a per-event heap allocation.
                let items = [
                    self.swings[0],
                    self.swings[1],
                    self.swings[2],
                    self.swings[3],
                ];
                let bullish = items[0].0 < 0.0
                    && items[1].0 > 0.0
                    && items[2].0 < 0.0
                    && items[3].0 > 0.0
                    && items[0].1 < items[2].1
                    && items[1].1 < items[3].1;
                let bearish = items[0].0 > 0.0
                    && items[1].0 < 0.0
                    && items[2].0 > 0.0
                    && items[3].0 < 0.0
                    && items[0].1 > items[2].1
                    && items[1].1 > items[3].1;
                let direction = if bullish {
                    Some(1.0)
                } else if bearish {
                    Some(-1.0)
                } else {
                    None
                };
                if let Some(direction) = direction {
                    bos = direction;
                    choch = if self.trend.is_some_and(|trend| trend != direction) {
                        direction
                    } else {
                        f64::NAN
                    };
                    level = items[1].1;
                    self.pending = Some((direction, level));
                }
            }
        }

        let value = BreakOfStructureChangeOfCharacterValue {
            bos,
            choch,
            level,
            broken,
        };
        self.value = Some(value);
        value
    }

    /// Computes or updates `value` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn value(&self) -> Option<BreakOfStructureChangeOfCharacterValue> {
        self.value
    }

    /// Reset the persistent state and clear the latest value.
    pub fn reset(&mut self) {
        self.swing.reset();
        self.swings.clear();
        self.pending = None;
        self.trend = None;
        self.value = None;
    }
}
