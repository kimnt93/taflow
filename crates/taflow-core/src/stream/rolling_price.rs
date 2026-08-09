//! Rolling midpoint and midprice streaming states.

use crate::error::{TaError, TaResult};

use super::{vhgw, MonotonicMax, MonotonicMin, RollingExtrema, StreamingIndicator};

#[cfg(test)]
use crate::indicators::{RollingMidpoint, RollingMidprice};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream::tests_extrema_support::{datasets, periods_and_lengths};

    #[test]
    fn midpoint_extend_slice_into_is_chunk_invariant() {
        for (period, len) in periods_and_lengths() {
            for data in datasets(len.min(4096)) {
                for chunk in [1usize, 7, data.len().max(1)] {
                    let mut reference = RollingMidpoint::new(period).unwrap();
                    let expected: Vec<f64> = data
                        .iter()
                        .map(|&v| reference.append(v).unwrap_or(f64::NAN))
                        .collect();
                    let mut state = RollingMidpoint::new(period).unwrap();
                    let mut out = Vec::new();
                    for piece in data.chunks(chunk) {
                        state.extend_slice_into(piece, &mut out);
                    }
                    assert_eq!(expected.len(), out.len());
                    for (e, a) in expected.iter().zip(&out) {
                        assert_eq!(e.to_bits(), a.to_bits(), "p={period} chunk={chunk}");
                    }
                    for &value in data.iter().take(64) {
                        assert_eq!(reference.append(value), state.append(value));
                    }
                }
            }
        }
    }

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

    #[test]
    fn midprice_bulk_matches_append_bitwise() {
        let base = lcg_series(5_000, 0x51D3_9E77_0011_2233);
        let high: Vec<f64> = base.iter().map(|v| v + 0.75).collect();
        let low: Vec<f64> = base.iter().map(|v| v - 0.75).collect();
        for period in [2usize, 5, 14, 30, 200] {
            let mut reference = RollingMidprice::new(period).unwrap();
            let expected: Vec<f64> = (0..base.len())
                .map(|i| reference.append(high[i], low[i]).unwrap_or(f64::NAN))
                .collect();
            for chunk in [1usize, 7, 97, base.len()] {
                let mut state = RollingMidprice::new(period).unwrap();
                let mut out = Vec::new();
                let mut offset = 0;
                while offset < base.len() {
                    let end = (offset + chunk).min(base.len());
                    state
                        .extend_slices_into(&high[offset..end], &low[offset..end], &mut out)
                        .unwrap();
                    offset = end;
                }
                assert_eq!(out.len(), base.len());
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

    #[test]
    fn midprice_bulk_validates_lengths() {
        let mut state = RollingMidprice::new(3).unwrap();
        let mut out = Vec::new();
        assert!(state
            .extend_slices_into(&[1.0, 2.0], &[1.0], &mut out)
            .is_err());
    }
}
