//! Batch implementation for `liquidity`.

use super::operator_states::*;
use super::*;
use crate::error::{TaError, TaResult};

/// Computes or updates `liquidity` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Compute the parkinson result for the supplied aligned series.
///
/// # Parameters
///
/// * `high` - Input series or configuration value.
/// * `low` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn liquidity(
    high: &[f64],
    low: &[f64],
    swing_length: usize,
    range_percent: f64,
) -> TaResult<(Vec<f64>, Vec<f64>, Vec<f64>)> {
    if high.len() != low.len() {
        return Err(TaError::LengthMismatch {
            expected: high.len(),
            got: low.len(),
        });
    }
    let mut state = Liquidity::new(swing_length, range_percent)?;
    let mut liquidity_out = Vec::with_capacity(high.len());
    let mut level = Vec::with_capacity(high.len());
    let mut swept = Vec::with_capacity(high.len());
    for (&high, &low) in high.iter().zip(low) {
        let value = state.append(high, low, f64::NAN);
        liquidity_out.push(value.liquidity);
        level.push(value.level);
        swept.push(value.swept);
    }
    Ok((liquidity_out, level, swept))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream::SwingHighLow;

    #[derive(Debug, Clone, Copy)]
    struct OraclePool {
        level: f64,
        count: usize,
    }

    /// The pre-optimisation `Liquidity::append` body, kept verbatim as the
    /// oracle: one insertion-ordered vector per side, swept by a full retain.
    struct Oracle {
        swing: SwingHighLow,
        high_pools: Vec<OraclePool>,
        low_pools: Vec<OraclePool>,
        range_percent: f64,
    }

    impl Oracle {
        fn new(swing_length: usize, range_percent: f64) -> Self {
            Self {
                swing: SwingHighLow::new(swing_length).unwrap(),
                high_pools: Vec::new(),
                low_pools: Vec::new(),
                range_percent,
            }
        }

        fn nearest_pool(pools: &[OraclePool], level: f64, range_percent: f64) -> Option<usize> {
            let mut best: Option<(usize, f64)> = None;
            for (index, pool) in pools.iter().enumerate() {
                let distance = (pool.level - level).abs();
                if distance <= range_percent * pool.level
                    && best.map_or(true, |(_, best_distance)| distance < best_distance)
                {
                    best = Some((index, distance));
                }
            }
            best.map(|(index, _)| index)
        }

        fn append(&mut self, high: f64, low: f64) -> (f64, f64, f64) {
            let mut liquidity = f64::NAN;
            let mut level = f64::NAN;
            let mut swept = f64::NAN;

            if let Some(swing) = self.swing.append(high, low) {
                if swing.signal > 0.0 {
                    if let Some(index) =
                        Self::nearest_pool(&self.high_pools, swing.level, self.range_percent)
                    {
                        let pool = &mut self.high_pools[index];
                        pool.level = pool.level.max(swing.level);
                        pool.count += 1;
                        if pool.count >= 2 {
                            liquidity = 1.0;
                            level = pool.level;
                        }
                    } else {
                        self.high_pools.push(OraclePool {
                            level: swing.level,
                            count: 1,
                        });
                    }
                } else if swing.signal < 0.0 {
                    if let Some(index) =
                        Self::nearest_pool(&self.low_pools, swing.level, self.range_percent)
                    {
                        let pool = &mut self.low_pools[index];
                        pool.level = pool.level.min(swing.level);
                        pool.count += 1;
                        if pool.count >= 2 {
                            liquidity = -1.0;
                            level = pool.level;
                        }
                    } else {
                        self.low_pools.push(OraclePool {
                            level: swing.level,
                            count: 1,
                        });
                    }
                }
            }

            self.high_pools.retain(|pool| {
                let swept_pool = pool.count >= 2 && high >= pool.level;
                if swept_pool {
                    swept = 1.0;
                    level = pool.level;
                }
                !swept_pool
            });
            self.low_pools.retain(|pool| {
                let swept_pool = pool.count >= 2 && low <= pool.level;
                if swept_pool {
                    swept = -1.0;
                    level = pool.level;
                }
                !swept_pool
            });

            (liquidity, level, swept)
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

    /// OHLC-shaped bars: a random walk plus a random half-range.
    fn bars(n: usize, seed: u64) -> (Vec<f64>, Vec<f64>) {
        let base = lcg_series(n, seed);
        let spread = lcg_series(n, seed ^ 0xABCD);
        let mut high = Vec::with_capacity(n);
        let mut low = Vec::with_capacity(n);
        for bar in 0..n {
            let half = (spread[bar] - 90.0) / 20.0 * 0.8 + 0.05;
            high.push(base[bar] + half);
            low.push(base[bar] - half);
        }
        (high, low)
    }

    #[test]
    fn streaming_matches_the_previous_pool_layout_bitwise() {
        let (high, low) = bars(5_000, 0x1190_0001);
        // Wide `range_percent` values force heavy pool clustering; narrow
        // values force many single-touch candidates.
        for (swing_length, range_percent) in [
            (1usize, 0.01f64),
            (2, 0.0),
            (5, 0.001),
            (5, 0.01),
            (20, 0.05),
            (50, 0.01),
            (50, 1.0),
        ] {
            let mut state = Liquidity::new(swing_length, range_percent).unwrap();
            let mut oracle = Oracle::new(swing_length, range_percent);
            for bar in 0..high.len() {
                let actual = state.append(high[bar], low[bar], f64::NAN);
                let expected = oracle.append(high[bar], low[bar]);
                let label = format!("swing {swing_length} range {range_percent} bar {bar}");
                assert_eq!(
                    actual.liquidity.to_bits(),
                    expected.0.to_bits(),
                    "{label} liquidity"
                );
                assert_eq!(
                    actual.level.to_bits(),
                    expected.1.to_bits(),
                    "{label} level"
                );
                assert_eq!(
                    actual.swept.to_bits(),
                    expected.2.to_bits(),
                    "{label} swept"
                );
            }
        }
    }

    #[test]
    fn batch_matches_per_bar_streaming_bitwise() {
        let (high, low) = bars(5_000, 0x1190_0002);
        for (swing_length, range_percent) in [(5usize, 0.01f64), (50, 0.01)] {
            let (liquidity, level, swept) =
                liquidity(&high, &low, swing_length, range_percent).unwrap();
            let mut state = Liquidity::new(swing_length, range_percent).unwrap();
            for bar in 0..high.len() {
                let expected = state.append(high[bar], low[bar], f64::NAN);
                assert_eq!(liquidity[bar].to_bits(), expected.liquidity.to_bits());
                assert_eq!(level[bar].to_bits(), expected.level.to_bits());
                assert_eq!(swept[bar].to_bits(), expected.swept.to_bits());
            }
        }
    }

    #[test]
    fn reset_restores_a_fresh_state() {
        let (high, low) = bars(1_000, 0x1190_0003);
        let mut state = Liquidity::new(5, 0.01).unwrap();
        for bar in 0..high.len() {
            state.append(high[bar], low[bar], f64::NAN);
        }
        state.reset();
        assert!(state.value().is_none());
        let mut fresh = Liquidity::new(5, 0.01).unwrap();
        let (high, low) = bars(1_000, 0x1190_0004);
        for bar in 0..high.len() {
            let after_reset = state.append(high[bar], low[bar], f64::NAN);
            let from_fresh = fresh.append(high[bar], low[bar], f64::NAN);
            assert_eq!(
                after_reset.liquidity.to_bits(),
                from_fresh.liquidity.to_bits()
            );
            assert_eq!(after_reset.level.to_bits(), from_fresh.level.to_bits());
            assert_eq!(after_reset.swept.to_bits(), from_fresh.swept.to_bits());
        }
    }
}
use super::operator_states::*;
use super::*;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq)]
/// Persistent Rust state or aligned output type for `LiquidityValue`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct LiquidityValue {
    pub liquidity: f64,
    pub level: f64,
    pub swept: f64,
}

