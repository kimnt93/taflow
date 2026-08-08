//! Incremental linear-regression indicator states.

use crate::error::TaResult;

use super::{invalid_period, StreamingIndicator, Window};

#[derive(Debug, Clone, Copy)]
struct RegressionValue {
    slope: f64,
    intercept: f64,
}

#[derive(Debug, Clone)]
struct RegressionCore {
    period: usize,
    period_f: f64,
    sum_x: f64,
    denominator: f64,
    window: Window,
    seeded: bool,
}

impl RegressionCore {
    fn new(period: usize) -> TaResult<Self> {
        if period < 2 {
            return Err(invalid_period("timeperiod", period, 2));
        }
        let period_f = period as f64;
        let sum_x = period_f * (period_f - 1.0) / 2.0;
        let sum_x2 = period_f * (period_f - 1.0) * (2.0 * period_f - 1.0) / 6.0;
        Ok(Self {
            period,
            period_f,
            sum_x,
            denominator: period_f * sum_x2 - sum_x * sum_x,
            window: Window::new(period)?,
            seeded: false,
        })
    }

    fn append(&mut self, input: f64) -> Option<RegressionValue> {
        if !self.seeded {
            self.window.push(input);
            if !self.window.is_full() {
                return None;
            }
            self.seeded = true;
        } else {
            self.window.push(input).expect("regression window is full");
        }
        let mut sum_y = 0.0;
        let mut weighted_sum = 0.0;
        for (index, &value) in self.window.iter().enumerate() {
            sum_y += value;
            weighted_sum += index as f64 * value;
        }
        let slope = (self.period_f * weighted_sum - self.sum_x * sum_y) / self.denominator;
        let intercept = (sum_y - slope * self.sum_x) / self.period_f;
        Some(RegressionValue { slope, intercept })
    }

    fn reset(&mut self) {
        self.window.clear();
        self.seeded = false;
    }

    /// Bulk kernel: recomputes each window over the contiguous input slice.
    ///
    /// The O(period) per-bar rescan is kept on purpose: the streaming state
    /// (and TA-Lib) accumulate `sum_y`/`weighted_sum` fresh per window in
    /// chronological order, and an O(1) sliding recurrence would change the
    /// low bits. Scanning `inputs[i+1-period..=i]` directly performs the
    /// same additions in the same order as the ring iterator in `append`,
    /// so outputs and post-run state stay bit-identical to per-bar appends.
    ///
    /// Returns the last emitted regression value (`None` while warming up).
    fn extend_map_into(
        &mut self,
        inputs: &[f64],
        output: &mut Vec<f64>,
        mut map: impl FnMut(RegressionValue) -> f64,
    ) -> Option<f64> {
        let period = self.period;
        let n = inputs.len();
        output.reserve(n);
        // Warm-up prologue: from index period-1 onward the ring contents are
        // exactly the trailing input-slice window, regardless of prior state.
        let prologue = n.min(period - 1);
        let mut last = None;
        for &input in &inputs[..prologue] {
            last = self.append(input).map(&mut map);
            output.push(last.unwrap_or(f64::NAN));
        }
        if n < period {
            return last;
        }
        for i in (period - 1)..n {
            let window = &inputs[i + 1 - period..=i];
            let mut sum_y = 0.0;
            let mut weighted_sum = 0.0;
            for (index, &value) in window.iter().enumerate() {
                sum_y += value;
                weighted_sum += index as f64 * value;
            }
            let slope = (self.period_f * weighted_sum - self.sum_x * sum_y) / self.denominator;
            let intercept = (sum_y - slope * self.sum_x) / self.period_f;
            let mapped = map(RegressionValue { slope, intercept });
            output.push(mapped);
            last = Some(mapped);
        }
        // Rebuild the ring so subsequent appends continue bit-identically.
        self.window.clear();
        for &input in &inputs[n - period..] {
            self.window.push(input);
        }
        self.seeded = true;
        last
    }
}

