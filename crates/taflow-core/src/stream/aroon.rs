use super::vhgw;
use crate::error::{TaError, TaResult};

/// Aroon (AROON) — two vHGW latest-wins index passes
///
/// Pass 1: scan high[] → aroon_up (latest-wins argmax indices)
/// Pass 2: scan low[]  → aroon_down (latest-wins argmin indices)
///
/// TA-Lib's Aroon tracker uses `>=`/`<=` everywhere (warm-up, fast path,
/// and rescan), so the tracked extremum index is always the LATEST window
/// maximizer/minimizer — exactly the tie rule of the vHGW indexed kernels.
///
/// Compute the aroon result for the supplied aligned series.
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
pub fn aroon(high: &[f64], low: &[f64], timeperiod: usize) -> TaResult<(Vec<f64>, Vec<f64>)> {
    let len = high.len();
    if len != low.len() {
        return Err(TaError::LengthMismatch {
            expected: len,
            got: low.len(),
        });
    }
    if timeperiod < 2 {
        return Err(TaError::InvalidParameter {
            name: "timeperiod",
            value: timeperiod.to_string(),
            reason: "must be >= 2",
        });
    }
    if len <= timeperiod {
        return Err(TaError::InsufficientData {
            need: timeperiod + 1,
            got: len,
        });
    }

    let inv_period = 100.0 / timeperiod as f64;

    let aroon_up = aroon_max_pass(high, timeperiod, inv_period);
    let aroon_down = aroon_min_pass(low, timeperiod, inv_period);

    Ok((aroon_down, aroon_up))
}

/// vHGW latest-wins argmax pass for aroon_up.
#[inline]
pub(crate) fn aroon_max_pass(data: &[f64], timeperiod: usize, inv_period: f64) -> Vec<f64> {
    let len = data.len();
    let window = timeperiod + 1;
    let mut output = vec![0.0_f64; len];
    output[..timeperiod].fill(f64::NAN);

    let mut indices = vec![0usize; len - timeperiod];
    vhgw::sliding_argmax_latest_into(data, window, &mut indices);
    for (offset, &highest_idx) in indices.iter().enumerate() {
        let today = timeperiod + offset;
        output[today] = (timeperiod - (today - highest_idx)) as f64 * inv_period;
    }
    output
}

