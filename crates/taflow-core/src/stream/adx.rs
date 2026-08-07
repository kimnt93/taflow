//! Stateful Average Directional Index.
//!
//! ADX advances Wilder-smoothed true range and directional movement, seeds
//! from the first full period of DX values, and then Wilder-smooths later DX.

use crate::error::TaResult;

use super::directional::DirectionalMovement;

/// Incremental ADX with TA-Lib-compatible seeding and lookback.
/// Persistent Rust state or aligned output type for `AverageDirectionalIndex`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct AverageDirectionalIndex {
    period: usize,
    period_f: f64,
    directional: DirectionalMovement,
    dx_sum: f64,
    dx_count: usize,
    value: Option<f64>,
}

impl AverageDirectionalIndex {
    /// Creates an ADX state with a period of at least two bars.
    pub fn new(period: usize) -> TaResult<Self> {
        Ok(Self {
            period,
            period_f: period as f64,
            directional: DirectionalMovement::new(period)?,
            dx_sum: 0.0,
            dx_count: 0,
            value: None,
        })
    }

    /// Appends one high, low, and close observation.
    pub fn append(&mut self, high: f64, low: f64, close: f64) -> Option<f64> {
        let directional = self.directional.append(high, low, close)?;
        self.value = if self.dx_count < self.period {
            self.dx_sum += directional.dx;
            self.dx_count += 1;
            (self.dx_count == self.period).then_some(self.dx_sum / self.period_f)
        } else {
            Some(
                (self.value.expect("ADX is seeded") * (self.period_f - 1.0) + directional.dx)
                    / self.period_f,
            )
        };
        self.value
    }

