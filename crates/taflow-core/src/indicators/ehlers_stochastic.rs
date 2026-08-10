use std::collections::VecDeque;

use crate::error::TaResult;
use crate::indicators::RoofingFilter;
use crate::stream::{invalid_period, StreamingIndicator};

/// Minus-one-to-one stochastic oscillator of an Ehlers roofing-filter output.
#[derive(Debug, Clone)]
pub struct EhlersStochastic {
    period: usize,
    roofing: RoofingFilter,
    filtered: VecDeque<f64>,
    previous_raw: Option<f64>,
    value: Option<f64>,
}

impl EhlersStochastic {
    /// Create an oscillator with a non-zero stochastic lookback.
    pub fn new(period: usize) -> TaResult<Self> {
        if period == 0 {
            return Err(invalid_period("period", period, 1));
        }
        Ok(Self {
            period,
            roofing: RoofingFilter::new(10, 48)?,
            filtered: VecDeque::with_capacity(period),
            previous_raw: None,
            value: None,
        })
    }

    /// Append one price and return the two-bar-smoothed normalized oscillator.
    pub fn append(&mut self, input: f64) -> Option<f64> {
        let filtered = self.roofing.append(input)?;
        if self.filtered.len() == self.period {
            self.filtered.pop_front();
        }
        self.filtered.push_back(filtered);
        if self.filtered.len() < self.period {
            self.value = None;
            return None;
        }
        let high = self
            .filtered
            .iter()
            .copied()
            .fold(f64::NEG_INFINITY, f64::max);
        let low = self.filtered.iter().copied().fold(f64::INFINITY, f64::min);
        let raw = if high > low {
            2.0 * (filtered - low) / (high - low) - 1.0
        } else {
            0.0
        };
        self.value = Some(
            self.previous_raw
                .map_or(raw, |previous| 0.5 * (raw + previous)),
        );
        self.previous_raw = Some(raw);
        self.value
    }

    /// Return the latest normalized oscillator value.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Clear child filter, stochastic window, and smoothing state.
    pub fn reset(&mut self) {
        self.roofing.reset();
        self.filtered.clear();
        self.previous_raw = None;
        self.value = None;
    }
}

impl StreamingIndicator for EhlersStochastic {
    type Output = f64;
    fn append(&mut self, input: f64) -> Option<f64> {
        Self::append(self, input)
    }
    fn value(&self) -> Option<f64> {
        self.value
    }
    fn reset(&mut self) {
        Self::reset(self)
    }
}
