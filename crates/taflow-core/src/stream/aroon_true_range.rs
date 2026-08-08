//! Aroon and true-range family streaming states.

use crate::error::{TaError, TaResult};

use super::{invalid_period, vhgw, MonotonicMax, MonotonicMin};

#[derive(Debug, Clone, Copy, PartialEq)]
/// Persistent Rust state or aligned output type for `AroonValue`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct AroonValue {
    pub down: f64,
    pub up: f64,
}

/// Stateful Aroon down/up pair over a `period + 1` bar window.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `Aroon`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct Aroon {
    period: usize,
    inverse_period: f64,
    index: usize,
    highs: MonotonicMax,
    lows: MonotonicMin,
    value: Option<AroonValue>,
}

impl Aroon {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(period: usize) -> TaResult<Self> {
        if period < 2 {
            return Err(invalid_period("timeperiod", period, 2));
        }
        Ok(Self {
            period,
            inverse_period: 100.0 / period as f64,
            index: 0,
            highs: MonotonicMax::new(period + 1)?,
            lows: MonotonicMin::new(period + 1)?,
            value: None,
        })
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, high: f64, low: f64) -> Option<AroonValue> {
        let current = self.index;
        self.index += 1;
        let highest = self.highs.append_indexed(high).map(|(index, _)| index);
        let lowest = self.lows.append_indexed(low).map(|(index, _)| index);
        self.value = highest.zip(lowest).map(|(highest, lowest)| AroonValue {
            down: (self.period - (current - lowest)) as f64 * self.inverse_period,
            up: (self.period - (current - highest)) as f64 * self.inverse_period,
        });
        self.value
    }

    /// Bulk kernel: two indexed vHGW passes over the `period + 1` window.
    ///
    /// Aroon is the one index-based family that is NOT path dependent: TA-Lib
    /// uses `>=`/`<=` in the warm-up, fast, and rescan branches alike, so the
    /// tracked index is always the latest window extremum — exactly the tie
    /// rule of [`vhgw::sliding_argmax_latest_into`]. Outputs and post-run state
    /// are bit-identical to per-bar [`Self::append`]; warm-up bars are NaN.
    pub fn extend_slices_into(
        &mut self,
        high: &[f64],
        low: &[f64],
        down_out: &mut Vec<f64>,
        up_out: &mut Vec<f64>,
    ) -> TaResult<()> {
        if high.len() != low.len() {
            return Err(TaError::LengthMismatch {
                expected: high.len(),
                got: low.len(),
            });
        }
        let n = high.len();
        let period = self.period;
        let window = period + 1;
        if self.index != 0 || n < window {
            down_out.reserve(n);
            up_out.reserve(n);
            for index in 0..n {
                match self.append(high[index], low[index]) {
                    Some(value) => {
                        down_out.push(value.down);
                        up_out.push(value.up);
                    }
                    None => {
                        down_out.push(f64::NAN);
                        up_out.push(f64::NAN);
                    }
                }
            }
            return Ok(());
        }

        let down_start = down_out.len();
        let up_start = up_out.len();
        down_out.resize(down_start + n, f64::NAN);
        up_out.resize(up_start + n, f64::NAN);
        let mut highest = vec![0usize; n - period];
        let mut lowest = vec![0usize; n - period];
        vhgw::sliding_argmax_latest_into(high, window, &mut highest);
        vhgw::sliding_argmin_latest_into(low, window, &mut lowest);
        let inverse_period = self.inverse_period;
        for offset in 0..(n - period) {
            let today = period + offset;
            down_out[down_start + today] =
                (period - (today - lowest[offset])) as f64 * inverse_period;
            up_out[up_start + today] = (period - (today - highest[offset])) as f64 * inverse_period;
        }
        self.highs.rebuild_from_full_run(high);
        self.lows.rebuild_from_full_run(low);
        self.index = n;
        self.value = Some(AroonValue {
            down: *down_out.last().expect("at least one warmed bar"),
            up: *up_out.last().expect("at least one warmed bar"),
        });
        Ok(())
    }

    /// Returns the computed value, aligned history, or a validation error.
    pub fn value(&self) -> Option<AroonValue> {
        self.value
    }

    /// Computes or updates `reset` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn reset(&mut self) {
        self.index = 0;
        self.highs.reset();
        self.lows.reset();
        self.value = None;
    }
}

#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `AroonOscillator`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct AroonOscillator {
    aroon: Aroon,
    value: Option<f64>,
}

impl AroonOscillator {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(period: usize) -> TaResult<Self> {
        Ok(Self {
            aroon: Aroon::new(period)?,
            value: None,
        })
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, high: f64, low: f64) -> Option<f64> {
        self.value = self
            .aroon
            .append(high, low)
            .map(|value| value.up - value.down);
        self.value
    }

