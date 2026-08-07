use crate::error::{TaError, TaResult};

/// Compute the add result for the supplied aligned series.
///
/// # Parameters
///
/// * `input0` - Input series or configuration value.
/// * `input1` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn add(input0: &[f64], input1: &[f64]) -> TaResult<Vec<f64>> {
    validate_pair(input0, input1)?;
    Ok(input0
        .iter()
        .zip(input1.iter())
        .map(|(a, b)| a + b)
        .collect())
}

/// Compute the sub result for the supplied aligned series.
///
/// # Parameters
///
/// * `input0` - Input series or configuration value.
/// * `input1` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn sub(input0: &[f64], input1: &[f64]) -> TaResult<Vec<f64>> {
    validate_pair(input0, input1)?;
    Ok(input0
        .iter()
        .zip(input1.iter())
        .map(|(a, b)| a - b)
        .collect())
}

/// Compute the mult result for the supplied aligned series.
///
/// # Parameters
///
/// * `input0` - Input series or configuration value.
/// * `input1` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn mult(input0: &[f64], input1: &[f64]) -> TaResult<Vec<f64>> {
    validate_pair(input0, input1)?;
    Ok(input0
        .iter()
        .zip(input1.iter())
        .map(|(a, b)| a * b)
        .collect())
}

/// Compute the div result for the supplied aligned series.
///
/// # Parameters
///
/// * `input0` - Input series or configuration value.
/// * `input1` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn div(input0: &[f64], input1: &[f64]) -> TaResult<Vec<f64>> {
    validate_pair(input0, input1)?;
    Ok(input0
        .iter()
        .zip(input1.iter())
        .map(|(a, b)| a / b)
        .collect())
}

/// Compute the rolling max result for the supplied aligned series.
///
/// # Parameters
///
/// * `input` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn rolling_max(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    validate_period(input, timeperiod)?;
    let len = input.len();
    let lookback = timeperiod - 1;
    let mut output = vec![0.0_f64; len];
    output[..lookback].fill(f64::NAN);

    let mut highest = input[0];
    let mut highest_idx: usize = 0;
    for j in 1..timeperiod {
        if input[j] >= highest {
            highest = input[j];
            highest_idx = j;
        }
    }
    output[lookback] = highest;

    let mut trailing_idx = 1;
    let mut today = timeperiod;

    while today < len {
        let v = input[today];
        if highest_idx < trailing_idx {
            // Rescan window using slice iterator (bounds-check-free)
            highest = input[trailing_idx];
            highest_idx = trailing_idx;
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
        output[today] = highest;
        trailing_idx += 1;
        today += 1;
    }
    Ok(output)
}

/// 滑动窗口最大值的索引 -- C TA-Lib compatible: ties keep FIRST occurrence
///
/// Compute the rolling argmax result for the supplied aligned series.
///
/// # Parameters
///
/// * `input` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn rolling_argmax(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    validate_period(input, timeperiod)?;
    let len = input.len();
    let lookback = timeperiod - 1;
    let mut output = vec![0.0_f64; len]; // C fills lookback with 0, not NaN

    let mut highest = input[0];
    let mut highest_idx: usize = 0;
    for j in 1..timeperiod {
        if input[j] > highest {
            highest = input[j];
            highest_idx = j;
        }
    }
    output[lookback] = highest_idx as f64;

    let mut trailing_idx = 1;
    let mut today = timeperiod;

    while today < len {
        let v = input[today];
        if highest_idx < trailing_idx {
            highest_idx = trailing_idx;
            highest = input[trailing_idx];
            for (j, &val) in input[trailing_idx + 1..=today].iter().enumerate() {
                if val > highest {
                    highest = val;
                    highest_idx = trailing_idx + 1 + j;
                }
            }
        } else if v >= highest {  // fast path: >= matches C (update on tie)
            highest_idx = today;
            highest = v;
        }
        output[today] = highest_idx as f64;
        trailing_idx += 1;
        today += 1;
    }
    Ok(output)
}

/// Compute the rolling min result for the supplied aligned series.
///
/// # Parameters
///
/// * `input` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn rolling_min(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    validate_period(input, timeperiod)?;
    let len = input.len();
    let lookback = timeperiod - 1;
    let mut output = vec![0.0_f64; len];
    output[..lookback].fill(f64::NAN);

    let mut lowest = input[0];
    let mut lowest_idx: usize = 0;
    for j in 1..timeperiod {
        if input[j] <= lowest {
            lowest = input[j];
            lowest_idx = j;
        }
    }
    output[lookback] = lowest;

    let mut trailing_idx = 1;
    let mut today = timeperiod;

    while today < len {
        let v = input[today];
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
        output[today] = lowest;
        trailing_idx += 1;
        today += 1;
    }
    Ok(output)
}

/// Compute the rolling argmin result for the supplied aligned series.
///
/// # Parameters
///
/// * `input` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn rolling_argmin(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    validate_period(input, timeperiod)?;
    let len = input.len();
    let lookback = timeperiod - 1;
    let mut output = vec![0.0_f64; len]; // C fills lookback with 0, not NaN

    let mut lowest = input[0];
    let mut lowest_idx: usize = 0;
    for j in 1..timeperiod {
        if input[j] < lowest {
            lowest = input[j];
            lowest_idx = j;
        }
    }
    output[lookback] = lowest_idx as f64;

    let mut trailing_idx = 1;
    let mut today = timeperiod;

    while today < len {
        let v = input[today];
        if lowest_idx < trailing_idx {
            lowest_idx = trailing_idx;
            lowest = input[trailing_idx];
            for (j, &val) in input[trailing_idx + 1..=today].iter().enumerate() {
                if val < lowest {
                    lowest = val;
                    lowest_idx = trailing_idx + 1 + j;
                }
            }
        } else if v <= lowest {  // fast path: <= matches C (update on tie)
            lowest_idx = today;
            lowest = v;
        }
        output[today] = lowest_idx as f64;
        trailing_idx += 1;
        today += 1;
    }
    Ok(output)
}

