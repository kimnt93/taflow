//! Stateful Parabolic SAR.
//!
//! SAR keeps only the current direction, extreme point, acceleration factor,
//! projected stop, and previous bar required by TA-Lib's reversal recurrence.

/// Incremental Parabolic SAR with a one-bar lookback.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `ParabolicSar`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
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

    /// Computes or updates `value` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Computes or updates `reset` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn reset(&mut self) {
        self.first_bar = None;
        self.initialized = false;
        self.is_long = false;
        self.sar = 0.0;
        self.extreme = 0.0;
        self.factor = self.acceleration;
        self.previous_high = 0.0;
        self.previous_low = 0.0;
        self.value = None;
    }

    /// Bulk kernel over aligned high/low slices.
    ///
    /// The recurrence is inherently serial, so the bulk path splits the
    /// warm-up prologue from a monomorphic steady loop, keeps recurrence fields
    /// in locals, and writes state back once. Its statement order matches
    /// `append`, leaving outputs and exit state bit-identical to scalar replay.
    pub fn extend_slice_into(&mut self, high: &[f64], low: &[f64], output: &mut Vec<f64>) {
        let len = high.len().min(low.len());
        let output_start = output.len();
        output.resize(output_start + len, f64::NAN);
        let mut index = 0;
        while index < len && !self.initialized {
            output[output_start + index] = self.append(high[index], low[index]).unwrap_or(f64::NAN);
            index += 1;
        }
        if index == len {
            return;
        }

        let acceleration = self.acceleration;
        let maximum = self.maximum;
        let mut is_long = self.is_long;
        let mut sar = self.sar;
        let mut extreme = self.extreme;
        let mut factor = self.factor;
        let mut previous_high = self.previous_high;
        let mut previous_low = self.previous_low;
        let mut value = self.value.expect("an initialized SAR always has a value");

        for ((&high, &low), output) in high[index..len]
            .iter()
            .zip(&low[index..len])
            .zip(&mut output[output_start + index..])
        {
            if is_long {
                if low <= sar {
                    is_long = false;
                    sar = extreme.max(previous_high).max(high);
                    value = sar;
                    factor = acceleration;
                    extreme = low;
                    sar += factor * (extreme - sar);
                    sar = sar.max(previous_high).max(high);
                } else {
                    value = sar;
                    if high > extreme {
                        extreme = high;
                        factor = (factor + acceleration).min(maximum);
                    }
                    sar += factor * (extreme - sar);
                    sar = sar.min(previous_low).min(low);
                }
            } else if high >= sar {
                is_long = true;
                sar = extreme.min(previous_low).min(low);
                value = sar;
                factor = acceleration;
                extreme = high;
                sar += factor * (extreme - sar);
                sar = sar.min(previous_low).min(low);
            } else {
                value = sar;
                if low < extreme {
                    extreme = low;
                    factor = (factor + acceleration).min(maximum);
                }
                sar += factor * (extreme - sar);
                sar = sar.max(previous_high).max(high);
            }
            previous_high = high;
            previous_low = low;
            *output = value;
        }

        self.is_long = is_long;
        self.sar = sar;
        self.extreme = extreme;
        self.factor = factor;
        self.previous_high = previous_high;
        self.previous_low = previous_low;
        self.value = Some(value);
    }
}

impl Default for ParabolicSar {
    fn default() -> Self {
        Self::new(0.02, 0.2)
    }
}
