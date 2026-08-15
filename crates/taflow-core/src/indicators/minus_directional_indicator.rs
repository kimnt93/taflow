//! Incremental Minus Directional Indicator (-DI).
use crate::error::TaResult;
use crate::stream::directional::DirectionalMovement;

/// Persistent Rust state or aligned output type for `MinusDirectionalIndicator`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct MinusDirectionalIndicator {
    directional: DirectionalMovement,
    value: Option<f64>,
}
impl MinusDirectionalIndicator {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(period: usize) -> TaResult<Self> {
        Ok(Self {
            directional: DirectionalMovement::new(period)?,
            value: None,
        })
    }
    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, high: f64, low: f64, close: f64) -> Option<f64> {
        self.value = self
            .directional
            .append(high, low, close)
            .map(|v| v.minus_di);
        self.value
    }

    /// Append aligned HLC slices while preserving scalar state and warm-up.
    pub fn extend_slices_into(
        &mut self,
        high: &[f64],
        low: &[f64],
        close: &[f64],
        output: &mut Vec<f64>,
    ) -> TaResult<()> {
        if high.len() != low.len() || high.len() != close.len() {
            return Err(crate::TaError::LengthMismatch {
                expected: high.len(),
                got: low.len().min(close.len()),
            });
        }
        let len = high.len();
        output.reserve(len);
        let mut index = 0;

        // Preserve the exact scalar seeding order; this prologue is bounded by
        // the configured period and subsequent chunks normally skip it.
        while index < len && self.value.is_none() {
            output.push(
                self.append(high[index], low[index], close[index])
                    .unwrap_or(f64::NAN),
            );
            index += 1;
        }
        if index == len {
            return Ok(());
        }

        let period = self.directional.period_f;
        let (mut previous_high, mut previous_low, mut previous_close) = self
            .directional
            .previous
            .expect("warm directional state has a previous bar");
        let mut smoothed_true_range = self.directional.true_range;
        let mut smoothed_plus_movement = self.directional.plus_dm;
        let mut smoothed_minus_movement = self.directional.minus_dm;
        let mut latest = self.value.expect("minus DI is seeded");

        for bar in index..len {
            let current_high = high[bar];
            let current_low = low[bar];
            let current_close = close[bar];
            let true_range = (current_high - current_low)
                .max((current_high - previous_close).abs())
                .max((current_low - previous_close).abs());
            let upward = current_high - previous_high;
            let downward = previous_low - current_low;
            let plus_movement = if upward > downward && upward > 0.0 {
                upward
            } else {
                0.0
            };
            let minus_movement = if downward > upward && downward > 0.0 {
                downward
            } else {
                0.0
            };

            smoothed_true_range = smoothed_true_range - smoothed_true_range / period + true_range;
            smoothed_plus_movement =
                smoothed_plus_movement - smoothed_plus_movement / period + plus_movement;
            smoothed_minus_movement =
                smoothed_minus_movement - smoothed_minus_movement / period + minus_movement;
            latest = if smoothed_true_range > 0.0 {
                100.0 * smoothed_minus_movement / smoothed_true_range
            } else {
                0.0
            };
            output.push(latest);

            previous_high = current_high;
            previous_low = current_low;
            previous_close = current_close;
        }

        self.directional.previous = Some((previous_high, previous_low, previous_close));
        self.directional.true_range = smoothed_true_range;
        self.directional.plus_dm = smoothed_plus_movement;
        self.directional.minus_dm = smoothed_minus_movement;
        self.directional.index += len - index;
        self.value = Some(latest);
        Ok(())
    }
    /// Computes or updates `value` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn value(&self) -> Option<f64> {
        self.value
    }
    /// Reset the persistent state and clear the latest value.
    pub fn reset(&mut self) {
        self.directional.reset();
        self.value = None;
    }
}
