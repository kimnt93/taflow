//! Batch implementation for `hilbert_transform_dominant_cycle_phase`.

use super::cycle::*;
use crate::error::{TaError, TaResult};

/// HT_DCPHASE - Hilbert Transform - Dominant Cycle Phase
///
/// Compute the hilbert transform dominant cycle phase result for the supplied aligned series.
///
/// # Parameters
///
/// * `input` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn hilbert_transform_dominant_cycle_phase(input: &[f64]) -> TaResult<Vec<f64>> {
    let len = input.len();
    let lookback = 63;

    if len <= lookback {
        return Err(TaError::InsufficientData {
            need: lookback + 1,
            got: len,
        });
    }

    let result = ht_dc_phase_core(input);
    Ok(result.dc_phase)
}