/// Causal liquidity-pool clustering with sweep detection. SwingHighLow highs and
/// lows are clustered into pools when they fall within a `range_percent`
/// price tolerance; a pool emits a signal once a second swing confirms it.
/// A pool is swept and removed when price trades beyond its level.
///
/// Pools are split per side into `*_candidates` (seen once, never sweepable)
/// and `*_confirmed` (seen twice or more, kept sorted by insertion `seq`).
/// The per-bar sweep pass therefore only scans confirmed pools instead of the
/// unbounded historical candidate list; outputs are identical to the previous
/// single-vector implementation.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `Liquidity`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct Liquidity {
    swing: SwingHighLow,
    high_candidates: Vec<LiquidityPool>,
    high_confirmed: Vec<LiquidityPool>,
    low_candidates: Vec<LiquidityPool>,
    low_confirmed: Vec<LiquidityPool>,
    next_seq: u64,
    range_percent: f64,
    value: Option<LiquidityValue>,
}

impl Liquidity {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(swing_length: usize, range_percent: f64) -> TaResult<Self> {
        validate_period(swing_length)?;
        if !(0.0..=1.0).contains(&range_percent) {
            return Err(TaError::InvalidParameter {
                name: "range_percent",
                value: range_percent.to_string(),
                reason: "must be between 0 and 1",
            });
        }
        Ok(Self {
            swing: SwingHighLow::new(swing_length)?,
            high_candidates: Vec::new(),
            high_confirmed: Vec::new(),
            low_candidates: Vec::new(),
            low_confirmed: Vec::new(),
            next_seq: 0,
            range_percent,
            value: None,
        })
    }

