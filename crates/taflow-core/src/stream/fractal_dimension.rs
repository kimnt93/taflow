//! Batch implementation for `fractal_dimension`.

use super::operator_states::*;
use crate::error::{TaError, TaResult};

/// Computes or updates `fractal_dimension` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Compute the fractal dimension result for the supplied aligned series.
///
/// # Parameters
///
/// * `input` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn fractal_dimension(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    let mut state = Hurst::new(timeperiod)?;
    Ok(input
        .iter()
        .map(|&value| {
            state
                .append(value)
                .map(|hurst| 2.0 - hurst)
                .unwrap_or(f64::NAN)
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lcg_series(n: usize, mut state: u64) -> Vec<f64> {
        (0..n)
            .map(|_| {
                state = state
                    .wrapping_mul(6364136223846793005)
                    .wrapping_add(1442695040888963407);
                90.0 + (state >> 11) as f64 / (1u64 << 53) as f64 * 20.0
            })
            .collect()
    }

    #[test]
    fn batch_matches_two_minus_streaming_hurst_bitwise() {
        // `FractalDimension` is `2 - Hurst`; `hurst.rs` carries the oracle test
        // for the estimator itself, so this pins the composition and the
        // NaN warm-up.
        let input = lcg_series(5_000, 0xF4AC_0001);
        for period in [2usize, 3, 20, 64] {
            let batch = fractal_dimension(&input, period).unwrap();
            let mut state = Hurst::new(period).unwrap();
            for (bar, &value) in input.iter().enumerate() {
                let expected = state
                    .append(value)
                    .map(|hurst| 2.0 - hurst)
                    .unwrap_or(f64::NAN);
                assert_eq!(
                    batch[bar].to_bits(),
                    expected.to_bits(),
                    "period {period} bar {bar}"
                );
            }
        }
    }

    #[test]
    fn chunked_feeding_matches_single_pass() {
        let input = lcg_series(5_000, 0xF4AC_0002);
        let reference = fractal_dimension(&input, 20).unwrap();
        for chunk in [1usize, 7, 97, 1_000] {
            let mut state = Hurst::new(20).unwrap();
            let mut produced = Vec::with_capacity(input.len());
            for piece in input.chunks(chunk) {
                for &value in piece {
                    produced.push(
                        state
                            .append(value)
                            .map(|hurst| 2.0 - hurst)
                            .unwrap_or(f64::NAN),
                    );
                }
            }
            for (bar, (actual, expected)) in produced.iter().zip(&reference).enumerate() {
                assert_eq!(
                    actual.to_bits(),
                    expected.to_bits(),
                    "chunk {chunk} bar {bar}"
                );
            }
        }
    }
}
