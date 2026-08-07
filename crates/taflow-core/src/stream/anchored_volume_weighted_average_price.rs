//! Stateful anchored volume-weighted average price with deviation bands.

/// Running weighted mean and standard-deviation bands reset at anchors.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `AnchoredVolumeWeightedAveragePrice`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct AnchoredVolumeWeightedAveragePrice {
    deviation: f64,
    weighted_price: f64,
    weighted_square: f64,
    volume: f64,
    value: Option<(f64, f64, f64)>,
}

impl AnchoredVolumeWeightedAveragePrice {
    /// Creates anchored volume-weighted price bands.
    pub fn new(deviation: f64) -> Self {
        Self {
            deviation,
            weighted_price: 0.0,
            weighted_square: 0.0,
            volume: 0.0,
            value: None,
        }
    }

    /// Appends one OHLCV bar and optionally starts a new anchor.
    pub fn append(
        &mut self,
        high: f64,
        low: f64,
        close: f64,
        volume: f64,
        anchor: bool,
    ) -> (f64, f64, f64) {
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
        let result = (
            mean,
            mean + self.deviation * variance.sqrt(),
            mean - self.deviation * variance.sqrt(),
        );
        self.value = Some(result);
        result
    }

    /// Returns the latest mean and deviation bands.
    pub fn value(&self) -> Option<(f64, f64, f64)> {
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
