//! Rolling variance, deviation, correlation, and beta states.

use std::collections::VecDeque;

use crate::error::TaResult;

use super::{invalid_period, StreamingIndicator, Window};

/// TA-Lib's `TA_STDDEV` collapses a variance below this threshold to zero
/// instead of taking its square root; replicated verbatim.
pub(super) const STDDEV_VARIANCE_EPSILON: f64 = 0.00000000000001;

/// TA-Lib's `TA_STDDEV` post-processing of a `TA_INT_VAR` output.
///
/// Bit-identical to the C reference: the `nbdev == 1.0` branch there skips the
/// multiply, and multiplying by exactly `1.0` is the identity in IEEE-754, so a
/// single form covers both.
#[inline]
pub(super) fn stddev_from_variance(variance: f64, nbdev: f64) -> f64 {
    if !(variance < STDDEV_VARIANCE_EPSILON) {
        variance.sqrt() * nbdev
    } else {
        0.0
    }
}

/// Sliding population moments in **TA-Lib's exact accumulation order**.
///
/// `TA_INT_VAR` adds the incoming bar to `periodTotal1`/`periodTotal2`, emits
/// `mean2 - mean1²` (both means obtained by *division* by the period), and only
/// then subtracts the trailing bar. Reproducing that order — rather than the
/// algebraically equivalent fused `sum += new - old` / `(new-old)(new+old)`
/// recurrence with a precomputed reciprocal — makes VAR and STDDEV **bitwise**
/// equal to TA-Lib over a 100k-bar AR(1) price series at every period measured
/// (5/14/30), instead of merely close to it.
///
/// This matters because TA-Lib's own sliding sums drift away from a fresh
/// per-window recomputation (measured: 1.8e-9 for VAR, 3.7e-9 for STDDEV at
/// p=5 over 100k bars — enough to break the `rtol=1e-8, atol=1e-10` oracle
/// gate on low-variance windows). A periodic reseed of our sums would push
/// *our* result towards the true value and therefore **away** from the oracle,
/// making the mismatch worse; matching the oracle's arithmetic removes it
/// entirely. Same reasoning as the verbatim `TA_CORREL` formula below.
///
/// Because the trailing value is subtracted after the emit, the resting
/// invariant is "`sum`/`sum_squares` cover the `period - 1` most recent
/// inputs", so the retained ring only needs `period - 1` slots and its
/// eviction *is* the trailing value TA-Lib reads through `trailingIdx`.
///
/// Note: no `mul_add` here — TA-Lib uses a plain multiply followed by a
/// separate add, and fusing them changes the low bits.
#[derive(Debug, Clone)]
struct RollingMoments {
    period: usize,
    period_f: f64,
    /// The `period - 1` most recent inputs; a push evicts TA-Lib's trailing bar.
    window: Window,
    sum: f64,
    sum_squares: f64,
}

impl RollingMoments {
    fn new(period: usize) -> TaResult<Self> {
        if period < 2 {
            return Err(invalid_period("timeperiod", period, 2));
        }
        Ok(Self {
            period,
            period_f: period as f64,
            window: Window::new(period - 1)?,
            sum: 0.0,
            sum_squares: 0.0,
        })
    }

    fn append(&mut self, input: f64) -> Option<f64> {
        self.sum += input;
        self.sum_squares += input * input;
        let variance = self.window.is_full().then(|| {
            let mean1 = self.sum / self.period_f;
            let mean2 = self.sum_squares / self.period_f;
            mean2 - mean1 * mean1
        });
        if let Some(old) = self.window.push(input) {
            self.sum -= old;
            self.sum_squares -= old * old;
        }
        variance
    }

    fn reset(&mut self) {
        self.window.clear();
        self.sum = 0.0;
        self.sum_squares = 0.0;
    }

