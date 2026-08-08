//! Incremental Plus Directional Indicator (+DI).
use super::directional::DirectionalMovement;
use crate::error::TaResult;

/// Compute the plus directional indicator result for the supplied aligned series.
///
/// # Parameters
///
/// * `high` - Input series or configuration value.
/// * `low` - Input series or configuration value.
/// * `close` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn plus_directional_indicator(
    high: &[f64],
    low: &[f64],
    close: &[f64],
    timeperiod: usize,
) -> TaResult<Vec<f64>> {
    if high.len() != low.len() || high.len() != close.len() {
        return Err(crate::TaError::LengthMismatch {
            expected: high.len(),
            got: low.len().min(close.len()),
        });
    }
    let mut state = PlusDirectionalIndicator::new(timeperiod)?;
    Ok(high
        .iter()
        .zip(low)
        .zip(close)
        .map(|((high, low), close)| state.append(*high, *low, *close).unwrap_or(f64::NAN))
        .collect())
}

/// Persistent Rust state or aligned output type for `PlusDirectionalIndicator`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct PlusDirectionalIndicator {
    directional: DirectionalMovement,
    value: Option<f64>,
}
impl PlusDirectionalIndicator {
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
        self.value = self.directional.append(high, low, close).map(|v| v.plus_di);
        self.value
    }
    /// Bulk kernel: once warm, advances the Wilder-smoothed TR/+DM/-DM
    /// recurrences in one loop with the scalar states held in locals, writing
    /// NaN during warm-up. Bit-identical to per-bar [`Self::append`] in
    /// outputs and post-run streaming state.
    pub fn extend_slices_into(
        &mut self,
        high: &[f64],
        low: &[f64],
        close: &[f64],
        output: &mut Vec<f64>,
    ) {
        let len = high.len().min(low.len()).min(close.len());
        output.reserve(len);
        let mut index = 0;
        // Warm-up prologue: per-bar appends until the Wilder sums are seeded.
        while index < len && self.value.is_none() {
            output.push(
                self.append(high[index], low[index], close[index])
                    .unwrap_or(f64::NAN),
            );
            index += 1;
        }
        if index == len {
            return;
        }

        let pf = self.directional.period_f;
        let (mut previous_high, mut previous_low, mut previous_close) = self
            .directional
            .previous
            .expect("warm directional state has a previous bar");
        let mut smoothed_tr = self.directional.true_range;
        let mut smoothed_pdm = self.directional.plus_dm;
        let mut smoothed_mdm = self.directional.minus_dm;
        let mut last = f64::NAN;
        for bar in index..len {
            let (high, low, close) = (high[bar], low[bar], close[bar]);
            let true_range = (high - low)
                .max((high - previous_close).abs())
                .max((low - previous_close).abs());
            let up = high - previous_high;
            let down = previous_low - low;
            let plus_dm = if up > down && up > 0.0 { up } else { 0.0 };
            let minus_dm = if down > up && down > 0.0 { down } else { 0.0 };
            previous_high = high;
            previous_low = low;
            previous_close = close;

            smoothed_tr = smoothed_tr - smoothed_tr / pf + true_range;
            smoothed_pdm = smoothed_pdm - smoothed_pdm / pf + plus_dm;
            smoothed_mdm = smoothed_mdm - smoothed_mdm / pf + minus_dm;
            last = if smoothed_tr > 0.0 {
                100.0 * smoothed_pdm / smoothed_tr
            } else {
                0.0
            };
            output.push(last);
        }

        self.directional.previous = Some((previous_high, previous_low, previous_close));
        self.directional.true_range = smoothed_tr;
        self.directional.plus_dm = smoothed_pdm;
        self.directional.minus_dm = smoothed_mdm;
        self.directional.index += len - index;
        self.value = Some(last);
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

#[cfg(test)]
mod tests {
    use super::*;

    fn lcg_bars(n: usize, mut state: u64) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
        let mut next = move || {
            state = state
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            (state >> 11) as f64 / (1u64 << 53) as f64
        };
        let mut high = Vec::with_capacity(n);
        let mut low = Vec::with_capacity(n);
        let mut close = Vec::with_capacity(n);
        for _ in 0..n {
            let base = 90.0 + next() * 20.0;
            let up = next() * 2.0;
            let down = next() * 2.0;
            high.push(base + up);
            low.push(base - down);
            close.push(base + (up - down) * next());
        }
        (high, low, close)
    }

    fn assert_same_bits(actual: &[f64], expected: &[f64], label: &str) {
        assert_eq!(actual.len(), expected.len(), "{label}: length");
        for (i, (a, b)) in actual.iter().zip(expected).enumerate() {
            assert_eq!(a.to_bits(), b.to_bits(), "{label}: bar {i}");
        }
    }

    #[test]
    fn bulk_is_bitwise_identical_to_per_bar_append() {
        let (high, low, close) = lcg_bars(5_000, 0x5EED_9D10);
        let (th, tl, tc) = lcg_bars(128, 0x7A11_9D10);
        for period in [1usize, 2, 14, 30] {
            let mut per_bar = PlusDirectionalIndicator::new(period).unwrap();
            let reference: Vec<f64> = (0..high.len())
                .map(|i| {
                    per_bar
                        .append(high[i], low[i], close[i])
                        .unwrap_or(f64::NAN)
                })
                .collect();
            let tail_reference: Vec<f64> = (0..th.len())
                .map(|i| per_bar.append(th[i], tl[i], tc[i]).unwrap_or(f64::NAN))
                .collect();

            for chunk in [usize::MAX, 1, 7, 97] {
                let mut state = PlusDirectionalIndicator::new(period).unwrap();
                let mut out = Vec::new();
                let mut start = 0;
                while start < high.len() {
                    let end = (start + chunk.min(high.len())).min(high.len());
                    state.extend_slices_into(
                        &high[start..end],
                        &low[start..end],
                        &close[start..end],
                        &mut out,
                    );
                    start = end;
                }
                let label = format!("p{period} chunk {chunk}");
                assert_same_bits(&out, &reference, &label);
                let tail_out: Vec<f64> = (0..th.len())
                    .map(|i| state.append(th[i], tl[i], tc[i]).unwrap_or(f64::NAN))
                    .collect();
                assert_same_bits(&tail_out, &tail_reference, &format!("{label} tail"));
            }
        }
    }
}
