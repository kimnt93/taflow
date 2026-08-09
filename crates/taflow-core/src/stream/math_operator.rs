use crate::error::{TaError, TaResult};

/// MINMAXINDEX -- original fused implementation (unused, kept for reference)
#[allow(dead_code)]
pub(crate) fn minmaxindex_fused(
    input: &[f64],
    timeperiod: usize,
) -> TaResult<(Vec<f64>, Vec<f64>)> {
    let len = input.len();
    let lookback = timeperiod - 1;
    let mut out_minidx = vec![0.0_f64; len];
    out_minidx[..lookback].fill(f64::NAN);
    let mut out_maxidx = vec![0.0_f64; len];
    out_maxidx[..lookback].fill(f64::NAN);

    let mut highest = input[0];
    let mut highest_idx: usize = 0;
    let mut lowest = input[0];
    let mut lowest_idx: usize = 0;
    for j in 1..timeperiod {
        if input[j] >= highest {
            highest = input[j];
            highest_idx = j;
        }
        if input[j] <= lowest {
            lowest = input[j];
            lowest_idx = j;
        }
    }
    out_maxidx[lookback] = highest_idx as f64;
    out_minidx[lookback] = lowest_idx as f64;

    let mut trailing_idx = 1;
    let mut today = timeperiod;

    while today < len {
        let v = input[today];

        if highest_idx < trailing_idx {
            highest_idx = trailing_idx;
            highest = input[trailing_idx];
            // Rescan window using slice iterator (bounds-check-free)
            for (j, &val) in input[trailing_idx + 1..=today].iter().enumerate() {
                if val >= highest {
                    highest = val;
                    highest_idx = trailing_idx + 1 + j;
                }
            }
        } else if v >= highest {
            highest_idx = today;
            highest = v;
        }

        if lowest_idx < trailing_idx {
            lowest_idx = trailing_idx;
            lowest = input[trailing_idx];
            // Rescan window using slice iterator (bounds-check-free)
            for (j, &val) in input[trailing_idx + 1..=today].iter().enumerate() {
                if val <= lowest {
                    lowest = val;
                    lowest_idx = trailing_idx + 1 + j;
                }
            }
        } else if v <= lowest {
            lowest_idx = today;
            lowest = v;
        }

        out_maxidx[today] = highest_idx as f64;
        out_minidx[today] = lowest_idx as f64;
        trailing_idx += 1;
        today += 1;
    }

    Ok((out_minidx, out_maxidx))
}

pub(crate) fn validate_pair(a: &[f64], b: &[f64]) -> TaResult<()> {
    if a.len() != b.len() {
        return Err(TaError::LengthMismatch {
            expected: a.len(),
            got: b.len(),
        });
    }
    Ok(())
}

pub(crate) fn validate_period(input: &[f64], timeperiod: usize) -> TaResult<()> {
    if timeperiod == 0 {
        return Err(TaError::InvalidParameter {
            name: "timeperiod",
            value: "0".to_string(),
            reason: "must be >= 1",
        });
    }
    if input.len() < timeperiod {
        return Err(TaError::InsufficientData {
            need: timeperiod,
            got: input.len(),
        });
    }
    Ok(())
}