    /// Bulk kernel: the same add / emit / subtract-trailing recurrence indexing
    /// the input slice directly, pushing `map(variance)` (NaN during warm-up).
    ///
    /// Uses exactly the same arithmetic in exactly the same order as
    /// [`Self::append`], so outputs and post-run state are bit-identical to
    /// per-bar appends for any chunking. Returns the last emitted value.
    fn extend_map_into(
        &mut self,
        inputs: &[f64],
        output: &mut Vec<f64>,
        mut map: impl FnMut(f64) -> f64,
    ) -> Option<f64> {
        // Number of retained (not-yet-subtracted) inputs = ring capacity.
        let trailing = self.period - 1;
        let n = inputs.len();
        output.reserve(n);
        // Warm-up prologue: after `period - 1` appends the ring holds exactly
        // `inputs[..period - 1]`, regardless of prior state.
        let prologue = n.min(trailing);
        let mut last = None;
        for &input in &inputs[..prologue] {
            last = self.append(input).map(&mut map);
            output.push(last.unwrap_or(f64::NAN));
        }
        if n <= trailing {
            return last;
        }
        // Steady loop: identical arithmetic to `append`, with the trailing
        // element read from the input slice instead of the ring.
        let period_f = self.period_f;
        let mut sum = self.sum;
        let mut sum_squares = self.sum_squares;
        for i in trailing..n {
            let input = inputs[i];
            sum += input;
            sum_squares += input * input;
            let mean1 = sum / period_f;
            let mean2 = sum_squares / period_f;
            let mapped = map(mean2 - mean1 * mean1);
            output.push(mapped);
            last = Some(mapped);
            let old = inputs[i - trailing];
            sum -= old;
            sum_squares -= old * old;
        }
        self.sum = sum;
        self.sum_squares = sum_squares;
        // Rebuild the ring so subsequent appends continue bit-identically.
        self.window.clear();
        for &input in &inputs[n - trailing..] {
            self.window.push(input);
        }
        last
    }
}

/// Stateful population variance. TA-Lib accepts but ignores `nbdev` for VAR.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `RollingVariance`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct RollingVariance {
    moments: RollingMoments,
    value: Option<f64>,
}

impl RollingVariance {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(period: usize, _nbdev: f64) -> TaResult<Self> {
        Ok(Self {
            moments: RollingMoments::new(period)?,
            value: None,
        })
    }
}

impl StreamingIndicator for RollingVariance {
    type Output = f64;

    /// Bulk kernel: slice-recurrence sliding moments, bit-identical to
    /// per-bar [`Self::append`] in outputs and post-run state.
    fn extend_slice_into(&mut self, inputs: &[f64], output: &mut Vec<f64>) {
        if inputs.is_empty() {
            return;
        }
        self.value = self
            .moments
            .extend_map_into(inputs, output, |variance| variance);
    }

    fn append(&mut self, input: f64) -> Option<f64> {
        self.value = self.moments.append(input);
        self.value
    }

    fn value(&self) -> Option<f64> {
        self.value
    }

    fn reset(&mut self) {
        self.moments.reset();
        self.value = None;
    }
}

/// Stateful population standard deviation multiplied by `nbdev`.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `RollingStandardDeviation`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct RollingStandardDeviation {
    moments: RollingMoments,
    nbdev: f64,
    value: Option<f64>,
}

impl RollingStandardDeviation {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(period: usize, nbdev: f64) -> TaResult<Self> {
        Ok(Self {
            moments: RollingMoments::new(period)?,
            nbdev,
            value: None,
        })
    }
}

impl StreamingIndicator for RollingStandardDeviation {
    type Output = f64;

    /// Bulk kernel: slice-recurrence sliding moments, bit-identical to
    /// per-bar [`Self::append`] in outputs and post-run state.
    fn extend_slice_into(&mut self, inputs: &[f64], output: &mut Vec<f64>) {
        if inputs.is_empty() {
            return;
        }
        let nbdev = self.nbdev;
        self.value = self.moments.extend_map_into(inputs, output, |variance| {
            stddev_from_variance(variance, nbdev)
        });
    }

    fn append(&mut self, input: f64) -> Option<f64> {
        let nbdev = self.nbdev;
        self.value = self
            .moments
            .append(input)
            .map(|variance| stddev_from_variance(variance, nbdev));
        self.value
    }

    fn value(&self) -> Option<f64> {
        self.value
    }

    fn reset(&mut self) {
        self.moments.reset();
        self.value = None;
    }
}

