use std::collections::VecDeque;

use crate::error::TaResult;
use crate::stream::invalid_period;

/// Rolling OLS beta of one asset's log returns on another asset's log returns.
#[derive(Debug, Clone)]
pub struct RollingPairwiseBeta {
    period: usize,
    previous: Option<(f64, f64)>,
    returns: VecDeque<(f64, f64)>,
    sum_a: f64,
    sum_b: f64,
    sum_bb: f64,
    sum_ab: f64,
    value: Option<f64>,
}

impl RollingPairwiseBeta {
    /// Create a state over `period` return pairs; at least two are required.
    pub fn new(period: usize) -> TaResult<Self> {
        if period < 2 {
            return Err(invalid_period("period", period, 2));
        }
        Ok(Self {
            period,
            previous: None,
            returns: VecDeque::with_capacity(period),
            sum_a: 0.0,
            sum_b: 0.0,
            sum_bb: 0.0,
            sum_ab: 0.0,
            value: None,
        })
    }

    /// Append aligned raw prices and return beta after `period + 1` valid prices.
    pub fn append(&mut self, asset: f64, benchmark: f64) -> Option<f64> {
        if !(asset > 0.0 && benchmark > 0.0 && asset.is_finite() && benchmark.is_finite()) {
            self.previous = None;
            self.value = None;
            return None;
        }
        let Some((previous_asset, previous_benchmark)) = self.previous.replace((asset, benchmark))
        else {
            self.value = None;
            return None;
        };
        let pair = (
            (asset / previous_asset).ln(),
            (benchmark / previous_benchmark).ln(),
        );
        if self.returns.len() == self.period {
            let old = self.returns.pop_front().expect("full return window");
            self.sum_a -= old.0;
            self.sum_b -= old.1;
            self.sum_bb -= old.1 * old.1;
            self.sum_ab -= old.0 * old.1;
        }
        self.returns.push_back(pair);
        self.sum_a += pair.0;
        self.sum_b += pair.1;
        self.sum_bb += pair.1 * pair.1;
        self.sum_ab += pair.0 * pair.1;
        self.value = (self.returns.len() == self.period).then(|| {
            let n = self.period as f64;
            let mean_a = self.sum_a / n;
            let mean_b = self.sum_b / n;
            let variance_b = (self.sum_bb / n - mean_b * mean_b).max(0.0);
            if variance_b == 0.0 {
                0.0
            } else {
                (self.sum_ab / n - mean_a * mean_b) / variance_b
            }
        });
        self.value
    }

    /// Return the latest beta, or `None` during warm-up.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Clear prices, return pairs, running sums, and the latest value.
    pub fn reset(&mut self) {
        self.previous = None;
        self.returns.clear();
        self.sum_a = 0.0;
        self.sum_b = 0.0;
        self.sum_bb = 0.0;
        self.sum_ab = 0.0;
        self.value = None;
    }
}
