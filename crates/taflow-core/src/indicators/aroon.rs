//! Persistent Aroon state.

use crate::error::{TaError, TaResult};
use crate::stream::{aroon_rescan::aroon_rescan, invalid_period, MonotonicMax, MonotonicMin};

/// Named Aroon down/up result for one warmed bar.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AroonValue {
    pub down: f64,
    pub up: f64,
}

/// Track the latest trailing high and low over a `period + 1` bar window.
#[derive(Debug, Clone)]
pub struct Aroon {
    period: usize,
    inverse_period: f64,
    index: usize,
    highs: MonotonicMax,
    lows: MonotonicMin,
    value: Option<AroonValue>,
}

impl Aroon {
    /// Create an Aroon state with a lookback of at least two bars.
    pub fn new(period: usize) -> TaResult<Self> {
        if period < 2 {
            return Err(invalid_period("timeperiod", period, 2));
        }
        Ok(Self {
            period,
            inverse_period: 100.0 / period as f64,
            index: 0,
            highs: MonotonicMax::new(period + 1)?,
            lows: MonotonicMin::new(period + 1)?,
            value: None,
        })
    }

    /// Append one chronological high/low pair.
    pub fn append(&mut self, high: f64, low: f64) -> Option<AroonValue> {
        let current = self.index;
        self.index += 1;
        let highest = self.highs.append_indexed(high).map(|(index, _)| index);
        let lowest = self.lows.append_indexed(low).map(|(index, _)| index);
        self.value = highest.zip(lowest).map(|(highest, lowest)| AroonValue {
            down: (self.period - (current - lowest)) as f64 * self.inverse_period,
            up: (self.period - (current - highest)) as f64 * self.inverse_period,
        });
        self.value
    }

    /// Append aligned slices, writing named down/up histories with NaN warm-up.
    pub fn extend_slices_into(
        &mut self,
        high: &[f64],
        low: &[f64],
        down_output: &mut Vec<f64>,
        up_output: &mut Vec<f64>,
    ) -> TaResult<()> {
        let length = self.validate_lengths(high, low)?;
        if self.index != 0 || length < self.period + 1 {
            down_output.reserve(length);
            up_output.reserve(length);
            for index in 0..length {
                match self.append(high[index], low[index]) {
                    Some(value) => {
                        down_output.push(value.down);
                        up_output.push(value.up);
                    }
                    None => {
                        down_output.push(f64::NAN);
                        up_output.push(f64::NAN);
                    }
                }
            }
            return Ok(());
        }

        let down_start = down_output.len();
        let up_start = up_output.len();
        down_output.resize(down_start + length, f64::NAN);
        up_output.resize(up_start + length, f64::NAN);
        let downs = &mut down_output[down_start..];
        let ups = &mut up_output[up_start..];
        aroon_rescan(
            high,
            low,
            self.period,
            self.inverse_period,
            |today, down, up| {
                downs[today] = down;
                ups[today] = up;
            },
        );
        self.finish_bulk_run(high, low);
        self.value = Some(AroonValue {
            down: *down_output.last().expect("at least one warmed bar"),
            up: *up_output.last().expect("at least one warmed bar"),
        });
        Ok(())
    }

    /// Append aligned slices for an owning oscillator without materializing
    /// intermediate down/up histories.
    pub(crate) fn extend_oscillator_into(
        &mut self,
        high: &[f64],
        low: &[f64],
        output: &mut Vec<f64>,
    ) -> TaResult<()> {
        let length = self.validate_lengths(high, low)?;
        if self.index != 0 || length < self.period + 1 {
            output.reserve(length);
            for index in 0..length {
                output.push(
                    self.append(high[index], low[index])
                        .map_or(f64::NAN, |value| value.up - value.down),
                );
            }
            return Ok(());
        }

        let start = output.len();
        output.resize(start + length, f64::NAN);
        let slots = &mut output[start..];
        let mut last = AroonValue { down: 0.0, up: 0.0 };
        aroon_rescan(
            high,
            low,
            self.period,
            self.inverse_period,
            |today, down, up| {
                slots[today] = up - down;
                last = AroonValue { down, up };
            },
        );
        self.finish_bulk_run(high, low);
        self.value = Some(last);
        Ok(())
    }

    fn validate_lengths(&self, high: &[f64], low: &[f64]) -> TaResult<usize> {
        if high.len() != low.len() {
            return Err(TaError::LengthMismatch {
                expected: high.len(),
                got: low.len(),
            });
        }
        Ok(high.len())
    }

    fn finish_bulk_run(&mut self, high: &[f64], low: &[f64]) {
        self.highs.rebuild_from_full_run(high);
        self.lows.rebuild_from_full_run(low);
        self.index = high.len();
    }

    /// Return the latest named result, or `None` during warm-up.
    pub fn value(&self) -> Option<AroonValue> {
        self.value
    }

    /// Restore fresh-state behavior without reallocating.
    pub fn reset(&mut self) {
        self.index = 0;
        self.highs.reset();
        self.lows.reset();
        self.value = None;
    }
}
