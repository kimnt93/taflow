//! Stateful volume and volume-derived streaming indicators.

use crate::error::{TaError, TaResult};

use super::{invalid_period, vhgw, MonotonicMax, MonotonicMin};

#[cfg(test)]
use crate::stream::{
    AccumulationDistribution, AccumulationDistributionOscillator, WilliamsPercentR,
};

pub(crate) fn ad_increment(high: f64, low: f64, close: f64, volume: f64) -> f64 {
    let range = high - low;
    if range > 0.0 {
        ((close - low) - (high - close)) / range * volume
    } else {
        0.0
    }
}

#[cfg(test)]
mod williams_bulk_tests {
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

    #[test]
    fn williams_percent_r_bulk_matches_append_bitwise() {
        let close = lcg_series(5_000, 0xA5A5_5A5A_1234_9876);
        let high: Vec<f64> = close.iter().map(|v| v + 1.25).collect();
        let low: Vec<f64> = close.iter().map(|v| v - 1.25).collect();
        // Degenerate variant with a zero range exercises the `range > 0` branch.
        let flat = vec![7.5_f64; 5_000];
        for (high, low, close) in [
            (high, low, close.clone()),
            (flat.clone(), flat.clone(), flat.clone()),
        ] {
            for period in [2usize, 5, 14, 30, 200] {
                let mut reference = WilliamsPercentR::new(period).unwrap();
                let expected: Vec<f64> = (0..close.len())
                    .map(|i| {
                        reference
                            .append(high[i], low[i], close[i])
                            .unwrap_or(f64::NAN)
                    })
                    .collect();
                for chunk in [1usize, 7, 97, close.len()] {
                    let mut state = WilliamsPercentR::new(period).unwrap();
                    let mut out = Vec::new();
                    let mut offset = 0;
                    while offset < close.len() {
                        let end = (offset + chunk).min(close.len());
                        state
                            .extend_slices_into(
                                &high[offset..end],
                                &low[offset..end],
                                &close[offset..end],
                                &mut out,
                            )
                            .unwrap();
                        offset = end;
                    }
                    assert_eq!(out.len(), close.len());
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
                            follow.append(high[i], low[i], close[i]),
                            state.append(high[i], low[i], close[i]),
                            "continue p={period} chunk={chunk}"
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn williams_percent_r_bulk_validates_lengths() {
        let mut state = WilliamsPercentR::new(5).unwrap();
        let mut out = Vec::new();
        assert!(state
            .extend_slices_into(&[1.0, 2.0], &[1.0], &[1.0, 2.0], &mut out)
            .is_err());
    }
}
