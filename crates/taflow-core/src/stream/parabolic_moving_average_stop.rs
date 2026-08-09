//! Persistent Parabolic Moving Average Stop state.

use crate::error::{TaError, TaResult};
use std::collections::VecDeque;

/// The aligned PMAX outputs for one bar.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ParabolicMovingAverageStopValue {
    pub stop: f64,
    pub trend: i32,
}

/// EMA/ATR PMAX state with causal trend bands.
#[derive(Debug, Clone)]
pub struct ParabolicMovingAverageStop {
    period: usize,
    multiplier: f64,
    closes: VecDeque<f64>,
    previous_close: Option<f64>,
    true_range_sum: f64,
    true_range_count: usize,
    atr: Option<f64>,
    ema: Option<f64>,
    lower: Option<f64>,
    upper: Option<f64>,
    trend: i32,
    value: Option<ParabolicMovingAverageStopValue>,
}

impl ParabolicMovingAverageStop {
    /// Creates PMAX with a positive EMA/ATR lookback and multiplier.
    pub fn new(period: usize, multiplier: f64) -> TaResult<Self> {
        if period < 1 {
            return Err(super::invalid_period("length", period, 1));
        }
        Ok(Self {
            period,
            multiplier,
            closes: VecDeque::with_capacity(period),
            previous_close: None,
            true_range_sum: 0.0,
            true_range_count: 0,
            atr: None,
            ema: None,
            lower: None,
            upper: None,
            trend: 1,
            value: None,
        })
    }

    /// Appends one chronological high/low/close bar.
    pub fn append(
        &mut self,
        high: f64,
        low: f64,
        close: f64,
    ) -> Option<ParabolicMovingAverageStopValue> {
        self.closes.push_back(close);
        if self.closes.len() > self.period {
            self.closes.pop_front();
        }

        let previous_close = self.previous_close;
        self.previous_close = Some(close);
        if let Some(previous_close) = previous_close {
            let true_range = (high - low)
                .max((high - previous_close).abs())
                .max((low - previous_close).abs());
            self.true_range_count += 1;
            self.true_range_sum += true_range;
            let seed_count = self.period.saturating_sub(1).max(1);
            if self.true_range_count < seed_count {
                // The first valid ATR is seeded at the first period-1 true
                // ranges, matching pandas-ta-classic's Wilder seed.
            } else if self.atr.is_none() {
                self.atr = Some(self.true_range_sum / seed_count as f64);
            } else {
                let alpha = 1.0 / self.period as f64;
                let atr = self.atr.unwrap();
                self.atr = Some(atr + alpha * (true_range - atr));
            }
        }

        if self.closes.len() == self.period {
            if self.ema.is_none() {
                self.ema = Some(self.closes.iter().sum::<f64>() / self.period as f64);
            } else {
                let alpha = 2.0 / (self.period as f64 + 1.0);
                let ema = self.ema.unwrap();
                self.ema = Some(ema + alpha * (close - ema));
            }
        }

        let (Some(atr), Some(ema)) = (self.atr, self.ema) else {
            return None;
        };
        let mut lower = ema - self.multiplier * atr;
        let mut upper = ema + self.multiplier * atr;
        if let (Some(previous_lower), Some(previous_upper), Some(previous_close)) =
            (self.lower, self.upper, previous_close)
        {
            if previous_close > previous_lower {
                lower = lower.max(previous_lower);
            }
            if previous_close < previous_upper {
                upper = upper.min(previous_upper);
            }
            if close > previous_upper {
                self.trend = 1;
            } else if close < previous_lower {
                self.trend = -1;
            }
        }
        self.lower = Some(lower);
        self.upper = Some(upper);
        let value = ParabolicMovingAverageStopValue {
            stop: if self.trend == 1 { lower } else { upper },
            trend: self.trend,
        };
        self.value = Some(value);
        Some(value)
    }

    /// Extends output through the same state machine as scalar replay.
    pub fn extend_slice_into(
        &mut self,
        high: &[f64],
        low: &[f64],
        close: &[f64],
        stops: &mut Vec<f64>,
        trends: &mut Vec<i32>,
    ) -> TaResult<()> {
        if high.len() != low.len() || high.len() != close.len() {
            return Err(TaError::LengthMismatch {
                expected: high.len(),
                got: low.len().max(close.len()),
            });
        }
        for ((&high, &low), &close) in high.iter().zip(low).zip(close) {
            if let Some(value) = self.append(high, low, close) {
                stops.push(value.stop);
                trends.push(value.trend);
            } else {
                stops.push(f64::NAN);
                trends.push(0);
            }
        }
        Ok(())
    }

    /// Returns the latest PMAX value after warm-up.
    pub fn value(&self) -> Option<ParabolicMovingAverageStopValue> {
        self.value
    }

    /// Restores fresh-state behavior without reallocating buffers.
    pub fn reset(&mut self) {
        self.closes.clear();
        self.previous_close = None;
        self.true_range_sum = 0.0;
        self.true_range_count = 0;
        self.atr = None;
        self.ema = None;
        self.lower = None;
        self.upper = None;
        self.trend = 1;
        self.value = None;
    }
}