    /// Returns the latest warmed output.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Restores the post-construction state.
    pub fn reset(&mut self) {
        self.directional.reset();
        self.dx_sum = 0.0;
        self.dx_count = 0;
        self.value = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_batch_and_reset_replay() {
        let close: Vec<f64> = (0..700)
            .map(|index| 100.0 + (index as f64 * 0.17).sin() * 8.0 + index as f64 * 0.01)
            .collect();
        let high: Vec<f64> = close.iter().map(|value| value + 1.3).collect();
        let low: Vec<f64> = close.iter().map(|value| value - 1.1).collect();
        for period in [2, 3, 14, 30] {
            let expected =
                crate::stream::average_directional_index(&high, &low, &close, period).unwrap();
            let mut state = AverageDirectionalIndex::new(period).unwrap();
            for index in 0..close.len() {
                match state.append(high[index], low[index], close[index]) {
                    Some(actual) => assert!((actual - expected[index]).abs() < 1e-12),
                    None => assert!(expected[index].is_nan()),
                }
            }
            let final_value = state.value();
            state.reset();
            for index in 0..close.len() {
                state.append(high[index], low[index], close[index]);
            }
            assert_eq!(state.value(), final_value);
        }
    }

    #[test]
    fn flat_prices_return_zero_after_warmup() {
        let mut state = AverageDirectionalIndex::new(14).unwrap();
        let values: Vec<_> = (0..50).map(|_| state.append(10.0, 10.0, 10.0)).collect();
        assert_eq!(values[27], Some(0.0));
    }
}
use crate::error::TaError;

/// Average Directional Index (ADX)
///
/// lookback = 2 * timeperiod - 1
///
/// Computes TR, +DM, -DM inline (no intermediate Vec allocations),
/// Compute the average directional index result for the supplied aligned series.
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
pub fn average_directional_index(
    high: &[f64],
    low: &[f64],
    close: &[f64],
    timeperiod: usize,
) -> TaResult<Vec<f64>> {
    let len = high.len();
    if len != low.len() || len != close.len() {
        return Err(TaError::LengthMismatch {
            expected: len,
            got: low.len().min(close.len()),
        });
    }
    if timeperiod < 2 {
        return Err(TaError::InvalidParameter {
            name: "timeperiod",
            value: timeperiod.to_string(),
            reason: "must be >= 2",
        });
    }
    let lookback = 2 * timeperiod - 1;
    if len <= lookback {
        return Err(TaError::InsufficientData {
            need: lookback + 1,
            got: len,
        });
    }

    let pf = timeperiod as f64;
    let mut output = vec![0.0_f64; len];
    output[..lookback].fill(f64::NAN);

    // Phase 1: Compute raw TR, +DM, -DM and seed Wilder sums.
    // Seed range: bars 1..timeperiod-1 (i.e. period-2 bars for the seed sum).
    let mut sum_tr: f64 = 0.0;
    let mut sum_pdm: f64 = 0.0;
    let mut sum_mdm: f64 = 0.0;

    for i in 1..timeperiod {
        let hl = high[i] - low[i];
        let hc = (high[i] - close[i - 1]).abs();
        let lc = (low[i] - close[i - 1]).abs();
        sum_tr += hl.max(hc).max(lc);

        let up = high[i] - high[i - 1];
        let down = low[i - 1] - low[i];
        if up > down && up > 0.0 {
            sum_pdm += up;
        } else if down > up && down > 0.0 {
            sum_mdm += down;
        }
    }

    // Phase 2: Wilder smooth TR/+DM/-DM from timeperiod..len, compute DX values.
    // We need DX values starting at index `timeperiod` to seed ADX.
    // ADX seed = SMA of first `timeperiod` DX values (indices timeperiod..2*timeperiod-1).
    let dx_start = timeperiod;
    let adx_start = dx_start + timeperiod - 1; // = 2*timeperiod - 1

    let mut dx_sum_for_adx_seed: f64 = 0.0;
    let mut dx_count_for_adx_seed: usize = 0;
    let mut prev_adx: f64 = 0.0;

    for i in timeperiod..len {
        // Compute TR, +DM, -DM for bar i
        let hl = high[i] - low[i];
        let hc = (high[i] - close[i - 1]).abs();
        let lc = (low[i] - close[i - 1]).abs();
        let tr_i = hl.max(hc).max(lc);

        let up = high[i] - high[i - 1];
        let down = low[i - 1] - low[i];
        let pdm_i = if up > down && up > 0.0 { up } else { 0.0 };
        let mdm_i = if down > up && down > 0.0 { down } else { 0.0 };

        // Wilder smoothing
        sum_tr = sum_tr - sum_tr / pf + tr_i;
        sum_pdm = sum_pdm - sum_pdm / pf + pdm_i;
        sum_mdm = sum_mdm - sum_mdm / pf + mdm_i;

        // Compute DX
        let dx_val = if sum_tr > 0.0 {
            let pdi = 100.0 * sum_pdm / sum_tr;
            let mdi = 100.0 * sum_mdm / sum_tr;
            let sum_di = pdi + mdi;
            if sum_di > 0.0 {
                100.0 * (pdi - mdi).abs() / sum_di
            } else {
                0.0
            }
        } else {
            f64::NAN
        };

        if i < adx_start {
            // Accumulate DX for ADX seed
            if !dx_val.is_nan() {
                dx_sum_for_adx_seed += dx_val;
                dx_count_for_adx_seed += 1;
            }
        } else if i == adx_start {
            // Include this DX in seed, then compute initial ADX = SMA(DX)
            if !dx_val.is_nan() {
                dx_sum_for_adx_seed += dx_val;
                dx_count_for_adx_seed += 1;
            }
            if dx_count_for_adx_seed > 0 {
                prev_adx = dx_sum_for_adx_seed / dx_count_for_adx_seed as f64;
                output[i] = prev_adx;
            }
        } else {
            // Wilder smooth ADX
            if !dx_val.is_nan() {
                prev_adx = (prev_adx * (pf - 1.0) + dx_val) / pf;
                output[i] = prev_adx;
            }
        }
    }

    Ok(output)
}
