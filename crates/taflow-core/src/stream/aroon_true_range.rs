//! Aroon and true-range family streaming states.

use crate::error::{TaError, TaResult};

use super::{invalid_period, MonotonicMax, MonotonicMin};

#[cfg(test)]
use crate::stream::{Aroon, AroonOscillator, AroonValue};

/// TA-Lib-exact AROON kernel over a whole slice.
///
/// `TA_AROON` tracks a `(index, value)` candidate per side and rescans the
/// live window when it ages past the trailing edge. Unlike MAXINDEX/MININDEX
/// this family is NOT path dependent: the warm-up seed, the fast path and the
/// rescan all use `>=`/`<=`, so the tracked index is always the LATEST window
/// extremum however it was reached. The rescan is amortized O(1) on
/// non-degenerate data and beats carrying indexed sliding-window state per
/// bar.
///
/// `emit(today, down, up)` is called once per warmed bar, `today` in
/// `period..len`; warm-up bars are the caller's business.
#[inline]
pub(crate) fn aroon_rescan<F>(
    high: &[f64],
    low: &[f64],
    period: usize,
    inverse_period: f64,
    mut emit: F,
) where
    F: FnMut(usize, f64, f64),
{
    debug_assert_eq!(high.len(), low.len());
    debug_assert!(high.len() > period);
    let len = high.len();
    let mut highest = high[0];
    let mut highest_index = 0usize;
    let mut lowest = low[0];
    let mut lowest_index = 0usize;
    for index in 1..=period {
        // Latest wins on every path, warm-up included.
        if high[index] >= highest {
            highest = high[index];
            highest_index = index;
        }
        if low[index] <= lowest {
            lowest = low[index];
            lowest_index = index;
        }
    }
    emit(
        period,
        (period - (period - lowest_index)) as f64 * inverse_period,
        (period - (period - highest_index)) as f64 * inverse_period,
    );

    let mut trailing = 1usize;
    for today in period + 1..len {
        if highest_index < trailing {
            highest = high[trailing];
            highest_index = trailing;
            for (offset, &value) in high[trailing + 1..=today].iter().enumerate() {
                if value >= highest {
                    highest = value;
                    highest_index = trailing + 1 + offset;
                }
            }
        } else if high[today] >= highest {
            highest = high[today];
            highest_index = today;
        }
        if lowest_index < trailing {
            lowest = low[trailing];
            lowest_index = trailing;
            for (offset, &value) in low[trailing + 1..=today].iter().enumerate() {
                if value <= lowest {
                    lowest = value;
                    lowest_index = trailing + 1 + offset;
                }
            }
        } else if low[today] <= lowest {
            lowest = low[today];
            lowest_index = today;
        }
        emit(
            today,
            (period - (today - lowest_index)) as f64 * inverse_period,
            (period - (today - highest_index)) as f64 * inverse_period,
        );
        trailing += 1;
    }
}

#[cfg(test)]
mod aroon_bulk_tests {
    use super::*;

    fn lcg_series(len: usize, seed: u64) -> Vec<f64> {
        let mut state = seed;
        (0..len)
            .map(|_| {
                state = state
                    .wrapping_mul(6364136223846793005)
                    .wrapping_add(1442695040888963407);
                ((state >> 33) % 100_003) as f64 / 101.0
            })
            .collect()
    }

    fn series() -> (Vec<f64>, Vec<f64>) {
        let base = lcg_series(5_000, 0x7777_3333_BBBB_1111);
        let high = base.iter().map(|v| v + 0.5).collect();
        let low = base.iter().map(|v| v - 0.5).collect();
        (high, low)
    }

