//! Causal cumulative observation count.

/// Return the one-based cumulative count aligned with the input series.
pub fn cumulative_count(input: &[f64]) -> Vec<f64> {
    let mut state = CumulativeCount::new();
    input.iter().map(|&value| state.append(value)).collect()
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn batch_stream_and_reset_match() {
        let input = [4.0, f64::NAN, 9.0];
        assert_eq!(cumulative_count(&input), vec![1.0, 2.0, 3.0]);
        let mut state = CumulativeCount::new();
        assert_eq!(state.append(4.0), 1.0);
        assert_eq!(state.append(f64::NAN), 2.0);
        state.reset();
        assert_eq!(state.value(), None);
        assert_eq!(state.append(9.0), 1.0);
    }
}
