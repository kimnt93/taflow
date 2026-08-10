use crate::error::TaResult;
use crate::stream::invalid_period;
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RollingLeadLagCrossCorrelationValue {
    pub lag: f64,
    pub correlation: f64,
}

/// Search a bounded rolling pair history for the strongest lagged correlation.
#[derive(Debug, Clone)]
pub struct RollingLeadLagCrossCorrelation {
    window: usize,
    max_lag: usize,
    capacity: usize,
    left: VecDeque<f64>,
    right: VecDeque<f64>,
    value: Option<RollingLeadLagCrossCorrelationValue>,
}

impl RollingLeadLagCrossCorrelation {
    /// Create a search over `-max_lag..=max_lag` using `window` observations.
    pub fn new(window: usize, max_lag: usize) -> TaResult<Self> {
        if window < 2 {
            return Err(invalid_period("window", window, 2));
        }
        if max_lag == 0 {
            return Err(invalid_period("max_lag", max_lag, 1));
        }
        let capacity = window + 2 * max_lag;
        Ok(Self {
            window,
            max_lag,
            capacity,
            left: VecDeque::with_capacity(capacity),
            right: VecDeque::with_capacity(capacity),
            value: None,
        })
    }

    fn correlation(&self, left_start: usize, right_start: usize) -> f64 {
        let n = self.window as f64;
        let (mut sx, mut sy, mut sxx, mut syy, mut sxy) = (0.0, 0.0, 0.0, 0.0, 0.0);
        for index in 0..self.window {
            let x = self.left[left_start + index];
            let y = self.right[right_start + index];
            sx += x;
            sy += y;
            sxx += x * x;
            syy += y * y;
            sxy += x * y;
        }
        let mx = sx / n;
        let my = sy / n;
        let denominator = ((sxx / n - mx * mx).max(0.0) * (syy / n - my * my).max(0.0)).sqrt();
        if denominator == 0.0 {
            0.0
        } else {
            ((sxy / n - mx * my) / denominator).clamp(-1.0, 1.0)
        }
    }

    /// Append one synchronized pair and return best lag and signed correlation.
    pub fn append(&mut self, left: f64, right: f64) -> Option<RollingLeadLagCrossCorrelationValue> {
        if self.left.len() == self.capacity {
            self.left.pop_front();
            self.right.pop_front();
        }
        self.left.push_back(left);
        self.right.push_back(right);
        if self.left.len() < self.capacity {
            self.value = None;
            return None;
        }
        let center = self.max_lag;
        let mut best_lag = 0.0;
        let mut best = self.correlation(center, center);
        let mut magnitude = best.abs();
        for distance in 1..=self.max_lag {
            for (lag, start) in [
                (-(distance as f64), center - distance),
                (distance as f64, center + distance),
            ] {
                let correlation = self.correlation(center, start);
                if correlation.abs() > magnitude {
                    magnitude = correlation.abs();
                    best = correlation;
                    best_lag = lag;
                }
            }
        }
        self.value = Some(RollingLeadLagCrossCorrelationValue {
            lag: best_lag,
            correlation: best,
        });
        self.value
    }

    /// Return the latest best lag/correlation pair.
    pub fn value(&self) -> Option<RollingLeadLagCrossCorrelationValue> {
        self.value
    }

    /// Clear both bounded histories and the latest output.
    pub fn reset(&mut self) {
        self.left.clear();
        self.right.clear();
        self.value = None;
    }
}
