//! Stateful extended Moving Average Convergence/Divergence.
//!
//! MACDEXT delays each fast/slow input stream by the difference between its
//! own lookback and the shared largest lookback, then feeds synchronized
//! differences through the selected signal moving average.

use crate::error::{TaError, TaResult};
use crate::ma_type::MaType;

use super::{moving_average::MovingAverageDispatcher, MovingAverageConvergenceDivergenceValue};

/// Incremental MACDEXT with aligned fast/slow seeds.
/// Persistent Rust state or aligned output type for `MovingAverageConvergenceDivergenceExtended`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct MovingAverageConvergenceDivergenceExtended {
    fast: MovingAverageDispatcher,
    slow: MovingAverageDispatcher,
    signal: MovingAverageDispatcher,
    fast_start: usize,
    slow_start: usize,
    index: usize,
    value: Option<MovingAverageConvergenceDivergenceValue>,
}

impl MovingAverageConvergenceDivergenceExtended {
    /// Creates a MACDEXT state with independently selected MA types.
    pub fn new(
        fastperiod: usize,
        fastmatype: MaType,
        slowperiod: usize,
        slowmatype: MaType,
        signalperiod: usize,
        signalmatype: MaType,
    ) -> TaResult<Self> {
        if fastperiod < 2 || slowperiod < 2 || signalperiod == 0 {
            return Err(TaError::InvalidParameter {
                name: "fastperiod/slowperiod/signalperiod",
                value: format!("{fastperiod}/{slowperiod}/{signalperiod}"),
                reason: "fastperiod >= 2, slowperiod >= 2, signalperiod >= 1",
            });
        }
        let (fastperiod, fastmatype, slowperiod, slowmatype) = if fastperiod < slowperiod {
            (fastperiod, fastmatype, slowperiod, slowmatype)
        } else {
            (slowperiod, slowmatype, fastperiod, fastmatype)
        };
        let fast_lookback = fastmatype.lookback(fastperiod);
        let slow_lookback = slowmatype.lookback(slowperiod);
        let largest_lookback = fast_lookback.max(slow_lookback);
        Ok(Self {
            fast: MovingAverageDispatcher::new(fastperiod, fastmatype)?,
            slow: MovingAverageDispatcher::new(slowperiod, slowmatype)?,
            signal: MovingAverageDispatcher::new(signalperiod, signalmatype)?,
            fast_start: largest_lookback - fast_lookback,
            slow_start: largest_lookback - slow_lookback,
            index: 0,
            value: None,
        })
    }

    /// Appends one close value.
    pub fn append(&mut self, input: f64) -> Option<MovingAverageConvergenceDivergenceValue> {
        let index = self.index;
        self.index += 1;
        let fast = if index >= self.fast_start {
            self.fast.append(input)
        } else {
            None
        };
        let slow = if index >= self.slow_start {
            self.slow.append(input)
        } else {
            None
        };
        self.value = fast.zip(slow).and_then(|(fast, slow)| {
            let macd = fast - slow;
            self.signal
                .append(macd)
                .map(|signal| MovingAverageConvergenceDivergenceValue {
                    macd,
                    signal,
                    histogram: macd - signal,
                })
        });
        self.value
    }

    /// Returns the latest warmed output.
    pub fn value(&self) -> Option<MovingAverageConvergenceDivergenceValue> {
        self.value
    }

    /// Restores the post-construction state.
    pub fn reset(&mut self) {
        self.fast.reset();
        self.slow.reset();
        self.signal.reset();
        self.index = 0;
        self.value = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_batch_for_every_moving_average_combination() {
        let input: Vec<f64> = (0..700)
            .map(|index| 100.0 + (index as f64 * 0.17).sin() * 8.0 + index as f64 * 0.01)
            .collect();
        for fast_code in 0..=8 {
            for slow_code in 0..=8 {
                for signal_code in 0..=8 {
                    let fast_type = MaType::try_from(fast_code).unwrap();
                    let slow_type = MaType::try_from(slow_code).unwrap();
                    let signal_type = MaType::try_from(signal_code).unwrap();
                    let expected = crate::stream::moving_average_convergence_divergence_extended(
                        &input,
                        7,
                        fast_type,
                        13,
                        slow_type,
                        5,
                        signal_type,
                    )
                    .unwrap();
                    let mut state = MovingAverageConvergenceDivergenceExtended::new(
                        7,
                        fast_type,
                        13,
                        slow_type,
                        5,
                        signal_type,
                    )
                    .unwrap();
                    for (index, input) in input.iter().copied().enumerate() {
                        match state.append(input) {
                            Some(actual) => {
                                assert!((actual.macd - expected.0[index]).abs() < 1e-8);
                                assert!((actual.signal - expected.1[index]).abs() < 1e-8);
                                assert!((actual.histogram - expected.2[index]).abs() < 1e-8);
                            }
                            None => assert!(expected.0[index].is_nan()),
                        }
                    }
                }
            }
        }
    }
}
// Batch extended Moving Average Convergence/Divergence.
//
// MACDEXT permits independently selected fast, slow, and signal moving
// averages. Fast and slow inputs are aligned to a shared largest lookback so
// their seeds reproduce TA-Lib's internal start-index calls.

use crate::ma_type::compute_ma;

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
///
/// # Parameters
///
/// * `input` - Chronological close-price series.
/// * Period and moving-average parameters configure each MACD leg.
///
/// # Returns
///
/// Three aligned arrays containing MACD, signal, and histogram values.
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

    if fastmatype == MaType::ExponentialMovingAverage
        && slowmatype == MaType::ExponentialMovingAverage
        && signalmatype == MaType::ExponentialMovingAverage
    {
        return super::macd::moving_average_convergence_divergence(
            input,
            fastperiod,
            slowperiod,
            signalperiod,
        );
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
