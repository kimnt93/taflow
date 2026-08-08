//! Batch implementation for `rolling_midprice`.

use super::vhgw;
use crate::error::{TaError, TaResult};

/// MIDPRICE — vHGW sliding extrema (O(n), period independent)
///
/// MIDPRICE = (highest_high + lowest_low) / 2
/// Compute the midprice result for the supplied aligned series.
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
pub fn rolling_midprice(high: &[f64], low: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    if timeperiod == 0 {
        return Err(TaError::InvalidParameter {
            name: "timeperiod",
            value: "0".to_string(),
            reason: "must be >= 1",
        });
    }
    let len = high.len();
    if len != low.len() {
        return Err(TaError::LengthMismatch {
            expected: len,
            got: low.len(),
        });
    }
    if len < timeperiod {
        return Err(TaError::InsufficientData {
            need: timeperiod,
            got: len,
        });
    }

    let lookback = timeperiod - 1;
    let mut output = vec![0.0_f64; len];
    output[..lookback].fill(f64::NAN);

    let window_count = len - lookback;
    let mut lowest = vec![0.0_f64; window_count];
    vhgw::sliding_max_into(high, timeperiod, &mut output[lookback..]);
    vhgw::sliding_min_into(low, timeperiod, &mut lowest);
    for (slot, &low_value) in output[lookback..].iter_mut().zip(&lowest) {
        *slot = (*slot + low_value) / 2.0;
    }
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream::tests_extrema_support::{datasets, periods_and_lengths};

    /// Original track-and-rescan implementation, kept verbatim as oracle.
    fn reference_rolling_midprice(
        high: &[f64],
        low: &[f64],
        timeperiod: usize,
    ) -> TaResult<Vec<f64>> {
        if timeperiod == 0 {
            return Err(TaError::InvalidParameter {
                name: "timeperiod",
                value: "0".to_string(),
                reason: "must be >= 1",
            });
        }
        let len = high.len();
        if len != low.len() {
            return Err(TaError::LengthMismatch {
                expected: len,
                got: low.len(),
            });
        }
        if len < timeperiod {
            return Err(TaError::InsufficientData {
                need: timeperiod,
                got: len,
            });
        }

        let lookback = timeperiod - 1;
        let mut output = vec![0.0_f64; len];
        output[..lookback].fill(f64::NAN);

        let mut highest = high[0];
        let mut highest_idx: usize = 0;
        let mut lowest = low[0];
        let mut lowest_idx: usize = 0;
        for j in 1..timeperiod {
            if high[j] >= highest {
                highest = high[j];
                highest_idx = j;
            }
            if low[j] <= lowest {
                lowest = low[j];
                lowest_idx = j;
            }
        }
        output[lookback] = (highest + lowest) / 2.0;

        let mut trailing_idx = 1;
        let mut today = timeperiod;

        while today < len {
            let h = high[today];
            let l = low[today];

            if highest_idx < trailing_idx {
                highest_idx = trailing_idx;
                highest = high[trailing_idx];
                for (j, &val) in high[trailing_idx + 1..=today].iter().enumerate() {
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
                for (j, &val) in low[trailing_idx + 1..=today].iter().enumerate() {
                    if val <= lowest {
                        lowest = val;
                        lowest_idx = trailing_idx + 1 + j;
                    }
                }
            } else if l <= lowest {
                lowest_idx = today;
                lowest = l;
            }

            output[today] = (highest + lowest) / 2.0;
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
                let expected = reference_rolling_midprice(&high, &low, period);
                let actual = rolling_midprice(&high, &low, period);
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
