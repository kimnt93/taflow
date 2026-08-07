//! Stateful Parabolic SAR Extended.
//!
//! SAREXT preserves TA-Lib's signed output, optional starting direction,
//! reversal offset, and independent long/short acceleration schedules.

/// Incremental extended Parabolic SAR with a one-bar lookback.
#[derive(Debug, Clone)]
pub struct ExtendedParabolicSar {
    start_value: f64,
    offset_on_reverse: f64,
    acceleration_init_long: f64,
    acceleration_long: f64,
    acceleration_max_long: f64,
    acceleration_init_short: f64,
    acceleration_short: f64,
    acceleration_max_short: f64,
    first_bar: Option<(f64, f64)>,
    initialized: bool,
    is_long: bool,
    sar: f64,
    extreme: f64,
    factor_long: f64,
    factor_short: f64,
    previous_high: f64,
    previous_low: f64,
    value: Option<f64>,
}

impl ExtendedParabolicSar {
    /// Creates a SAREXT state with TA-Lib's complete parameter surface.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        start_value: f64,
        offset_on_reverse: f64,
        acceleration_init_long: f64,
        acceleration_long: f64,
        acceleration_max_long: f64,
        acceleration_init_short: f64,
        acceleration_short: f64,
        acceleration_max_short: f64,
    ) -> Self {
        Self {
            start_value,
            offset_on_reverse,
            acceleration_init_long,
            acceleration_long,
            acceleration_max_long,
            acceleration_init_short,
            acceleration_short,
            acceleration_max_short,
            first_bar: None,
            initialized: false,
            is_long: false,
            sar: 0.0,
            extreme: 0.0,
            factor_long: acceleration_init_long,
            factor_short: acceleration_init_short,
            previous_high: 0.0,
            previous_low: 0.0,
            value: None,
        }
    }

    /// Appends one high and low bar.
    pub fn append(&mut self, high: f64, low: f64) -> Option<f64> {
        if self.first_bar.is_none() {
            self.first_bar = Some((high, low));
            return None;
        }
        if !self.initialized {
            let (first_high, first_low) = self.first_bar.expect("first SAREXT bar is stored");
            self.is_long = if self.start_value == 0.0 {
                let minus_move = first_low - low;
                let plus_move = high - first_high;
                !(minus_move > 0.0 && minus_move > plus_move)
            } else {
                self.start_value > 0.0
            };
            if self.start_value == 0.0 {
                if self.is_long {
                    self.extreme = high;
                    self.sar = first_low;
                } else {
                    self.extreme = low;
                    self.sar = first_high;
                }
            } else if self.start_value > 0.0 {
                self.extreme = high;
                self.sar = self.start_value;
            } else {
                self.extreme = low;
                self.sar = self.start_value.abs();
            }
            self.factor_long = self.acceleration_init_long;
            self.factor_short = self.acceleration_init_short;
            self.previous_high = high;
            self.previous_low = low;
            self.initialized = true;
            self.advance(high, low, high, low);
            return self.value;
        }

        let previous_high = self.previous_high;
        let previous_low = self.previous_low;
        self.previous_high = high;
        self.previous_low = low;
        self.advance(high, low, previous_high, previous_low);
        self.value
    }

    fn advance(&mut self, high: f64, low: f64, previous_high: f64, previous_low: f64) {
        if self.is_long {
            if low <= self.sar {
                self.is_long = false;
                self.sar = self.extreme.max(previous_high).max(high);
                if self.offset_on_reverse != 0.0 {
                    self.sar += self.sar * self.offset_on_reverse;
                }
                self.value = Some(-self.sar);
                self.factor_short = self.acceleration_init_short;
                self.extreme = low;
                self.sar += self.factor_short * (self.extreme - self.sar);
                self.sar = self.sar.max(previous_high).max(high);
            } else {
                self.value = Some(self.sar);
                if high > self.extreme {
                    self.extreme = high;
                    self.factor_long =
                        (self.factor_long + self.acceleration_long).min(self.acceleration_max_long);
                }
                self.sar += self.factor_long * (self.extreme - self.sar);
                self.sar = self.sar.min(previous_low).min(low);
            }
        } else if high >= self.sar {
            self.is_long = true;
            self.sar = self.extreme.min(previous_low).min(low);
            if self.offset_on_reverse != 0.0 {
                self.sar -= self.sar * self.offset_on_reverse;
            }
            self.value = Some(self.sar);
            self.factor_long = self.acceleration_init_long;
            self.extreme = high;
            self.sar += self.factor_long * (self.extreme - self.sar);
            self.sar = self.sar.min(previous_low).min(low);
        } else {
            self.value = Some(-self.sar);
            if low < self.extreme {
                self.extreme = low;
                self.factor_short =
                    (self.factor_short + self.acceleration_short).min(self.acceleration_max_short);
            }
            self.sar += self.factor_short * (self.extreme - self.sar);
            self.sar = self.sar.max(previous_high).max(high);
        }
    }

    pub fn value(&self) -> Option<f64> {
        self.value
    }

    pub fn reset(&mut self) {
        *self = Self::new(
            self.start_value,
            self.offset_on_reverse,
            self.acceleration_init_long,
            self.acceleration_long,
            self.acceleration_max_long,
            self.acceleration_init_short,
            self.acceleration_short,
            self.acceleration_max_short,
        );
    }
}

impl Default for ExtendedParabolicSar {
    fn default() -> Self {
        Self::new(0.0, 0.0, 0.02, 0.02, 0.2, 0.02, 0.02, 0.2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::overlap;

    #[test]
    fn matches_batch_with_asymmetric_parameters_and_reversals() {
        let center: Vec<f64> = (0..300)
            .map(|index| 100.0 + (index as f64 * 0.21).sin() * 12.0)
            .collect();
        let high: Vec<f64> = center.iter().map(|value| value + 1.5).collect();
        let low: Vec<f64> = center.iter().map(|value| value - 1.2).collect();
        let expected =
            overlap::extended_parabolic_sar(&high, &low, 0.0, 0.01, 0.03, 0.02, 0.25, 0.04, 0.03, 0.3).unwrap();
        let mut state = ExtendedParabolicSar::new(0.0, 0.01, 0.03, 0.02, 0.25, 0.04, 0.03, 0.3);
        for index in 0..center.len() {
            let actual = state.append(high[index], low[index]);
            if expected[index].is_nan() {
                assert_eq!(actual, None);
            } else {
                assert!((actual.unwrap() - expected[index]).abs() < 1e-12);
            }
        }
        let expected_final = state.value();
        state.reset();
        for index in 0..center.len() {
            state.append(high[index], low[index]);
        }
        assert_eq!(state.value(), expected_final);
    }
}