macro_rules! regression_indicator {
    ($name:ident, $calculate:expr) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            core: RegressionCore,
            value: Option<f64>,
        }

        impl $name {
            /// Computes or updates `new` through the native Rust kernel.
            ///
            /// Parameters are the typed series and configuration values in the signature.
            ///
            /// Returns the computed value, aligned history, or a validation error.
            pub fn new(period: usize) -> TaResult<Self> {
                Ok(Self {
                    core: RegressionCore::new(period)?,
                    value: None,
                })
            }
        }

        impl StreamingIndicator for $name {
            type Output = f64;

            /// Bulk kernel over the contiguous input slice; bit-identical to
            /// per-bar `append` in outputs and post-run state.
            fn extend_slice_into(&mut self, inputs: &[f64], output: &mut Vec<f64>) {
                if inputs.is_empty() {
                    return;
                }
                let period = self.core.period;
                self.value = self
                    .core
                    .extend_map_into(inputs, output, |regression| $calculate(regression, period));
            }

            fn append(&mut self, input: f64) -> Option<f64> {
                let period = self.core.period;
                self.value = self
                    .core
                    .append(input)
                    .map(|regression| $calculate(regression, period));
                self.value
            }

            fn value(&self) -> Option<f64> {
                self.value
            }

            fn reset(&mut self) {
                self.core.reset();
                self.value = None;
            }
        }
    };
}

regression_indicator!(Linearreg, |value: RegressionValue, period: usize| value
    .intercept
    + value.slope * (period - 1) as f64);
regression_indicator!(LinearregSlope, |value: RegressionValue, _| value.slope);
regression_indicator!(LinearregIntercept, |value: RegressionValue, _| value
    .intercept);
regression_indicator!(LinearregAngle, |value: RegressionValue, _| value
    .slope
    .atan()
    .to_degrees());
regression_indicator!(Tsf, |value: RegressionValue, period: usize| value.intercept
    + value.slope * period as f64);

#[cfg(test)]
mod tests {
    use super::*;

    fn lcg_series(n: usize, mut state: u64) -> Vec<f64> {
        (0..n)
            .map(|_| {
                state = state
                    .wrapping_mul(6364136223846793005)
                    .wrapping_add(1442695040888963407);
                90.0 + (state >> 11) as f64 / (1u64 << 53) as f64 * 20.0
            })
            .collect()
    }

    fn assert_same_bits(actual: &[f64], expected: &[f64], label: &str) {
        assert_eq!(actual.len(), expected.len(), "{label}: length");
        for (i, (a, b)) in actual.iter().zip(expected).enumerate() {
            assert_eq!(a.to_bits(), b.to_bits(), "{label}: bar {i}");
        }
    }

    fn check<S, N>(new_state: N, label: &str)
    where
        S: StreamingIndicator<Output = f64>,
        N: Fn(usize) -> S,
    {
        let input = lcg_series(5_000, 0x5EED_0133);
        let tail = lcg_series(256, 0x7A11_0033);
        for period in [2usize, 5, 14, 30, 200] {
            let mut per_bar = new_state(period);
            let reference: Vec<f64> = input
                .iter()
                .map(|&x| per_bar.append(x).unwrap_or(f64::NAN))
                .collect();
            let tail_reference: Vec<f64> = tail
                .iter()
                .map(|&x| per_bar.append(x).unwrap_or(f64::NAN))
                .collect();

            for chunk in [usize::MAX, 1, 7, 97] {
                let mut state = new_state(period);
                let mut out = Vec::new();
                for piece in input.chunks(chunk.min(input.len())) {
                    state.extend_slice_into(piece, &mut out);
                }
                assert_same_bits(
                    &out,
                    &reference,
                    &format!("{label} p{period} chunk {chunk}"),
                );
                let tail_out: Vec<f64> = tail
                    .iter()
                    .map(|&x| state.append(x).unwrap_or(f64::NAN))
                    .collect();
                assert_same_bits(
                    &tail_out,
                    &tail_reference,
                    &format!("{label} p{period} chunk {chunk} tail"),
                );
            }
        }
    }

    #[test]
    fn linearreg_bulk_is_bitwise_identical_to_per_bar_append() {
        check(|p| Linearreg::new(p).unwrap(), "LINEARREG");
    }

    #[test]
    fn linearreg_slope_bulk_is_bitwise_identical_to_per_bar_append() {
        check(|p| LinearregSlope::new(p).unwrap(), "LINEARREG_SLOPE");
    }

    #[test]
    fn linearreg_intercept_bulk_is_bitwise_identical_to_per_bar_append() {
        check(
            |p| LinearregIntercept::new(p).unwrap(),
            "LINEARREG_INTERCEPT",
        );
    }

    #[test]
    fn linearreg_angle_bulk_is_bitwise_identical_to_per_bar_append() {
        check(|p| LinearregAngle::new(p).unwrap(), "LINEARREG_ANGLE");
    }

    #[test]
    fn tsf_bulk_is_bitwise_identical_to_per_bar_append() {
        check(|p| Tsf::new(p).unwrap(), "TSF");
    }
}
