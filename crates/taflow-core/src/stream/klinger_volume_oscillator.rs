//! Stateful Klinger volume oscillator.

use super::invalid_period;
use crate::error::TaResult;

/// Fast/slow EMA difference and signal for one bar.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct KlingerVolumeOscillatorValue {
    pub oscillator: f64,
    pub signal: f64,
}

/// Fast/slow EMA difference of signed volume force and its signal EMA.
#[derive(Debug, Clone)]
pub struct KlingerVolumeOscillator {
    previous_typical_price: Option<f64>,
    fast_average: SeededEma,
    slow_average: SeededEma,
    signal_average: SeededEma,
    oscillator_output: f64,
    signal_output: f64,
    value: Option<KlingerVolumeOscillatorValue>,
}

#[derive(Debug, Clone)]
struct SeededEma {
    period: usize,
    count: usize,
    sum: f64,
    value: Option<f64>,
}

impl SeededEma {
    fn new(period: usize) -> Self {
        Self {
            period,
            count: 0,
            sum: 0.0,
            value: None,
        }
    }

    fn append(&mut self, input: f64) -> Option<f64> {
        self.count += 1;
        self.value = if self.count < self.period {
            self.sum += input;
            None
        } else if self.count == self.period {
            self.sum += input;
            Some(self.sum / self.period as f64)
        } else {
            let alpha = 2.0 / (self.period as f64 + 1.0);
            self.value
                .map(|previous| previous + alpha * (input - previous))
        };
        self.value
    }

    fn reset(&mut self) {
        self.count = 0;
        self.sum = 0.0;
        self.value = None;
    }
}

impl KlingerVolumeOscillator {
    /// Creates the oscillator with positive fast, slow, and signal periods.
    pub fn new(fast: usize, slow: usize, signal: usize) -> TaResult<Self> {
        for (name, period) in [("fast", fast), ("slow", slow), ("signal", signal)] {
            if period < 1 {
                return Err(invalid_period(name, period, 1));
            }
        }
        Ok(Self {
            previous_typical_price: None,
            fast_average: SeededEma::new(fast),
            slow_average: SeededEma::new(slow),
            signal_average: SeededEma::new(signal),
            oscillator_output: f64::NAN,
            signal_output: f64::NAN,
            value: None,
        })
    }

    /// Appends one OHLCV bar and returns oscillator and signal values.
    pub fn append(
        &mut self,
        high: f64,
        low: f64,
        close: f64,
        volume: f64,
    ) -> Option<KlingerVolumeOscillatorValue> {
        let typical_price = (high + low + close) / 3.0;
        let trend = self.previous_typical_price.map_or(1.0, |previous| {
            if typical_price > previous {
                1.0
            } else if typical_price < previous {
                -1.0
            } else {
                0.0
            }
        });
        let force = trend * volume;
        self.previous_typical_price = Some(typical_price);
        let fast_average = self.fast_average.append(force);
        let slow_average = self.slow_average.append(force);
        let oscillator = fast_average
            .zip(slow_average)
            .map_or(f64::NAN, |(fast, slow)| fast - slow);
        let signal_average = if oscillator.is_nan() {
            f64::NAN
        } else {
            self.signal_average.append(oscillator).unwrap_or(f64::NAN)
        };
        self.oscillator_output = oscillator;
        self.signal_output = signal_average;
        let result = signal_average
            .is_finite()
            .then_some(KlingerVolumeOscillatorValue {
                oscillator,
                signal: signal_average,
            });
        self.value = result;
        result
    }

    /// Returns aligned oscillator and signal outputs for the latest bar.
    pub fn outputs(&self) -> (f64, f64) {
        (self.oscillator_output, self.signal_output)
    }

    /// Extends aligned OHLCV slices through the scalar state machine.
    pub fn extend_slice_into(
        &mut self,
        high: &[f64],
        low: &[f64],
        close: &[f64],
        volume: &[f64],
        oscillator: &mut Vec<f64>,
        signal: &mut Vec<f64>,
    ) -> TaResult<()> {
        if high.len() != low.len() || high.len() != close.len() || high.len() != volume.len() {
            return Err(crate::error::TaError::LengthMismatch {
                expected: high.len(),
                got: low.len().max(close.len()).max(volume.len()),
            });
        }
        for (((&high, &low), &close), &volume) in high.iter().zip(low).zip(close).zip(volume) {
            self.append(high, low, close, volume);
            let (oscillator_value, signal_value) = self.outputs();
            oscillator.push(oscillator_value);
            signal.push(signal_value);
        }
        Ok(())
    }

    /// Returns the latest oscillator and signal pair.
    pub fn value(&self) -> Option<KlingerVolumeOscillatorValue> {
        self.value
    }
    /// Clears all EMA and previous-price state.
    pub fn reset(&mut self) {
        self.previous_typical_price = None;
        self.fast_average.reset();
        self.slow_average.reset();
        self.signal_average.reset();
        self.oscillator_output = f64::NAN;
        self.signal_output = f64::NAN;
        self.value = None;
    }
}