/// Stateful average absolute deviation with TA-Lib's newest-to-oldest summation order.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `RollingAverageDeviation`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct RollingAverageDeviation {
    period: usize,
    window: Window,
    value: Option<f64>,
}

impl RollingAverageDeviation {
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
            window: Window::new(period)?,
            value: None,
        })
    }
}

impl StreamingIndicator for RollingAverageDeviation {
    type Output = f64;

    /// Bulk kernel: the O(period) mean + deviation rescans are inherent to
    /// TA-Lib's AVGDEV semantics (newest-to-oldest summation order), but here
    /// they run over the contiguous input slice instead of the ring iterator.
    /// Bit-identical to per-bar [`Self::append`] in outputs and state.
    ///
    /// Note: maintaining an incremental running sum for the mean would change
    /// the summation order (and therefore low bits) versus the per-window
    /// newest-to-oldest rescan, so the rescan is kept.
    fn extend_slice_into(&mut self, inputs: &[f64], output: &mut Vec<f64>) {
        let period = self.period;
        let n = inputs.len();
        output.reserve(n);
        // Warm-up prologue: from index period-1 onward the ring contents are
        // exactly the trailing input-slice window, regardless of prior state.
        let prologue = n.min(period - 1);
        for &input in &inputs[..prologue] {
            output.push(self.append(input).unwrap_or(f64::NAN));
        }
        if n < period {
            return;
        }
        let period_f = period as f64;
        let mut last = f64::NAN;
        for i in (period - 1)..n {
            let window = &inputs[i + 1 - period..=i];
            // Newest-to-oldest, exactly like `window.iter().rev()` in append.
            let mut sum = 0.0;
            for &value in window.iter().rev() {
                sum += value;
            }
            let mean = sum / period_f;
            let mut deviation = 0.0;
            for &value in window.iter().rev() {
                deviation += (value - mean).abs();
            }
            last = deviation / period_f;
            output.push(last);
        }
        self.value = Some(last);
        // Rebuild the ring so subsequent appends continue bit-identically.
        self.window.clear();
        for &input in &inputs[n - period..] {
            self.window.push(input);
        }
    }

    fn append(&mut self, input: f64) -> Option<f64> {
        self.window.push(input);
        self.value = self.window.is_full().then(|| {
            let period = self.period as f64;
            let mean = self.window.iter().rev().sum::<f64>() / period;
            self.window
                .iter()
                .rev()
                .map(|value| (*value - mean).abs())
                .sum::<f64>()
                / period
        });
        self.value
    }

    fn value(&self) -> Option<f64> {
        self.value
    }

    fn reset(&mut self) {
        self.window.clear();
        self.value = None;
    }
}

#[derive(Debug, Clone, Copy)]
struct PairMoments {
    sx: f64,
    sy: f64,
    sxx: f64,
    syy: f64,
    sxy: f64,
}

/// Reseed cadence for sliding pair moments, in absolute appends.
///
/// Every `PAIR_MOMENTS_RESEED_INTERVAL`-th append (counted from
/// construction/reset, so the reseed bars are the same regardless of how the
/// input is chunked) the five sums are recomputed from the retained window
/// in serial oldest-to-newest order. This bounds subtractive-cancellation
/// drift to at most 63 slide steps instead of letting it grow with the
/// series length: measured over 200k LCG bars the worst deviation from a
/// fresh per-window recomputation falls from 1.6e-11 to 6.7e-13.
/// Amortized cost is `period / 64` extra element accumulations per bar.
/// The batch kernels (`rolling_corr`, `rolling_beta`) apply the identical
/// cadence so streaming stays bitwise equal to batch.
/// TA-Lib rejects a correlation window when the variance product falls below
/// this threshold (`TA_CORREL`); replicated verbatim.
pub(super) const CORREL_DENOMINATOR_EPSILON: f64 = 0.00000000000001;

pub(super) const PAIR_MOMENTS_RESEED_INTERVAL: u64 = 64;

#[derive(Debug, Clone)]
struct RollingPairMoments {
    period: usize,
    window: VecDeque<(f64, f64)>,
    moments: PairMoments,
    /// Total appends since construction/reset.
    count: u64,
}