/// Compute the rolling sum result for the supplied aligned series.
///
/// # Parameters
///
/// * `input` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn rolling_sum(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    validate_period(input, timeperiod)?;
    let len = input.len();
    let lookback = timeperiod - 1;
    let mut output = vec![0.0_f64; len];
    output[..lookback].fill(f64::NAN);
    // Keep the same left-to-right arithmetic order as `RollingSum::append`.
    // This makes a batch call and an extend/append split bitwise identical.
    let mut s = 0.0_f64;
    for &value in &input[..timeperiod] {
        s += value;
    }
    output[lookback] = s;
    for i in timeperiod..len {
        s -= input[i - timeperiod];
        s += input[i];
        output[i] = s;
    }
    Ok(output)
}

/// MINMAX -- fused single-pass: find both max and min in one scan.
///
/// Compute the rolling minmax result for the supplied aligned series.
///
/// # Parameters
///
/// * `input` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn rolling_minmax(input: &[f64], timeperiod: usize) -> TaResult<(Vec<f64>, Vec<f64>)> {
    validate_period(input, timeperiod)?;
    let len = input.len();
    let lookback = timeperiod - 1;
    let mut out_min = vec![0.0_f64; len];
    out_min[..lookback].fill(f64::NAN);
    let mut out_max = vec![0.0_f64; len];
    out_max[..lookback].fill(f64::NAN);

    let mut highest = input[0];
    let mut highest_idx: usize = 0;
    let mut lowest = input[0];
    let mut lowest_idx: usize = 0;
    for j in 1..timeperiod {
        if input[j] >= highest { highest = input[j]; highest_idx = j; }
        if input[j] <= lowest { lowest = input[j]; lowest_idx = j; }
    }
    out_max[lookback] = highest;
    out_min[lookback] = lowest;

    let mut trailing_idx = 1;
    let mut today = timeperiod;

    while today < len {
        let v = input[today];

        if highest_idx < trailing_idx {
            highest_idx = trailing_idx;
            highest = input[trailing_idx];
            for (j, &val) in input[trailing_idx + 1..=today].iter().enumerate() {
                if val >= highest { highest = val; highest_idx = trailing_idx + 1 + j; }
            }
        } else if v >= highest {
            highest_idx = today;
            highest = v;
        }

        if lowest_idx < trailing_idx {
            lowest_idx = trailing_idx;
            lowest = input[trailing_idx];
            for (j, &val) in input[trailing_idx + 1..=today].iter().enumerate() {
                if val <= lowest { lowest = val; lowest_idx = trailing_idx + 1 + j; }
            }
        } else if v <= lowest {
            lowest_idx = today;
            lowest = v;
        }

        out_max[today] = highest;
        out_min[today] = lowest;
        trailing_idx += 1;
        today += 1;
    }

    Ok((out_min, out_max))
}

/// Compute the rolling minmax index result for the supplied aligned series.
///
/// # Parameters
///
/// * `input` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn rolling_minmax_index(input: &[f64], timeperiod: usize) -> TaResult<(Vec<f64>, Vec<f64>)> {
    validate_period(input, timeperiod)?;
    let len = input.len();
    let lookback = timeperiod - 1;
    let mut out_minidx = vec![0.0_f64; len]; // C fills lookback with 0, not NaN
    let mut out_maxidx = vec![0.0_f64; len];

    let mut highest = input[0];
    let mut highest_idx: usize = 0;
    let mut lowest = input[0];
    let mut lowest_idx: usize = 0;
    // Ties keep FIRST occurrence: use > for max, < for min
    for j in 1..timeperiod {
        if input[j] > highest { highest = input[j]; highest_idx = j; }
        if input[j] < lowest { lowest = input[j]; lowest_idx = j; }
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
            for (j, &val) in input[trailing_idx + 1..=today].iter().enumerate() {
                if val > highest { highest = val; highest_idx = trailing_idx + 1 + j; }
            }
        } else if v >= highest {  // fast path: >= matches C
            highest_idx = today;
            highest = v;
        }

        if lowest_idx < trailing_idx {
            lowest_idx = trailing_idx;
            lowest = input[trailing_idx];
            for (j, &val) in input[trailing_idx + 1..=today].iter().enumerate() {
                if val < lowest { lowest = val; lowest_idx = trailing_idx + 1 + j; }
            }
        } else if v <= lowest {  // fast path: <= matches C
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

/// MINMAXINDEX -- original fused implementation (unused, kept for reference)
#[allow(dead_code)]
fn minmaxindex_fused(input: &[f64], timeperiod: usize) -> TaResult<(Vec<f64>, Vec<f64>)> {
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

fn validate_pair(a: &[f64], b: &[f64]) -> TaResult<()> {
    if a.len() != b.len() {
        return Err(TaError::LengthMismatch {
            expected: a.len(),
            got: b.len(),
        });
    }
    Ok(())
}

fn validate_period(input: &[f64], timeperiod: usize) -> TaResult<()> {
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
