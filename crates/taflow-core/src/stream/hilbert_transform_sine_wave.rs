//! Batch implementation for `hilbert_transform_sine_wave`.

use super::cycle::*;
use crate::error::{TaError, TaResult};

/// HT_SINE - Hilbert Transform - SineWave
///
/// Returns (sine, leadsine).
/// Compute the hilbert transform sine wave result for the supplied aligned series.
///
/// # Parameters
///
/// * `input` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn hilbert_transform_sine_wave(input: &[f64]) -> TaResult<(Vec<f64>, Vec<f64>)> {
    let len = input.len();
    let lookback = 63;

    if len <= lookback {
        return Err(TaError::InsufficientData {
            need: lookback + 1,
            got: len,
        });
    }

    let result = ht_dc_phase_core(input);

    let mut sine = vec![0.0_f64; len];
    sine[..lookback].fill(f64::NAN);
    let mut leadsine = vec![0.0_f64; len];
    leadsine[..lookback].fill(f64::NAN);

    for i in result.first_valid..len {
        let phase = result.dc_phase[i];
        if !phase.is_nan() {
            sine[i] = (phase * DEG2RAD).sin();
            leadsine[i] = ((phase + 45.0) * DEG2RAD).sin();
        }
    }

    Ok((sine, leadsine))
}
