use super::rolling_extrema::{MonotonicArgmax, MonotonicArgmin};
use crate::error::TaResult;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RollingMinMaxIndexValue {
    pub minimum: usize,
    pub maximum: usize,
}

#[derive(Debug, Clone)]
pub struct RollingMinMaxIndex {
    maximum: MonotonicArgmax,
    minimum: MonotonicArgmin,
    value: Option<RollingMinMaxIndexValue>,
}

impl RollingMinMaxIndex {
    pub fn new(period: usize) -> TaResult<Self> {
        Ok(Self {
            maximum: MonotonicArgmax::new(period)?,
            minimum: MonotonicArgmin::new(period)?,
            value: None,
        })
    }

    pub fn append(&mut self, input: f64) -> RollingMinMaxIndexValue {
        let value = RollingMinMaxIndexValue {
            minimum: self.minimum.append(input).unwrap_or(0),
            maximum: self.maximum.append(input).unwrap_or(0),
        };
        self.value = Some(value);
        value
    }

    pub fn extend_slices_into(
        &mut self,
        inputs: &[f64],
        min_out: &mut Vec<f64>,
        max_out: &mut Vec<f64>,
    ) {
        min_out.reserve(inputs.len());
        max_out.reserve(inputs.len());
        for &input in inputs {
            let value = self.append(input);
            min_out.push(value.minimum as f64);
            max_out.push(value.maximum as f64);
        }
    }

    pub fn value(&self) -> Option<RollingMinMaxIndexValue> {
        self.value
    }

    pub fn reset(&mut self) {
        self.maximum.reset();
        self.minimum.reset();
        self.value = None;
    }
}
