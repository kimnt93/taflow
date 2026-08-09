//! Stateful Variable Index Dynamic Average.

use super::StreamingIndicator;
use crate::error::{TaError, TaResult};

/// CMO-modulated exponential average aligned with pandas-ta-classic.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `VariableIndexDynamicAverage`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct VariableIndexDynamicAverage {
    period: usize,
    alpha: f64,
    /// Fixed ring of the last `period` close changes, oldest at `head`.
    changes: Box<[f64]>,
    head: usize,
    len: usize,
    up_sum: f64,
    down_sum: f64,
    previous_close: Option<f64>,
    count: usize,
    seed_sum: f64,
    value: Option<f64>,
}

impl VariableIndexDynamicAverage {
    /// Creates VIDYA from a positive period and alpha in `(0, 1]`.
    pub fn new(period: usize, alpha: f64) -> TaResult<Self> {
        if period < 1 {
            return Err(TaError::InvalidParameter {
                name: "length",
                value: period.to_string(),
                reason: "must be positive",
            });
        }
        if !(0.0..=1.0).contains(&alpha) || alpha == 0.0 {
            return Err(TaError::InvalidParameter {
                name: "alpha",
                value: alpha.to_string(),
                reason: "must be in (0, 1]",
            });
        }
        Ok(Self {
            period,
            alpha,
            changes: vec![0.0; period].into_boxed_slice(),
            head: 0,
            len: 0,
            up_sum: 0.0,
            down_sum: 0.0,
            previous_close: None,
            count: 0,
            seed_sum: 0.0,
            value: None,
        })
    }

    #[inline]
    fn push_change(&mut self, change: f64) {
        let capacity = self.changes.len();
        if self.len < capacity {
            self.changes[self.len] = change;
            self.len += 1;
        } else {
            let old = self.changes[self.head];
            if old > 0.0 {
                self.up_sum -= old;
            } else {
                self.down_sum += old;
            }
            self.changes[self.head] = change;
            self.head += 1;
            if self.head == capacity {
                self.head = 0;
            }
        }
        if change > 0.0 {
            self.up_sum += change;
        } else {
            self.down_sum -= change;
        }
    }

    #[inline]
    fn weight(up: f64, down: f64) -> f64 {
        let total = up + down;
        if total == 0.0 {
            0.0
        } else {
            (up - down).abs() / total
        }
    }

    #[inline]
    fn advance(&self, weight: f64, input: f64, previous_value: f64) -> f64 {
        self.alpha * weight * input + (1.0 - self.alpha * weight) * previous_value
    }
}

impl StreamingIndicator for VariableIndexDynamicAverage {
    type Output = f64;

    fn append(&mut self, input: f64) -> Option<f64> {
        if let Some(previous) = self.previous_close {
            self.push_change(input - previous);
        }
        self.previous_close = Some(input);
        self.count += 1;
        if self.count <= self.period {
            self.seed_sum += input;
            self.value = if self.count == self.period {
                Some(self.seed_sum / self.period as f64)
            } else {
                None
            };
            return self.value;
        }
        let weight = Self::weight(self.up_sum, self.down_sum);
        let previous_value = self.value.expect("initialized above");
        self.value = Some(self.advance(weight, input, previous_value));
        self.value
    }

    fn value(&self) -> Option<f64> {
        self.value
    }

    fn reset(&mut self) {
        self.changes.fill(0.0);
        self.head = 0;
        self.len = 0;
        self.up_sum = 0.0;
        self.down_sum = 0.0;
        self.previous_close = None;
        self.count = 0;
        self.seed_sum = 0.0;
        self.value = None;
    }

