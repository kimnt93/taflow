//! Batch moving average with variable period.
//!
//! MAVP truncates and clamps the period supplied for each output bar, then
//! selects that bar from the corresponding full-history moving average. Its
//! global warm-up is the selected MA type's lookback at `maxperiod`.

use std::collections::HashMap;

use crate::error::{TaError, TaResult};
use crate::ma_type::{compute_ma, MaType};

fn selected_period(value: f64, minperiod: usize, maxperiod: usize) -> usize {
    (value as usize).clamp(minperiod, maxperiod)
}

/// Computes a TA-Lib-compatible variable-period moving average.
pub fn moving_average_variable_period(
    input: &[f64],
    periods: &[f64],
    minperiod: usize,
    maxperiod: usize,
    matype: MaType,
) -> TaResult<Vec<f64>> {
    let len = input.len();
    if len != periods.len() {
        return Err(TaError::LengthMismatch {
            expected: len,
            got: periods.len(),
        });
    }
    if minperiod == 0 || maxperiod < minperiod {
        return Err(TaError::InvalidParameter {
            name: "minperiod/maxperiod",
            value: format!("{minperiod}/{maxperiod}"),
            reason: "minperiod >= 1 and maxperiod >= minperiod required",
        });
    }

    let lookback = matype.lookback(maxperiod);
    if len <= lookback {
        return Err(TaError::InsufficientData {
            need: lookback + 1,
            got: len,
        });
    }

    let mut output = vec![f64::NAN; len];
    let mut averages = HashMap::<usize, (usize, Vec<f64>)>::new();
    for index in lookback..len {
        let period = selected_period(periods[index], minperiod, maxperiod);
        if !averages.contains_key(&period) {
            let source_start = lookback - matype.lookback(period);
            averages.insert(
                period,
                (
                    source_start,
                    compute_ma(&input[source_start..], period, matype)?,
                ),
            );
        }
        let (source_start, values) = &averages[&period];
        output[index] = values[index - source_start];
    }
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn truncates_clamps_and_dispatches_every_moving_average_type() {
        let input: Vec<f64> = (0..700)
            .map(|index| 100.0 + (index as f64 * 0.17).sin() * 8.0)
            .collect();
        let requested = [1.9, 3.8, 7.2, 11.9, 50.0];
        let periods: Vec<f64> = (0..input.len())
            .map(|index| requested[index % requested.len()])
            .collect();
        for code in 0..=8 {
            let ma_type = MaType::try_from(code).unwrap();
            let output = moving_average_variable_period(&input, &periods, 2, 12, ma_type).unwrap();
            let lookback = ma_type.lookback(12);
            assert!(output[..lookback].iter().all(|value| value.is_nan()));
            assert!(output[lookback..].iter().all(|value| value.is_finite()));
        }
    }
}
