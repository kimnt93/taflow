//! Persistent rising-direction state.

use crate::error::TaResult;
use crate::stream::{operator_states::validate_period, StreamingIndicator};
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct Rising {
    values: VecDeque<f64>,
    period: usize,
    value: Option<f64>,
}

impl Rising {
    pub fn new(period: usize) -> TaResult<Self> {
        validate_period(period)?;
        Ok(Self {
            values: VecDeque::with_capacity(period + 1),
            period,
            value: None,
        })
    }
    pub fn append(&mut self, input: f64) -> Option<f64> {
        if self.values.len() == self.period + 1 {
            self.values.pop_front();
        }
        self.values.push_back(input);
        self.value = (self.values.len() == self.period + 1).then(|| {
            if input > self.values.front().copied().unwrap() {
                1.0
            } else {
                0.0
            }
        });
        self.value
    }
    /// Append a slice and write aligned values, using `NaN` during warm-up.
    pub fn extend_slice_into(&mut self, input: &[f64], output: &mut Vec<f64>) {
        if input.is_empty() {
            return;
        }
        let output_start = output.len();
        output.resize(output_start + input.len(), f64::NAN);

        let retained = self.values.len();
        let warmup = self.period.saturating_sub(retained);
        let retained_end = input.len().min(self.period);
        let retained_begin = warmup.min(retained_end);
        let retained_start = retained.saturating_sub(self.period);
        for ((&current, &lagged), output) in input[retained_begin..retained_end]
            .iter()
            .zip(self.values.iter().skip(retained_start))
            .zip(&mut output[output_start + retained_begin..output_start + retained_end])
        {
            *output = if current > lagged { 1.0 } else { 0.0 };
        }
        if input.len() > self.period {
            for ((&current, &lagged), output) in input[self.period..]
                .iter()
                .zip(&input[..input.len() - self.period])
                .zip(&mut output[output_start + self.period..])
            {
                *output = if current > lagged { 1.0 } else { 0.0 };
            }
        }
        self.value =
            (retained + input.len() > self.period).then(|| output[output_start + input.len() - 1]);

        let capacity = self.period + 1;
        if input.len() >= capacity {
            self.values.clear();
            self.values.extend(&input[input.len() - capacity..]);
        } else {
            let overflow = (retained + input.len()).saturating_sub(capacity);
            self.values.drain(..overflow);
            self.values.extend(input);
        }
    }
    pub fn value(&self) -> Option<f64> {
        self.value
    }
    pub fn reset(&mut self) {
        self.values.clear();
        self.value = None;
    }
}

impl StreamingIndicator for Rising {
    type Output = f64;

    fn append(&mut self, input: f64) -> Option<Self::Output> {
        Self::append(self, input)
    }

    fn value(&self) -> Option<Self::Output> {
        Self::value(self)
    }

    fn reset(&mut self) {
        Self::reset(self);
    }
}
