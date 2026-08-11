/// Numerically stable one-pass univariate moments through second order.
#[derive(Debug, Clone, Default)]
pub struct OnlineMoments {
    count: usize,
    mean: f64,
    second_central_moment: f64,
}

impl OnlineMoments {
    /// Construct an empty accumulator.
    pub fn new() -> Self {
        Self::default()
    }

    /// Update the accumulator with one already validated observation.
    pub fn append(&mut self, value: f64) {
        self.count += 1;
        let delta = value - self.mean;
        self.mean += delta / self.count as f64;
        let delta_after = value - self.mean;
        self.second_central_moment += delta * delta_after;
    }

    /// Clear all observations.
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    /// Number of observations accumulated.
    pub fn len(&self) -> usize {
        self.count
    }

    /// Whether the accumulator is empty.
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    /// Arithmetic mean, or `None` when empty.
    pub fn mean(&self) -> Option<f64> {
        (self.count != 0).then_some(self.mean)
    }

    /// Sum of squared deviations from the current mean.
    pub fn second_central_moment(&self) -> f64 {
        self.second_central_moment
    }

    /// Variance using the requested degrees-of-freedom correction.
    pub fn variance(&self, degrees_of_freedom: usize) -> Option<f64> {
        (self.count > degrees_of_freedom)
            .then(|| self.second_central_moment / (self.count - degrees_of_freedom) as f64)
    }

    /// Standard deviation using the requested degrees-of-freedom correction.
    pub fn standard_deviation(&self, degrees_of_freedom: usize) -> Option<f64> {
        self.variance(degrees_of_freedom).map(f64::sqrt)
    }
}