/// vHGW latest-wins argmin pass for aroon_down.
#[inline]
pub(crate) fn aroon_min_pass(data: &[f64], timeperiod: usize, inv_period: f64) -> Vec<f64> {
    let len = data.len();
    let window = timeperiod + 1;
    let mut output = vec![0.0_f64; len];
    output[..timeperiod].fill(f64::NAN);

    let mut indices = vec![0usize; len - timeperiod];
    vhgw::sliding_argmin_latest_into(data, window, &mut indices);
    for (offset, &lowest_idx) in indices.iter().enumerate() {
        let today = timeperiod + offset;
        output[today] = (timeperiod - (today - lowest_idx)) as f64 * inv_period;
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream::tests_extrema_support::{datasets, periods_and_lengths};

    /// Original track-and-rescan passes, kept verbatim as oracle.
    fn reference_aroon_max_pass(data: &[f64], timeperiod: usize, inv_period: f64) -> Vec<f64> {
        let len = data.len();
        let window = timeperiod + 1;
        let mut output = vec![0.0_f64; len];
        output[..timeperiod].fill(f64::NAN);

        let mut highest = data[0];
        let mut highest_idx: usize = 0;
        for j in 1..window {
            if data[j] >= highest {
                highest = data[j];
                highest_idx = j;
            }
        }
        output[timeperiod] = (timeperiod - (timeperiod - highest_idx)) as f64 * inv_period;

        let mut trailing_idx = 1;
        let mut today = timeperiod + 1;

        while today < len {
            let h = data[today];
            if highest_idx < trailing_idx {
                highest_idx = trailing_idx;
                highest = data[trailing_idx];
                for (j, &val) in data[trailing_idx + 1..today + 1].iter().enumerate() {
                    if val >= highest {
                        highest = val;
                        highest_idx = trailing_idx + 1 + j;
                    }
                }
            } else if h >= highest {
                highest_idx = today;
                highest = h;
            }
            output[today] = (timeperiod - (today - highest_idx)) as f64 * inv_period;
            trailing_idx += 1;
            today += 1;
        }
        output
    }

    fn reference_aroon_min_pass(data: &[f64], timeperiod: usize, inv_period: f64) -> Vec<f64> {
        let len = data.len();
        let window = timeperiod + 1;
        let mut output = vec![0.0_f64; len];
        output[..timeperiod].fill(f64::NAN);

        let mut lowest = data[0];
        let mut lowest_idx: usize = 0;
        for j in 1..window {
            if data[j] <= lowest {
                lowest = data[j];
                lowest_idx = j;
            }
        }
        output[timeperiod] = (timeperiod - (timeperiod - lowest_idx)) as f64 * inv_period;

        let mut trailing_idx = 1;
        let mut today = timeperiod + 1;

        while today < len {
            let l = data[today];
            if lowest_idx < trailing_idx {
                lowest_idx = trailing_idx;
                lowest = data[trailing_idx];
                for (j, &val) in data[trailing_idx + 1..today + 1].iter().enumerate() {
                    if val <= lowest {
                        lowest = val;
                        lowest_idx = trailing_idx + 1 + j;
                    }
                }
            } else if l <= lowest {
                lowest_idx = today;
                lowest = l;
            }
            output[today] = (timeperiod - (today - lowest_idx)) as f64 * inv_period;
            trailing_idx += 1;
            today += 1;
        }
        output
    }

    #[test]
    fn passes_match_reference_bitwise() {
        for (period, len) in periods_and_lengths() {
            for data in datasets(len) {
                if data.len() <= period {
                    assert!(aroon(&data, &data, period).is_err());
                    continue;
                }
                let inv_period = 100.0 / period as f64;
                let expected_up = reference_aroon_max_pass(&data, period, inv_period);
                let expected_down = reference_aroon_min_pass(&data, period, inv_period);
                let actual_up = aroon_max_pass(&data, period, inv_period);
                let actual_down = aroon_min_pass(&data, period, inv_period);
                for (e, a) in expected_up.iter().zip(&actual_up) {
                    assert_eq!(e.to_bits(), a.to_bits(), "up p={period} len={len}");
                }
                for (e, a) in expected_down.iter().zip(&actual_down) {
                    assert_eq!(e.to_bits(), a.to_bits(), "down p={period} len={len}");
                }
            }
        }
    }
}
use super::aroon_true_range::*;
use super::*;

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

    /// Bulk kernel: one fused [`aroon_rescan`] pass writing both aligned
    /// series straight into their output caches — no index scratch buffers, no
    /// second combining pass. Outputs and post-run state are bit-identical to
    /// per-bar [`Self::append`]; warm-up bars are NaN.
    pub fn extend_slices_into(
        &mut self,
        high: &[f64],
        low: &[f64],
        down_out: &mut Vec<f64>,
        up_out: &mut Vec<f64>,
    ) -> TaResult<()> {
        let n = self.check_bulk_lengths(high, low)?;
        if self.index != 0 || n < self.period + 1 {
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
        let downs = &mut down_out[down_start..];
        let ups = &mut up_out[up_start..];
        aroon_rescan(
            high,
            low,
            self.period,
            self.inverse_period,
            |today, down, up| {
                downs[today] = down;
                ups[today] = up;
            },
        );
        self.finish_bulk_run(high, low);
        self.value = Some(AroonValue {
            down: *down_out.last().expect("at least one warmed bar"),
            up: *up_out.last().expect("at least one warmed bar"),
        });
        Ok(())
    }

    /// Bulk kernel for [`AroonOscillator`]: the same single rescan pass, with
    /// `up - down` formed in registers so the oscillator never materializes
    /// the two component series.
    pub(crate) fn extend_oscillator_into(
        &mut self,
        high: &[f64],
        low: &[f64],
        output: &mut Vec<f64>,
    ) -> TaResult<()> {
        let n = self.check_bulk_lengths(high, low)?;
        if self.index != 0 || n < self.period + 1 {
            output.reserve(n);
            for index in 0..n {
                output.push(
                    self.append(high[index], low[index])
                        .map_or(f64::NAN, |value| value.up - value.down),
                );
            }
            return Ok(());
        }

        let start = output.len();
        output.resize(start + n, f64::NAN);
        let slots = &mut output[start..];
        let mut last = AroonValue { down: 0.0, up: 0.0 };
        aroon_rescan(
            high,
            low,
            self.period,
            self.inverse_period,
            |today, down, up| {
                slots[today] = up - down;
                last = AroonValue { down, up };
            },
        );
        self.finish_bulk_run(high, low);
        self.value = Some(last);
        Ok(())
    }

    fn check_bulk_lengths(&self, high: &[f64], low: &[f64]) -> TaResult<usize> {
        if high.len() != low.len() {
            return Err(TaError::LengthMismatch {
                expected: high.len(),
                got: low.len(),
            });
        }
        Ok(high.len())
    }

    /// Restores the monotonic deques and bar counter a full from-empty
    /// `append` run would have left.
    fn finish_bulk_run(&mut self, high: &[f64], low: &[f64]) {
        self.highs.rebuild_from_full_run(high);
        self.lows.rebuild_from_full_run(low);
        self.index = high.len();
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