impl RollingPairMoments {
    fn new(period: usize) -> TaResult<Self> {
        if period < 2 {
            return Err(invalid_period("timeperiod", period, 2));
        }
        Ok(Self {
            period,
            window: VecDeque::with_capacity(period),
            moments: PairMoments {
                sx: 0.0,
                sy: 0.0,
                sxx: 0.0,
                syy: 0.0,
                sxy: 0.0,
            },
            count: 0,
        })
    }

    fn append(&mut self, x: f64, y: f64) -> Option<PairMoments> {
        if self.window.len() == self.period {
            let (old_x, old_y) = self.window.pop_front().expect("pair window is full");
            self.moments.sx += x - old_x;
            self.moments.sy += y - old_y;
            self.moments.sxx += x * x - old_x * old_x;
            self.moments.syy += y * y - old_y * old_y;
            self.moments.sxy += x * y - old_x * old_y;
        } else {
            self.moments.sx += x;
            self.moments.sy += y;
            self.moments.sxx += x * x;
            self.moments.syy += y * y;
            self.moments.sxy += x * y;
        }
        self.window.push_back((x, y));
        self.count += 1;
        if self.window.len() == self.period && self.count % PAIR_MOMENTS_RESEED_INTERVAL == 0 {
            self.reseed_serial();
        }
        (self.window.len() == self.period).then_some(self.moments)
    }

    /// Recomputes all five sums from the window, oldest to newest, with the
    /// same per-element accumulation the warm-up path uses.
    fn reseed_serial(&mut self) {
        let mut moments = PairMoments {
            sx: 0.0,
            sy: 0.0,
            sxx: 0.0,
            syy: 0.0,
            sxy: 0.0,
        };
        for &(x, y) in &self.window {
            moments.sx += x;
            moments.sy += y;
            moments.sxx += x * x;
            moments.syy += y * y;
            moments.sxy += x * y;
        }
        self.moments = moments;
    }

    fn reset(&mut self) {
        self.window.clear();
        self.moments = PairMoments {
            sx: 0.0,
            sy: 0.0,
            sxx: 0.0,
            syy: 0.0,
            sxy: 0.0,
        };
        self.count = 0;
    }

    fn reseed_linear_sums_with_batch_order(&mut self) -> PairMoments {
        let x: Vec<f64> = self.window.iter().map(|value| value.0).collect();
        let y: Vec<f64> = self.window.iter().map(|value| value.1).collect();
        self.moments.sx = crate::simd::sum_f64(&x);
        self.moments.sy = crate::simd::sum_f64(&y);
        self.moments
    }
}

/// Stateful Pearson correlation over paired observations.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `RollingCorrelation`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct RollingCorrelation {
    period: f64,
    moments: RollingPairMoments,
    seeded: bool,
    value: Option<f64>,
}

impl RollingCorrelation {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(period: usize) -> TaResult<Self> {
        Ok(Self {
            period: period as f64,
            moments: RollingPairMoments::new(period)?,
            seeded: false,
            value: None,
        })
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, x: f64, y: f64) -> Option<f64> {
        let moments = self.moments.append(x, y).map(|moments| {
            if self.seeded {
                moments
            } else {
                self.seeded = true;
                self.moments.reseed_linear_sums_with_batch_order()
            }
        });
        self.value = moments.map(|m| {
            // TA-Lib's TA_CORREL divides by the period inside each term rather
            // than scaling the numerator and denominator by it. The forms are
            // algebraically equal but not numerically; matching C exactly keeps
            // near-zero correlations inside the oracle's tolerance.
            let numerator = m.sxy - ((m.sx * m.sy) / self.period);
            let denominator =
                (m.sxx - ((m.sx * m.sx) / self.period)) * (m.syy - ((m.sy * m.sy) / self.period));
            if !(denominator < CORREL_DENOMINATOR_EPSILON) {
                numerator / denominator.sqrt()
            } else {
                0.0
            }
        });
        self.value
    }

