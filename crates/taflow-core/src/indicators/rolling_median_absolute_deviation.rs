use crate::error::TaResult;
use crate::indicators::rolling_statistic_helpers::RollingValues;
use crate::stream::StreamingIndicator;

/// Raw rolling median absolute deviation, without Gaussian scaling.
#[derive(Debug, Clone)]
pub struct RollingMedianAbsoluteDeviation {
    values: RollingValues,
    scratch: Vec<f64>,
    value: Option<f64>,
}

impl RollingMedianAbsoluteDeviation {
    /// Create a rolling MAD state with a non-zero `period`.
    pub fn new(period: usize) -> TaResult<Self> {
        Ok(Self {
            values: RollingValues::new(period)?,
            scratch: Vec::with_capacity(period),
            value: None,
        })
    }

    /// Append one observation and return the raw MAD when the window is full.
    pub fn append(&mut self, input: f64) -> Option<f64> {
        self.values.push(input);
        if !self.values.is_full() {
            self.value = None;
            return None;
        }
        self.scratch.clear();
        self.scratch.extend(self.values.iter().copied());
        self.scratch.sort_by(f64::total_cmp);
        let median = median_sorted(&self.scratch);
        for item in &mut self.scratch {
            *item = (*item - median).abs();
        }
        self.scratch.sort_by(f64::total_cmp);
        self.value = Some(median_sorted(&self.scratch));
        self.value
    }

    /// Return the latest MAD, or `None` during warm-up.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Clear the rolling window and reusable scratch storage.
    pub fn reset(&mut self) {
        self.values.clear();
        self.scratch.clear();
        self.value = None;
    }
}

fn median_sorted(values: &[f64]) -> f64 {
    let middle = values.len() / 2;
    if values.len() % 2 == 0 {
        (values[middle - 1] + values[middle]) * 0.5
    } else {
        values[middle]
    }
}

impl StreamingIndicator for RollingMedianAbsoluteDeviation {
    type Output = f64;
    fn append(&mut self, input: f64) -> Option<f64> {
        Self::append(self, input)
    }
    fn value(&self) -> Option<f64> {
        self.value
    }
    fn reset(&mut self) {
        Self::reset(self)
    }
}
