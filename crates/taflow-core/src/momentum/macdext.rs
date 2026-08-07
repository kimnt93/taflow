//! Batch extended Moving Average Convergence/Divergence.
//!
//! MACDEXT permits independently selected fast, slow, and signal moving
//! averages. Fast and slow inputs are aligned to a shared largest lookback so
//! their seeds reproduce TA-Lib's internal start-index calls.

use crate::error::{TaError, TaResult};
use crate::ma_type::{compute_ma, MaType};

fn ma_from_aligned_start(
    input: &[f64],
    start: usize,
    period: usize,
    ma_type: MaType,
) -> TaResult<Vec<f64>> {
    let lookback = ma_type.lookback(period);
    let source_start = start - lookback;
    let values = compute_ma(&input[source_start..], period, ma_type)?;
    Ok(values[lookback..].to_vec())
}

/// Computes aligned MACDEXT, signal, and histogram arrays.
pub fn moving_average_convergence_divergence_extended(
    input: &[f64],
    fastperiod: usize,
    fastmatype: MaType,
    slowperiod: usize,
    slowmatype: MaType,
    signalperiod: usize,
    signalmatype: MaType,
) -> TaResult<(Vec<f64>, Vec<f64>, Vec<f64>)> {
    if fastperiod < 2 || slowperiod < 2 || signalperiod == 0 {
        return Err(TaError::InvalidParameter {
            name: "fastperiod/slowperiod/signalperiod",
            value: format!("{fastperiod}/{slowperiod}/{signalperiod}"),
            reason: "fastperiod >= 2, slowperiod >= 2, signalperiod >= 1",
        });
    }
    let (fp, fmt, sp, smt) = if fastperiod < slowperiod {
        (fastperiod, fastmatype, slowperiod, slowmatype)
    } else {
        (slowperiod, slowmatype, fastperiod, fastmatype)
    };

    if fastmatype == MaType::Ema && slowmatype == MaType::Ema && signalmatype == MaType::Ema {
        return super::macd::moving_average_convergence_divergence(input, fastperiod, slowperiod, signalperiod);
    }

    let len = input.len();
    let largest_lookback = fmt.lookback(fp).max(smt.lookback(sp));
    let signal_lookback = signalmatype.lookback(signalperiod);
    let total_lookback = largest_lookback + signal_lookback;
    if len <= total_lookback {
        return Err(TaError::InsufficientData {
            need: total_lookback + 1,
            got: len,
        });
    }

    let fast_ma = ma_from_aligned_start(input, largest_lookback, fp, fmt)?;
    let slow_ma = ma_from_aligned_start(input, largest_lookback, sp, smt)?;
    let macd_valid: Vec<f64> = fast_ma
        .iter()
        .zip(slow_ma.iter())
        .map(|(fast, slow)| fast - slow)
        .collect();
    let signal_ma = compute_ma(&macd_valid, signalperiod, signalmatype)?;

    let mut macd_line = vec![f64::NAN; len];
    let mut signal_line = vec![f64::NAN; len];
    let mut histogram = vec![f64::NAN; len];
    for index in signal_lookback..signal_ma.len() {
        let bar = largest_lookback + index;
        macd_line[bar] = macd_valid[index];
        signal_line[bar] = signal_ma[index];
        histogram[bar] = macd_valid[index] - signal_ma[index];
    }
    Ok((macd_line, signal_line, histogram))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aligns_every_moving_average_combination() {
        let input: Vec<f64> = (0..700)
            .map(|index| 100.0 + (index as f64 * 0.17).sin() * 8.0)
            .collect();
        for fast_code in 0..=8 {
            for slow_code in 0..=8 {
                for signal_code in 0..=8 {
                    let fast_type = MaType::try_from(fast_code).unwrap();
                    let slow_type = MaType::try_from(slow_code).unwrap();
                    let signal_type = MaType::try_from(signal_code).unwrap();
                    let (macd, signal, histogram) =
                        moving_average_convergence_divergence_extended(&input, 7, fast_type, 13, slow_type, 5, signal_type).unwrap();
                    let start =
                        fast_type.lookback(7).max(slow_type.lookback(13)) + signal_type.lookback(5);
                    assert!(macd[..start].iter().all(|value| value.is_nan()));
                    assert!(signal[..start].iter().all(|value| value.is_nan()));
                    assert!(histogram[..start].iter().all(|value| value.is_nan()));
                    assert!(!macd[start].is_nan());
                }
            }
        }
    }
}
