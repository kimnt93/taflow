use crate::stream::StreamingIndicator;

/// Persistent one-based count of processed observations.
#[derive(Debug, Clone, Default)]
pub struct CumulativeCount {
    count: usize,
    value: Option<f64>,
}

impl CumulativeCount {
    /// Create an empty counter.
    pub fn new() -> Self {
        Self::default()
    }

    /// Count one aligned observation and return the new one-based count.
    pub fn append(&mut self, _input: f64) -> f64 {
        self.count += 1;
        let value = self.count as f64;
        self.value = Some(value);
        value
    }

    /// Extends the state with an aligned slice.
    pub fn extend_slice_into(&mut self, input: &[f64], output: &mut Vec<f64>) {
        output.reserve(input.len());
        output.extend(input.iter().map(|&value| self.append(value)));
    }

    /// Return the latest cumulative count.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Reset the count to its empty state.
    pub fn reset(&mut self) {
        self.count = 0;
        self.value = None;
    }
}

impl StreamingIndicator for CumulativeCount {
    type Output = f64;

    fn append(&mut self, input: f64) -> Option<f64> {
        Some(Self::append(self, input))
    }

    fn value(&self) -> Option<f64> {
        Self::value(self)
    }

    fn reset(&mut self) {
        Self::reset(self);
    }

    fn extend_slice_into(&mut self, input: &[f64], output: &mut Vec<f64>) {
        Self::extend_slice_into(self, input, output);
    }
}
