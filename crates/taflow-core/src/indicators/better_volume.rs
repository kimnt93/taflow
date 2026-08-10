use std::collections::VecDeque;

use crate::error::TaResult;
use crate::stream::invalid_period;

/// Volume-spread-analysis effort relative to price-range result.
#[derive(Debug, Clone)]
pub struct BetterVolume {
    period: usize,
    volumes: VecDeque<f64>,
    ranges: VecDeque<f64>,
    volume_sum: f64,
    range_sum: f64,
    value: Option<f64>,
}

impl BetterVolume {
    /// Create a Better Volume state using trailing simple averages.
    pub fn new(period: usize) -> TaResult<Self> {
        if period == 0 {
            return Err(invalid_period("period", period, 1));
        }
        Ok(Self {
            period,
            volumes: VecDeque::with_capacity(period),
            ranges: VecDeque::with_capacity(period),
            volume_sum: 0.0,
            range_sum: 0.0,
            value: None,
        })
    }

    /// Append one OHLCV bar; close is accepted to preserve the OHLCV API order.
    pub fn append(&mut self, high: f64, low: f64, _close: f64, volume: f64) -> Option<f64> {
        let range = high - low;
        if self.volumes.len() == self.period {
            self.volume_sum -= self.volumes.pop_front().expect("full volume window");
            self.range_sum -= self.ranges.pop_front().expect("full range window");
        }
        self.volumes.push_back(volume);
        self.ranges.push_back(range);
        self.volume_sum += volume;
        self.range_sum += range;
        self.value = (self.volumes.len() == self.period).then(|| {
            let n = self.period as f64;
            let average_volume = self.volume_sum / n;
            let average_range = self.range_sum / n;
            let relative_volume = if average_volume > 0.0 {
                volume / average_volume
            } else {
                0.0
            };
            let relative_range = if average_range > 0.0 {
                range / average_range
            } else {
                0.0
            };
            relative_volume - relative_range
        });
        self.value
    }

    /// Return the latest oscillator value, or `None` during warm-up.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Clear both windows, sums, and the latest value.
    pub fn reset(&mut self) {
        self.volumes.clear();
        self.ranges.clear();
        self.volume_sum = 0.0;
        self.range_sum = 0.0;
        self.value = None;
    }
}
