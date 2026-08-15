//! Stateful Directional Movement Index.

use crate::error::TaResult;

use crate::stream::directional::DirectionalMovement;

/// Incremental DX with TA-Lib-compatible Wilder smoothing and lookback.
/// Persistent Rust state or aligned output type for `DirectionalMovementIndex`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct DirectionalMovementIndex {
    directional: DirectionalMovement,
    value: Option<f64>,
}

impl DirectionalMovementIndex {
    /// Creates a DX state with a period of at least two bars.
    pub fn new(period: usize) -> TaResult<Self> {
        Ok(Self {
            directional: DirectionalMovement::new(period)?,
            value: None,
        })
    }

    /// Appends one high, low, and close observation.
    pub fn append(&mut self, high: f64, low: f64, close: f64) -> Option<f64> {
        self.value = self
            .directional
            .append(high, low, close)
            .map(|value| value.dx);
        self.value
    }

    /// Append aligned HLC slices through a fused Wilder recurrence.
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
        let mut latest = self.value.expect("DX is seeded");

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
                let plus_di = 100.0 * smoothed_plus_movement / smoothed_true_range;
                let minus_di = 100.0 * smoothed_minus_movement / smoothed_true_range;
                let sum = plus_di + minus_di;
                if sum > 0.0 {
                    100.0 * (plus_di - minus_di).abs() / sum
                } else {
                    0.0
                }
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

    /// Returns the latest warmed output.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Restores the post-construction state.
    pub fn reset(&mut self) {
        self.directional.reset();
        self.value = None;
    }
}
