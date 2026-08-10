//! Incremental volume Relative Strength Index.

use crate::error::TaResult;
use crate::stream::invalid_period;

/// Wilder RSI applied to bar-over-bar changes in volume.
///
/// The first volume establishes a baseline. The next `period` changes seed
/// average gains and losses; subsequent changes use Wilder smoothing. A flat
/// volume stream returns the neutral value 50. This maps to Wickra `VolumeRsi`
/// 0.9.9 and has no direct TA-Lib equivalent.
#[derive(Debug, Clone)]
pub struct VolumeRelativeStrengthIndex {
    period: usize,
    previous_volume: Option<f64>,
    seed_gains: f64,
    seed_losses: f64,
    seed_count: usize,
    average_gain: Option<f64>,
    average_loss: Option<f64>,
    value: Option<f64>,
}

impl VolumeRelativeStrengthIndex {
    /// Creates the index with a positive Wilder smoothing period.
    pub fn new(period: usize) -> TaResult<Self> {
        if period == 0 {
            return Err(invalid_period("period", period, 1));
        }
        Ok(Self {
            period,
            previous_volume: None,
            seed_gains: 0.0,
            seed_losses: 0.0,
            seed_count: 0,
            average_gain: None,
            average_loss: None,
            value: None,
        })
    }

    /// Appends one volume observation and returns the latest warm value.
    pub fn append(&mut self, volume: f64) -> Option<f64> {
        let Some(previous_volume) = self.previous_volume.replace(volume) else {
            self.value = None;
            return None;
        };
        let change = volume - previous_volume;
        let gain = change.max(0.0);
        let loss = (-change).max(0.0);

        if let (Some(average_gain), Some(average_loss)) = (self.average_gain, self.average_loss) {
            let period = self.period as f64;
            let next_gain = (average_gain * (period - 1.0) + gain) / period;
            let next_loss = (average_loss * (period - 1.0) + loss) / period;
            self.average_gain = Some(next_gain);
            self.average_loss = Some(next_loss);
            self.value = Some(Self::from_averages(next_gain, next_loss));
            return self.value;
        }

        self.seed_gains += gain;
        self.seed_losses += loss;
        self.seed_count += 1;
        if self.seed_count == self.period {
            let period = self.period as f64;
            let average_gain = self.seed_gains / period;
            let average_loss = self.seed_losses / period;
            self.average_gain = Some(average_gain);
            self.average_loss = Some(average_loss);
            self.value = Some(Self::from_averages(average_gain, average_loss));
        }
        self.value
    }

    /// Returns the latest RSI value, or `None` during warm-up.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Restores fresh-state behavior without reallocating.
    pub fn reset(&mut self) {
        self.previous_volume = None;
        self.seed_gains = 0.0;
        self.seed_losses = 0.0;
        self.seed_count = 0;
        self.average_gain = None;
        self.average_loss = None;
        self.value = None;
    }

    fn from_averages(average_gain: f64, average_loss: f64) -> f64 {
        let total = average_gain + average_loss;
        if total == 0.0 {
            50.0
        } else {
            100.0 * average_gain / total
        }
    }
}
