//! Rolling Wickra-compatible Kendall rank correlation.

use crate::error::{TaError, TaResult};

/// Persistent rolling Kendall tau-a state for paired observations.
#[derive(Debug, Clone)]
pub struct RollingKendallRankCorrelation {
    period: usize,
    x: Box<[f64]>,
    y: Box<[f64]>,
    head: usize,
    len: usize,
    concordance_balance: i64,
    x_ties: i64,
    y_ties: i64,
    value: Option<f64>,
}

impl RollingKendallRankCorrelation {
    /// Create a trailing Kendall correlation over at least two pairs.
    pub fn new(period: usize) -> TaResult<Self> {
        if period < 2 {
            return Err(TaError::InvalidParameter {
                name: "period",
                value: period.to_string(),
                reason: "must be at least 2",
            });
        }
        Ok(Self {
            period,
            x: vec![0.0; period].into_boxed_slice(),
            y: vec![0.0; period].into_boxed_slice(),
            head: 0,
            len: 0,
            concordance_balance: 0,
            x_ties: 0,
            y_ties: 0,
            value: None,
        })
    }

    #[inline]
    fn pair_score(x0: f64, y0: f64, x1: f64, y1: f64) -> i64 {
        let product = (x1 - x0) * (y1 - y0);
        if product > 0.0 {
            1
        } else if product < 0.0 {
            -1
        } else {
            0
        }
    }

    #[inline]
    fn current_value(&self) -> f64 {
        Self::correlation(
            self.period,
            self.concordance_balance,
            self.x_ties,
            self.y_ties,
        )
    }

    #[inline]
    fn correlation(period: usize, balance: i64, x_ties: i64, y_ties: i64) -> f64 {
        let pairs = (period * (period - 1) / 2) as i64;
        let denominator = ((pairs - x_ties) as f64 * (pairs - y_ties) as f64).sqrt();
        if denominator == 0.0 {
            0.0
        } else {
            balance as f64 / denominator
        }
    }

    /// Append one chronological pair, updating only evicted/new contributions.
    pub fn append(&mut self, x: f64, y: f64) -> Option<f64> {
        if self.len == self.period {
            let old_x = self.x[self.head];
            let old_y = self.y[self.head];
            let mut index = self.head + 1;
            if index == self.period {
                index = 0;
            }
            for _ in 1..self.period {
                let other_x = self.x[index];
                let other_y = self.y[index];
                self.concordance_balance -= Self::pair_score(old_x, old_y, other_x, other_y);
                self.x_ties -= i64::from(old_x == other_x);
                self.y_ties -= i64::from(old_y == other_y);
                self.concordance_balance += Self::pair_score(other_x, other_y, x, y);
                self.x_ties += i64::from(other_x == x);
                self.y_ties += i64::from(other_y == y);
                index += 1;
                if index == self.period {
                    index = 0;
                }
            }
            self.x[self.head] = x;
            self.y[self.head] = y;
            self.head += 1;
            if self.head == self.period {
                self.head = 0;
            }
        } else {
            let mut index = self.head;
            for _ in 0..self.len {
                self.concordance_balance += Self::pair_score(self.x[index], self.y[index], x, y);
                self.x_ties += i64::from(self.x[index] == x);
                self.y_ties += i64::from(self.y[index] == y);
                index += 1;
                if index == self.period {
                    index = 0;
                }
            }
            let mut tail = self.head + self.len;
            if tail >= self.period {
                tail -= self.period;
            }
            self.x[tail] = x;
            self.y[tail] = y;
            self.len += 1;
        }
        self.value = (self.len == self.period).then(|| self.current_value());
        self.value
    }

    /// Bulk-append aligned slices and restore exact scalar continuation state.
    pub fn extend_slices_into(
        &mut self,
        x: &[f64],
        y: &[f64],
        output: &mut Vec<f64>,
    ) -> TaResult<()> {
        if x.len() != y.len() {
            return Err(TaError::LengthMismatch {
                expected: x.len(),
                got: y.len(),
            });
        }
        let n = x.len();
        output.reserve(n);
        let prologue = n.min(self.period);
        for index in 0..prologue {
            output.push(self.append(x[index], y[index]).unwrap_or(f64::NAN));
        }
        if n <= self.period {
            return Ok(());
        }

        let mut balance = self.concordance_balance;
        let mut x_ties = self.x_ties;
        let mut y_ties = self.y_ties;
        for index in self.period..n {
            let oldest = index - self.period;
            for other in oldest + 1..index {
                balance -= Self::pair_score(x[oldest], y[oldest], x[other], y[other]);
                x_ties -= i64::from(x[oldest] == x[other]);
                y_ties -= i64::from(y[oldest] == y[other]);
                balance += Self::pair_score(x[other], y[other], x[index], y[index]);
                x_ties += i64::from(x[other] == x[index]);
                y_ties += i64::from(y[other] == y[index]);
            }
            output.push(Self::correlation(self.period, balance, x_ties, y_ties));
        }

        self.head = 0;
        self.len = self.period;
        self.x.copy_from_slice(&x[n - self.period..]);
        self.y.copy_from_slice(&y[n - self.period..]);
        self.concordance_balance = balance;
        self.x_ties = x_ties;
        self.y_ties = y_ties;
        self.value = Some(Self::correlation(self.period, balance, x_ties, y_ties));
        Ok(())
    }

    pub fn value(&self) -> Option<f64> {
        self.value
    }

    pub fn reset(&mut self) {
        self.head = 0;
        self.len = 0;
        self.concordance_balance = 0;
        self.x_ties = 0;
        self.y_ties = 0;
        self.value = None;
    }
}