    /// Bulk kernel: O(1) add/evict sliding pair-moments indexing the input
    /// slices directly. Bit-identical to per-bar [`Self::append`] in outputs
    /// and post-run state (including the one-time batch-order reseed of the
    /// linear sums, which happens inside the prologue appends).
    pub fn extend_slices_into(
        &mut self,
        input0: &[f64],
        input1: &[f64],
        output: &mut Vec<f64>,
    ) -> TaResult<()> {
        if input0.len() != input1.len() {
            return Err(crate::TaError::LengthMismatch {
                expected: input0.len(),
                got: input1.len(),
            });
        }
        let period = self.moments.period;
        let n = input0.len();
        output.reserve(n);
        // Warm-up prologue: after `period` appends the pair window holds
        // exactly the first `period` slice pairs, regardless of prior state,
        // and the reseed (if due) has been applied by `append`.
        let prologue = n.min(period);
        for i in 0..prologue {
            output.push(self.append(input0[i], input1[i]).unwrap_or(f64::NAN));
        }
        if n <= period {
            return Ok(());
        }
        // Steady loop: identical arithmetic to the full-window branch of
        // `RollingPairMoments::append`, evicted pair read from the slices.
        let period_f = self.period;
        let mut m = self.moments.moments;
        let mut count = self.moments.count;
        let mut last = f64::NAN;
        for i in period..n {
            let x = input0[i];
            let y = input1[i];
            let old_x = input0[i - period];
            let old_y = input1[i - period];
            m.sx += x - old_x;
            m.sy += y - old_y;
            m.sxx += x * x - old_x * old_x;
            m.syy += y * y - old_y * old_y;
            m.sxy += x * y - old_x * old_y;
            count += 1;
            if count % PAIR_MOMENTS_RESEED_INTERVAL == 0 {
                // Same absolute-append cadence and serial oldest-to-newest
                // order as `RollingPairMoments::reseed_serial`.
                m = PairMoments {
                    sx: 0.0,
                    sy: 0.0,
                    sxx: 0.0,
                    syy: 0.0,
                    sxy: 0.0,
                };
                for j in i + 1 - period..=i {
                    let x = input0[j];
                    let y = input1[j];
                    m.sx += x;
                    m.sy += y;
                    m.sxx += x * x;
                    m.syy += y * y;
                    m.sxy += x * y;
                }
            }
            let numerator = m.sxy - ((m.sx * m.sy) / period_f);
            let denominator =
                (m.sxx - ((m.sx * m.sx) / period_f)) * (m.syy - ((m.sy * m.sy) / period_f));
            last = if !(denominator < CORREL_DENOMINATOR_EPSILON) {
                numerator / denominator.sqrt()
            } else {
                0.0
            };
            output.push(last);
        }
        self.moments.moments = m;
        self.moments.count = count;
        self.value = Some(last);
        // Rebuild the pair window so subsequent appends continue bit-identically.
        self.moments.window.clear();
        for i in n - period..n {
            self.moments.window.push_back((input0[i], input1[i]));
        }
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
        self.moments.reset();
        self.seeded = false;
        self.value = None;
    }
}

/// Stateful TA-Lib BETA over percentage returns of two input series.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `RollingBeta`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct RollingBeta {
    period: f64,
    previous: Option<(f64, f64)>,
    returns: RollingPairMoments,
    value: Option<f64>,
}

