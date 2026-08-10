//! Incremental Volume Zone Oscillator.

use crate::error::TaResult;
use crate::indicators::ExponentialMovingAverage;
use crate::stream::StreamingIndicator;

/// EMA-smoothed signed volume as a percentage of EMA-smoothed volume.
///
/// The first close establishes direction. Later volume is positive, negative,
/// or zero when close rises, falls, or remains unchanged. Both EMA states use
/// an SMA seed, matching Wickra `VZO` 0.9.9 and producing the first value after
/// `timeperiod + 1` bars.
#[derive(Debug, Clone)]
pub struct VolumeZoneOscillator {
    signed_volume_average: ExponentialMovingAverage,
    volume_average: ExponentialMovingAverage,
    previous_close: Option<f64>,
    value: Option<f64>,
}

impl VolumeZoneOscillator {
    /// Creates the oscillator with a positive EMA smoothing period.
    pub fn new(timeperiod: usize) -> TaResult<Self> {
        Ok(Self {
            signed_volume_average: ExponentialMovingAverage::new(timeperiod)?,
            volume_average: ExponentialMovingAverage::new(timeperiod)?,
            previous_close: None,
            value: None,
        })
    }

    /// Appends one close/volume bar and returns the latest warm value.
    pub fn append(&mut self, close: f64, volume: f64) -> Option<f64> {
        let Some(previous_close) = self.previous_close.replace(close) else {
            self.value = None;
            return None;
        };

        let signed_volume = if close > previous_close {
            volume
        } else if close < previous_close {
            -volume
        } else {
            0.0
        };
        let signed_average = self.signed_volume_average.append(signed_volume);
        let volume_average = self.volume_average.append(volume);

        self.value = signed_average.zip(volume_average).map(|(signed, total)| {
            if total == 0.0 {
                0.0
            } else {
                100.0 * signed / total
            }
        });
        self.value
    }

    /// Returns the latest oscillator value, or `None` during warm-up.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Clears direction, both EMA states, and the latest value.
    pub fn reset(&mut self) {
        self.signed_volume_average.reset();
        self.volume_average.reset();
        self.previous_close = None;
        self.value = None;
    }
}
