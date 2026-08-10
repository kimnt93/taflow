use crate::error::TaResult;
use crate::indicators::rolling_statistic_helpers::RollingValues;
use crate::stream::StreamingIndicator;

/// Rolling Kelly fraction estimated from win frequency and average payoff.
#[derive(Debug, Clone)]
pub struct RollingKellyCriterion {
    values: RollingValues,
    value: Option<f64>,
}
impl RollingKellyCriterion {
    /// Creates the estimate with a positive rolling period.
    pub fn new(timeperiod: usize) -> TaResult<Self> {
        Ok(Self {
            values: RollingValues::new(timeperiod)?,
            value: None,
        })
    }
    /// Appends one return and returns the latest warm Kelly estimate.
    pub fn append(&mut self, input: f64) -> Option<f64> {
        if !input.is_finite() {
            return None;
        }
        self.values.push(input);
        self.value = self.values.is_full().then(|| {
            let mut wins = 0usize;
            let mut losses = 0usize;
            let (mut wins_sum, mut losses_sum) = (0.0, 0.0);
            for &sample in self.values.iter() {
                if sample > 0.0 {
                    wins += 1;
                    wins_sum += sample;
                } else if sample < 0.0 {
                    losses += 1;
                    losses_sum -= sample;
                }
            }
            let probability = wins as f64 / self.values.window().len() as f64;
            if losses == 0 {
                probability
            } else if wins == 0 {
                -1.0
            } else {
                let average_win = wins_sum / wins as f64;
                let average_loss = losses_sum / losses as f64;
                probability - (1.0 - probability) / (average_win / average_loss)
            }
        });
        self.value
    }
    /// Returns the latest Kelly fraction, or `None` during warm-up.
    pub fn value(&self) -> Option<f64> {
        self.value
    }
    /// Clears the rolling window and latest estimate.
    pub fn reset(&mut self) {
        self.values.clear();
        self.value = None;
    }
}
impl StreamingIndicator for RollingKellyCriterion {
    type Output = f64;
    fn append(&mut self, x: f64) -> Option<f64> {
        Self::append(self, x)
    }
    fn value(&self) -> Option<f64> {
        Self::value(self)
    }
    fn reset(&mut self) {
        Self::reset(self);
    }
}
