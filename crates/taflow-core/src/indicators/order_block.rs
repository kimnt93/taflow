use super::*;
use crate::error::{TaError, TaResult};
use crate::indicators::AverageTrueRange;
use crate::stream::operator_states::*;
use crate::stream::SwingHighLow;

#[derive(Debug, Clone, Copy, PartialEq)]
/// Persistent Rust state or aligned output type for `OrderBlockValue`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct OrderBlockValue {
    pub ob: f64,
    pub top: f64,
    pub bottom: f64,
    pub ob_volume: f64,
    pub mitigated: f64,
}

/// Causal order-block detection with volatile-bar exclusion and directional
/// mitigation. Dual pivot scales: `swing_length` locates the structure
/// interval, `internal_length` locates the extreme block within it. Bars
/// whose range is at least `threshold * ATR(atr_period)` are excluded from
/// being order blocks.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `OrderBlock`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct OrderBlock {
    atr: AverageTrueRange,
    internal: SwingHighLow,
    structure: SwingHighLow,
    internal_low: Option<(f64, f64, bool)>,
    internal_high: Option<(f64, f64, bool)>,
    structure_low: Option<f64>,
    structure_high: Option<f64>,
    threshold: f64,
    zones: Vec<ObZone>,
    value: Option<OrderBlockValue>,
}

impl OrderBlock {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(
        swing_length: usize,
        internal_length: usize,
        atr_period: usize,
        threshold: f64,
    ) -> TaResult<Self> {
        validate_period(swing_length)?;
        validate_period(internal_length)?;
        if atr_period == 0 {
            return Err(TaError::InvalidParameter {
                name: "atr_period",
                value: atr_period.to_string(),
                reason: "must be >= 1",
            });
        }
        if threshold < 0.0 {
            return Err(TaError::InvalidParameter {
                name: "threshold",
                value: threshold.to_string(),
                reason: "must be >= 0",
            });
        }
        Ok(Self {
            atr: AverageTrueRange::new(atr_period)?,
            internal: SwingHighLow::new(internal_length)?,
            structure: SwingHighLow::new(swing_length)?,
            internal_low: None,
            internal_high: None,
            structure_low: None,
            structure_high: None,
            threshold,
            zones: Vec::new(),
            value: None,
        })
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, high: f64, low: f64, close: f64, volume: f64) -> OrderBlockValue {
        let atr = self.atr.append(high, low, close);
        let volatile = atr.is_some_and(|atr| high - low >= self.threshold * atr);

        let mut ob = f64::NAN;
        let mut top = f64::NAN;
        let mut bottom = f64::NAN;
        let mut ob_volume = f64::NAN;

        if let Some(internal_swing) = self.internal.append(high, low) {
            match internal_swing.signal {
                signal if signal > 0.0 => {
                    self.internal_high = Some((internal_swing.level, volume, volatile));
                    if let (Some(structure_high), Some((low_level, low_volume, false))) =
                        (self.structure_high, self.internal_low)
                    {
                        if internal_swing.level > structure_high {
                            ob = 1.0;
                            top = internal_swing.level;
                            bottom = low_level;
                            ob_volume = low_volume;
                            self.zones.push(ObZone {
                                direction: ob,
                                top,
                                bottom,
                            });
                            self.structure_high = Some(internal_swing.level);
                        }
                    }
                }
                signal if signal < 0.0 => {
                    self.internal_low = Some((internal_swing.level, volume, volatile));
                    if let (Some(structure_low), Some((high_level, high_volume, false))) =
                        (self.structure_low, self.internal_high)
                    {
                        if internal_swing.level < structure_low {
                            ob = -1.0;
                            top = high_level;
                            bottom = internal_swing.level;
                            ob_volume = high_volume;
                            self.zones.push(ObZone {
                                direction: ob,
                                top,
                                bottom,
                            });
                            self.structure_low = Some(internal_swing.level);
                        }
                    }
                }
                _ => {}
            }
        }

        if let Some(structure_swing) = self.structure.append(high, low) {
            match structure_swing.signal {
                signal if signal > 0.0 => self.structure_high = Some(structure_swing.level),
                signal if signal < 0.0 => self.structure_low = Some(structure_swing.level),
                _ => {}
            }
        }

        let mut mitigated = f64::NAN;
        self.zones.retain(|zone| {
            let filled = (zone.direction > 0.0 && low <= zone.bottom)
                || (zone.direction < 0.0 && high >= zone.top);
            if filled {
                mitigated = zone.direction;
            }
            !filled
        });

        let value = OrderBlockValue {
            ob,
            top,
            bottom,
            ob_volume,
            mitigated,
        };
        self.value = Some(value);
        value
    }

    /// Computes or updates `value` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn value(&self) -> Option<OrderBlockValue> {
        self.value
    }

    /// Reset the persistent state and clear the latest value.
    pub fn reset(&mut self) {
        self.atr.reset();
        self.internal.reset();
        self.structure.reset();
        self.internal_low = None;
        self.internal_high = None;
        self.structure_low = None;
        self.structure_high = None;
        self.zones.clear();
        self.value = None;
    }
}
