//! Incremental Demand Index.

use crate::error::TaResult;
use crate::indicators::ExponentialMovingAverage;
use crate::stream::StreamingIndicator;

/// EMA-smoothed price/volume pressure based on consecutive closes.
///
/// For each bar after the first, pressure is volume multiplied by close return
/// and by one plus the high-low range normalized by the previous close. A zero
/// previous close contributes zero pressure. This streaming definition matches
/// Wickra `DemandIndex` 0.9.9.
#[derive(Debug, Clone)]
pub struct DemandIndex {
    pressure_average: ExponentialMovingAverage,
    previous_close: Option<f64>,
    value: Option<f64>,
}

impl DemandIndex {
    /// Creates the index with a positive EMA smoothing period.
    pub fn new(timeperiod: usize) -> TaResult<Self> {
        Ok(Self {
            pressure_average: ExponentialMovingAverage::new(timeperiod)?,
            previous_close: None,
            value: None,
        })
    }

    /// Appends one high/low/close/volume bar and returns the latest warm value.
    pub fn append(&mut self, high: f64, low: f64, close: f64, volume: f64) -> Option<f64> {
        let Some(previous_close) = self.previous_close.replace(close) else {
            self.value = None;
            return None;
        };

        let pressure = if previous_close == 0.0 {
            0.0
        } else {
            let close_return = (close - previous_close) / previous_close;
            let normalized_range = (high - low) / previous_close;
            volume * close_return * (1.0 + normalized_range)
        };
        self.value = self.pressure_average.append(pressure);
        self.value
    }

    /// Returns the latest demand value, or `None` during warm-up.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Clears the previous close, EMA state, and latest value.
    pub fn reset(&mut self) {
        self.pressure_average.reset();
        self.previous_close = None;
        self.value = None;
    }
}