    fn extend_slice_into(&mut self, inputs: &[f64], output: &mut Vec<f64>) {
        output.reserve(inputs.len());
        for &input in inputs {
            output.push(self.append(input).unwrap_or(f64::NAN));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lcg_series(len: usize, mut seed: u64) -> Vec<f64> {
        (0..len)
            .map(|_| {
                seed = seed
                    .wrapping_mul(6364136223846793005)
                    .wrapping_add(1442695040888963407);
                100.0 + ((seed >> 11) as f64 / (1u64 << 53) as f64 - 0.5) * 20.0
            })
            .collect()
    }

    /// pandas-ta-classic recurrence using a deque for the CMO window.
    mod oracle {
        use std::collections::VecDeque;

        pub struct Vidya {
            period: usize,
            alpha: f64,
            closes: VecDeque<f64>,
            count: usize,
            seed_sum: f64,
            pub value: Option<f64>,
        }

        impl Vidya {
            pub fn new(period: usize, alpha: f64) -> Self {
                Self {
                    period,
                    alpha,
                    closes: VecDeque::with_capacity(period + 1),
                    count: 0,
                    seed_sum: 0.0,
                    value: None,
                }
            }

            pub fn append(&mut self, input: f64) -> Option<f64> {
                self.closes.push_back(input);
                if self.closes.len() > self.period + 1 {
                    self.closes.pop_front();
                }
                self.count += 1;
                if self.count <= self.period {
                    self.seed_sum += input;
                    self.value = if self.count == self.period {
                        Some(self.seed_sum / self.period as f64)
                    } else {
                        None
                    };
                    return self.value;
                }
                let mut up = 0.0;
                let mut down = 0.0;
                let mut previous = self.closes.front().copied().unwrap_or(input);
                for &close in self.closes.iter().skip(1) {
                    let change = close - previous;
                    if change > 0.0 {
                        up += change;
                    } else {
                        down -= change;
                    }
                    previous = close;
                }
                let total = up + down;
                let weight = if total == 0.0 {
                    0.0
                } else {
                    (up - down).abs() / total
                };
                let previous_value = self.value.expect("initialized above");
                self.value = Some(
                    self.alpha * weight * input + (1.0 - self.alpha * weight) * previous_value,
                );
                self.value
            }
        }
    }

    fn oracle_outputs(input: &[f64], period: usize, alpha: f64) -> Vec<Option<f64>> {
        let mut state = oracle::Vidya::new(period, alpha);
        input.iter().map(|&bar| state.append(bar)).collect()
    }

    #[test]
    fn append_matches_oracle_within_fp_tolerance() {
        let input = lcg_series(5_000, 0x71da_0000_0000_0001);
        for period in [1_usize, 2, 5, 14, 30] {
            let expected = oracle_outputs(&input, period, 0.2);
            let mut state = VariableIndexDynamicAverage::new(period, 0.2).unwrap();
            for (index, (&bar, want)) in input.iter().zip(&expected).enumerate() {
                match (state.append(bar), *want) {
                    (None, None) => {}
                    (Some(got), Some(want)) => assert!(
                        (got - want).abs() <= 1.0e-10,
                        "period {period} bar {index}: {got} != {want}"
                    ),
                    (got, want) => panic!("period {period} bar {index}: {got:?} != {want:?}"),
                }
            }
        }
    }

    #[test]
    fn bulk_and_chunked_match_append_bitwise() {
        let input = lcg_series(5_000, 0x71da_0000_0000_0002);
        for period in [1_usize, 2, 5, 14, 30] {
            let mut reference = VariableIndexDynamicAverage::new(period, 0.2).unwrap();
            let mut expected = Vec::new();
            reference.extend_slice_into(&input, &mut expected);
            for chunk in [1_usize, 7, 97, 5_000] {
                let mut state = VariableIndexDynamicAverage::new(period, 0.2).unwrap();
                let mut output = Vec::new();
                for window in input.chunks(chunk) {
                    state.extend_slice_into(window, &mut output);
                }
                for (index, (got, want)) in output.iter().zip(&expected).enumerate() {
                    assert_eq!(
                        got.to_bits(),
                        want.to_bits(),
                        "chunk {chunk} period {period} @{index}"
                    );
                }
            }
        }
    }

    #[test]
    fn continue_after_bulk_matches_append() {
        let input = lcg_series(5_000, 0x71da_0000_0000_0003);
        for period in [2_usize, 14] {
            let mut reference = VariableIndexDynamicAverage::new(period, 0.35).unwrap();
            let expected: Vec<Option<f64>> =
                input.iter().map(|&bar| reference.append(bar)).collect();
            let split = 2_777;
            let mut state = VariableIndexDynamicAverage::new(period, 0.35).unwrap();
            let mut output = Vec::new();
            state.extend_slice_into(&input[..split], &mut output);
            for (index, (&bar, want)) in input[split..].iter().zip(&expected[split..]).enumerate() {
                assert_eq!(
                    state.append(bar).map(f64::to_bits),
                    want.map(f64::to_bits),
                    "period {period} continuation bar {index}"
                );
            }
        }
    }

    #[test]
    fn reset_restores_initial_behaviour() {
        let input = lcg_series(400, 0x71da_0000_0000_0004);
        let mut state = VariableIndexDynamicAverage::new(11, 0.2).unwrap();
        let first: Vec<Option<f64>> = input.iter().map(|&bar| state.append(bar)).collect();
        state.reset();
        assert!(state.value().is_none());
        let second: Vec<Option<f64>> = input.iter().map(|&bar| state.append(bar)).collect();
        for (a, b) in first.iter().zip(&second) {
            assert_eq!(a.map(f64::to_bits), b.map(f64::to_bits));
        }
    }
}
