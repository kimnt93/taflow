//! Stateful Klinger volume oscillator.

use crate::error::TaResult;
use super::invalid_period;

/// Fast/slow EMA difference of signed volume force and its signal EMA.
#[derive(Debug, Clone)]
pub struct KlingerVolumeOscillator {
    fast: usize,
    slow: usize,
    signal: usize,
    previous_typical_price: Option<f64>,
    fast_average: Option<f64>,
    slow_average: Option<f64>,
    signal_average: Option<f64>,
    value: Option<(f64, f64)>,
}

impl KlingerVolumeOscillator {
    /// Creates the oscillator with positive fast, slow, and signal periods.
    pub fn new(fast: usize, slow: usize, signal: usize) -> TaResult<Self> {
        for (name, period) in [("fast", fast), ("slow", slow), ("signal", signal)] {
            if period < 1 { return Err(invalid_period(name, period, 1)); }
        }
        Ok(Self { fast, slow, signal, previous_typical_price: None, fast_average: None, slow_average: None, signal_average: None, value: None })
    }

    /// Appends one OHLCV bar and returns oscillator and signal values.
    pub fn append(&mut self, high: f64, low: f64, close: f64, volume: f64) -> (f64, f64) {
        let typical_price = (high + low + close) / 3.0;
        let trend = if self.previous_typical_price.map_or(true, |previous| typical_price >= previous) { 1.0 } else { -1.0 };
        let force = trend * volume * (high - low);
        self.previous_typical_price = Some(typical_price);
        let fast_alpha = 2.0 / (self.fast as f64 + 1.0);
        let slow_alpha = 2.0 / (self.slow as f64 + 1.0);
        let signal_alpha = 2.0 / (self.signal as f64 + 1.0);
        let fast_average = *self.fast_average.get_or_insert(force);
        self.fast_average = Some(fast_average + fast_alpha * (force - fast_average));
        let slow_average = *self.slow_average.get_or_insert(force);
        self.slow_average = Some(slow_average + slow_alpha * (force - slow_average));
        let oscillator = self.fast_average.unwrap() - self.slow_average.unwrap();
        let signal_average = *self.signal_average.get_or_insert(oscillator);
        self.signal_average = Some(signal_average + signal_alpha * (oscillator - signal_average));
        let result = (oscillator, self.signal_average.unwrap()); self.value = Some(result); result
    }

    /// Returns the latest oscillator and signal pair.
    pub fn value(&self) -> Option<(f64, f64)> { self.value }
    /// Clears all EMA and previous-price state.
    pub fn reset(&mut self) { self.previous_typical_price = None; self.fast_average = None; self.slow_average = None; self.signal_average = None; self.value = None; }
}
