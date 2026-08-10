use crate::error::TaResult;
use crate::indicators::rolling_statistic_helpers::RollingValues;
use crate::stream::StreamingIndicator;

/// Mean maximum depth of distinct drawdown episodes in a rolling window.
#[derive(Debug, Clone)]
pub struct RollingAverageDrawdown {
    values: RollingValues,
    value: Option<f64>,
}

impl RollingAverageDrawdown {
    /// Create a rolling average-drawdown state with a non-zero `period`.
    pub fn new(period: usize) -> TaResult<Self> {
        Ok(Self {
            values: RollingValues::new(period)?,
            value: None,
        })
    }

    /// Append one equity-curve observation and return the latest result.
    pub fn append(&mut self, input: f64) -> Option<f64> {
        self.values.push(input);
        self.value = self.values.is_full().then(|| {
            let mut peak = f64::NEG_INFINITY;
            let mut episode_peak = 0.0_f64;
            let mut episode_trough = 0.0_f64;
            let mut depth_sum = 0.0;
            let mut episodes = 0_u32;
            let mut in_drawdown = false;

            for &equity in self.values.iter() {
                if equity >= peak {
                    if in_drawdown {
                        if episode_peak > 0.0 {
                            depth_sum += (episode_peak - episode_trough) / episode_peak;
                            episodes += 1;
                        }
                        in_drawdown = false;
                    }
                    peak = equity;
                } else if in_drawdown {
                    episode_trough = episode_trough.min(equity);
                } else {
                    in_drawdown = true;
                    episode_peak = peak;
                    episode_trough = equity;
                }
            }
            if in_drawdown && episode_peak > 0.0 {
                depth_sum += (episode_peak - episode_trough) / episode_peak;
                episodes += 1;
            }
            if episodes == 0 {
                0.0
            } else {
                depth_sum / f64::from(episodes)
            }
        });
        self.value
    }

    /// Return the latest value, or `None` during warm-up.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Restore fresh-state behavior while retaining allocated buffers.
    pub fn reset(&mut self) {
        self.values.clear();
        self.value = None;
    }
}

impl StreamingIndicator for RollingAverageDrawdown {
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