impl RollingBeta {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(period: usize) -> TaResult<Self> {
        Ok(Self {
            period: period as f64,
            previous: None,
            returns: RollingPairMoments::new(period)?,
            value: None,
        })
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, input0: f64, input1: f64) -> Option<f64> {
        let Some((previous0, previous1)) = self.previous.replace((input0, input1)) else {
            return None;
        };
        let x = (input0 - previous0) / previous0;
        let y = (input1 - previous1) / previous1;
        self.value = self.returns.append(x, y).map(|m| {
            let denominator = self.period * m.sxx - m.sx * m.sx;
            if denominator > 0.0 {
                (self.period * m.sxy - m.sx * m.sy) / denominator
            } else {
                0.0
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
        self.previous = None;
        self.returns.reset();
        self.value = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    fn assert_same_bits(actual: &[f64], expected: &[f64], label: &str) {
        assert_eq!(actual.len(), expected.len(), "{label}: length");
        for (i, (a, b)) in actual.iter().zip(expected).enumerate() {
            assert_eq!(a.to_bits(), b.to_bits(), "{label}: bar {i}");
        }
    }

    fn check_single_input<S, N>(new_state: N, label: &str)
    where
        S: StreamingIndicator<Output = f64>,
        N: Fn(usize) -> S,
    {
        let input = lcg_series(5_000, 0x5EED_0057);
        let tail = lcg_series(256, 0x7A11_0057);
        for period in [2usize, 5, 14, 30, 200] {
            let mut per_bar = new_state(period);
            let reference: Vec<f64> = input
                .iter()
                .map(|&x| per_bar.append(x).unwrap_or(f64::NAN))
                .collect();
            let tail_reference: Vec<f64> = tail
                .iter()
                .map(|&x| per_bar.append(x).unwrap_or(f64::NAN))
                .collect();

            // 5,000 bars with chunk sizes that are mutually coprime with the
            // periods, so the bulk prologue/steady split lands at many
            // different offsets inside the window.
            for chunk in [usize::MAX, 1, 7, 10, 97, 1000] {
                let mut state = new_state(period);
                let mut out = Vec::new();
                for piece in input.chunks(chunk.min(input.len())) {
                    state.extend_slice_into(piece, &mut out);
                }
                assert_same_bits(
                    &out,
                    &reference,
                    &format!("{label} p{period} chunk {chunk}"),
                );
                let tail_out: Vec<f64> = tail
                    .iter()
                    .map(|&x| state.append(x).unwrap_or(f64::NAN))
                    .collect();
                assert_same_bits(
                    &tail_out,
                    &tail_reference,
                    &format!("{label} p{period} chunk {chunk} tail"),
                );
            }
        }
    }

    #[test]
    fn var_bulk_is_bitwise_identical_to_per_bar_append() {
        check_single_input(|p| RollingVariance::new(p, 1.0).unwrap(), "VAR");
    }

    #[test]
    fn stddev_bulk_is_bitwise_identical_to_per_bar_append() {
        check_single_input(|p| RollingStandardDeviation::new(p, 1.7).unwrap(), "STDDEV");
    }

    #[test]
    fn avgdev_bulk_is_bitwise_identical_to_per_bar_append() {
        check_single_input(|p| RollingAverageDeviation::new(p).unwrap(), "AVGDEV");
    }

    #[test]
    fn correl_bulk_is_bitwise_identical_to_per_bar_append() {
        let x = lcg_series(5_000, 0x5EED_C0DE);
        let y = lcg_series(5_000, 0xC0FF_EE01);
        let tail_x = lcg_series(256, 0x7A11_C0DE);
        let tail_y = lcg_series(256, 0x7A11_EE01);
        for period in [2usize, 5, 14, 30, 200] {
            let mut per_bar = RollingCorrelation::new(period).unwrap();
            let reference: Vec<f64> = x
                .iter()
                .zip(&y)
                .map(|(&a, &b)| per_bar.append(a, b).unwrap_or(f64::NAN))
                .collect();
            let tail_reference: Vec<f64> = tail_x
                .iter()
                .zip(&tail_y)
                .map(|(&a, &b)| per_bar.append(a, b).unwrap_or(f64::NAN))
                .collect();

            // 5,000 bars cross the 256-append reseed cadence ~19 times, so
            // every chunking (and the tail) crosses reseed boundaries.
            for chunk in [usize::MAX, 1, 7, 10, 97, 1000] {
                let mut state = RollingCorrelation::new(period).unwrap();
                let mut out = Vec::new();
                let mut start = 0;
                while start < x.len() {
                    let end = (start + chunk.min(x.len())).min(x.len());
                    state
                        .extend_slices_into(&x[start..end], &y[start..end], &mut out)
                        .unwrap();
                    start = end;
                }
                assert_same_bits(&out, &reference, &format!("CORREL p{period} chunk {chunk}"));
                let tail_out: Vec<f64> = tail_x
                    .iter()
                    .zip(&tail_y)
                    .map(|(&a, &b)| state.append(a, b).unwrap_or(f64::NAN))
                    .collect();
                assert_same_bits(
                    &tail_out,
                    &tail_reference,
                    &format!("CORREL p{period} chunk {chunk} tail"),
                );
            }
        }
    }

    #[test]
    fn correl_streaming_matches_batch_bitwise_across_reseeds() {
        let x = lcg_series(5_000, 0x5EED_CBAF);
        let y = lcg_series(5_000, 0xC0FF_CBAF);
        for period in [2usize, 14, 30, 200, 256] {
            let batch = crate::stream::rolling_corr(&x, &y, period).unwrap();
            let mut state = RollingCorrelation::new(period).unwrap();
            let streaming: Vec<f64> = x
                .iter()
                .zip(&y)
                .map(|(&a, &b)| state.append(a, b).unwrap_or(f64::NAN))
                .collect();
            assert_same_bits(
                &streaming,
                &batch,
                &format!("CORREL batch parity p{period}"),
            );
        }
    }

    #[test]
    fn beta_streaming_matches_batch_bitwise_across_reseeds() {
        let x = lcg_series(5_000, 0x5EED_BE7A);
        let y = lcg_series(5_000, 0xC0FF_BE7A);
        for period in [2usize, 14, 30, 200, 256] {
            let batch = crate::stream::rolling_beta(&x, &y, period).unwrap();
            let mut state = RollingBeta::new(period).unwrap();
            let streaming: Vec<f64> = x
                .iter()
                .zip(&y)
                .map(|(&a, &b)| state.append(a, b).unwrap_or(f64::NAN))
                .collect();
            assert_same_bits(&streaming, &batch, &format!("BETA batch parity p{period}"));
        }
    }

    /// Literal transcription of TA-Lib's `TA_INT_VAR` accumulation loop.
    ///
    /// This is the contract `RollingMoments` must reproduce **bit for bit**:
    /// add the incoming bar, emit `mean2 - mean1²` with both means obtained by
    /// division, then subtract the trailing bar.
    fn talib_var_reference(input: &[f64], period: usize) -> Vec<f64> {
        let mut output = vec![f64::NAN; input.len()];
        let period_f = period as f64;
        let (mut total1, mut total2) = (0.0, 0.0);
        for &value in &input[..period - 1] {
            total1 += value;
            total2 += value * value;
        }
        let mut trailing = 0;
        for i in (period - 1)..input.len() {
            let value = input[i];
            total1 += value;
            total2 += value * value;
            let mean1 = total1 / period_f;
            let mean2 = total2 / period_f;
            output[i] = mean2 - mean1 * mean1;
            let old = input[trailing];
            trailing += 1;
            total1 -= old;
            total2 -= old * old;
        }
        output
    }

    /// Fresh two-pass population variance over `input[end + 1 - period..=end]`,
    /// accumulating nothing across bars.
    fn exact_variance(input: &[f64], end: usize, period: usize) -> f64 {
        let window = &input[end + 1 - period..=end];
        let period_f = period as f64;
        let mut sum = 0.0;
        for &value in window {
            sum += value;
        }
        let mean = sum / period_f;
        let mut squares = 0.0;
        for &value in window {
            squares += (value - mean) * (value - mean);
        }
        squares / period_f
    }

    #[test]
    fn var_and_stddev_reproduce_the_talib_recurrence_bitwise() {
        let input = lcg_series(5_000, 0x5EED_1234);
        for period in [2usize, 5, 14, 30, 200] {
            let reference = talib_var_reference(&input, period);
            let mut var = RollingVariance::new(period, 1.0).unwrap();
            let streamed: Vec<f64> = input
                .iter()
                .map(|&x| var.append(x).unwrap_or(f64::NAN))
                .collect();
            assert_same_bits(&streamed, &reference, &format!("VAR TA-order p{period}"));

            let mut bulk_state = RollingVariance::new(period, 1.0).unwrap();
            let mut bulk = Vec::new();
            for piece in input.chunks(997) {
                bulk_state.extend_slice_into(piece, &mut bulk);
            }
            assert_same_bits(&bulk, &reference, &format!("VAR TA-order bulk p{period}"));

            let expected_std: Vec<f64> = reference
                .iter()
                .map(|&v| {
                    if v.is_nan() {
                        f64::NAN
                    } else {
                        stddev_from_variance(v, 1.0)
                    }
                })
                .collect();
            let mut std = RollingStandardDeviation::new(period, 1.0).unwrap();
            let streamed: Vec<f64> = input
                .iter()
                .map(|&x| std.append(x).unwrap_or(f64::NAN))
                .collect();
            assert_same_bits(
                &streamed,
                &expected_std,
                &format!("STDDEV TA-order p{period}"),
            );

            // The batch kernels must agree with the states bit for bit too.
            let batch_var = crate::stream::rolling_var(&input, period, 1.0).unwrap();
            assert_same_bits(&batch_var, &reference, &format!("VAR batch p{period}"));
            let batch_std = crate::stream::rolling_std(&input, period, 1.0).unwrap();
            assert_same_bits(
                &batch_std,
                &expected_std,
                &format!("STDDEV batch p{period}"),
            );
        }
    }

    /// VAR/STDDEV deliberately reproduce TA-Lib's sliding accumulator rather
    /// than a periodically reseeded (more accurate) one, because the oracle
    /// gate compares against TA-Lib, not against the true value: reseeding
    /// moves us towards truth and therefore *away* from the oracle. See
    /// `RollingMoments`' docs.
    ///
    /// So the invariant worth testing at scale is that the state never diverges
    /// from that recurrence — checked bitwise at every bar — while the residual
    /// against a fresh per-window recomputation stays bounded (it is TA-Lib's
    /// own drift, ~1e-9 on price-scale data; the loose bound here only has to
    /// catch a genuinely broken accumulator, which lands orders of magnitude
    /// higher).
    #[test]
    fn var_and_stddev_track_the_talib_recurrence_over_1m_bars() {
        let input = lcg_series(1_000_000, 0x5EED_1E6A);
        for period in [14usize, 30] {
            let reference = talib_var_reference(&input, period);
            let mut var = RollingVariance::new(period, 1.0).unwrap();
            for i in 0..input.len() {
                let value = var.append(input[i]).unwrap_or(f64::NAN);
                assert_eq!(
                    value.to_bits(),
                    reference[i].to_bits(),
                    "VAR p{period} bar {i}: diverged from the TA-Lib recurrence"
                );
                if (i + 1) % 50_000 == 0 {
                    let exact = exact_variance(&input, i, period);
                    let drift = (value - exact).abs();
                    assert!(
                        drift < 1e-9,
                        "VAR p{period} bar {i}: drift {drift:e} vs a fresh window"
                    );
                }
            }
        }
    }

    /// Exact per-window correlation, recomputed fresh in serial order.
    fn exact_correl(x: &[f64], y: &[f64], end: usize, period: usize) -> f64 {
        let (mut sx, mut sy, mut sxx, mut syy, mut sxy) = (0.0, 0.0, 0.0, 0.0, 0.0);
        for j in end + 1 - period..=end {
            let (a, b) = (x[j], y[j]);
            sx += a;
            sy += b;
            sxx += a * a;
            syy += b * b;
            sxy += a * b;
        }
        let n = period as f64;
        let num = sxy - ((sx * sy) / n);
        let denom = (sxx - ((sx * sx) / n)) * (syy - ((sy * sy) / n));
        if !(denom < CORREL_DENOMINATOR_EPSILON) {
            num / denom.sqrt()
        } else {
            0.0
        }
    }

    #[test]
    fn correl_streaming_drift_stays_bounded_over_200k_bars() {
        let x = lcg_series(200_000, 0x5EED_D21F);
        let y = lcg_series(200_000, 0xC0FF_D21F);
        for period in [14usize, 30] {
            let mut state = RollingCorrelation::new(period).unwrap();
            for i in 0..x.len() {
                let Some(value) = state.append(x[i], y[i]) else {
                    continue;
                };
                let probe = (i + 1) % 5_000 == 0 || i + 1 == x.len();
                if probe {
                    let exact = exact_correl(&x, &y, i, period);
                    let drift = (value - exact).abs();
                    assert!(
                        drift < 1e-12,
                        "CORREL p{period} bar {i}: drift {drift:e} vs exact"
                    );
                }
            }
        }
    }
}
