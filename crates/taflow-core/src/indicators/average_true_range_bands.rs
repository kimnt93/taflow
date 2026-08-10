use crate::error::{TaError, TaResult};
use crate::stream::invalid_period;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AverageTrueRangeBandsValue {
    pub upper: f64,
    pub middle: f64,
    pub lower: f64,
}

/// Current close wrapped by a multiple of Wilder Average True Range.
#[derive(Debug, Clone)]
pub struct AverageTrueRangeBands {
    period: usize,
    multiplier: f64,
    previous_close: Option<f64>,
    true_range_count: usize,
    true_range_sum: f64,
    average_true_range: f64,
    value: Option<AverageTrueRangeBandsValue>,
}

impl AverageTrueRangeBands {
    /// Create ATR bands with a positive period and multiplier.
    pub fn new(period: usize, multiplier: f64) -> TaResult<Self> {
        if period == 0 {
            return Err(invalid_period("period", period, 1));
        }
        if !multiplier.is_finite() || multiplier <= 0.0 {
            return Err(TaError::InvalidParameter {
                name: "multiplier",
                value: multiplier.to_string(),
                reason: "must be finite and positive",
            });
        }
        Ok(Self {
            period,
            multiplier,
            previous_close: None,
            true_range_count: 0,
            true_range_sum: 0.0,
            average_true_range: 0.0,
            value: None,
        })
    }

    /// Append one high/low/close bar and return close-centered ATR bands.
    ///
    /// The first bar contributes `high - low` to the Wilder seed, matching
    /// Wickra `AtrBands`; this intentionally differs from TA-Lib's standalone
    /// ATR convention, which discards the first bar's range.
    pub fn append(
        &mut self,
        high: f64,
        low: f64,
        close: f64,
    ) -> Option<AverageTrueRangeBandsValue> {
        let true_range = match self.previous_close.replace(close) {
            Some(previous_close) => (high - low)
                .max((high - previous_close).abs())
                .max((low - previous_close).abs()),
            None => high - low,
        };
        self.true_range_count += 1;
        if self.true_range_count < self.period {
            self.true_range_sum += true_range;
            self.value = None;
            return None;
        }
        if self.true_range_count == self.period {
            self.average_true_range = (self.true_range_sum + true_range) / self.period as f64;
        } else {
            let period = self.period as f64;
            self.average_true_range =
                (self.average_true_range * (period - 1.0) + true_range) / period;
        }
        let width = self.multiplier * self.average_true_range;
        self.value = Some(AverageTrueRangeBandsValue {
            upper: close + width,
            middle: close,
            lower: close - width,
        });
        self.value
    }

    /// Return the latest three bands, or `None` during ATR warm-up.
    pub fn value(&self) -> Option<AverageTrueRangeBandsValue> {
        self.value
    }

    /// Restore fresh-state behavior without reallocating.
    pub fn reset(&mut self) {
        self.previous_close = None;
        self.true_range_count = 0;
        self.true_range_sum = 0.0;
        self.average_true_range = 0.0;
        self.value = None;
    }
}