    /// Nearest pool by absolute distance; ties resolve to the earliest
    /// inserted pool (`seq`), matching the original first-match-wins scan
    /// over a single insertion-ordered vector.
    fn nearest_pool(
        candidates: &[LiquidityPool],
        confirmed: &[LiquidityPool],
        level: f64,
        range_percent: f64,
    ) -> Option<PoolSlot> {
        let mut best: Option<(PoolSlot, f64, u64)> = None;
        let mut consider = |slot: PoolSlot, pool: &LiquidityPool| {
            let distance = (pool.level - level).abs();
            if distance <= range_percent * pool.level
                && best.map_or(true, |(_, best_distance, best_seq)| {
                    distance < best_distance || (distance == best_distance && pool.seq < best_seq)
                })
            {
                best = Some((slot, distance, pool.seq));
            }
        };
        for (index, pool) in candidates.iter().enumerate() {
            consider(PoolSlot::Candidate(index), pool);
        }
        for (index, pool) in confirmed.iter().enumerate() {
            consider(PoolSlot::Confirmed(index), pool);
        }
        best.map(|(slot, _, _)| slot)
    }

    /// Merge a swing into the pools of one side; returns the emitted level if
    /// the pool is (or becomes) confirmed. `merge_level` is `f64::max` for
    /// highs and `f64::min` for lows.
    fn merge_swing(
        candidates: &mut Vec<LiquidityPool>,
        confirmed: &mut Vec<LiquidityPool>,
        next_seq: &mut u64,
        swing_level: f64,
        range_percent: f64,
        merge_level: fn(f64, f64) -> f64,
    ) -> Option<f64> {
        match Self::nearest_pool(candidates, confirmed, swing_level, range_percent) {
            Some(PoolSlot::Confirmed(index)) => {
                let pool = &mut confirmed[index];
                pool.level = merge_level(pool.level, swing_level);
                Some(pool.level)
            }
            Some(PoolSlot::Candidate(index)) => {
                // Second touch: promote to confirmed. `swap_remove` is fine
                // because candidate order is irrelevant (ties use `seq`);
                // confirmed stays sorted by `seq` to preserve sweep order.
                let mut pool = candidates.swap_remove(index);
                pool.level = merge_level(pool.level, swing_level);
                let position = confirmed.partition_point(|entry| entry.seq < pool.seq);
                confirmed.insert(position, pool);
                Some(confirmed[position].level)
            }
            None => {
                candidates.push(LiquidityPool {
                    level: swing_level,
                    seq: *next_seq,
                });
                *next_seq += 1;
                None
            }
        }
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, high: f64, low: f64, _close: f64) -> LiquidityValue {
        let mut liquidity = f64::NAN;
        let mut level = f64::NAN;
        let mut swept = f64::NAN;

        if let Some(swing) = self.swing.append(high, low) {
            if swing.signal > 0.0 {
                if let Some(emitted) = Self::merge_swing(
                    &mut self.high_candidates,
                    &mut self.high_confirmed,
                    &mut self.next_seq,
                    swing.level,
                    self.range_percent,
                    f64::max,
                ) {
                    liquidity = 1.0;
                    level = emitted;
                }
            } else if swing.signal < 0.0 {
                if let Some(emitted) = Self::merge_swing(
                    &mut self.low_candidates,
                    &mut self.low_confirmed,
                    &mut self.next_seq,
                    swing.level,
                    self.range_percent,
                    f64::min,
                ) {
                    liquidity = -1.0;
                    level = emitted;
                }
            }
        }

        // Sweep pass over confirmed pools only (candidates can never satisfy
        // the original `count >= 2` predicate). Confirmed pools are kept in
        // insertion order, so the last swept pool sets the outputs exactly as
        // the original combined retain did.
        self.high_confirmed.retain(|pool| {
            let swept_pool = high >= pool.level;
            if swept_pool {
                swept = 1.0;
                level = pool.level;
            }
            !swept_pool
        });
        self.low_confirmed.retain(|pool| {
            let swept_pool = low <= pool.level;
            if swept_pool {
                swept = -1.0;
                level = pool.level;
            }
            !swept_pool
        });

        let value = LiquidityValue {
            liquidity,
            level,
            swept,
        };
        self.value = Some(value);
        value
    }

    /// Computes or updates `value` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn value(&self) -> Option<LiquidityValue> {
        self.value
    }

    /// Reset the persistent state and clear the latest value.
    pub fn reset(&mut self) {
        self.swing.reset();
        self.high_candidates.clear();
        self.high_confirmed.clear();
        self.low_candidates.clear();
        self.low_confirmed.clear();
        self.next_seq = 0;
        self.value = None;
    }
}
