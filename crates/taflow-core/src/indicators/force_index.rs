use crate::error::TaResult;
use crate::indicators::ExponentialMovingAverage;
use crate::stream::StreamingIndicator;

/// EMA-smoothed close-to-close price force weighted by volume.
#[derive(Debug, Clone)]
pub struct ForceIndex {
    previous_close: Option<f64>,
    average: ExponentialMovingAverage,
    count: usize,
    value: Option<f64>,
}

impl ForceIndex {
    /// Create a Force Index with the requested EMA smoothing period.
    pub fn new(period: usize) -> TaResult<Self> {
        Ok(Self {
            previous_close: None,
            average: ExponentialMovingAverage::new(period)?,
            count: 0,
            value: None,
        })
    }

    /// Append one close/volume pair and return the smoothed force when ready.
    pub fn append(&mut self, close: f64, volume: f64) -> Option<f64> {
        self.count += 1;
        let Some(previous) = self.previous_close.replace(close) else {
            return None;
        };
        self.value = self.average.append((close - previous) * volume);
        self.value
    }

    /// Return the latest smoothed force, or `None` during warm-up.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Return the number of processed close/volume pairs.
    pub fn len(&self) -> usize {
        self.count
    }

    /// Return whether no pairs have been processed.
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    /// Clear previous-close, EMA, count, and latest-value state.
    pub fn reset(&mut self) {
        self.previous_close = None;
        self.average.reset();
        self.count = 0;
        self.value = None;
    }
}
