//! Stateful fixed-bin session volume profile levels.

use crate::error::{TaError, TaResult};

/// Computes point of control and value-area bounds for each session bar.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `SessionVolumeLevels`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct SessionVolumeLevels {
    bins: usize,
    value_area: f64,
    low: Option<f64>,
    high: f64,
    step: f64,
    histogram: Vec<f64>,
    value: Option<(f64, f64, f64)>,
}

impl SessionVolumeLevels {
    /// Creates a profile with a positive bin count and value-area fraction.
    pub fn new(bins: usize, value_area: f64) -> TaResult<Self> {
        if bins < 1 {
            return Err(super::invalid_period("bins", bins, 1));
        }
        if !(0.0..=1.0).contains(&value_area) || value_area == 0.0 {
            return Err(TaError::InvalidParameter {
                name: "value_area",
                value: value_area.to_string(),
                reason: "must be in (0, 1]",
            });
        }
        Ok(Self {
            bins,
            value_area,
            low: None,
            high: 0.0,
            step: 1.0,
            histogram: vec![0.0; bins],
            value: None,
        })
    }

    /// Appends one OHLCV bar and optionally starts a new anchored session.
    pub fn append(
        &mut self,
        high: f64,
        low: f64,
        close: f64,
        volume: f64,
        anchor: bool,
    ) -> (f64, f64, f64) {
        if anchor || self.low.is_none() {
            self.low = Some(low);
            self.high = high;
            self.step = ((high - low) / self.bins as f64).max(1.0e-12);
            self.histogram.fill(0.0);
        }
        let session_low = self.low.as_mut().expect("initialized above");
        *session_low = session_low.min(low);
        self.high = self.high.max(high);
        let low_value = *session_low;
        let index =
            (((close - low_value) / self.step) as isize).clamp(0, self.bins as isize - 1) as usize;
        self.histogram[index] += volume;
        let histogram = &self.histogram[..];
        // Single fused pass replaces the `max_by` comparator scan plus a
        // separate `iter().sum()`. The comparator kept the accumulator unless
        // the candidate was strictly greater (NaN compared as `Equal`, and the
        // index tie-break then favoured the accumulator), which is exactly
        // `histogram[poc] < value`; the total is still summed in index order,
        // so both results are bit-identical.
        let mut poc = 0usize;
        let mut total = 0.0;
        for (bin, &value) in histogram.iter().enumerate() {
            if histogram[poc] < value {
                poc = bin;
            }
            total += value;
        }
        let target = total * self.value_area;
        let (mut left, mut right, mut accumulated) = (poc, poc, histogram[poc]);
        while accumulated < target && (left > 0 || right + 1 < self.bins) {
            if left == 0 {
                right += 1;
            } else if right + 1 == self.bins {
                left -= 1;
            } else if histogram[left - 1] >= histogram[right + 1] {
                left -= 1;
            } else {
                right += 1;
            }
            // Re-summed in index order over the widened range: the running
            // total the expansion would produce associates differently, so
            // the rescan is kept for bit-exactness.
            accumulated = histogram[left..=right].iter().sum();
        }
        let result = (
            (poc as f64 + 0.5) * self.step + low_value,
            (right as f64 + 0.5) * self.step + low_value,
            (left as f64 + 0.5) * self.step + low_value,
        );
        self.value = Some(result);
        result
    }

