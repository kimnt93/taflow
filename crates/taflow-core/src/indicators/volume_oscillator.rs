//! Incremental volume oscillator.

use crate::error::{TaError, TaResult};
use crate::stream::operator_states::RollingMean;

/// Percentage difference between fast and slow simple averages of volume.
///
/// The oscillator is `100 * (fast_average - slow_average) / slow_average`.
/// It warms up after `slow` observations and returns zero when the slow-volume
/// average is zero. This definition matches Wickra `VolumeOscillator` 0.9.9.
#[derive(Debug, Clone)]
pub struct VolumeOscillator {
    fast_average: RollingMean,
    slow_average: RollingMean,
    value: Option<f64>,
}

impl VolumeOscillator {
    /// Creates a volume oscillator with positive periods where `fast < slow`.
    pub fn new(fast: usize, slow: usize) -> TaResult<Self> {
        if fast == 0 || slow == 0 || fast >= slow {
            return Err(TaError::InvalidParameter {
                name: "periods",
                value: format!("{fast}/{slow}"),
                reason: "must satisfy 1 <= fast < slow",
            });
        }

        Ok(Self {
            fast_average: RollingMean::new(fast)?,
            slow_average: RollingMean::new(slow)?,
            value: None,
        })
    }

    /// Appends one volume observation and returns the latest warm value.
    pub fn append(&mut self, volume: f64) -> Option<f64> {
        let fast = self.fast_average.append(volume);
        let slow = self.slow_average.append(volume);

        self.value = fast.zip(slow).map(|(fast, slow)| {
            if slow == 0.0 {
                0.0
            } else {
                100.0 * (fast - slow) / slow
            }
        });
        self.value
    }

    /// Returns the latest oscillator value, or `None` during warm-up.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Restores fresh-state behavior without reallocating rolling windows.
    pub fn reset(&mut self) {
        self.fast_average.reset();
        self.slow_average.reset();
        self.value = None;
    }
}
