use std::collections::VecDeque;

use crate::error::TaResult;
use crate::stream::invalid_period;

/// Rolling F-statistic testing whether the second series predicts the first.
#[derive(Debug, Clone)]
pub struct RollingGrangerCausality {
    period: usize,
    lag: usize,
    pairs: VecDeque<(f64, f64)>,
    value: Option<f64>,
}

impl RollingGrangerCausality {
    /// Create a Granger state with enough residual degrees of freedom.
    pub fn new(period: usize, lag: usize) -> TaResult<Self> {
        if lag == 0 {
            return Err(invalid_period("lag", lag, 1));
        }
        let minimum = 3 * lag + 2;
        if period < minimum {
            return Err(invalid_period("period", period, minimum));
        }
        Ok(Self {
            period,
            lag,
            pairs: VecDeque::with_capacity(period),
            value: None,
        })
    }

    /// Append aligned observations and return the unrestricted-model F-statistic.
    pub fn append(&mut self, dependent: f64, predictor: f64) -> Option<f64> {
        if self.pairs.len() == self.period {
            self.pairs.pop_front();
        }
        self.pairs.push_back((dependent, predictor));
        if self.pairs.len() < self.period {
            self.value = None;
            return None;
        }

        let observations = self.period - self.lag;
        let restricted_width = self.lag + 1;
        let unrestricted_width = 2 * self.lag + 1;
        let restricted =
            regression_rss(&self.pairs, self.lag, observations, restricted_width, false);
        let unrestricted = regression_rss(
            &self.pairs,
            self.lag,
            observations,
            unrestricted_width,
            true,
        );
        self.value = match (restricted, unrestricted) {
            (Some(restricted_rss), Some(unrestricted_rss)) => {
                let degrees = (observations - unrestricted_width) as f64;
                let denominator = unrestricted_rss / degrees;
                let statistic =
                    ((restricted_rss - unrestricted_rss) / self.lag as f64) / denominator;
                Some(statistic.max(0.0))
            }
            _ => Some(0.0),
        };
        self.value
    }

    /// Return the latest F-statistic, or `None` during warm-up.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Clear the rolling observations and latest statistic.
    pub fn reset(&mut self) {
        self.pairs.clear();
        self.value = None;
    }
}

/// Fit the restricted or unrestricted autoregression through normal equations.
fn regression_rss(
    pairs: &VecDeque<(f64, f64)>,
    lag: usize,
    observations: usize,
    width: usize,
    include_predictor: bool,
) -> Option<f64> {
    let mut normal = vec![vec![0.0; width]; width];
    let mut rhs = vec![0.0; width];
    let mut row = vec![0.0; width];
    for offset in 0..observations {
        let now = lag + offset;
        row.fill(0.0);
        row[0] = 1.0;
        for back in 1..=lag {
            row[back] = pairs[now - back].0;
        }
        if include_predictor {
            for back in 1..=lag {
                row[lag + back] = pairs[now - back].1;
            }
        }
        let target = pairs[now].0;
        for left in 0..width {
            rhs[left] += row[left] * target;
            for right in 0..width {
                normal[left][right] += row[left] * row[right];
            }
        }
    }
    let coefficients = solve(normal, rhs)?;
    let mut rss = 0.0;
    for offset in 0..observations {
        let now = lag + offset;
        row.fill(0.0);
        row[0] = 1.0;
        for back in 1..=lag {
            row[back] = pairs[now - back].0;
        }
        if include_predictor {
            for back in 1..=lag {
                row[lag + back] = pairs[now - back].1;
            }
        }
        let prediction: f64 = row
            .iter()
            .zip(&coefficients)
            .map(|(input, coefficient)| input * coefficient)
            .sum();
        let residual = pairs[now].0 - prediction;
        rss += residual * residual;
    }
    Some(rss)
}

/// Solve a small dense system using Gaussian elimination.
fn solve(mut matrix: Vec<Vec<f64>>, mut rhs: Vec<f64>) -> Option<Vec<f64>> {
    let dimension = rhs.len();
    for column in 0..dimension {
        let pivot = matrix[column][column];
        if pivot.abs() < 1e-12 {
            return None;
        }
        let pivot_row = matrix[column].clone();
        for row in (column + 1)..dimension {
            let factor = matrix[row][column] / pivot;
            for cell in column..dimension {
                matrix[row][cell] -= factor * pivot_row[cell];
            }
            rhs[row] -= factor * rhs[column];
        }
    }
    let mut solution = vec![0.0; dimension];
    for row in (0..dimension).rev() {
        let known: f64 = ((row + 1)..dimension)
            .map(|column| matrix[row][column] * solution[column])
            .sum();
        solution[row] = (rhs[row] - known) / matrix[row][row];
    }
    Some(solution)
}
