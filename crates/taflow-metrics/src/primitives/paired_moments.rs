/// Numerically stable one-pass moments for aligned observation pairs.
#[derive(Debug, Clone, Default)]
pub struct PairedMoments {
    count: usize,
    primary_mean: f64,
    benchmark_mean: f64,
    primary_second_moment: f64,
    benchmark_second_moment: f64,
    co_moment: f64,
}

impl PairedMoments {
    /// Construct an empty paired accumulator.
    pub fn new() -> Self {
        Self::default()
    }

    /// Update with one already validated aligned pair.
    pub fn append(&mut self, primary: f64, benchmark: f64) {
        self.count += 1;
        let count = self.count as f64;
        let primary_delta = primary - self.primary_mean;
        let benchmark_delta = benchmark - self.benchmark_mean;
        self.primary_mean += primary_delta / count;
        self.benchmark_mean += benchmark_delta / count;
        self.primary_second_moment += primary_delta * (primary - self.primary_mean);
        self.benchmark_second_moment += benchmark_delta * (benchmark - self.benchmark_mean);
        self.co_moment += primary_delta * (benchmark - self.benchmark_mean);
    }

    /// Clear all observations.
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    /// Number of aligned pairs accumulated.
    pub fn len(&self) -> usize {
        self.count
    }

    /// Whether the accumulator is empty.
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    /// Primary and benchmark means, or `None` when empty.
    pub fn means(&self) -> Option<(f64, f64)> {
        (self.count != 0).then_some((self.primary_mean, self.benchmark_mean))
    }

    /// Primary variance with the requested degrees-of-freedom correction.
    pub fn primary_variance(&self, degrees_of_freedom: usize) -> Option<f64> {
        (self.count > degrees_of_freedom)
            .then(|| self.primary_second_moment / (self.count - degrees_of_freedom) as f64)
    }

    /// Benchmark variance with the requested degrees-of-freedom correction.
    pub fn benchmark_variance(&self, degrees_of_freedom: usize) -> Option<f64> {
        (self.count > degrees_of_freedom)
            .then(|| self.benchmark_second_moment / (self.count - degrees_of_freedom) as f64)
    }

    /// Covariance with the requested degrees-of-freedom correction.
    pub fn covariance(&self, degrees_of_freedom: usize) -> Option<f64> {
        (self.count > degrees_of_freedom)
            .then(|| self.co_moment / (self.count - degrees_of_freedom) as f64)
    }

    /// Pearson correlation, or `None` for insufficient or zero-variance data.
    pub fn correlation(&self) -> Option<f64> {
        let denominator = (self.primary_second_moment * self.benchmark_second_moment).sqrt();
        (self.count >= 2 && denominator != 0.0).then(|| self.co_moment / denominator)
    }
}
