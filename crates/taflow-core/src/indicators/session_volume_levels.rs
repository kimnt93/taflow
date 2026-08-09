//! Persistent fixed-bin session volume profile levels.

use crate::error::{TaError, TaResult};

/// Point of control and value-area bounds for one session bar.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SessionVolumeLevelsValue {
    pub point_of_control: f64,
    pub value_area_high: f64,
    pub value_area_low: f64,
}

/// Fixed-bin session volume profile state.
#[derive(Debug, Clone)]
pub struct SessionVolumeLevels {
    bins: usize,
    value_area: f64,
    low: Option<f64>,
    high: f64,
    step: f64,
    histogram: Vec<f64>,
    value: Option<SessionVolumeLevelsValue>,
}

impl SessionVolumeLevels {
    pub fn new(bins: usize, value_area: f64) -> TaResult<Self> {
        if bins < 1 {
            return Err(crate::indicators::invalid_period("bins", bins, 1));
        }
        if !(0.0..=1.0).contains(&value_area) || value_area == 0.0 {
            return Err(TaError::InvalidParameter {
                name: "value_area",
                value: value_area.to_string(),
                reason: "must be in (0, 1]",
            });
        }
        Ok(Self {
            bins,
            value_area,
            low: None,
            high: 0.0,
            step: 1.0,
            histogram: vec![0.0; bins],
            value: None,
        })
    }

    pub fn append(
        &mut self,
        high: f64,
        low: f64,
        close: f64,
        volume: f64,
        anchor: bool,
    ) -> SessionVolumeLevelsValue {
        if anchor || self.low.is_none() {
            self.low = Some(low);
            self.high = high;
            self.step = ((high - low) / self.bins as f64).max(1.0e-12);
            self.histogram.fill(0.0);
        }
        let session_low = self.low.as_mut().expect("initialized above");
        *session_low = session_low.min(low);
        self.high = self.high.max(high);
        let low_value = *session_low;
        let index =
            (((close - low_value) / self.step) as isize).clamp(0, self.bins as isize - 1) as usize;
        self.histogram[index] += volume;
        let mut poc = 0usize;
        let mut total = 0.0;
        for (bin, &value) in self.histogram.iter().enumerate() {
            if self.histogram[poc] < value {
                poc = bin;
            }
            total += value;
        }
        let target = total * self.value_area;
        let (mut left, mut right, mut accumulated) = (poc, poc, self.histogram[poc]);
        while accumulated < target && (left > 0 || right + 1 < self.bins) {
            if left == 0 {
                right += 1;
            } else if right + 1 == self.bins {
                left -= 1;
            } else if self.histogram[left - 1] >= self.histogram[right + 1] {
                left -= 1;
            } else {
                right += 1;
            }
            accumulated = self.histogram[left..=right].iter().sum();
        }
        let value = SessionVolumeLevelsValue {
            point_of_control: (poc as f64 + 0.5) * self.step + low_value,
            value_area_high: (right as f64 + 0.5) * self.step + low_value,
            value_area_low: (left as f64 + 0.5) * self.step + low_value,
        };
        self.value = Some(value);
        value
    }

    pub fn extend_slice_into(
        &mut self,
        high: &[f64],
        low: &[f64],
        close: &[f64],
        volume: &[f64],
        anchor: &[bool],
        point_of_control: &mut Vec<f64>,
        value_area_high: &mut Vec<f64>,
        value_area_low: &mut Vec<f64>,
    ) -> TaResult<()> {
        if high.len() != low.len()
            || high.len() != close.len()
            || high.len() != volume.len()
            || high.len() != anchor.len()
        {
            return Err(TaError::LengthMismatch {
                expected: high.len(),
                got: low
                    .len()
                    .max(close.len())
                    .max(volume.len())
                    .max(anchor.len()),
            });
        }
        for ((((&high, &low), &close), &volume), &anchor) in
            high.iter().zip(low).zip(close).zip(volume).zip(anchor)
        {
            let value = self.append(high, low, close, volume, anchor);
            point_of_control.push(value.point_of_control);
            value_area_high.push(value.value_area_high);
            value_area_low.push(value.value_area_low);
        }
        Ok(())
    }

    pub fn value(&self) -> Option<SessionVolumeLevelsValue> {
        self.value
    }

    pub fn reset(&mut self) {
        self.low = None;
        self.high = 0.0;
        self.step = 1.0;
        self.histogram.fill(0.0);
        self.value = None;
    }
}
