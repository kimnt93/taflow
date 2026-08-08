//! Batch implementation for `aroon_oscillator`.

use super::vhgw;
use crate::error::{TaError, TaResult};

/// Aroon Oscillator — vHGW latest-wins index passes with pre-multiplied inv_period
///
/// TA-Lib's tracker uses `>=`/`<=` on every path, so the extremum index is
/// always the latest window maximizer/minimizer — the vHGW indexed tie rule.
/// Compute the aroon oscillator result for the supplied aligned series.
///
/// # Parameters
///
/// * `high` - Input series or configuration value.
/// * `low` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn aroon_oscillator(high: &[f64], low: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    let len = high.len();
    if len != low.len() {
        return Err(TaError::LengthMismatch {
            expected: len,
            got: low.len(),
        });
    }
    if timeperiod < 2 {
        return Err(TaError::InvalidParameter {
            name: "timeperiod",
            value: timeperiod.to_string(),
            reason: "must be >= 2",
        });
    }
    if len <= timeperiod {
        return Err(TaError::InsufficientData {
            need: timeperiod + 1,
            got: len,
        });
    }

    let inv_period = 100.0 / timeperiod as f64;
    let window = timeperiod + 1;
    let mut output = vec![0.0_f64; len];
    output[..timeperiod].fill(f64::NAN);

    let window_count = len - timeperiod;
    let mut highest_indices = vec![0usize; window_count];
    let mut lowest_indices = vec![0usize; window_count];
    vhgw::sliding_argmax_latest_into(high, window, &mut highest_indices);
    vhgw::sliding_argmin_latest_into(low, window, &mut lowest_indices);

    for (offset, (&highest_idx, &lowest_idx)) in
        highest_indices.iter().zip(&lowest_indices).enumerate()
    {
        let today = timeperiod + offset;
        let up = (timeperiod - (today - highest_idx)) as f64 * inv_period;
        let down = (timeperiod - (today - lowest_idx)) as f64 * inv_period;
        output[today] = up - down;
    }

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream::tests_extrema_support::{datasets, periods_and_lengths};

    /// Original single-pass track-and-rescan implementation, kept verbatim
    /// as oracle.
    fn reference_aroon_oscillator(
        high: &[f64],
        low: &[f64],
        timeperiod: usize,
    ) -> TaResult<Vec<f64>> {
        let len = high.len();
        if len != low.len() {
            return Err(TaError::LengthMismatch {
                expected: len,
                got: low.len(),
            });
        }
        if timeperiod < 2 {
            return Err(TaError::InvalidParameter {
                name: "timeperiod",
                value: timeperiod.to_string(),
                reason: "must be >= 2",
            });
        }
        if len <= timeperiod {
            return Err(TaError::InsufficientData {
                need: timeperiod + 1,
                got: len,
            });
        }

        let inv_period = 100.0 / timeperiod as f64;
        let window = timeperiod + 1;
        let mut output = vec![0.0_f64; len];
        output[..timeperiod].fill(f64::NAN);

        let mut highest = high[0];
        let mut highest_idx: usize = 0;
        let mut lowest = low[0];
        let mut lowest_idx: usize = 0;
        for j in 1..window {
            if high[j] >= highest {
                highest = high[j];
                highest_idx = j;
            }
            if low[j] <= lowest {
                lowest = low[j];
                lowest_idx = j;
            }
        }
        {
            let up = (timeperiod - (timeperiod - highest_idx)) as f64 * inv_period;
            let down = (timeperiod - (timeperiod - lowest_idx)) as f64 * inv_period;
            output[timeperiod] = up - down;
        }

        let mut trailing_idx = 1;
        let mut today = timeperiod + 1;

        while today < len {
            let h = high[today];
            let l = low[today];

            if highest_idx < trailing_idx {
                highest_idx = trailing_idx;
                highest = high[trailing_idx];
                for (j, &val) in high[trailing_idx + 1..today + 1].iter().enumerate() {
                    if val >= highest {
                        highest = val;
                        highest_idx = trailing_idx + 1 + j;
                    }
                }
            } else if h >= highest {
                highest_idx = today;
                highest = h;
            }

            if lowest_idx < trailing_idx {
                lowest_idx = trailing_idx;
                lowest = low[trailing_idx];
                for (j, &val) in low[trailing_idx + 1..today + 1].iter().enumerate() {
                    if val <= lowest {
                        lowest = val;
                        lowest_idx = trailing_idx + 1 + j;
                    }
                }
            } else if l <= lowest {
                lowest_idx = today;
                lowest = l;
            }

            let up = (timeperiod - (today - highest_idx)) as f64 * inv_period;
            let down = (timeperiod - (today - lowest_idx)) as f64 * inv_period;
            output[today] = up - down;
            trailing_idx += 1;
            today += 1;
        }

        Ok(output)
    }

    #[test]
    fn matches_reference_bitwise() {
        for (period, len) in periods_and_lengths() {
            for data in datasets(len) {
                let high: Vec<f64> = data.iter().map(|v| v + 1.5).collect();
                let low: Vec<f64> = data.iter().map(|v| v - 1.5).collect();
                let expected = reference_aroon_oscillator(&high, &low, period);
                let actual = aroon_oscillator(&high, &low, period);
                match (expected, actual) {
                    (Ok(expected), Ok(actual)) => {
                        assert_eq!(expected.len(), actual.len());
                        for (e, a) in expected.iter().zip(&actual) {
                            assert_eq!(e.to_bits(), a.to_bits(), "p={period} len={len}");
                        }
                    }
                    (Err(_), Err(_)) => {}
                    _ => panic!("error parity mismatch p={period} len={len}"),
                }
            }
        }
    }
}
