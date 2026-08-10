use crate::error::TaResult;
use crate::indicators::rolling_statistic_helpers::RollingValues;
use crate::stream::StreamingIndicator;

#[derive(Debug, Clone)]
pub struct RollingKellyCriterion {
    values: RollingValues,
    value: Option<f64>,
}
impl RollingKellyCriterion {
    pub fn new(timeperiod: usize) -> TaResult<Self> {
        Ok(Self {
            values: RollingValues::new(timeperiod)?,
            value: None,
        })
    }
    pub fn append(&mut self, input: f64) -> Option<f64> {
        self.values.push(input);
        self.value = self.values.is_full().then(|| {
            let mut wins = 0usize;
            let (mut sum, mut wins_sum) = (0.0, 0.0);
            for &sample in self.values.iter() {
                sum += sample;
                if sample > 0.0 {
                    wins += 1;
                    wins_sum += sample;
                }
            }
            if wins == 0 {
                -1.0
            } else {
                let probability = wins as f64 / self.values.window().len() as f64;
                let average_win = wins_sum / wins as f64;
                let average_loss = if sum < wins_sum {
                    (wins_sum - sum) / (self.values.window().len() - wins).max(1) as f64
                } else {
                    0.0
                };
                if average_loss > 0.0 {
                    probability - (1.0 - probability) / (average_win / average_loss)
                } else {
                    0.0
                }
            }
        });
        self.value
    }
    pub fn value(&self) -> Option<f64> {
        self.value
    }
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