    /// Random, quantized (dense ties), constant (every bar a tie), monotone up
    /// (never rescans) and monotone down (rescans every bar) — one dataset per
    /// branch of the rescan machine.
    fn bulk_datasets() -> Vec<(Vec<f64>, Vec<f64>)> {
        const LEN: usize = 5_000;
        let (high, low) = series();
        let quantized: Vec<f64> = (0..LEN).map(|i| ((i * 7) % 5) as f64).collect();
        let constant = vec![13.25_f64; LEN];
        let increasing: Vec<f64> = (0..LEN).map(|i| i as f64 * 0.5).collect();
        let decreasing: Vec<f64> = (0..LEN).map(|i| LEN as f64 - i as f64 * 0.5).collect();
        vec![
            (high, low),
            (quantized.clone(), quantized),
            (constant.clone(), constant),
            (increasing.clone(), increasing),
            (decreasing.clone(), decreasing),
        ]
    }

    const BULK_CHUNKS: [usize; 5] = [1, 7, 10, 97, 1000];

    #[test]
    fn aroon_bulk_matches_append_bitwise() {
        for (high, low) in bulk_datasets() {
            for period in [2usize, 5, 14, 30, 200] {
                let mut reference = Aroon::new(period).unwrap();
                let expected: Vec<AroonValue> = (0..high.len())
                    .map(|i| {
                        reference.append(high[i], low[i]).unwrap_or(AroonValue {
                            down: f64::NAN,
                            up: f64::NAN,
                        })
                    })
                    .collect();
                for chunk in BULK_CHUNKS.iter().copied().chain([high.len()]) {
                    let mut state = Aroon::new(period).unwrap();
                    let (mut down, mut up) = (Vec::new(), Vec::new());
                    let mut offset = 0;
                    while offset < high.len() {
                        let end = (offset + chunk).min(high.len());
                        state
                            .extend_slices_into(
                                &high[offset..end],
                                &low[offset..end],
                                &mut down,
                                &mut up,
                            )
                            .unwrap();
                        offset = end;
                    }
                    assert_eq!(down.len(), high.len());
                    for (i, expected) in expected.iter().enumerate() {
                        assert_eq!(
                            expected.down.to_bits(),
                            down[i].to_bits(),
                            "down p={period} chunk={chunk} i={i}"
                        );
                        assert_eq!(
                            expected.up.to_bits(),
                            up[i].to_bits(),
                            "up p={period} chunk={chunk} i={i}"
                        );
                    }
                    let mut follow = reference.clone();
                    for i in 0..256 {
                        assert_eq!(
                            follow.append(high[i], low[i]),
                            state.append(high[i], low[i]),
                            "continue p={period} chunk={chunk}"
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn aroon_oscillator_bulk_matches_append_bitwise() {
        for (high, low) in bulk_datasets() {
            for period in [2usize, 5, 14, 30, 200] {
                let mut reference = AroonOscillator::new(period).unwrap();
                let expected: Vec<f64> = (0..high.len())
                    .map(|i| reference.append(high[i], low[i]).unwrap_or(f64::NAN))
                    .collect();
                for chunk in BULK_CHUNKS.iter().copied().chain([high.len()]) {
                    let mut state = AroonOscillator::new(period).unwrap();
                    let mut out = Vec::new();
                    let mut offset = 0;
                    while offset < high.len() {
                        let end = (offset + chunk).min(high.len());
                        state
                            .extend_slices_into(&high[offset..end], &low[offset..end], &mut out)
                            .unwrap();
                        offset = end;
                    }
                    for (i, e) in expected.iter().enumerate() {
                        assert_eq!(
                            e.to_bits(),
                            out[i].to_bits(),
                            "p={period} chunk={chunk} i={i}"
                        );
                    }
                    let mut follow = reference.clone();
                    for i in 0..256 {
                        assert_eq!(
                            follow.append(high[i], low[i]),
                            state.append(high[i], low[i]),
                            "continue p={period} chunk={chunk}"
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn aroon_bulk_validates_lengths() {
        let mut state = Aroon::new(5).unwrap();
        let (mut down, mut up) = (Vec::new(), Vec::new());
        assert!(state
            .extend_slices_into(&[1.0, 2.0], &[1.0], &mut down, &mut up)
            .is_err());
    }
}
