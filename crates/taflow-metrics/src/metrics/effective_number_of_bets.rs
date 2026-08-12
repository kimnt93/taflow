use crate::{MetricError, MetricResult, NanPolicy};

/// Entropy-effective count of independent non-negative risk contributions.
#[derive(Debug, Clone)]
pub struct EffectiveNumberOfBets {
    nan_policy: NanPolicy,
    contribution_sum: f64,
    contribution_log_sum: f64,
    count: usize,
}

impl EffectiveNumberOfBets {
    /// Construct an empty diversification-distribution state.
    pub fn new(nan_policy: NanPolicy) -> MetricResult<Self> {
        Ok(Self {
            nan_policy,
            contribution_sum: 0.0,
            contribution_log_sum: 0.0,
            count: 0,
        })
    }

    /// Build PCA-independent risk contributions from weights and covariance.
    pub fn from_weights_and_covariance(
        weights: &[f64],
        covariance: &[f64],
        nan_policy: NanPolicy,
    ) -> MetricResult<Self> {
        let dimension = weights.len();
        if dimension == 0 || covariance.len() != dimension * dimension {
            return Err(MetricError::InvalidParameter {
                name: "covariance",
                value: format!("{} elements", covariance.len()),
                reason: "must be a nonempty square matrix matching weights",
            });
        }
        if weights.iter().any(|value| !value.is_finite())
            || covariance.iter().any(|value| !value.is_finite())
        {
            return Err(MetricError::InvalidParameter {
                name: "weights/covariance",
                value: "non-finite value".to_owned(),
                reason: "all values must be finite",
            });
        }
        for row in 0..dimension {
            for column in 0..dimension {
                let left = covariance[row * dimension + column];
                let right = covariance[column * dimension + row];
                if (left - right).abs() > 1e-12 * left.abs().max(right.abs()).max(1.0) {
                    return Err(MetricError::InvalidParameter {
                        name: "covariance",
                        value: "asymmetric matrix".to_owned(),
                        reason: "must be symmetric within relative tolerance 1e-12",
                    });
                }
            }
        }

        let (eigenvalues, eigenvectors) = Self::symmetric_eigendecomposition(covariance, dimension);
        let scale = covariance
            .iter()
            .map(|value| value.abs())
            .fold(0.0, f64::max);
        if eigenvalues
            .iter()
            .any(|value| *value < -1e-10 * scale.max(1.0))
        {
            return Err(MetricError::InvalidParameter {
                name: "covariance",
                value: "negative eigenvalue".to_owned(),
                reason: "must be positive semidefinite",
            });
        }

        let mut state = Self::new(nan_policy)?;
        for component in 0..dimension {
            let exposure = (0..dimension)
                .map(|asset| eigenvectors[asset * dimension + component] * weights[asset])
                .sum::<f64>();
            state.append(eigenvalues[component].max(0.0) * exposure * exposure)?;
        }
        Ok(state)
    }

    /// Append one non-negative independent risk contribution.
    pub fn append(&mut self, contribution: f64) -> MetricResult<Option<f64>> {
        if contribution.is_nan() {
            return match self.nan_policy {
                NanPolicy::Omit => Ok(self.value()),
                NanPolicy::Raise => Err(MetricError::InvalidObservation {
                    domain: "risk contribution",
                    position: self.count,
                    value: contribution.to_string(),
                    reason: "NaN is rejected by nan_policy='raise'",
                }),
            };
        }
        if !contribution.is_finite() || contribution < 0.0 {
            return Err(MetricError::InvalidObservation {
                domain: "risk contribution",
                position: self.count,
                value: contribution.to_string(),
                reason: "must be finite and non-negative",
            });
        }
        self.contribution_sum += contribution;
        if contribution > 0.0 {
            self.contribution_log_sum += contribution * contribution.ln();
        }
        self.count += 1;
        Ok(self.value())
    }

    /// Append independent risk contributions through the same state.
    pub fn extend(&mut self, contributions: &[f64]) -> MetricResult<Option<f64>> {
        for &contribution in contributions {
            self.append(contribution)?;
        }
        Ok(self.value())
    }

    /// Return `exp(-sum(p * ln(p)))` for normalized risk contributions.
    pub fn value(&self) -> Option<f64> {
        if self.count == 0 || self.contribution_sum <= 0.0 {
            return None;
        }
        Some((self.contribution_sum.ln() - self.contribution_log_sum / self.contribution_sum).exp())
    }

    /// Return the current entropy-effective count in O(1).
    pub fn compute(&self) -> Option<f64> {
        self.value()
    }

    /// Clear contributions while preserving missing-value policy.
    pub fn reset(&mut self) {
        self.contribution_sum = 0.0;
        self.contribution_log_sum = 0.0;
        self.count = 0;
    }

    /// Return the number of valid contributions.
    pub fn len(&self) -> usize {
        self.count
    }

    /// Return whether no contribution was processed.
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    fn symmetric_eigendecomposition(matrix: &[f64], dimension: usize) -> (Vec<f64>, Vec<f64>) {
        let mut values = matrix.to_vec();
        let mut vectors = vec![0.0; dimension * dimension];
        for index in 0..dimension {
            vectors[index * dimension + index] = 1.0;
        }
        let tolerance = 16.0
            * f64::EPSILON
            * matrix
                .iter()
                .map(|value| value.abs())
                .fold(0.0, f64::max)
                .max(f64::MIN_POSITIVE);
        for _ in 0..(50 * dimension * dimension) {
            let mut row = 0;
            let mut column = 0;
            let mut largest = 0.0;
            for candidate_row in 0..dimension {
                for candidate_column in (candidate_row + 1)..dimension {
                    let magnitude = values[candidate_row * dimension + candidate_column].abs();
                    if magnitude > largest {
                        largest = magnitude;
                        row = candidate_row;
                        column = candidate_column;
                    }
                }
            }
            if largest <= tolerance {
                break;
            }
            let rr = values[row * dimension + row];
            let cc = values[column * dimension + column];
            let rc = values[row * dimension + column];
            let angle = 0.5 * (2.0 * rc).atan2(cc - rr);
            let cosine = angle.cos();
            let sine = angle.sin();
            for index in 0..dimension {
                if index != row && index != column {
                    let ir = values[index * dimension + row];
                    let ic = values[index * dimension + column];
                    let new_ir = cosine * ir - sine * ic;
                    let new_ic = sine * ir + cosine * ic;
                    values[index * dimension + row] = new_ir;
                    values[row * dimension + index] = new_ir;
                    values[index * dimension + column] = new_ic;
                    values[column * dimension + index] = new_ic;
                }
            }
            values[row * dimension + row] =
                cosine * cosine * rr - 2.0 * sine * cosine * rc + sine * sine * cc;
            values[column * dimension + column] =
                sine * sine * rr + 2.0 * sine * cosine * rc + cosine * cosine * cc;
            values[row * dimension + column] = 0.0;
            values[column * dimension + row] = 0.0;
            for index in 0..dimension {
                let vr = vectors[index * dimension + row];
                let vc = vectors[index * dimension + column];
                vectors[index * dimension + row] = cosine * vr - sine * vc;
                vectors[index * dimension + column] = sine * vr + cosine * vc;
            }
        }
        let eigenvalues = (0..dimension)
            .map(|index| values[index * dimension + index])
            .collect();
        (eigenvalues, vectors)
    }
}
