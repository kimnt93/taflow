use crate::error::TaResult;
use crate::stream::rolling_extrema::RollingExtrema;
use crate::stream::vhgw;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RollingMinMaxValue {
    pub minimum: f64,
    pub maximum: f64,
}

#[derive(Debug, Clone)]
pub struct RollingMinMax {
    extrema: RollingExtrema,
    value: Option<RollingMinMaxValue>,
}

impl RollingMinMax {
    pub fn new(period: usize) -> TaResult<Self> {
        Ok(Self {
            extrema: RollingExtrema::new(period)?,
            value: None,
        })
    }

    pub fn append(&mut self, input: f64) -> Option<RollingMinMaxValue> {
        self.value = self
            .extrema
            .append(input)
            .map(|(maximum, minimum)| RollingMinMaxValue { minimum, maximum });
        self.value
    }

    pub fn extend_slices_into(
        &mut self,
        inputs: &[f64],
        min_out: &mut Vec<f64>,
        max_out: &mut Vec<f64>,
    ) {
        let period = self.extrema.period();
        if self.extrema.count() != 0 || inputs.len() < period {
            min_out.reserve(inputs.len());
            max_out.reserve(inputs.len());
            for &input in inputs {
                match self.append(input) {
                    Some(value) => {
                        min_out.push(value.minimum);
                        max_out.push(value.maximum);
                    }
                    None => {
                        min_out.push(f64::NAN);
                        max_out.push(f64::NAN);
                    }
                }
            }
            return;
        }
        let min_start = min_out.len();
        let max_start = max_out.len();
        min_out.resize(min_start + inputs.len(), f64::NAN);
        max_out.resize(max_start + inputs.len(), f64::NAN);
        vhgw::sliding_max_into(inputs, period, &mut max_out[max_start + period - 1..]);
        vhgw::sliding_min_into(inputs, period, &mut min_out[min_start + period - 1..]);
        self.extrema.rebuild_from_full_run(inputs);
        self.value = Some(RollingMinMaxValue {
            minimum: *min_out.last().expect("at least one warmed bar"),
            maximum: *max_out.last().expect("at least one warmed bar"),
        });
    }

    pub fn value(&self) -> Option<RollingMinMaxValue> {
        self.value
    }

    pub fn reset(&mut self) {
        self.extrema.reset();
        self.value = None;
    }
}
