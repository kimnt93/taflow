use crate::error::TaResult;
use crate::stream::invalid_period;
use std::collections::VecDeque;

/// Rolling Engle-Granger outputs matching Wickra `Cointegration`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RollingCointegrationValue {
    pub hedge_ratio: f64,
    pub spread: f64,
    pub augmented_dickey_fuller_statistic: f64,
}

/// Rolling OLS hedge ratio and no-constant ADF test of its residual spread.
#[derive(Debug, Clone)]
pub struct RollingCointegration {
    period: usize,
    augmented_dickey_fuller_lags: usize,
    pairs: VecDeque<(f64, f64)>,
    sum_left: f64,
    sum_right: f64,
    sum_right_squared: f64,
    sum_product: f64,
    spreads: Vec<f64>,
    normal: Vec<f64>,
    normal_work: Vec<f64>,
    rhs: Vec<f64>,
    rhs_work: Vec<f64>,
    solution: Vec<f64>,
    value: Option<RollingCointegrationValue>,
}

impl RollingCointegration {
    /// Create a rolling Engle-Granger test.
    ///
    /// `period` must be at least `2 * augmented_dickey_fuller_lags + 4` so the
    /// ADF regression retains a residual degree of freedom.
    pub fn new(period: usize, augmented_dickey_fuller_lags: usize) -> TaResult<Self> {
        let minimum = 2 * augmented_dickey_fuller_lags + 4;
        if period < minimum {
            return Err(invalid_period("period", period, minimum));
        }
        let dimension = augmented_dickey_fuller_lags + 1;
        Ok(Self {
            period,
            augmented_dickey_fuller_lags,
            pairs: VecDeque::with_capacity(period),
            sum_left: 0.0,
            sum_right: 0.0,
            sum_right_squared: 0.0,
            sum_product: 0.0,
            spreads: vec![0.0; period],
            normal: vec![0.0; dimension * dimension],
            normal_work: vec![0.0; dimension * dimension],
            rhs: vec![0.0; dimension],
            rhs_work: vec![0.0; dimension],
            solution: vec![0.0; dimension],
            value: None,
        })
    }

    /// Append one synchronized pair and return hedge ratio, spread, and ADF t-statistic.
    pub fn append(&mut self, left: f64, right: f64) -> Option<RollingCointegrationValue> {
        if self.pairs.len() == self.period {
            let (old_left, old_right) = self.pairs.pop_front().expect("full window");
            self.sum_left -= old_left;
            self.sum_right -= old_right;
            self.sum_right_squared -= old_right * old_right;
            self.sum_product -= old_left * old_right;
        }
        self.pairs.push_back((left, right));
        self.sum_left += left;
        self.sum_right += right;
        self.sum_right_squared += right * right;
        self.sum_product += left * right;
        if self.pairs.len() < self.period {
            self.value = None;
            return None;
        }

        let count = self.period as f64;
        let mean_left = self.sum_left / count;
        let mean_right = self.sum_right / count;
        let variance_right = (self.sum_right_squared / count - mean_right * mean_right).max(0.0);
        let (hedge_ratio, intercept) = if variance_right == 0.0 {
            (0.0, mean_left)
        } else {
            let covariance = self.sum_product / count - mean_left * mean_right;
            let hedge_ratio = covariance / variance_right;
            (hedge_ratio, mean_left - hedge_ratio * mean_right)
        };
        for (index, &(left, right)) in self.pairs.iter().enumerate() {
            self.spreads[index] = left - (intercept + hedge_ratio * right);
        }
        let spread = self.spreads[self.period - 1];
        let statistic = self.augmented_dickey_fuller_statistic();
        self.value = Some(RollingCointegrationValue {
            hedge_ratio,
            spread,
            augmented_dickey_fuller_statistic: statistic,
        });
        self.value
    }

    fn augmented_dickey_fuller_statistic(&mut self) -> f64 {
        let dimension = self.augmented_dickey_fuller_lags + 1;
        let first = self.augmented_dickey_fuller_lags + 1;
        let observations = self.period - first;
        if observations <= dimension {
            return 0.0;
        }
        self.normal.fill(0.0);
        self.rhs.fill(0.0);
        for index in first..self.period {
            let difference = self.spreads[index] - self.spreads[index - 1];
            for row in 0..dimension {
                let left = self.regressor(index, row);
                self.rhs[row] += left * difference;
                for column in 0..dimension {
                    self.normal[row * dimension + column] += left * self.regressor(index, column);
                }
            }
        }
        self.normal_work.copy_from_slice(&self.normal);
        self.rhs_work.copy_from_slice(&self.rhs);
        if !solve_in_place(
            &mut self.normal_work,
            &mut self.rhs_work,
            &mut self.solution,
            dimension,
        ) {
            return 0.0;
        }
        let rho = self.solution[0];
        let mut residual_sum = 0.0;
        for index in first..self.period {
            let difference = self.spreads[index] - self.spreads[index - 1];
            let prediction = (0..dimension)
                .map(|column| self.regressor(index, column) * self.solution[column])
                .sum::<f64>();
            residual_sum += (difference - prediction).powi(2);
        }
        let residual_variance = residual_sum / (observations - dimension) as f64;
        self.normal_work.copy_from_slice(&self.normal);
        self.rhs_work.fill(0.0);
        self.rhs_work[0] = 1.0;
        if !solve_in_place(
            &mut self.normal_work,
            &mut self.rhs_work,
            &mut self.solution,
            dimension,
        ) {
            return 0.0;
        }
        let variance_rho = residual_variance * self.solution[0];
        if variance_rho <= 0.0 {
            0.0
        } else {
            rho / variance_rho.sqrt()
        }
    }

    fn regressor(&self, index: usize, column: usize) -> f64 {
        if column == 0 {
            self.spreads[index - 1]
        } else {
            self.spreads[index - column] - self.spreads[index - column - 1]
        }
    }

    /// Return the latest complete rolling test output.
    pub fn value(&self) -> Option<RollingCointegrationValue> {
        self.value
    }

    /// Clear rolling observations and output while retaining all scratch buffers.
    pub fn reset(&mut self) {
        self.pairs.clear();
        self.sum_left = 0.0;
        self.sum_right = 0.0;
        self.sum_right_squared = 0.0;
        self.sum_product = 0.0;
        self.value = None;
    }
}

fn solve_in_place(
    matrix: &mut [f64],
    rhs: &mut [f64],
    solution: &mut [f64],
    dimension: usize,
) -> bool {
    for column in 0..dimension {
        let pivot = matrix[column * dimension + column];
        if pivot.abs() < 1e-12 {
            return false;
        }
        for row in (column + 1)..dimension {
            let factor = matrix[row * dimension + column] / pivot;
            for entry in column..dimension {
                matrix[row * dimension + entry] -= factor * matrix[column * dimension + entry];
            }
            rhs[row] -= factor * rhs[column];
        }
    }
    solution.fill(0.0);
    for row in (0..dimension).rev() {
        let known = ((row + 1)..dimension)
            .map(|column| matrix[row * dimension + column] * solution[column])
            .sum::<f64>();
        solution[row] = (rhs[row] - known) / matrix[row * dimension + row];
    }
    true
}
