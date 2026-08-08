//! Batch implementation for `break_of_structure_change_of_character`.

use super::operator_states::*;
use crate::error::{TaError, TaResult};

/// Computes break-of-structure and change-of-character events.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Compute the close to close sigma result for the supplied aligned series.
///
/// # Parameters
///
/// * `input` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn break_of_structure_change_of_character(
    high: &[f64],
    low: &[f64],
    close: &[f64],
    swing_length: usize,
) -> TaResult<(Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>)> {
    if high.len() != low.len() || low.len() != close.len() {
        return Err(TaError::LengthMismatch {
            expected: high.len(),
            got: low.len().max(close.len()),
        });
    }
    let mut state = BreakOfStructureChangeOfCharacter::new(swing_length)?;
    let mut bos = Vec::with_capacity(high.len());
    let mut choch = Vec::with_capacity(high.len());
    let mut level = Vec::with_capacity(high.len());
    let mut broken = Vec::with_capacity(high.len());
    for ((&high, &low), &close) in high.iter().zip(low).zip(close) {
        let value = state.append(high, low, close);
        bos.push(value.bos);
        choch.push(value.choch);
        level.push(value.level);
        broken.push(value.broken);
    }
    Ok((bos, choch, level, broken))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream::SwingHighLow;
    use std::collections::VecDeque;

    /// The pre-optimisation `BreakOfStructureChangeOfCharacter::append` body,
    /// kept verbatim as the oracle (per-event `Vec` collect included).
    struct Oracle {
        swing: SwingHighLow,
        swings: VecDeque<(f64, f64)>,
        pending: Option<(f64, f64)>,
        trend: Option<f64>,
    }

    impl Oracle {
        fn new(swing_length: usize) -> Self {
            Self {
                swing: SwingHighLow::new(swing_length).unwrap(),
                swings: VecDeque::with_capacity(4),
                pending: None,
                trend: None,
            }
        }

        fn append(&mut self, high: f64, low: f64, close: f64) -> [f64; 4] {
            let mut bos = f64::NAN;
            let mut choch = f64::NAN;
            let mut level = f64::NAN;
            let mut broken = f64::NAN;

            if let Some((direction, pending_level)) = self.pending {
                let crossed = (direction > 0.0 && close > pending_level)
                    || (direction < 0.0 && close < pending_level);
                if crossed {
                    broken = direction;
                    level = pending_level;
                    self.pending = None;
                    self.trend = Some(direction);
                }
            }

            if let Some(swing) = self.swing.append(high, low) {
                self.swings.push_back((swing.signal, swing.level));
                if self.swings.len() > 4 {
                    self.swings.pop_front();
                }
                if self.swings.len() == 4 {
                    let items: Vec<_> = self.swings.iter().copied().collect();
                    let bullish = items[0].0 < 0.0
                        && items[1].0 > 0.0
                        && items[2].0 < 0.0
                        && items[3].0 > 0.0
                        && items[0].1 < items[2].1
                        && items[1].1 < items[3].1;
                    let bearish = items[0].0 > 0.0
                        && items[1].0 < 0.0
                        && items[2].0 > 0.0
                        && items[3].0 < 0.0
                        && items[0].1 > items[2].1
                        && items[1].1 > items[3].1;
                    let direction = if bullish {
                        Some(1.0)
                    } else if bearish {
                        Some(-1.0)
                    } else {
                        None
                    };
                    if let Some(direction) = direction {
                        bos = direction;
                        choch = if self.trend.is_some_and(|trend| trend != direction) {
                            direction
                        } else {
                            f64::NAN
                        };
                        level = items[1].1;
                        self.pending = Some((direction, level));
                    }
                }
            }

            [bos, choch, level, broken]
        }
    }

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

    fn bars(n: usize, seed: u64) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
        let base = lcg_series(n, seed);
        let spread = lcg_series(n, seed ^ 0xABCD);
        let mut high = Vec::with_capacity(n);
        let mut low = Vec::with_capacity(n);
        let mut close = Vec::with_capacity(n);
        for bar in 0..n {
            let half = (spread[bar] - 90.0) / 20.0 * 0.8 + 0.05;
            high.push(base[bar] + half);
            low.push(base[bar] - half);
            close.push(base[bar]);
        }
        (high, low, close)
    }

    #[test]
    fn streaming_matches_the_previous_detector_bitwise() {
        let (high, low, close) = bars(5_000, 0xB05C_0001);
        for swing_length in [1usize, 2, 3, 5, 20, 50] {
            let mut state = BreakOfStructureChangeOfCharacter::new(swing_length).unwrap();
            let mut oracle = Oracle::new(swing_length);
            for bar in 0..high.len() {
                let actual = state.append(high[bar], low[bar], close[bar]);
                let expected = oracle.append(high[bar], low[bar], close[bar]);
                let label = format!("swing {swing_length} bar {bar}");
                assert_eq!(actual.bos.to_bits(), expected[0].to_bits(), "{label} bos");
                assert_eq!(
                    actual.choch.to_bits(),
                    expected[1].to_bits(),
                    "{label} choch"
                );
                assert_eq!(
                    actual.level.to_bits(),
                    expected[2].to_bits(),
                    "{label} level"
                );
                assert_eq!(
                    actual.broken.to_bits(),
                    expected[3].to_bits(),
                    "{label} broken"
                );
            }
        }
    }

    #[test]
    fn batch_matches_per_bar_streaming_bitwise() {
        let (high, low, close) = bars(5_000, 0xB05C_0002);
        for swing_length in [1usize, 5, 50] {
            let (bos, choch, level, broken) =
                break_of_structure_change_of_character(&high, &low, &close, swing_length).unwrap();
            let mut state = BreakOfStructureChangeOfCharacter::new(swing_length).unwrap();
            for bar in 0..high.len() {
                let expected = state.append(high[bar], low[bar], close[bar]);
                assert_eq!(bos[bar].to_bits(), expected.bos.to_bits(), "bar {bar}");
                assert_eq!(choch[bar].to_bits(), expected.choch.to_bits(), "bar {bar}");
                assert_eq!(level[bar].to_bits(), expected.level.to_bits(), "bar {bar}");
                assert_eq!(
                    broken[bar].to_bits(),
                    expected.broken.to_bits(),
                    "bar {bar}"
                );
            }
        }
    }

    #[test]
    fn reset_restores_a_fresh_state() {
        let (high, low, close) = bars(1_000, 0xB05C_0003);
        let mut state = BreakOfStructureChangeOfCharacter::new(5).unwrap();
        for bar in 0..high.len() {
            state.append(high[bar], low[bar], close[bar]);
        }
        state.reset();
        assert!(state.value().is_none());
        let mut fresh = BreakOfStructureChangeOfCharacter::new(5).unwrap();
        let (high, low, close) = bars(1_000, 0xB05C_0004);
        for bar in 0..high.len() {
            let after_reset = state.append(high[bar], low[bar], close[bar]);
            let from_fresh = fresh.append(high[bar], low[bar], close[bar]);
            assert_eq!(after_reset.bos.to_bits(), from_fresh.bos.to_bits());
            assert_eq!(after_reset.choch.to_bits(), from_fresh.choch.to_bits());
            assert_eq!(after_reset.level.to_bits(), from_fresh.level.to_bits());
            assert_eq!(after_reset.broken.to_bits(), from_fresh.broken.to_bits());
        }
    }
}
