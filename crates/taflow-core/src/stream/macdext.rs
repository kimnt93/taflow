//! Stateful extended Moving Average Convergence/Divergence.
//!
//! MACDEXT delays each fast/slow input stream by the difference between its
//! own lookback and the shared largest lookback, then feeds synchronized
//! differences through the selected signal moving average.

use crate::error::{TaError, TaResult};
use crate::ma_type::MaType;

use super::{moving_average::MovingAverage, MacdValue};

/// Incremental MACDEXT with aligned fast/slow seeds.
pub struct MacdExt {
    fast: MovingAverage,
    slow: MovingAverage,
    signal: MovingAverage,
    fast_start: usize,
    slow_start: usize,
    index: usize,
    value: Option<MacdValue>,
}

impl MacdExt {
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
            fast: MovingAverage::new(fastperiod, fastmatype)?,
            slow: MovingAverage::new(slowperiod, slowmatype)?,
            signal: MovingAverage::new(signalperiod, signalmatype)?,
            fast_start: largest_lookback - fast_lookback,
            slow_start: largest_lookback - slow_lookback,
            index: 0,
            value: None,
        })
    }

    /// Appends one close value.
    pub fn append(&mut self, input: f64) -> Option<MacdValue> {
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
            self.signal.append(macd).map(|signal| MacdValue {
                macd,
                signal,
                histogram: macd - signal,
            })
        });
        self.value
    }

    /// Returns the latest warmed output.
    pub fn value(&self) -> Option<MacdValue> {
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
    use crate::momentum;

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
                    let expected =
                        momentum::moving_average_convergence_divergence_extended(&input, 7, fast_type, 13, slow_type, 5, signal_type)
                            .unwrap();
                    let mut state =
                        MacdExt::new(7, fast_type, 13, slow_type, 5, signal_type).unwrap();
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
