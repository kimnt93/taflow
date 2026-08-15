use crate::error::TaResult;
use crate::stream::rolling_extrema::{tracked_index_rescan_into, MonotonicArgmax, MonotonicArgmin};

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
        let period = self.maximum.period();
        if self.maximum.count() == 0 && inputs.len() >= period {
            let min_start = min_out.len();
            let max_start = max_out.len();
            min_out.resize(min_start + inputs.len(), 0.0);
            max_out.resize(max_start + inputs.len(), 0.0);
            let minimum =
                tracked_index_rescan_into::<false>(inputs, period, &mut min_out[min_start..]);
            let maximum =
                tracked_index_rescan_into::<true>(inputs, period, &mut max_out[max_start..]);
            self.minimum.rebuild_from_full_run(inputs, minimum);
            self.maximum.rebuild_from_full_run(inputs, maximum);
            self.value = Some(RollingMinMaxIndexValue { minimum, maximum });
            return;
        }
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
