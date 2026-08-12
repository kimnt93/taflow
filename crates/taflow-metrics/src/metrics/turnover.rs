use crate::{MetricError, MetricResult, NanPolicy};

/// Mean one-way turnover of a chronological risky-asset weight stream.
#[derive(Debug, Clone)]
pub struct Turnover {
    nan_policy: NanPolicy,
    previous_weight: Option<f64>,
    total_absolute_change: f64,
    valid_count: usize,
    bound: bool,
}

impl Turnover {
    /// Construct an empty weight-turnover state.
    pub fn new(nan_policy: NanPolicy) -> MetricResult<Self> {
        Ok(Self {
            nan_policy,
            previous_weight: None,
            total_absolute_change: 0.0,
            valid_count: 0,
            bound: false,
        })
    }

    pub fn from_weights(&mut self, weights: &[f64]) -> MetricResult<&mut Self> {
        self.bound = true;
        self.extend(weights)?;
        Ok(self)
    }

    /// Append one risky-asset portfolio weight and return mean turnover to date.
    pub fn append(&mut self, weight: f64) -> MetricResult<Option<f64>> {
        if !self.bound {
            return Err(MetricError::InvalidParameter {
                name: "input_kind",
                value: "unbound".to_owned(),
                reason: "call from_weights before append or extend",
            });
        }
        if weight.is_nan() {
            return match self.nan_policy {
                NanPolicy::Omit => Ok(self.value()),
                NanPolicy::Raise => Err(MetricError::InvalidObservation {
                    domain: "portfolio weight",
                    position: self.valid_count,
                    value: weight.to_string(),
                    reason: "NaN is rejected by nan_policy='raise'",
                }),
            };
        }
        if !weight.is_finite() {
            return Err(MetricError::InvalidObservation {
                domain: "portfolio weight",
                position: self.valid_count,
                value: weight.to_string(),
                reason: "must be finite",
            });
        }
        if let Some(previous) = self.previous_weight {
            self.total_absolute_change += (weight - previous).abs();
        }
        self.previous_weight = Some(weight);
        self.valid_count += 1;
        Ok(self.value())
    }

    /// Append chronological weights through the same persistent state.
    pub fn extend(&mut self, weights: &[f64]) -> MetricResult<Option<f64>> {
        for &weight in weights {
            self.append(weight)?;
        }
        Ok(self.value())
    }

    /// Return mean absolute weight change across valid transitions.
    pub fn value(&self) -> Option<f64> {
        (self.valid_count >= 2).then(|| self.total_absolute_change / (self.valid_count - 1) as f64)
    }

    /// Return the current scalar without replaying weights.
    pub fn compute(&self) -> Option<f64> {
        self.value()
    }

    /// Restore fresh-state behavior while preserving missing-value policy.
    pub fn reset(&mut self) {
        self.previous_weight = None;
        self.total_absolute_change = 0.0;
        self.valid_count = 0;
    }

    /// Return the number of valid weights processed.
    pub fn len(&self) -> usize {
        self.valid_count
    }

    /// Return whether no valid weight was processed.
    pub fn is_empty(&self) -> bool {
        self.valid_count == 0
    }
}
