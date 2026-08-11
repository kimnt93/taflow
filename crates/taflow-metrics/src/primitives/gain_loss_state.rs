/// Positive, negative, and breakeven sufficient statistics.
#[derive(Debug, Clone, Default)]
pub struct GainLossState {
    count: usize,
    gain_count: usize,
    loss_count: usize,
    breakeven_count: usize,
    gross_gain: f64,
    gross_loss: f64,
}

impl GainLossState {
    /// Construct an empty gain/loss accumulator.
    pub fn new() -> Self {
        Self::default()
    }

    /// Append one already validated observation.
    pub fn append(&mut self, value: f64) {
        self.count += 1;
        if value > 0.0 {
            self.gain_count += 1;
            self.gross_gain += value;
        } else if value < 0.0 {
            self.loss_count += 1;
            self.gross_loss += value;
        } else {
            self.breakeven_count += 1;
        }
    }

    /// Clear all observations.
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    /// Number of all observations.
    pub fn len(&self) -> usize {
        self.count
    }

    /// Whether no observations have been accumulated.
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    /// Number of strictly positive observations.
    pub fn gain_count(&self) -> usize {
        self.gain_count
    }

    /// Number of strictly negative observations.
    pub fn loss_count(&self) -> usize {
        self.loss_count
    }

    /// Number of exact-zero observations.
    pub fn breakeven_count(&self) -> usize {
        self.breakeven_count
    }

    /// Sum of strictly positive observations.
    pub fn gross_gain(&self) -> f64 {
        self.gross_gain
    }

    /// Signed sum of strictly negative observations.
    pub fn gross_loss(&self) -> f64 {
        self.gross_loss
    }

    /// Mean strictly positive observation, if one exists.
    pub fn average_gain(&self) -> Option<f64> {
        (self.gain_count != 0).then(|| self.gross_gain / self.gain_count as f64)
    }

    /// Mean strictly negative observation, if one exists.
    pub fn average_loss(&self) -> Option<f64> {
        (self.loss_count != 0).then(|| self.gross_loss / self.loss_count as f64)
    }
}
