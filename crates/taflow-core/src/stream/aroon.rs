use super::vhgw;
use crate::error::{TaError, TaResult};

/// Aroon (AROON) — two vHGW latest-wins index passes
///
/// Pass 1: scan high[] → aroon_up (latest-wins argmax indices)
/// Pass 2: scan low[]  → aroon_down (latest-wins argmin indices)
///
/// TA-Lib's Aroon tracker uses `>=`/`<=` everywhere (warm-up, fast path,
/// and rescan), so the tracked extremum index is always the LATEST window
/// maximizer/minimizer — exactly the tie rule of the vHGW indexed kernels.
///
/// Compute the aroon result for the supplied aligned series.
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
pub fn aroon(high: &[f64], low: &[f64], timeperiod: usize) -> TaResult<(Vec<f64>, Vec<f64>)> {
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

    let aroon_up = aroon_max_pass(high, timeperiod, inv_period);
    let aroon_down = aroon_min_pass(low, timeperiod, inv_period);

    Ok((aroon_down, aroon_up))
}

/// vHGW latest-wins argmax pass for aroon_up.
#[inline]
pub(crate) fn aroon_max_pass(data: &[f64], timeperiod: usize, inv_period: f64) -> Vec<f64> {
    let len = data.len();
    let window = timeperiod + 1;
    let mut output = vec![0.0_f64; len];
    output[..timeperiod].fill(f64::NAN);

    let mut indices = vec![0usize; len - timeperiod];
    vhgw::sliding_argmax_latest_into(data, window, &mut indices);
    for (offset, &highest_idx) in indices.iter().enumerate() {
        let today = timeperiod + offset;
        output[today] = (timeperiod - (today - highest_idx)) as f64 * inv_period;
    }
    output
}

/// vHGW latest-wins argmin pass for aroon_down.
#[inline]
pub(crate) fn aroon_min_pass(data: &[f64], timeperiod: usize, inv_period: f64) -> Vec<f64> {
    let len = data.len();
    let window = timeperiod + 1;
    let mut output = vec![0.0_f64; len];
    output[..timeperiod].fill(f64::NAN);

    let mut indices = vec![0usize; len - timeperiod];
    vhgw::sliding_argmin_latest_into(data, window, &mut indices);
    for (offset, &lowest_idx) in indices.iter().enumerate() {
        let today = timeperiod + offset;
        output[today] = (timeperiod - (today - lowest_idx)) as f64 * inv_period;
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream::tests_extrema_support::{datasets, periods_and_lengths};

    /// Original track-and-rescan passes, kept verbatim as oracle.
    fn reference_aroon_max_pass(data: &[f64], timeperiod: usize, inv_period: f64) -> Vec<f64> {
        let len = data.len();
        let window = timeperiod + 1;
        let mut output = vec![0.0_f64; len];
        output[..timeperiod].fill(f64::NAN);

        let mut highest = data[0];
        let mut highest_idx: usize = 0;
        for j in 1..window {
            if data[j] >= highest {
                highest = data[j];
                highest_idx = j;
            }
        }
        output[timeperiod] = (timeperiod - (timeperiod - highest_idx)) as f64 * inv_period;

        let mut trailing_idx = 1;
        let mut today = timeperiod + 1;

        while today < len {
            let h = data[today];
            if highest_idx < trailing_idx {
                highest_idx = trailing_idx;
                highest = data[trailing_idx];
                for (j, &val) in data[trailing_idx + 1..today + 1].iter().enumerate() {
                    if val >= highest {
                        highest = val;
                        highest_idx = trailing_idx + 1 + j;
                    }
                }
            } else if h >= highest {
                highest_idx = today;
                highest = h;
            }
            output[today] = (timeperiod - (today - highest_idx)) as f64 * inv_period;
            trailing_idx += 1;
            today += 1;
        }
        output
    }

    fn reference_aroon_min_pass(data: &[f64], timeperiod: usize, inv_period: f64) -> Vec<f64> {
        let len = data.len();
        let window = timeperiod + 1;
        let mut output = vec![0.0_f64; len];
        output[..timeperiod].fill(f64::NAN);

        let mut lowest = data[0];
        let mut lowest_idx: usize = 0;
        for j in 1..window {
            if data[j] <= lowest {
                lowest = data[j];
                lowest_idx = j;
            }
        }
        output[timeperiod] = (timeperiod - (timeperiod - lowest_idx)) as f64 * inv_period;

        let mut trailing_idx = 1;
        let mut today = timeperiod + 1;

        while today < len {
            let l = data[today];
            if lowest_idx < trailing_idx {
                lowest_idx = trailing_idx;
                lowest = data[trailing_idx];
                for (j, &val) in data[trailing_idx + 1..today + 1].iter().enumerate() {
                    if val <= lowest {
                        lowest = val;
                        lowest_idx = trailing_idx + 1 + j;
                    }
                }
            } else if l <= lowest {
                lowest_idx = today;
                lowest = l;
            }
            output[today] = (timeperiod - (today - lowest_idx)) as f64 * inv_period;
            trailing_idx += 1;
            today += 1;
        }
        output
    }

    #[test]
    fn passes_match_reference_bitwise() {
        for (period, len) in periods_and_lengths() {
            for data in datasets(len) {
                if data.len() <= period {
                    assert!(aroon(&data, &data, period).is_err());
                    continue;
                }
                let inv_period = 100.0 / period as f64;
                let expected_up = reference_aroon_max_pass(&data, period, inv_period);
                let expected_down = reference_aroon_min_pass(&data, period, inv_period);
                let actual_up = aroon_max_pass(&data, period, inv_period);
                let actual_down = aroon_min_pass(&data, period, inv_period);
                for (e, a) in expected_up.iter().zip(&actual_up) {
                    assert_eq!(e.to_bits(), a.to_bits(), "up p={period} len={len}");
                }
                for (e, a) in expected_down.iter().zip(&actual_down) {
                    assert_eq!(e.to_bits(), a.to_bits(), "down p={period} len={len}");
                }
            }
        }
    }
}
