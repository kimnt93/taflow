/// Lower-partial second-moment accumulator over all observations.
#[derive(Debug, Clone)]
pub struct DownsideMomentState {
    required_return: f64,
    count: usize,
    squared_shortfall_sum: f64,
}

impl DownsideMomentState {
    /// Construct a state for a per-period minimum acceptable return.
    pub fn new(required_return: f64) -> Self {
        Self {
            required_return,
            count: 0,
            squared_shortfall_sum: 0.0,
        }
    }

    /// Append one already validated observation.
    pub fn append(&mut self, value: f64) {
        let shortfall = (value - self.required_return).min(0.0);
        self.squared_shortfall_sum += shortfall * shortfall;
        self.count += 1;
    }

    /// Clear observations while preserving the required return.
    pub fn reset(&mut self) {
        self.count = 0;
        self.squared_shortfall_sum = 0.0;
    }

    /// Number of observations accumulated.
    pub fn len(&self) -> usize {
        self.count
    }

    /// Whether no observations have been accumulated.
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    /// Configured per-period minimum acceptable return.
    pub fn required_return(&self) -> f64 {
        self.required_return
    }

    /// Mean squared shortfall over all observations.
    pub fn mean_squared_shortfall(&self) -> Option<f64> {
        (self.count != 0).then(|| self.squared_shortfall_sum / self.count as f64)
    }

    /// Square root of the mean squared shortfall.
    pub fn downside_deviation(&self) -> Option<f64> {
        self.mean_squared_shortfall().map(f64::sqrt)
    }
}
