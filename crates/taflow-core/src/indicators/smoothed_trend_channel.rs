//! Stateful SSL Channel.

use crate::error::TaResult;
use std::collections::VecDeque;

/// Rolling high/low averages with a causal bullish/bearish side state.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `SmoothedTrendChannel`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct SmoothedTrendChannel {
    period: usize,
    highs: VecDeque<f64>,
    lows: VecDeque<f64>,
    high_sum: f64,
    low_sum: f64,
    side: i32,
    value: Option<(f64, f64)>,
}

impl SmoothedTrendChannel {
    /// Creates an SSL Channel with a positive rolling period.
    pub fn new(period: usize) -> TaResult<Self> {
        if period < 1 {
            return Err(crate::indicators::invalid_period("length", period, 1));
        }
        Ok(Self {
            period,
            highs: VecDeque::with_capacity(period),
            lows: VecDeque::with_capacity(period),
            high_sum: 0.0,
            low_sum: 0.0,
            side: 1,
            value: None,
        })
    }
}

impl SmoothedTrendChannel {
    /// Appends one aligned high/low/close bar.
    pub fn append(&mut self, high: f64, low: f64, close: f64) -> Option<(f64, f64)> {
        self.highs.push_back(high);
        self.high_sum += high;
        self.lows.push_back(low);
        self.low_sum += low;
        if self.highs.len() > self.period {
            self.high_sum -= self.highs.pop_front().unwrap();
            self.low_sum -= self.lows.pop_front().unwrap();
        }
        if self.highs.len() < self.period {
            return None;
        }
        let average_high = self.high_sum / self.period as f64;
        let average_low = self.low_sum / self.period as f64;
        if close > average_high {
            self.side = 1;
        } else if close < average_low {
            self.side = -1;
        }
        self.value = Some(if self.side > 0 {
            (average_low, average_high)
        } else {
            (average_high, average_low)
        });
        self.value
    }

    /// Returns the latest channel pair when warm.
    pub fn value(&self) -> Option<(f64, f64)> {
        self.value
    }

    /// Extends aligned OHLC slices while preserving scalar state semantics.
    pub fn extend_slice_into(
        &mut self,
        high: &[f64],
        low: &[f64],
        close: &[f64],
        lower: &mut Vec<f64>,
        upper: &mut Vec<f64>,
    ) -> TaResult<()> {
        if high.len() != low.len() || high.len() != close.len() {
            return Err(crate::error::TaError::LengthMismatch {
                expected: high.len(),
                got: low.len().max(close.len()),
            });
        }
        lower.reserve(high.len());
        upper.reserve(high.len());
        for ((&high, &low), &close) in high.iter().zip(low).zip(close) {
            let value = self.append(high, low, close);
            match value {
                Some((lower_value, upper_value)) => {
                    lower.push(lower_value);
                    upper.push(upper_value);
                }
                None => {
                    lower.push(f64::NAN);
                    upper.push(f64::NAN);
                }
            }
        }
        Ok(())
    }

    /// Clears history and restores the initial bullish side.
    pub fn reset(&mut self) {
        self.highs.clear();
        self.lows.clear();
        self.high_sum = 0.0;
        self.low_sum = 0.0;
        self.side = 1;
        self.value = None;
    }
}
