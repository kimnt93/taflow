//! Stateful Parabolic SAR.
//!
//! SAR keeps only the current direction, extreme point, acceleration factor,
//! projected stop, and previous bar required by TA-Lib's reversal recurrence.

/// Incremental Parabolic SAR with a one-bar lookback.
#[derive(Debug, Clone)]
pub struct ParabolicSar {
    acceleration: f64,
    maximum: f64,
    first_bar: Option<(f64, f64)>,
    initialized: bool,
    is_long: bool,
    sar: f64,
    extreme: f64,
    factor: f64,
    previous_high: f64,
    previous_low: f64,
    value: Option<f64>,
}

impl ParabolicSar {
    /// Creates a SAR state with the supplied acceleration step and maximum.
    pub fn new(acceleration: f64, maximum: f64) -> Self {
        Self {
            acceleration,
            maximum,
            first_bar: None,
            initialized: false,
            is_long: false,
            sar: 0.0,
            extreme: 0.0,
            factor: acceleration,
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
            let (first_high, first_low) = self.first_bar.expect("first SAR bar is stored");
            let minus_move = first_low - low;
            let plus_move = high - first_high;
            self.is_long = !(minus_move > 0.0 && minus_move > plus_move);
            if self.is_long {
                self.extreme = high;
                self.sar = first_low;
            } else {
                self.extreme = low;
                self.sar = first_high;
            }
            self.factor = self.acceleration;
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
                self.value = Some(self.sar);
                self.factor = self.acceleration;
                self.extreme = low;
                self.sar += self.factor * (self.extreme - self.sar);
                self.sar = self.sar.max(previous_high).max(high);
            } else {
                self.value = Some(self.sar);
                if high > self.extreme {
                    self.extreme = high;
                    self.factor = (self.factor + self.acceleration).min(self.maximum);
                }
                self.sar += self.factor * (self.extreme - self.sar);
                self.sar = self.sar.min(previous_low).min(low);
            }
        } else if high >= self.sar {
            self.is_long = true;
            self.sar = self.extreme.min(previous_low).min(low);
            self.value = Some(self.sar);
            self.factor = self.acceleration;
            self.extreme = high;
            self.sar += self.factor * (self.extreme - self.sar);
            self.sar = self.sar.min(previous_low).min(low);
        } else {
            self.value = Some(self.sar);
            if low < self.extreme {
                self.extreme = low;
                self.factor = (self.factor + self.acceleration).min(self.maximum);
            }
            self.sar += self.factor * (self.extreme - self.sar);
            self.sar = self.sar.max(previous_high).max(high);
        }
    }

    pub fn value(&self) -> Option<f64> {
        self.value
    }

    pub fn reset(&mut self) {
        *self = Self::new(self.acceleration, self.maximum);
    }
}

impl Default for ParabolicSar {
    fn default() -> Self {
        Self::new(0.02, 0.2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::overlap;

    #[test]
    fn matches_batch_through_multiple_reversals() {
        let center: Vec<f64> = (0..300)
            .map(|index| 100.0 + (index as f64 * 0.21).sin() * 12.0)
            .collect();
        let high: Vec<f64> = center.iter().map(|value| value + 1.5).collect();
        let low: Vec<f64> = center.iter().map(|value| value - 1.2).collect();
        let expected = overlap::parabolic_sar(&high, &low, 0.02, 0.2).unwrap();
        let mut state = ParabolicSar::new(0.02, 0.2);
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
