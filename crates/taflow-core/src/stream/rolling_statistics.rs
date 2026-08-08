//! Rolling variance, deviation, correlation, and beta states.

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

/// TA-Lib rejects a correlation window when the variance product falls below
/// this threshold (`TA_CORREL`); replicated verbatim.
pub(super) const CORREL_DENOMINATOR_EPSILON: f64 = 0.00000000000001;

/// TA-Lib's `TA_IS_ZERO` macro, used verbatim by `TA_BETA` for both the
/// previous-price divisor and the regression denominator.
#[inline]
pub(super) fn ta_is_zero(value: f64) -> bool {
    (-0.00000001 < value) && (value < 0.00000001)
}

/// TA-Lib's `TA_BETA` percentage return, including its zero-price guard.
#[inline]
pub(super) fn beta_return(current: f64, previous: f64) -> f64 {
    if !ta_is_zero(previous) {
        (current - previous) / previous
    } else {
        0.0
    }
}

/// Fixed ring of `(x, y)` pairs stored **interleaved** in one allocation.
///
/// Both halves of a bar are always read and written together, so one
/// interleaved buffer costs one cache line and one index computation per bar
/// where two parallel [`Window`]s cost two of each.
#[derive(Debug, Clone)]
struct PairRing {
    /// `2 * capacity` slots, laid out `x0, y0, x1, y1, …`.
    buf: Box<[f64]>,
    /// Slot index (always even) of the oldest pair.
    head: usize,
    /// Number of pairs currently held.
    len: usize,
    capacity: usize,
}

impl PairRing {
    fn new(capacity: usize) -> TaResult<Self> {
        if capacity == 0 {
            return Err(invalid_period("capacity", capacity, 1));
        }
        Ok(Self {
            buf: vec![0.0; capacity * 2].into_boxed_slice(),
            head: 0,
            len: 0,
            capacity,
        })
    }

    /// Appends `(x, y)`, returning the pair evicted from a full ring.
    #[inline]
    fn push(&mut self, x: f64, y: f64) -> Option<(f64, f64)> {
        let slots = self.buf.len();
        if self.len == self.capacity {
            let head = self.head;
            let evicted = (self.buf[head], self.buf[head + 1]);
            self.buf[head] = x;
            self.buf[head + 1] = y;
            let next = head + 2;
            self.head = if next == slots { 0 } else { next };
            Some(evicted)
        } else {
            let mut tail = self.head + self.len * 2;
            if tail >= slots {
                tail -= slots;
            }
            self.buf[tail] = x;
            self.buf[tail + 1] = y;
            self.len += 1;
            None
        }
    }

    #[inline]
    fn is_full(&self) -> bool {
        self.len == self.capacity
    }

    #[inline]
    fn clear(&mut self) {
        self.head = 0;
        self.len = 0;
    }
}

/// The five sliding sums `TA_CORREL` maintains.
#[derive(Debug, Clone, Copy)]
struct PairMoments {
    sx: f64,
    sy: f64,
    sxx: f64,
    syy: f64,
    sxy: f64,
}

impl PairMoments {
    const ZERO: Self = Self {
        sx: 0.0,
        sy: 0.0,
        sxx: 0.0,
        syy: 0.0,
        sxy: 0.0,
    };
}

/// TA-Lib's `TA_CORREL` emit expression.
///
/// The period divides *inside* each term rather than scaling numerator and
/// denominator; the forms are algebraically equal but not numerically, and
/// matching C exactly is what keeps near-zero correlations bit-identical.
#[inline]
fn correl_of(sx: f64, sy: f64, sxx: f64, syy: f64, sxy: f64, period: f64) -> f64 {
    let numerator = sxy - ((sx * sy) / period);
    let denominator = (sxx - ((sx * sx) / period)) * (syy - ((sy * sy) / period));
    if !(denominator < CORREL_DENOMINATOR_EPSILON) {
        numerator / denominator.sqrt()
    } else {
        0.0
    }
}

