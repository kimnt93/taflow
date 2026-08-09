//! Stateful anchored volume-weighted average price with deviation bands.

use crate::{TaError, TaResult};

/// Named anchored volume-weighted average price output for one bar.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AnchoredVolumeWeightedAveragePriceValue {
    /// Volume-weighted average typical price since the latest anchor.
    pub volume_weighted_average_price: f64,
    /// Average plus the configured weighted standard-deviation multiple.
    pub upper_band: f64,
    /// Average minus the configured weighted standard-deviation multiple.
    pub lower_band: f64,
}

/// Running weighted mean and standard-deviation bands reset at anchors.
#[derive(Debug, Clone)]
pub struct AnchoredVolumeWeightedAveragePrice {
    standard_deviation_multiplier: f64,
    weighted_price: f64,
    weighted_square: f64,
    volume: f64,
    value: Option<AnchoredVolumeWeightedAveragePriceValue>,
}

impl AnchoredVolumeWeightedAveragePrice {
    /// Creates anchored volume-weighted price bands.
    pub fn new(standard_deviation_multiplier: f64) -> TaResult<Self> {
        if !standard_deviation_multiplier.is_finite() || standard_deviation_multiplier < 0.0 {
            return Err(TaError::InvalidParameter {
                name: "standard_deviation_multiplier",
                value: standard_deviation_multiplier.to_string(),
                reason: "must be finite and non-negative",
            });
        }
        Ok(Self {
            standard_deviation_multiplier,
            weighted_price: 0.0,
            weighted_square: 0.0,
            volume: 0.0,
            value: None,
        })
    }

    /// Appends one OHLCV bar and optionally starts a new anchor.
    pub fn append(
        &mut self,
        high: f64,
        low: f64,
        close: f64,
        volume: f64,
        anchor: bool,
    ) -> AnchoredVolumeWeightedAveragePriceValue {
        if anchor {
            self.weighted_price = 0.0;
            self.weighted_square = 0.0;
            self.volume = 0.0;
        }
        let typical = (high + low + close) / 3.0;
        self.weighted_price += typical * volume;
        self.weighted_square += typical * typical * volume;
        self.volume += volume;
        let mean = if self.volume == 0.0 {
            f64::NAN
        } else {
            self.weighted_price / self.volume
        };
        let variance = if self.volume == 0.0 {
            f64::NAN
        } else {
            (self.weighted_square / self.volume - mean * mean).max(0.0)
        };
        let deviation = self.standard_deviation_multiplier * variance.sqrt();
        let result = AnchoredVolumeWeightedAveragePriceValue {
            volume_weighted_average_price: mean,
            upper_band: mean + deviation,
            lower_band: mean - deviation,
        };
        self.value = Some(result);
        result
    }

    /// Appends aligned slices into the three named output histories.
    #[allow(clippy::too_many_arguments)]
    pub fn extend_slices_into(
        &mut self,
        high: &[f64],
        low: &[f64],
        close: &[f64],
        volume: &[f64],
        anchor: &[bool],
        average_output: &mut Vec<f64>,
        upper_output: &mut Vec<f64>,
        lower_output: &mut Vec<f64>,
    ) -> TaResult<()> {
        let length = high.len();
        for actual in [low.len(), close.len(), volume.len(), anchor.len()] {
            if actual != length {
                return Err(TaError::LengthMismatch {
                    expected: length,
                    got: actual,
                });
            }
        }

        average_output.reserve(length);
        upper_output.reserve(length);
        lower_output.reserve(length);
        for ((((&high, &low), &close), &volume), &anchor) in
            high.iter().zip(low).zip(close).zip(volume).zip(anchor)
        {
            let value = self.append(high, low, close, volume, anchor);
            average_output.push(value.volume_weighted_average_price);
            upper_output.push(value.upper_band);
            lower_output.push(value.lower_band);
        }
        Ok(())
    }

    /// Returns the latest named mean and deviation bands.
    pub fn value(&self) -> Option<AnchoredVolumeWeightedAveragePriceValue> {
        self.value
    }

    /// Clears running weighted moments.
    pub fn reset(&mut self) {
        self.weighted_price = 0.0;
        self.weighted_square = 0.0;
        self.volume = 0.0;
        self.value = None;
    }
}
