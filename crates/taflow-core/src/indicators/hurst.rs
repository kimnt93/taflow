//! Rolling Hurst exponent using chunked rescaled-range regression.

use crate::error::{TaError, TaResult};
use std::collections::VecDeque;

/// Persistent Wickra-compatible Hurst exponent state.
#[derive(Debug, Clone)]
pub struct Hurst {
    values: VecDeque<f64>,
    period: usize,
    chunks: usize,
    scratch: Box<[f64]>,
    value: Option<f64>,
}

impl Hurst {
    /// Create a trailing estimator with at least two samples per chunk.
    pub fn new(period: usize, chunks: usize) -> TaResult<Self> {
        if chunks < 2 || period < 2 * chunks {
            return Err(TaError::InvalidParameter {
                name: "period/chunks",
                value: format!("{period}/{chunks}"),
                reason: "chunks must be >= 2 and period >= 2 * chunks",
            });
        }
        Ok(Self {
            values: VecDeque::with_capacity(period),
            period,
            chunks,
            scratch: vec![0.0; period].into_boxed_slice(),
            value: None,
        })
    }

    fn rescaled_range(values: &[f64]) -> Option<f64> {
        let mean = values.iter().sum::<f64>() / values.len() as f64;
        let mut cumulative = 0.0;
        let mut minimum = f64::INFINITY;
        let mut maximum = f64::NEG_INFINITY;
        let mut squared = 0.0;
        for &value in values {
            let deviation = value - mean;
            cumulative += deviation;
            minimum = minimum.min(cumulative);
            maximum = maximum.max(cumulative);
            squared += deviation * deviation;
        }
        let range = maximum - minimum;
        let deviation = (squared / values.len() as f64).sqrt();
        (range > 0.0 && deviation > 0.0).then_some(range / deviation)
    }

    /// Append one observation and return the fitted log-log slope.
    pub fn append(&mut self, input: f64) -> Option<f64> {
        if self.values.len() == self.period {
            self.values.pop_front();
        }
        self.values.push_back(input);
        if self.values.len() < self.period {
            self.value = None;
            return None;
        }
        let (front, back) = self.values.as_slices();
        self.scratch[..front.len()].copy_from_slice(front);
        self.scratch[front.len()..].copy_from_slice(back);

        let mut sum_x = 0.0;
        let mut sum_y = 0.0;
        let mut sum_xy = 0.0;
        let mut sum_xx = 0.0;
        let mut count = 0;
        for chunk_count in 1..=self.chunks {
            let size = self.period / chunk_count;
            let mut average = 0.0;
            let mut used = 0;
            for chunk in 0..chunk_count {
                let start = chunk * size;
                if let Some(value) = Self::rescaled_range(&self.scratch[start..start + size]) {
                    average += value;
                    used += 1;
                }
            }
            if used == 0 {
                continue;
            }
            let x = (size as f64).ln();
            let y = (average / used as f64).ln();
            sum_x += x;
            sum_y += y;
            sum_xy += x * y;
            sum_xx += x * x;
            count += 1;
        }
        self.value = Some(if count < 2 {
            0.5
        } else {
            let count = count as f64;
            ((count * sum_xy - sum_x * sum_y) / (count * sum_xx - sum_x * sum_x)).clamp(0.0, 1.0)
        });
        self.value
    }

    /// Return the latest estimate, or `None` during warm-up.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Reset the bounded window without reallocating it.
    pub fn reset(&mut self) {
        self.values.clear();
        self.value = None;
    }
}