/// Sliding pair moments in **`TA_CORREL`'s exact accumulation order**.
///
/// The C steady-state loop removes the trailing bar's five contributions
/// *first*, then adds the incoming bar's, then emits — the mirror image of
/// `TA_INT_VAR` (add / emit / remove), which is why `RollingBeta` cannot share
/// this struct: `TA_BETA` follows the `TA_INT_VAR` ordering over its return
/// series and has its own [`RollingReturnMoments`].
///
/// Reproducing the order verbatim — instead of the algebraically equivalent
/// fused `sum += new - old` recurrence, and instead of periodically reseeding
/// the sums from the retained window — makes CORREL **bitwise** equal to
/// `talib.CORREL` on the harness's AR(1) series at every period and length
/// measured (p=5/14/30 at 100k and 1M bars) rather than 4.8e-10-close.
///
/// Reseeding was measured and removed: it pushes our sums towards the true
/// per-window value and therefore *away* from TA-Lib's own drifting
/// accumulator, which is what the oracle gate compares against. Same
/// reasoning as [`RollingMoments`], and dropping it also removes a
/// `period / 64` per-bar tax.
///
/// Because the trailing pair is removed before the incoming one is added, the
/// resting invariant is "the sums cover the `period` most recent pairs", so
/// the rings hold `period` entries and their eviction *is* TA-Lib's
/// `trailingIdx` read.
///
/// Note: no `mul_add` anywhere — TA-Lib multiplies then adds separately, and
/// contracting the pair changes the low bits.
#[derive(Debug, Clone)]
struct RollingPairMoments {
    period: usize,
    window: PairRing,
    moments: PairMoments,
}

impl RollingPairMoments {
    fn new(period: usize) -> TaResult<Self> {
        if period < 2 {
            return Err(invalid_period("timeperiod", period, 2));
        }
        Ok(Self {
            period,
            window: PairRing::new(period)?,
            moments: PairMoments::ZERO,
        })
    }

    #[inline]
    fn append(&mut self, x: f64, y: f64) -> Option<PairMoments> {
        let m = &mut self.moments;
        if let Some((trailing_x, trailing_y)) = self.window.push(x, y) {
            // "Remove trailing values", in TA_CORREL's statement order.
            m.sx -= trailing_x;
            m.sxx -= trailing_x * trailing_x;
            m.sxy -= trailing_x * trailing_y;
            m.sy -= trailing_y;
            m.syy -= trailing_y * trailing_y;
        }
        // "Add new values", likewise.
        m.sx += x;
        m.sxx += x * x;
        m.sxy += x * y;
        m.sy += y;
        m.syy += y * y;
        self.window.is_full().then_some(self.moments)
    }

    fn reset(&mut self) {
        self.window.clear();
        self.moments = PairMoments::ZERO;
    }
}

/// The four sliding sums `TA_BETA` maintains over the two return series.
#[derive(Debug, Clone, Copy)]
struct ReturnMoments {
    sx: f64,
    sy: f64,
    sxx: f64,
    sxy: f64,
}

impl ReturnMoments {
    const ZERO: Self = Self {
        sx: 0.0,
        sy: 0.0,
        sxx: 0.0,
        sxy: 0.0,
    };
}

/// TA-Lib's `TA_BETA` emit expression.
#[inline]
fn beta_of(sx: f64, sy: f64, sxx: f64, sxy: f64, period: f64) -> f64 {
    let denominator = (period * sxx) - (sx * sx);
    if !ta_is_zero(denominator) {
        ((period * sxy) - (sx * sy)) / denominator
    } else {
        0.0
    }
}

/// Sliding pair moments in **`TA_BETA`'s exact accumulation order**.
///
/// `TA_BETA` seeds `period - 1` returns, then per bar adds the incoming
/// return, emits, and *only then* removes the trailing one — the opposite
/// nesting from [`RollingPairMoments`] (`TA_CORREL`), which is why the two
/// cannot share a struct. Reproducing it makes BETA **bitwise** equal to
/// `talib.BETA` at p=5/30 over 100k and 1M bars.
///
/// The resting invariant is therefore "the sums cover the `period - 1` most
/// recent returns", so the rings only need `period - 1` slots and their
/// eviction is TA-Lib's `trailingIdx` return.
#[derive(Debug, Clone)]
struct RollingReturnMoments {
    period: usize,
    window: PairRing,
    moments: ReturnMoments,
}

impl RollingReturnMoments {
    fn new(period: usize) -> TaResult<Self> {
        if period < 2 {
            return Err(invalid_period("timeperiod", period, 2));
        }
        Ok(Self {
            period,
            window: PairRing::new(period - 1)?,
            moments: ReturnMoments::ZERO,
        })
    }

