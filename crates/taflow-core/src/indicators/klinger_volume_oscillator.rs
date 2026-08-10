use super::invalid_period;
use crate::error::TaResult;
use crate::indicators::ExponentialMovingAverage;
use crate::stream::StreamingIndicator;

/// Klinger volume-force oscillator and its signal EMA.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct KlingerVolumeOscillatorValue {
    pub oscillator: f64,
    pub signal: f64,
}

/// Trend-aware cumulative-money-flow volume oscillator.
#[derive(Debug, Clone)]
pub struct KlingerVolumeOscillator {
    previous_measurement: Option<f64>,
    trend: i8,
    cumulative_measurement: f64,
    fast_average: ExponentialMovingAverage,
    slow_average: ExponentialMovingAverage,
    signal_average: ExponentialMovingAverage,
    oscillator_output: f64,
    signal_output: f64,
    value: Option<KlingerVolumeOscillatorValue>,
}

impl KlingerVolumeOscillator {
    /// Create Klinger and signal lines with positive fast/slow/signal periods.
    pub fn new(fast: usize, slow: usize, signal: usize) -> TaResult<Self> {
        for (name, period) in [("fast", fast), ("slow", slow), ("signal", signal)] {
            if period == 0 {
                return Err(invalid_period(name, period, 1));
            }
        }
        if fast >= slow {
            return Err(crate::error::TaError::InvalidParameter {
                name: "fast",
                value: fast.to_string(),
                reason: "must be less than slow",
            });
        }
        Ok(Self {
            previous_measurement: None,
            trend: 0,
            cumulative_measurement: 0.0,
            fast_average: ExponentialMovingAverage::new(fast)?,
            slow_average: ExponentialMovingAverage::new(slow)?,
            signal_average: ExponentialMovingAverage::new(signal)?,
            oscillator_output: f64::NAN,
            signal_output: f64::NAN,
            value: None,
        })
    }

    /// Append one high/low/close/volume bar and update both output lines.
    pub fn append(
        &mut self,
        high: f64,
        low: f64,
        close: f64,
        volume: f64,
    ) -> Option<KlingerVolumeOscillatorValue> {
        let measurement = high + low + close;
        let Some(previous) = self.previous_measurement.replace(measurement) else {
            return None;
        };
        let next_trend = if measurement > previous {
            1
        } else if measurement < previous {
            -1
        } else {
            self.trend
        };
        if next_trend != self.trend || self.trend == 0 {
            self.cumulative_measurement = previous + measurement;
        } else {
            self.cumulative_measurement += measurement;
        }
        self.trend = next_trend;
        let force = if self.cumulative_measurement == 0.0 {
            0.0
        } else {
            volume
                * (2.0 * (measurement / self.cumulative_measurement - 1.0)).abs()
                * f64::from(next_trend)
                * 100.0
        };

        let fast = self.fast_average.append(force);
        let slow = self.slow_average.append(force);
        self.oscillator_output = fast.zip(slow).map_or(f64::NAN, |(a, b)| a - b);
        self.signal_output = if self.oscillator_output.is_nan() {
            f64::NAN
        } else {
            self.signal_average
                .append(self.oscillator_output)
                .unwrap_or(f64::NAN)
        };
        self.value = self
            .signal_output
            .is_finite()
            .then_some(KlingerVolumeOscillatorValue {
                oscillator: self.oscillator_output,
                signal: self.signal_output,
            });
        self.value
    }

    /// Return aligned oscillator and signal scalars, using NaN during warm-up.
    pub fn outputs(&self) -> (f64, f64) {
        (self.oscillator_output, self.signal_output)
    }

    /// Extend aligned OHLCV slices through the same scalar state machine.
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

    /// Return both lines once the signal EMA has warmed.
    pub fn value(&self) -> Option<KlingerVolumeOscillatorValue> {
        self.value
    }

    /// Clear all trend, cumulative measurement, EMA, and output state.
    pub fn reset(&mut self) {
        self.previous_measurement = None;
        self.trend = 0;
        self.cumulative_measurement = 0.0;
        self.fast_average.reset();
        self.slow_average.reset();
        self.signal_average.reset();
        self.oscillator_output = f64::NAN;
        self.signal_output = f64::NAN;
        self.value = None;
    }
}