    /// Bulk kernel: delegates to [`Aroon::extend_slices_into`] and subtracts
    /// the two aligned series. Bit-identical to per-bar [`Self::append`].
    pub fn extend_slices_into(
        &mut self,
        high: &[f64],
        low: &[f64],
        output: &mut Vec<f64>,
    ) -> TaResult<()> {
        let mut down = Vec::with_capacity(high.len());
        let mut up = Vec::with_capacity(high.len());
        self.aroon
            .extend_slices_into(high, low, &mut down, &mut up)?;
        output.reserve(down.len());
        for (up, down) in up.iter().zip(&down) {
            output.push(up - down);
        }
        self.value = self.aroon.value().map(|value| value.up - value.down);
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
        self.aroon.reset();
        self.value = None;
    }
}

/// Stateful Average True Range.  Each appended bar is `(high, low, close)`.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `AverageTrueRange`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct AverageTrueRange {
    period: usize,
    previous_close: Option<f64>,
    tr_count: usize,
    tr_sum: f64,
    value: Option<f64>,
}

/// Stateful true range. The first bar has no previous close and is not warm.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `TrueRange`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct TrueRange {
    previous_close: Option<f64>,
    value: Option<f64>,
}

impl TrueRange {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new() -> Self {
        Self {
            previous_close: None,
            value: None,
        }
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, high: f64, low: f64, close: f64) -> Option<f64> {
        let previous = self.previous_close.replace(close)?;
        self.value = Some(
            (high - low)
                .max((high - previous).abs())
                .max((low - previous).abs()),
        );
        self.value
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
        self.previous_close = None;
        self.value = None;
    }
}

impl Default for TrueRange {
    fn default() -> Self {
        Self::new()
    }
}

/// Stateful normalized ATR, matching `NATR = ATR / close * 100`.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `NormalizedAverageTrueRange`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct NormalizedAverageTrueRange {
    atr: AverageTrueRange,
    value: Option<f64>,
}

impl NormalizedAverageTrueRange {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(period: usize) -> TaResult<Self> {
        Ok(Self {
            atr: AverageTrueRange::new(period)?,
            value: None,
        })
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, high: f64, low: f64, close: f64) -> Option<f64> {
        self.value = self.atr.append(high, low, close).map(|atr| {
            if close == 0.0 {
                0.0
            } else {
                atr / close * 100.0
            }
        });
        self.value
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
        self.atr.reset();
        self.value = None;
    }
}

impl AverageTrueRange {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(period: usize) -> TaResult<Self> {
        if period == 0 {
            return Err(invalid_period("timeperiod", period, 1));
        }
        Ok(Self {
            period,
            previous_close: None,
            tr_count: 0,
            tr_sum: 0.0,
            value: None,
        })
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, high: f64, low: f64, close: f64) -> Option<f64> {
        let Some(previous_close) = self.previous_close.replace(close) else {
            return None;
        };
        let true_range = (high - low)
            .max((high - previous_close).abs())
            .max((low - previous_close).abs());
        self.tr_count += 1;

        if self.tr_count < self.period {
            self.tr_sum += true_range;
            return None;
        }

        if self.tr_count == self.period {
            self.value = Some((self.tr_sum + true_range) / self.period as f64);
        } else if let Some(previous) = self.value {
            let period = self.period as f64;
            self.value = Some((previous * (period - 1.0) + true_range) / period);
        }
        self.value
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
        self.previous_close = None;
        self.tr_count = 0;
        self.tr_sum = 0.0;
        self.value = None;
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

    #[test]
    fn aroon_bulk_matches_append_bitwise() {
        let (high, low) = series();
        // Quantized duplicates stress the latest-wins tie rule.
        let quantized: Vec<f64> = (0..high.len()).map(|i| ((i * 7) % 5) as f64).collect();
        for (high, low) in [(high, low), (quantized.clone(), quantized.clone())] {
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
                for chunk in [1usize, 7, 97, high.len()] {
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
        let (high, low) = series();
        for period in [2usize, 5, 14, 30, 200] {
            let mut reference = AroonOscillator::new(period).unwrap();
            let expected: Vec<f64> = (0..high.len())
                .map(|i| reference.append(high[i], low[i]).unwrap_or(f64::NAN))
                .collect();
            for chunk in [1usize, 7, 97, high.len()] {
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

    #[test]
    fn aroon_bulk_validates_lengths() {
        let mut state = Aroon::new(5).unwrap();
        let (mut down, mut up) = (Vec::new(), Vec::new());
        assert!(state
            .extend_slices_into(&[1.0, 2.0], &[1.0], &mut down, &mut up)
            .is_err());
    }
}