    #[inline]
    fn append(&mut self, x: f64, y: f64) -> Option<ReturnMoments> {
        let m = &mut self.moments;
        // "Add new values", in TA_BETA's statement order.
        m.sxx += x * x;
        m.sxy += x * y;
        m.sx += x;
        m.sy += y;
        // A full retained ring means the sums now cover `period` returns.
        let emitted = self.window.is_full().then_some(self.moments);
        if let Some((trailing_x, trailing_y)) = self.window.push(x, y) {
            // "Remove the trailing", after the output is written.
            let m = &mut self.moments;
            m.sxx -= trailing_x * trailing_x;
            m.sxy -= trailing_x * trailing_y;
            m.sx -= trailing_x;
            m.sy -= trailing_y;
        }
        emitted
    }

    fn reset(&mut self) {
        self.window.clear();
        self.moments = ReturnMoments::ZERO;
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
            value: None,
        })
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, x: f64, y: f64) -> Option<f64> {
        let period = self.period;
        self.value = self
            .moments
            .append(x, y)
            .map(|m| correl_of(m.sx, m.sy, m.sxx, m.syy, m.sxy, period));
        self.value
    }

    /// Bulk kernel: `TA_CORREL`'s remove-trailing / add-new / emit recurrence
    /// indexing the input slices directly. Bit-identical to per-bar
    /// [`Self::append`] in outputs and post-run state.
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
        // Warm-up prologue: after `period` appends the pair ring holds exactly
        // the first `period` slice pairs, regardless of prior state.
        let prologue = n.min(period);
        for i in 0..prologue {
            output.push(self.append(input0[i], input1[i]).unwrap_or(f64::NAN));
        }
        if n <= period {
            return Ok(());
        }
        // Branch-free steady loop: identical arithmetic in identical order to
        // `RollingPairMoments::append`, but every access is a zipped slice
        // iterator, so there is no ring traffic and no bounds check. The
        // `TrustedLen` `extend` writes each result straight into the output's
        // spare capacity — a `resize` prologue would cost a second full pass
        // over the buffer just to pre-fill it.
        let period_f = self.period;
        let mut m = self.moments.moments;
        let steady = n - period;
        output.extend(
            input0[period..]
                .iter()
                .zip(&input1[period..])
                .zip(&input0[..steady])
                .zip(&input1[..steady])
                .map(|(((&x, &y), &tx), &ty)| {
                    // "Remove trailing values", "add new values", then emit.
                    m.sx -= tx;
                    m.sxx -= tx * tx;
                    m.sxy -= tx * ty;
                    m.sy -= ty;
                    m.syy -= ty * ty;
                    m.sx += x;
                    m.sxx += x * x;
                    m.sxy += x * y;
                    m.sy += y;
                    m.syy += y * y;
                    correl_of(m.sx, m.sy, m.sxx, m.syy, m.sxy, period_f)
                }),
        );
        self.moments.moments = m;
        self.value = output.last().copied();
        // Rebuild the pair ring so subsequent appends continue bit-identically.
        self.moments.window.clear();
        for i in n - period..n {
            self.moments.window.push(input0[i], input1[i]);
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
    returns: RollingReturnMoments,
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
            returns: RollingReturnMoments::new(period)?,
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
        let x = beta_return(input0, previous0);
        let y = beta_return(input1, previous1);
        let period = self.period;
        self.value = self
            .returns
            .append(x, y)
            .map(|m| beta_of(m.sx, m.sy, m.sxx, m.sxy, period));
        self.value
    }

    /// Bulk kernel: `TA_BETA`'s add-new / emit / remove-trailing recurrence
    /// over returns recomputed straight from the input slices — exactly what
    /// the C loop does through its `trailingLastPrice` cursors, so no return
    /// series is materialized and no ring is touched inside the loop.
    /// Bit-identical to per-bar [`Self::append`] in outputs and post-run state.
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
        let period = self.returns.period;
        let n = input0.len();
        output.reserve(n);
        // Warm-up prologue: after `period` appends the retained ring holds
        // exactly the `period - 1` returns of `inputs[..period]`, and
        // `previous` is `inputs[period - 1]` — regardless of prior state.
        let prologue = n.min(period);
        for i in 0..prologue {
            output.push(self.append(input0[i], input1[i]).unwrap_or(f64::NAN));
        }
        if n <= period {
            return Ok(());
        }
        // Branch-free steady loop over eight zipped slices: per series, the
        // incoming bar and its predecessor (the new return) and the trailing
        // bar and its predecessor (the return leaving the window). Output slot
        // `k` is bar `i = k + period`. The `TrustedLen` `extend` writes into
        // the output's spare capacity without a pre-fill pass.
        let period_f = self.period;
        let mut m = self.returns.moments;
        let steady = n - period;
        let series0 = input0[period..]
            .iter()
            .zip(&input0[period - 1..n - 1])
            .zip(&input0[1..steady + 1])
            .zip(&input0[..steady]);
        let series1 = input1[period..]
            .iter()
            .zip(&input1[period - 1..n - 1])
            .zip(&input1[1..steady + 1])
            .zip(&input1[..steady]);
        output.extend(series0.zip(series1).map(
            |(
                (((&new0, &previous0), &trailing0), &trailing_previous0),
                (((&new1, &previous1), &trailing1), &trailing_previous1),
            )| {
                let x = beta_return(new0, previous0);
                let y = beta_return(new1, previous1);
                m.sxx += x * x;
                m.sxy += x * y;
                m.sx += x;
                m.sy += y;

                // TA-Lib reads the trailing return before writing the output.
                let tx = beta_return(trailing0, trailing_previous0);
                let ty = beta_return(trailing1, trailing_previous1);

                let out = beta_of(m.sx, m.sy, m.sxx, m.sxy, period_f);

                m.sxx -= tx * tx;
                m.sxy -= tx * ty;
                m.sx -= tx;
                m.sy -= ty;
                out
            },
        ));
        self.returns.moments = m;
        self.value = output.last().copied();
        // Rebuild the retained ring (the `period - 1` most recent returns) and
        // `previous`, so subsequent appends continue bit-identically.
        self.returns.window.clear();
        for i in n - period + 1..n {
            self.returns.window.push(
                beta_return(input0[i], input0[i - 1]),
                beta_return(input1[i], input1[i - 1]),
            );
        }
        self.previous = Some((input0[n - 1], input1[n - 1]));
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

            // 5,000 bars with chunk sizes mutually coprime with the periods,
            // so the bulk prologue/steady split lands at many different
            // offsets inside the window; each run is followed by a 256-bar
            // continue-after-bulk tail of per-bar appends.
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
    fn beta_bulk_is_bitwise_identical_to_per_bar_append() {
        let x = lcg_series(5_000, 0x5EED_B0B0);
        let y = lcg_series(5_000, 0xC0FF_B0B0);
        let tail_x = lcg_series(256, 0x7A11_B0B0);
        let tail_y = lcg_series(256, 0x7A11_B0B1);
        for period in [2usize, 5, 14, 30, 200] {
            let mut per_bar = RollingBeta::new(period).unwrap();
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

            // Chunk sizes mutually coprime with the periods, so the bulk
            // prologue/steady split lands at many different offsets inside the
            // window; each run ends with a continue-after-bulk tail.
            for chunk in [usize::MAX, 1, 7, 10, 97, 1000] {
                let mut state = RollingBeta::new(period).unwrap();
                let mut out = Vec::new();
                let mut start = 0;
                while start < x.len() {
                    let end = (start + chunk.min(x.len())).min(x.len());
                    state
                        .extend_slices_into(&x[start..end], &y[start..end], &mut out)
                        .unwrap();
                    start = end;
                }
                assert_same_bits(&out, &reference, &format!("BETA p{period} chunk {chunk}"));
                let tail_out: Vec<f64> = tail_x
                    .iter()
                    .zip(&tail_y)
                    .map(|(&a, &b)| state.append(a, b).unwrap_or(f64::NAN))
                    .collect();
                assert_same_bits(
                    &tail_out,
                    &tail_reference,
                    &format!("BETA p{period} chunk {chunk} tail"),
                );
            }
        }
    }

    #[test]
    fn correl_streaming_matches_batch_bitwise() {
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
    fn beta_streaming_matches_batch_bitwise() {
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

    /// Literal transcription of TA-Lib's `TA_CORREL` accumulation loop.
    ///
    /// Seed the five sums over the first window, then per bar: remove the
    /// trailing pair's contributions, add the incoming pair's, emit. Note the
    /// per-statement order inside each phase (x, x², xy, y, y²) and that the
    /// period divides inside each term of the emit.
    fn talib_correl_reference(x: &[f64], y: &[f64], period: usize) -> Vec<f64> {
        let n = x.len();
        let mut output = vec![f64::NAN; n];
        let period_f = period as f64;
        let (mut sx, mut sy, mut sxx, mut syy, mut sxy) = (0.0, 0.0, 0.0, 0.0, 0.0);
        let mut trailing = 0usize;
        for today in 0..period {
            let a = x[today];
            sx += a;
            sxx += a * a;
            let b = y[today];
            sxy += a * b;
            sy += b;
            syy += b * b;
        }
        let mut trailing_x = x[trailing];
        let mut trailing_y = y[trailing];
        trailing += 1;
        let temp = (sxx - ((sx * sx) / period_f)) * (syy - ((sy * sy) / period_f));
        output[period - 1] = if !(temp < 0.00000000000001) {
            (sxy - ((sx * sy) / period_f)) / temp.sqrt()
        } else {
            0.0
        };
        for today in period..n {
            // Remove trailing values.
            sx -= trailing_x;
            sxx -= trailing_x * trailing_x;
            sxy -= trailing_x * trailing_y;
            sy -= trailing_y;
            syy -= trailing_y * trailing_y;
            // Add new values.
            let a = x[today];
            sx += a;
            sxx += a * a;
            let b = y[today];
            sxy += a * b;
            sy += b;
            syy += b * b;
            trailing_x = x[trailing];
            trailing_y = y[trailing];
            trailing += 1;
            let temp = (sxx - ((sx * sx) / period_f)) * (syy - ((sy * sy) / period_f));
            output[today] = if !(temp < 0.00000000000001) {
                (sxy - ((sx * sy) / period_f)) / temp.sqrt()
            } else {
                0.0
            };
        }
        output
    }

    /// Literal transcription of TA-Lib's `TA_BETA` accumulation loop:
    /// seed `period - 1` returns, then per bar add the incoming return, read
    /// the trailing one, emit, and only then remove the trailing one.
    fn talib_beta_reference(x: &[f64], y: &[f64], period: usize) -> Vec<f64> {
        let n = x.len();
        let mut output = vec![f64::NAN; n];
        let period_f = period as f64;
        let (mut sx, mut sy, mut sxx, mut sxy) = (0.0, 0.0, 0.0, 0.0);
        let mut last_x = x[0];
        let mut last_y = y[0];
        let mut trailing_last_x = x[0];
        let mut trailing_last_y = y[0];
        let mut trailing = 1usize;
        for i in 1..period {
            let a = beta_return(x[i], last_x);
            last_x = x[i];
            let b = beta_return(y[i], last_y);
            last_y = y[i];
            sxx += a * a;
            sxy += a * b;
            sx += a;
            sy += b;
        }
        for i in period..n {
            let a = beta_return(x[i], last_x);
            last_x = x[i];
            let b = beta_return(y[i], last_y);
            last_y = y[i];
            sxx += a * a;
            sxy += a * b;
            sx += a;
            sy += b;

            let trailing_x = beta_return(x[trailing], trailing_last_x);
            trailing_last_x = x[trailing];
            let trailing_y = beta_return(y[trailing], trailing_last_y);
            trailing_last_y = y[trailing];
            trailing += 1;

            let temp = (period_f * sxx) - (sx * sx);
            output[i] = if !ta_is_zero(temp) {
                ((period_f * sxy) - (sx * sy)) / temp
            } else {
                0.0
            };

            sxx -= trailing_x * trailing_x;
            sxy -= trailing_x * trailing_y;
            sx -= trailing_x;
            sy -= trailing_y;
        }
        output
    }

    #[test]
    fn correl_reproduces_the_talib_recurrence_bitwise() {
        let x = lcg_series(5_000, 0x5EED_C0C0);
        let y = lcg_series(5_000, 0xC0FF_C0C0);
        for period in [2usize, 5, 14, 30, 200] {
            let reference = talib_correl_reference(&x, &y, period);

            let mut state = RollingCorrelation::new(period).unwrap();
            let streamed: Vec<f64> = x
                .iter()
                .zip(&y)
                .map(|(&a, &b)| state.append(a, b).unwrap_or(f64::NAN))
                .collect();
            assert_same_bits(&streamed, &reference, &format!("CORREL TA-order p{period}"));

            let mut bulk_state = RollingCorrelation::new(period).unwrap();
            let mut bulk = Vec::new();
            let mut start = 0;
            while start < x.len() {
                let end = (start + 997).min(x.len());
                bulk_state
                    .extend_slices_into(&x[start..end], &y[start..end], &mut bulk)
                    .unwrap();
                start = end;
            }
            assert_same_bits(
                &bulk,
                &reference,
                &format!("CORREL TA-order bulk p{period}"),
            );

            let batch = crate::stream::rolling_corr(&x, &y, period).unwrap();
            assert_same_bits(
                &batch,
                &reference,
                &format!("CORREL TA-order batch p{period}"),
            );
        }
    }

    #[test]
    fn beta_reproduces_the_talib_recurrence_bitwise() {
        let x = lcg_series(5_000, 0x5EED_BEEF);
        let y = lcg_series(5_000, 0xC0FF_BEEF);
        for period in [2usize, 5, 14, 30, 200] {
            let reference = talib_beta_reference(&x, &y, period);

            let mut state = RollingBeta::new(period).unwrap();
            let streamed: Vec<f64> = x
                .iter()
                .zip(&y)
                .map(|(&a, &b)| state.append(a, b).unwrap_or(f64::NAN))
                .collect();
            assert_same_bits(&streamed, &reference, &format!("BETA TA-order p{period}"));

            let mut bulk_state = RollingBeta::new(period).unwrap();
            let mut bulk = Vec::new();
            let mut start = 0;
            while start < x.len() {
                let end = (start + 997).min(x.len());
                bulk_state
                    .extend_slices_into(&x[start..end], &y[start..end], &mut bulk)
                    .unwrap();
                start = end;
            }
            assert_same_bits(&bulk, &reference, &format!("BETA TA-order bulk p{period}"));

            let batch = crate::stream::rolling_beta(&x, &y, period).unwrap();
            assert_same_bits(
                &batch,
                &reference,
                &format!("BETA TA-order batch p{period}"),
            );
        }
    }

    /// CORREL and BETA deliberately reproduce TA-Lib's sliding accumulators
    /// rather than periodically reseeded (more accurate) ones: the oracle gate
    /// compares against TA-Lib, whose own sums drift, so reseeding moves us
    /// towards truth and therefore *away* from the oracle. Measured against
    /// `talib.CORREL`/`talib.BETA` on the harness's AR(1) series, the verbatim
    /// order is bitwise identical at 100k and 1M bars where the reseeded
    /// version was 1.5e-9 / 2.7e-9 off.
    ///
    /// So the invariant tested at scale is that the state never diverges from
    /// that recurrence — checked bitwise at every bar — while the residual
    /// against a fresh per-window recomputation stays bounded (that residual
    /// is TA-Lib's own drift; the loose bound only has to catch a genuinely
    /// broken accumulator, which lands orders of magnitude higher).
    #[test]
    fn correl_and_beta_track_the_talib_recurrence_over_1m_bars() {
        let x = lcg_series(1_000_000, 0x5EED_1E6B);
        let y = lcg_series(1_000_000, 0xC0FF_1E6B);
        for period in [14usize, 30] {
            let correl_reference = talib_correl_reference(&x, &y, period);
            let mut correl = RollingCorrelation::new(period).unwrap();
            let beta_reference = talib_beta_reference(&x, &y, period);
            let mut beta = RollingBeta::new(period).unwrap();
            for i in 0..x.len() {
                let value = correl.append(x[i], y[i]).unwrap_or(f64::NAN);
                assert_eq!(
                    value.to_bits(),
                    correl_reference[i].to_bits(),
                    "CORREL p{period} bar {i}: diverged from the TA-Lib recurrence"
                );
                let beta_value = beta.append(x[i], y[i]).unwrap_or(f64::NAN);
                assert_eq!(
                    beta_value.to_bits(),
                    beta_reference[i].to_bits(),
                    "BETA p{period} bar {i}: diverged from the TA-Lib recurrence"
                );
                if (i + 1) % 50_000 == 0 {
                    let exact = exact_correl(&x, &y, i, period);
                    let drift = (value - exact).abs();
                    assert!(
                        drift < 1e-9,
                        "CORREL p{period} bar {i}: drift {drift:e} vs a fresh window"
                    );
                }
            }
        }
    }
}
