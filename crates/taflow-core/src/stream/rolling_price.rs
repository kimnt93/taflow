//! Rolling midpoint and midprice streaming states.

use crate::error::{TaError, TaResult};

use super::{vhgw, MonotonicMax, MonotonicMin, RollingExtrema, StreamingIndicator};

/// Stateful midpoint of the rolling highest and lowest input values.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `RollingMidpoint`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct RollingMidpoint {
    extrema: RollingExtrema,
    value: Option<f64>,
}

impl RollingMidpoint {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(period: usize) -> TaResult<Self> {
        Ok(Self {
            extrema: RollingExtrema::new(period)?,
            value: None,
        })
    }
}

impl StreamingIndicator for RollingMidpoint {
    type Output = f64;

    fn append(&mut self, input: f64) -> Option<f64> {
        self.value = self
            .extrema
            .append(input)
            .map(|(maximum, minimum)| (maximum + minimum) * 0.5);
        self.value
    }

    fn value(&self) -> Option<f64> {
        self.value
    }

    fn reset(&mut self) {
        self.extrema.reset();
        self.value = None;
    }

    fn extend_slice_into(&mut self, inputs: &[f64], output: &mut Vec<f64>) {
        let period = self.extrema.period();
        if self.extrema.count() != 0 || inputs.len() < period {
            output.reserve(inputs.len());
            output.extend(
                inputs
                    .iter()
                    .copied()
                    .map(|input| self.append(input).unwrap_or(f64::NAN)),
            );
            return;
        }
        let start = output.len();
        output.resize(start + inputs.len(), f64::NAN);
        let warm = start + period - 1;
        let mut lowest = vec![0.0_f64; inputs.len() - (period - 1)];
        vhgw::sliding_max_into(inputs, period, &mut output[warm..]);
        vhgw::sliding_min_into(inputs, period, &mut lowest);
        for (slot, &minimum) in output[warm..].iter_mut().zip(&lowest) {
            *slot = (*slot + minimum) * 0.5;
        }
        self.extrema.rebuild_from_full_run(inputs);
        self.value = output.last().copied();
    }
}

/// Stateful midpoint of rolling high maxima and low minima.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `RollingMidprice`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct RollingMidprice {
    highs: MonotonicMax,
    lows: MonotonicMin,
    value: Option<f64>,
}

impl RollingMidprice {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(period: usize) -> TaResult<Self> {
        Ok(Self {
            highs: MonotonicMax::new(period)?,
            lows: MonotonicMin::new(period)?,
            value: None,
        })
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, high: f64, low: f64) -> Option<f64> {
        let maximum = self.highs.append(high);
        let minimum = self.lows.append(low);
        self.value = maximum.zip(minimum).map(|(high, low)| (high + low) * 0.5);
        self.value
    }

    /// Bulk kernel: one vHGW max pass over `high` and one vHGW min pass over
    /// `low`, midpoint applied in place. The trailing `period` inputs are
    /// replayed to rebuild the monotonic deques, so outputs and post-run state
    /// are bit-identical to per-bar [`Self::append`]; warm-up bars are NaN.
    pub fn extend_slices_into(
        &mut self,
        high: &[f64],
        low: &[f64],
        output: &mut Vec<f64>,
    ) -> TaResult<()> {
        if high.len() != low.len() {
            return Err(TaError::LengthMismatch {
                expected: high.len(),
                got: low.len(),
            });
        }
        let n = high.len();
        let period = self.highs.period();
        if self.highs.count() != 0 || n < period {
            output.reserve(n);
            for index in 0..n {
                output.push(self.append(high[index], low[index]).unwrap_or(f64::NAN));
            }
            return Ok(());
        }
        let start = output.len();
        output.resize(start + n, f64::NAN);
        let warm = start + period - 1;
        let mut lowest = vec![0.0_f64; n - (period - 1)];
        vhgw::sliding_max_into(high, period, &mut output[warm..]);
        vhgw::sliding_min_into(low, period, &mut lowest);
        for (slot, &minimum) in output[warm..].iter_mut().zip(&lowest) {
            *slot = (*slot + minimum) * 0.5;
        }
        self.highs.rebuild_from_full_run(high);
        self.lows.rebuild_from_full_run(low);
        self.value = output.last().copied();
        Ok(())
    }

    /// Computes or updates `value` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Computes or updates `reset` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn reset(&mut self) {
        self.highs.reset();
        self.lows.reset();
        self.value = None;
    }
}

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
