use super::operator_states::*;
use super::*;
use crate::error::{TaError, TaResult};

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
