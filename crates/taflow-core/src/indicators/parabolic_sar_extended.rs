//! Stateful Parabolic SAR Extended.
//!
//! SAREXT preserves TA-Lib's signed output, optional starting direction,
//! reversal offset, and independent long/short acceleration schedules.

/// Computes an aligned extended Parabolic SAR vector from high and low slices.
///
/// # Parameters
///
/// * `high`, `low` - Equal-length chronological price series.
/// * Remaining arguments configure starting value, reversal offset, and acceleration schedules.
///
/// # Returns
///
/// An aligned vector of signed extended Parabolic SAR values.
#[allow(clippy::too_many_arguments)]

/// Incremental extended Parabolic SAR with a one-bar lookback.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `ParabolicSarExtended`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct ParabolicSarExtended {
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

impl ParabolicSarExtended {
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
        self.factor_long = self.acceleration_init_long;
        self.factor_short = self.acceleration_init_short;
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

        let offset_on_reverse = self.offset_on_reverse;
        let acceleration_init_long = self.acceleration_init_long;
        let acceleration_long = self.acceleration_long;
        let acceleration_max_long = self.acceleration_max_long;
        let acceleration_init_short = self.acceleration_init_short;
        let acceleration_short = self.acceleration_short;
        let acceleration_max_short = self.acceleration_max_short;
        let mut is_long = self.is_long;
        let mut sar = self.sar;
        let mut extreme = self.extreme;
        let mut factor_long = self.factor_long;
        let mut factor_short = self.factor_short;
        let mut previous_high = self.previous_high;
        let mut previous_low = self.previous_low;
        let mut value = self
            .value
            .expect("an initialized SAREXT always has a value");

        for ((&high, &low), output) in high[index..len]
            .iter()
            .zip(&low[index..len])
            .zip(&mut output[output_start + index..])
        {
            if is_long {
                if low <= sar {
                    is_long = false;
                    sar = extreme.max(previous_high).max(high);
                    if offset_on_reverse != 0.0 {
                        sar += sar * offset_on_reverse;
                    }
                    value = -sar;
                    factor_short = acceleration_init_short;
                    extreme = low;
                    sar += factor_short * (extreme - sar);
                    sar = sar.max(previous_high).max(high);
                } else {
                    value = sar;
                    if high > extreme {
                        extreme = high;
                        factor_long = (factor_long + acceleration_long).min(acceleration_max_long);
                    }
                    sar += factor_long * (extreme - sar);
                    sar = sar.min(previous_low).min(low);
                }
            } else if high >= sar {
                is_long = true;
                sar = extreme.min(previous_low).min(low);
                if offset_on_reverse != 0.0 {
                    sar -= sar * offset_on_reverse;
                }
                value = sar;
                factor_long = acceleration_init_long;
                extreme = high;
                sar += factor_long * (extreme - sar);
                sar = sar.min(previous_low).min(low);
            } else {
                value = -sar;
                if low < extreme {
                    extreme = low;
                    factor_short = (factor_short + acceleration_short).min(acceleration_max_short);
                }
                sar += factor_short * (extreme - sar);
                sar = sar.max(previous_high).max(high);
            }
            previous_high = high;
            previous_low = low;
            *output = value;
        }

        self.is_long = is_long;
        self.sar = sar;
        self.extreme = extreme;
        self.factor_long = factor_long;
        self.factor_short = factor_short;
        self.previous_high = previous_high;
        self.previous_low = previous_low;
        self.value = Some(value);
    }
}

impl Default for ParabolicSarExtended {
    fn default() -> Self {
        Self::new(0.0, 0.0, 0.02, 0.02, 0.2, 0.02, 0.02, 0.2)
    }
}