    /// Returns point of control, value-area high, and value-area low.
    pub fn value(&self) -> Option<(f64, f64, f64)> {
        self.value
    }
    /// Clears profile and session state.
    pub fn reset(&mut self) {
        self.low = None;
        self.high = 0.0;
        self.step = 1.0;
        self.histogram.fill(0.0);
        self.value = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The pre-optimisation `SessionVolumeLevels::append` body, kept verbatim
    /// as the oracle (`max_by` comparator plus a separate total sum).
    struct Oracle {
        bins: usize,
        value_area: f64,
        low: Option<f64>,
        high: f64,
        step: f64,
        histogram: Vec<f64>,
    }

    impl Oracle {
        fn new(bins: usize, value_area: f64) -> Self {
            Self {
                bins,
                value_area,
                low: None,
                high: 0.0,
                step: 1.0,
                histogram: vec![0.0; bins],
            }
        }

        fn append(
            &mut self,
            high: f64,
            low: f64,
            close: f64,
            volume: f64,
            anchor: bool,
        ) -> (f64, f64, f64) {
            if anchor || self.low.is_none() {
                self.low = Some(low);
                self.high = high;
                self.step = ((high - low) / self.bins as f64).max(1.0e-12);
                self.histogram.fill(0.0);
            }
            let session_low = self.low.as_mut().expect("initialized above");
            *session_low = session_low.min(low);
            self.high = self.high.max(high);
            let low_value = *session_low;
            let index = (((close - low_value) / self.step) as isize)
                .clamp(0, self.bins as isize - 1) as usize;
            self.histogram[index] += volume;
            let poc = self
                .histogram
                .iter()
                .enumerate()
                .max_by(|(a, x), (b, y)| {
                    x.partial_cmp(y)
                        .unwrap_or(std::cmp::Ordering::Equal)
                        .then_with(|| b.cmp(a))
                })
                .map(|(i, _)| i)
                .unwrap_or(0);
            let total: f64 = self.histogram.iter().sum();
            let target = total * self.value_area;
            let (mut left, mut right, mut accumulated) = (poc, poc, self.histogram[poc]);
            while accumulated < target && (left > 0 || right + 1 < self.bins) {
                if left == 0 {
                    right += 1;
                } else if right + 1 == self.bins {
                    left -= 1;
                } else if self.histogram[left - 1] >= self.histogram[right + 1] {
                    left -= 1;
                } else {
                    right += 1;
                }
                accumulated = self.histogram[left..=right].iter().sum();
            }
            (
                (poc as f64 + 0.5) * self.step + low_value,
                (right as f64 + 0.5) * self.step + low_value,
                (left as f64 + 0.5) * self.step + low_value,
            )
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

    fn ohlcv(n: usize, seed: u64) -> (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>) {
        let base = lcg_series(n, seed);
        let spread = lcg_series(n, seed ^ 0xABCD);
        let raw_volume = lcg_series(n, seed ^ 0x1234);
        let mut high = Vec::with_capacity(n);
        let mut low = Vec::with_capacity(n);
        let mut close = Vec::with_capacity(n);
        let mut volume = Vec::with_capacity(n);
        for bar in 0..n {
            let half = (spread[bar] - 90.0) / 20.0 * 0.8 + 0.05;
            high.push(base[bar] + half);
            low.push(base[bar] - half);
            close.push(base[bar]);
            volume.push(raw_volume[bar] * 100.0);
        }
        (high, low, close, volume)
    }

    #[test]
    fn streaming_matches_the_previous_profile_scan_bitwise() {
        let (high, low, close, volume) = ohlcv(5_000, 0x5717_0001);
        for (bins, value_area) in [(1usize, 0.7f64), (2, 1.0), (5, 0.3), (24, 0.7), (100, 0.95)] {
            for session in [1usize, 7, 390, 100_000] {
                let mut state = SessionVolumeLevels::new(bins, value_area).unwrap();
                let mut oracle = Oracle::new(bins, value_area);
                for bar in 0..high.len() {
                    let anchor = bar % session == 0;
                    let actual = state.append(high[bar], low[bar], close[bar], volume[bar], anchor);
                    let expected =
                        oracle.append(high[bar], low[bar], close[bar], volume[bar], anchor);
                    let label =
                        format!("bins {bins} area {value_area} session {session} bar {bar}");
                    assert_eq!(actual.0.to_bits(), expected.0.to_bits(), "{label} poc");
                    assert_eq!(actual.1.to_bits(), expected.1.to_bits(), "{label} vah");
                    assert_eq!(actual.2.to_bits(), expected.2.to_bits(), "{label} val");
                }
            }
        }
    }

    #[test]
    fn zero_volume_ties_pick_the_same_bin() {
        // Every bin stays at 0.0, so the point-of-control tie-break (lowest
        // index wins) and the `total == 0` value-area path are exercised.
        let (high, low, close, _) = ohlcv(500, 0x5717_0002);
        let mut state = SessionVolumeLevels::new(24, 0.7).unwrap();
        let mut oracle = Oracle::new(24, 0.7);
        for bar in 0..high.len() {
            let actual = state.append(high[bar], low[bar], close[bar], 0.0, bar == 0);
            let expected = oracle.append(high[bar], low[bar], close[bar], 0.0, bar == 0);
            assert_eq!(actual.0.to_bits(), expected.0.to_bits(), "bar {bar}");
            assert_eq!(actual.1.to_bits(), expected.1.to_bits(), "bar {bar}");
            assert_eq!(actual.2.to_bits(), expected.2.to_bits(), "bar {bar}");
        }
    }

    #[test]
    fn reset_restores_a_fresh_state() {
        let (high, low, close, volume) = ohlcv(1_000, 0x5717_0003);
        let mut state = SessionVolumeLevels::new(24, 0.7).unwrap();
        for bar in 0..high.len() {
            state.append(high[bar], low[bar], close[bar], volume[bar], bar % 390 == 0);
        }
        state.reset();
        assert!(state.value().is_none());
        let mut fresh = SessionVolumeLevels::new(24, 0.7).unwrap();
        let (high, low, close, volume) = ohlcv(1_000, 0x5717_0004);
        for bar in 0..high.len() {
            let anchor = bar % 390 == 0;
            let after_reset = state.append(high[bar], low[bar], close[bar], volume[bar], anchor);
            let from_fresh = fresh.append(high[bar], low[bar], close[bar], volume[bar], anchor);
            assert_eq!(after_reset.0.to_bits(), from_fresh.0.to_bits());
            assert_eq!(after_reset.1.to_bits(), from_fresh.1.to_bits());
            assert_eq!(after_reset.2.to_bits(), from_fresh.2.to_bits());
        }
    }
}
