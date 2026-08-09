use std::collections::VecDeque;

use super::{
    CumulativeMaximum, ExponentialMovingAverage, MonotonicMax, MonotonicMin, SchaffTrendCycle,
    SchaffTrendCycleValue, SimpleMovingAverage, StreamingIndicator, Window,
};
use crate::error::{TaError, TaResult};
use crate::indicators::{RollingMedian, RollingStandardDeviation};

pub(crate) fn validate_period(timeperiod: usize) -> TaResult<()> {
    if timeperiod == 0 {
        return Err(TaError::InvalidParameter {
            name: "timeperiod",
            value: timeperiod.to_string(),
            reason: "must be >= 1",
        });
    }
    Ok(())
}

pub(crate) fn validate_quantile(quantile: f64) -> TaResult<()> {
    if !(0.0..=1.0).contains(&quantile) {
        return Err(TaError::InvalidParameter {
            name: "quantile",
            value: quantile.to_string(),
            reason: "must be between 0 and 1",
        });
    }
    Ok(())
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct ObZone {
    pub(crate) direction: f64,
    pub(crate) top: f64,
    pub(crate) bottom: f64,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct LiquidityPool {
    pub(crate) level: f64,
    /// Insertion order across both lists of one side; reproduces the original
    /// single-vector scan order for nearest-pool tie-breaks and sweep output.
    pub(crate) seq: u64,
}

/// Location of the nearest matching pool: candidate list or confirmed list.
#[derive(Debug, Clone, Copy)]
pub(crate) enum PoolSlot {
    Candidate(usize),
    Confirmed(usize),
}

#[derive(Debug, Clone)]
pub(crate) struct RollingPairMoments {
    values: VecDeque<(f64, f64)>,
    timeperiod: usize,
    /// Sample variance of the `y` window, computed in the covariance pass so
    /// consumers (`OrnsteinUhlenbeckHalfLife`) do not rescan the window.
    pub(crate) var_y: f64,
    value: Option<f64>,
}

impl RollingPairMoments {
    pub(crate) fn new(timeperiod: usize) -> TaResult<Self> {
        validate_period(timeperiod)?;
        Ok(Self {
            values: VecDeque::with_capacity(timeperiod),
            timeperiod,
            var_y: f64::NAN,
            value: None,
        })
    }

    pub(crate) fn append(&mut self, x: f64, y: f64) -> Option<f64> {
        if self.values.len() == self.timeperiod {
            self.values.pop_front();
        }
        self.values.push_back((x, y));
        self.value = if self.values.len() == self.timeperiod {
            let n = self.timeperiod as f64;
            // Contiguous two-slice scans with fused accumulators: each
            // accumulator adds the same terms in the same order as the
            // original per-quantity passes, so results are bit-identical.
            let (front, back) = self.values.as_slices();
            let mut sum_x = 0.0;
            let mut sum_y = 0.0;
            for &(x, y) in front {
                sum_x += x;
                sum_y += y;
            }
            for &(x, y) in back {
                sum_x += x;
                sum_y += y;
            }
            let mean_x = sum_x / n;
            let mean_y = sum_y / n;
            let mut cov = 0.0;
            let mut squared_y = 0.0;
            for &(x, y) in front {
                let delta_y = y - mean_y;
                cov += (x - mean_x) * delta_y;
                squared_y += delta_y * delta_y;
            }
            for &(x, y) in back {
                let delta_y = y - mean_y;
                cov += (x - mean_x) * delta_y;
                squared_y += delta_y * delta_y;
            }
            self.var_y = squared_y / (n - 1.0);
            Some(cov / (n - 1.0))
        } else {
            None
        };
        self.value
    }

    pub(crate) fn value(&self) -> Option<f64> {
        self.value
    }

    pub(crate) fn reset(&mut self) {
        self.values.clear();
        self.var_y = f64::NAN;
        self.value = None;
    }
}

/// SMA of the true-range series with pandas-ta's NaN-at-bar-0 convention.
///
/// The true range of bar 0 is NaN and is excluded from every window, so the
/// first valid band lands at bar `period` (windows over bars `1..=period`)
/// instead of `period - 1`.
#[derive(Debug, Clone)]
pub(crate) struct SqueezeTrBand {
    period: usize,
    window: Window,
    sum: f64,
    value: Option<f64>,
}

impl SqueezeTrBand {
    pub(crate) fn new(period: usize) -> TaResult<Self> {
        Ok(Self {
            period,
            window: Window::new(period)?,
            sum: 0.0,
            value: None,
        })
    }

    pub(crate) fn append(&mut self, tr: f64) -> Option<f64> {
        if !tr.is_nan() {
            if let Some(old) = self.window.push(tr) {
                self.sum -= old;
            }
            self.sum += tr;
        }
        self.value = self.window.is_full().then(|| self.sum / self.period as f64);
        self.value
    }

    pub(crate) fn reset(&mut self) {
        self.window.clear();
        self.sum = 0.0;
        self.value = None;
    }
}

/// Python `round(value, 8)` semantics: round half to even at 1e-8 scale.
pub(crate) fn round8(value: f64) -> f64 {
    const SCALE: f64 = 1e8;
    let scaled = value * SCALE;
    let floor = scaled.floor();
    let diff = scaled - floor;
    let rounded = if diff < 0.5 {
        floor
    } else if diff > 0.5 {
        floor + 1.0
    } else if floor % 2.0 == 0.0 {
        floor
    } else {
        floor + 1.0
    };
    rounded / SCALE
}

/// O(1) amortized rolling extremum (min or max) via a monotonic deque.
///
/// Mirrors pandas `rolling(period).min()/max()`: a NaN input voids the window
/// and the output resumes only after `period` consecutive non-NaN values.
#[derive(Debug, Clone)]
pub(crate) struct RollingExtremum {
    period: usize,
    is_min: bool,
    deque: VecDeque<(usize, f64)>,
    index: usize,
    warm: usize,
    value: Option<f64>,
}

impl RollingExtremum {
    pub(crate) fn new(period: usize, is_min: bool) -> TaResult<Self> {
        if period == 0 {
            return Err(TaError::InvalidParameter {
                name: "timeperiod",
                value: period.to_string(),
                reason: "must be >= 1",
            });
        }
        Ok(Self {
            period,
            is_min,
            deque: VecDeque::new(),
            index: 0,
            warm: 0,
            value: None,
        })
    }

    pub(crate) fn append(&mut self, x: f64) -> Option<f64> {
        let index = self.index;
        self.index += 1;
        if x.is_nan() {
            self.deque.clear();
            self.warm = 0;
            self.value = None;
            return None;
        }
        self.warm = (self.warm + 1).min(self.period);
        while let Some(&(old, _)) = self.deque.front() {
            if old + self.period <= index {
                self.deque.pop_front();
            } else {
                break;
            }
        }
        while let Some(&(_, value)) = self.deque.back() {
            let dominated = if self.is_min { value >= x } else { value <= x };
            if dominated {
                self.deque.pop_back();
            } else {
                break;
            }
        }
        self.deque.push_back((index, x));
        self.value = (self.warm >= self.period).then(|| self.deque.front().unwrap().1);
        self.value
    }

    pub(crate) fn reset(&mut self) {
        self.deque.clear();
        self.index = 0;
        self.warm = 0;
        self.value = None;
    }
}

/// Body of [`SchaffTrendCycle::extend_slices_into`].
///
/// A free function because `#[multiversion]` cannot annotate a method that
/// takes `self`. The dispatch gets a hardware FMA for the two `mul_add` EMA
/// steps instead of a libm `fma()` call per bar; `mul_add` is explicitly fused
/// either way, so results are bit-identical.
#[allow(unexpected_cfgs)]
#[multiversion::multiversion(targets("x86_64+avx2+fma", "x86_64+avx", "x86_64+sse4.2"))]
pub(crate) fn schaff_trend_cycle_bulk(
    state: &mut SchaffTrendCycle,
    close: &[f64],
    stc_out: &mut Vec<f64>,
    macd_out: &mut Vec<f64>,
    stoch_out: &mut Vec<f64>,
) {
    stc_out.reserve(close.len());
    macd_out.reserve(close.len());
    stoch_out.reserve(close.len());
    let mut index = 0;
    // Warm-up prologue: per-bar appends until the slow EMA is seeded
    // (the fast EMA warms no later than the slow one).
    while index < close.len() && state.slow_ema.current().is_none() {
        let value = state.append(close[index]);
        stc_out.push(value.stc);
        macd_out.push(value.macd);
        stoch_out.push(value.stoch);
        index += 1;
    }
    if index == close.len() {
        return;
    }

    let fast_k = state.fast_ema.smoothing();
    let slow_k = state.slow_ema.smoothing();
    let mut fast = state.fast_ema.current().expect("warm fast EMA");
    let mut slow = state.slow_ema.current().expect("warm slow EMA");
    let factor = state.factor;
    let mut last = state.value;
    for &close_value in &close[index..] {
        fast = fast_k.mul_add(close_value - fast, fast);
        slow = slow_k.mul_add(close_value - slow, slow);
        let macd = fast - slow;

        let lowest = state.xmacd_low.append(macd).unwrap_or(f64::NAN);
        let highest = state.xmacd_high.append(macd).unwrap_or(f64::NAN);
        let range = non_zero(highest - lowest);
        if lowest > 0.0 {
            state.stoch1 = 100.0 * ((macd - lowest) / range);
        }
        state.pf = round8(state.pf + factor * (state.stoch1 - state.pf));

        let lowest_pf = state.pf_low.append(state.pf).unwrap_or(f64::NAN);
        let highest_pf = state.pf_high.append(state.pf).unwrap_or(f64::NAN);
        let range_pf = non_zero(highest_pf - lowest_pf);
        if range_pf > 0.0 {
            state.stoch2 = 100.0 * ((state.pf - lowest_pf) / range_pf);
        }
        state.pff = round8(state.pff + factor * (state.stoch2 - state.pff));

        let value = SchaffTrendCycleValue {
            stc: state.pff,
            macd,
            stoch: state.pf,
        };
        stc_out.push(value.stc);
        macd_out.push(value.macd);
        stoch_out.push(value.stoch);
        last = Some(value);
    }

    let appended = close.len() - index;
    state.fast_ema.store_bulk_state(fast, appended);
    state.slow_ema.store_bulk_state(slow, appended);
    state.value = last;
}

/// pandas-ta `non_zero_range(max, min)`: `max − min`, substituting
/// `f64::EPSILON` for an exact zero so flat windows avoid 0/0 division. The
/// package adds the epsilon to the whole series when *any* element is zero;
/// that global perturbation is far below the 1e-8 smoothing precision, so a
/// per-bar guard is equivalent in effect.
pub(crate) fn non_zero(difference: f64) -> f64 {
    if difference == 0.0 {
        f64::EPSILON
    } else {
        difference
    }
}

/// Rolling window sum with pandas `rolling(period).sum()` semantics: NaN
/// inputs are skipped and the output appears once `period` non-NaN values are
/// in the window (used for the Vortex true-range and movement sums).
#[derive(Debug, Clone)]
struct RollingSum {
    period: usize,
    window: Window,
    count: usize,
    sum: f64,
    value: Option<f64>,
}

impl RollingSum {
    pub(crate) fn new(period: usize) -> TaResult<Self> {
        if period == 0 {
            return Err(TaError::InvalidParameter {
                name: "timeperiod",
                value: period.to_string(),
                reason: "must be >= 1",
            });
        }
        Ok(Self {
            period,
            window: Window::new(period)?,
            count: 0,
            sum: 0.0,
            value: None,
        })
    }

    pub(crate) fn append(&mut self, x: f64) -> Option<f64> {
        if let Some(old) = self.window.push(x) {
            if !old.is_nan() {
                self.sum -= old;
                self.count -= 1;
            }
        }
        if !x.is_nan() {
            self.sum += x;
            self.count += 1;
        }
        self.value = (self.count >= self.period).then_some(self.sum);
        self.value
    }

    pub(crate) fn reset(&mut self) {
        self.window.clear();
        self.count = 0;
        self.sum = 0.0;
        self.value = None;
    }
}

/// ROC → SMA pair used by KST: `(close − close[roc]) / close[roc]` fed into an
/// SMA once the shift window is warm.
#[derive(Debug, Clone)]
pub(crate) struct KstRocSma {
    pub(crate) close_window: Window,
    pub(crate) sma: SimpleMovingAverage,
}

impl KstRocSma {
    pub(crate) fn new(roc_period: usize, sma_period: usize) -> TaResult<Self> {
        Ok(Self {
            close_window: Window::new(roc_period)?,
            sma: SimpleMovingAverage::new(sma_period)?,
        })
    }

    pub(crate) fn append(&mut self, close: f64) -> Option<f64> {
        match self.close_window.push(close) {
            Some(previous) => self.sma.append((close - previous) / previous),
            None => None,
        }
    }

    pub(crate) fn reset(&mut self) {
        self.close_window.clear();
        self.sma.reset();
    }
}

/// Rolling mean with pandas `min_periods=0` semantics: defined whenever the
/// window holds at least one non-NaN value (KST signal-line warm-up).
#[derive(Debug, Clone)]
pub(crate) struct RollingMeanMin0 {
    period: usize,
    window: Window,
    count: usize,
    sum: f64,
    value: Option<f64>,
}

impl RollingMeanMin0 {
    pub(crate) fn new(period: usize) -> TaResult<Self> {
        if period == 0 {
            return Err(TaError::InvalidParameter {
                name: "timeperiod",
                value: period.to_string(),
                reason: "must be >= 1",
            });
        }
        Ok(Self {
            period,
            window: Window::new(period)?,
            count: 0,
            sum: 0.0,
            value: None,
        })
    }

    pub(crate) fn append(&mut self, x: f64) -> Option<f64> {
        if let Some(old) = self.window.push(x) {
            if !old.is_nan() {
                self.sum -= old;
                self.count -= 1;
            }
        }
        if !x.is_nan() {
            self.sum += x;
            self.count += 1;
        }
        self.value = (self.count > 0).then_some(self.sum / self.count as f64);
        self.value
    }

    pub(crate) fn reset(&mut self) {
        self.window.clear();
        self.count = 0;
        self.sum = 0.0;
        self.value = None;
    }
}

#[derive(Debug, Clone)]
pub(crate) struct RollingMean {
    values: VecDeque<f64>,
    timeperiod: usize,
    sum: f64,
    value: Option<f64>,
}

impl RollingMean {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(timeperiod: usize) -> TaResult<Self> {
        validate_period(timeperiod)?;
        Ok(Self {
            values: VecDeque::with_capacity(timeperiod),
            timeperiod,
            sum: 0.0,
            value: None,
        })
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, input: f64) -> Option<f64> {
        if self.values.len() == self.timeperiod {
            self.sum -= self.values.pop_front().expect("ring is full");
        }
        self.values.push_back(input);
        self.sum += input;
        self.value = if self.values.len() == self.timeperiod {
            Some(self.sum / self.timeperiod as f64)
        } else {
            None
        };
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

    /// Reset the persistent state and clear the latest value.
    pub fn reset(&mut self) {
        self.values.clear();
        self.sum = 0.0;
        self.value = None;
    }
}

pub(crate) fn ewm_alpha(timeperiod: usize) -> TaResult<f64> {
    validate_period(timeperiod)?;
    Ok(2.0 / (timeperiod as f64 + 1.0))
}

pub(crate) fn weighted_mean(values: &VecDeque<f64>) -> f64 {
    let denominator = (values.len() * (values.len() + 1) / 2) as f64;
    values
        .iter()
        .enumerate()
        .map(|(i, &v)| v * (i + 1) as f64)
        .sum::<f64>()
        / denominator
}

/// Slice twin of [`weighted_mean`]: identical iteration order and arithmetic
/// (`v * (i + 1)` accumulated oldest → newest), so results are bit-identical
/// when the slice holds the same values front-to-back as the deque.
pub(crate) fn weighted_mean_slice(values: &[f64]) -> f64 {
    let denominator = (values.len() * (values.len() + 1) / 2) as f64;
    values
        .iter()
        .enumerate()
        .map(|(i, &v)| v * (i + 1) as f64)
        .sum::<f64>()
        / denominator
}

/// Fixed-capacity FIFO whose live window is always one contiguous slice.
///
/// Backed by a double-write buffer of `2 * capacity`: every value is stored
/// at `pos` and `pos + capacity`, so the logical window (oldest → newest) is
/// a single `&[f64]` — per-window rescans read straight-line memory instead
/// of chasing a deque. Allocates exactly once; `clear` never reallocates.
#[derive(Debug, Clone)]
pub(crate) struct ContiguousWindow {
    buf: Box<[f64]>,
    cap: usize,
    len: usize,
    /// Next write slot in `0..cap`.
    pos: usize,
}

impl ContiguousWindow {
    pub(crate) fn new(cap: usize) -> Self {
        debug_assert!(cap >= 1);
        Self {
            buf: vec![0.0; 2 * cap].into_boxed_slice(),
            cap,
            len: 0,
            pos: 0,
        }
    }

    #[inline]
    pub(crate) fn push(&mut self, value: f64) {
        self.buf[self.pos] = value;
        self.buf[self.pos + self.cap] = value;
        self.pos += 1;
        if self.pos == self.cap {
            self.pos = 0;
        }
        if self.len < self.cap {
            self.len += 1;
        }
    }

    #[inline]
    pub(crate) fn len(&self) -> usize {
        self.len
    }

    #[inline]
    pub(crate) fn is_full(&self) -> bool {
        self.len == self.cap
    }

    /// The live window, oldest → newest, as one contiguous slice.
    #[inline]
    pub(crate) fn window(&self) -> &[f64] {
        // Newest element sits at `pos - 1` (mod cap); its double-write copy
        // at `pos - 1 + cap` ends the contiguous run of the last `len` values.
        let end = self.pos + self.cap;
        &self.buf[end - self.len..end]
    }

    #[inline]
    pub(crate) fn clear(&mut self) {
        self.len = 0;
        self.pos = 0;
    }
}

/// One monotonic staircase answering rolling max/min for **several** window
/// lengths at once.
///
/// A single deque built for the longest period `P` holds exactly the elements
/// not dominated by a later one, values strictly monotone from front to back.
/// For any `p <= P` the extremum of the last `p` bars is the first entry whose
/// index is still inside that shorter window — found by binary search. Three
/// nested windows therefore need one staircase per side instead of one deque
/// each. Backed by a fixed ring (`Box<[(usize, f64)]>`), never a `VecDeque`.
#[derive(Debug, Clone)]
pub(crate) struct MultiPeriodStaircase {
    buf: Box<[(usize, f64)]>,
    head: usize,
    len: usize,
    /// Number of observations consumed since construction/reset.
    index: usize,
    /// Longest window this staircase serves.
    longest: usize,
    /// `true` for a max staircase, `false` for a min staircase.
    maximum: bool,
}

impl MultiPeriodStaircase {
    pub(crate) fn new(longest: usize, maximum: bool) -> Self {
        debug_assert!(longest >= 1);
        Self {
            buf: vec![(0usize, 0.0f64); longest].into_boxed_slice(),
            head: 0,
            len: 0,
            index: 0,
            longest,
            maximum,
        }
    }

    #[inline]
    pub(crate) fn entry(&self, offset: usize) -> (usize, f64) {
        let capacity = self.buf.len();
        let mut slot = self.head + offset;
        if slot >= capacity {
            slot -= capacity;
        }
        self.buf[slot]
    }

    /// Pushes one observation, evicting entries that can never be an
    /// extremum again. Pop-on-equal (newest wins) matches `MonotonicMax`.
    pub(crate) fn push(&mut self, value: f64) {
        let capacity = self.buf.len();
        let index = self.index;
        self.index += 1;
        while self.len > 0 {
            let (_, back) = self.entry(self.len - 1);
            let dominated = if self.maximum {
                back <= value
            } else {
                back >= value
            };
            if !dominated {
                break;
            }
            self.len -= 1;
        }
        // Drop aged-out entries *before* inserting: the live entries then all
        // carry distinct indices inside the longest window, so they always fit
        // the ring's `longest` slots.
        let first_valid = index.saturating_add(1).saturating_sub(self.longest);
        while self.len > 0 && self.entry(0).0 < first_valid {
            self.head += 1;
            if self.head == capacity {
                self.head = 0;
            }
            self.len -= 1;
        }
        let mut tail = self.head + self.len;
        if tail >= capacity {
            tail -= capacity;
        }
        self.buf[tail] = (index, value);
        self.len += 1;
    }

    /// The extremum over the last `period` observations, or `None` while
    /// fewer than `period` observations have been seen.
    pub(crate) fn extremum(&self, period: usize) -> Option<f64> {
        debug_assert!(period <= self.longest);
        if self.index < period {
            return None;
        }
        let first_valid = self.index - period;
        // Entries are index-ascending: find the first one inside the window.
        let mut low = 0;
        let mut high = self.len;
        while low < high {
            let middle = (low + high) / 2;
            if self.entry(middle).0 < first_valid {
                low = middle + 1;
            } else {
                high = middle;
            }
        }
        Some(self.entry(low).1)
    }

    pub(crate) fn reset(&mut self) {
        self.head = 0;
        self.len = 0;
        self.index = 0;
    }
}

#[derive(Debug, Clone)]
pub(crate) struct MassEma {
    period: usize,
    alpha: f64,
    count: usize,
    value: Option<f64>,
}

impl MassEma {
    pub(crate) fn new(period: usize) -> Self {
        Self {
            period,
            alpha: 2.0 / (period as f64 + 1.0),
            count: 0,
            value: None,
        }
    }

    pub(crate) fn append(&mut self, input: f64) -> Option<f64> {
        self.count += 1;
        let value = self
            .value
            .map_or(input, |previous| previous + self.alpha * (input - previous));
        self.value = Some(value);
        (self.count >= self.period).then_some(value)
    }

    pub(crate) fn reset(&mut self) {
        self.count = 0;
        self.value = None;
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum VolumeIndexMode {
    Negative,
    Positive,
}

#[derive(Debug, Clone)]
pub(crate) struct VolumeIndex {
    mode: VolumeIndexMode,
    previous_close: Option<f64>,
    previous_volume: Option<f64>,
    pub(crate) value: f64,
}

impl VolumeIndex {
    pub(crate) fn new(mode: VolumeIndexMode) -> Self {
        Self {
            mode,
            previous_close: None,
            previous_volume: None,
            value: 1000.0,
        }
    }
    pub(crate) fn append(&mut self, close: f64, volume: f64) -> f64 {
        if let (Some(previous_close), Some(previous_volume)) =
            (self.previous_close, self.previous_volume)
        {
            let active = match self.mode {
                VolumeIndexMode::Negative => volume < previous_volume,
                VolumeIndexMode::Positive => volume > previous_volume,
            };
            if active && previous_close != 0.0 {
                self.value *= 1.0 + (close - previous_close) / previous_close;
            }
        }
        self.previous_close = Some(close);
        self.previous_volume = Some(volume);
        self.value
    }
    pub(crate) fn reset(&mut self) {
        self.previous_close = None;
        self.previous_volume = None;
        self.value = 1000.0;
    }
}

include!("operator_states_tests.rs");
