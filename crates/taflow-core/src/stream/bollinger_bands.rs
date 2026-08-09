//! Stateful Bollinger Bands.
//!
//! The selected moving-average type controls only the middle band.  As in
//! TA-Lib, both outer bands use population deviation around the rolling SMA.

use multiversion::multiversion;

use crate::error::TaResult;
use crate::indicators::RollingStandardDeviation;
use crate::ma_type::MaType;

use super::{
    invalid_period, moving_average_dispatcher::MovingAverageDispatcher, StreamingIndicator, Window,
};

/// Steady-state kernel of [`BollingerBands::extend_slices_into`].
///
/// Split out so it can carry `#[multiversion]`: the loop runs a `mul_add` per
/// bar, which a portable build without runtime dispatch lowers to a libm
/// `fma()` call. `mul_add` is explicitly fused either way, so the dispatched
/// variants are bit-identical.
#[allow(unexpected_cfgs)]
#[multiversion(targets("x86_64+avx2+fma", "x86_64+avx", "x86_64+sse4.2"))]
fn bbands_steady_loop(
    inputs: &[f64],
    period: usize,
    accumulators: [&mut f64; 3],
    constants: [f64; 4],
    upper_out: &mut Vec<f64>,
    middle_out: &mut Vec<f64>,
    lower_out: &mut Vec<f64>,
) -> BollingerBandsValue {
    let [sma_sum_out, moments_sum_out, moments_sum_squares_out] = accumulators;
    let [period_f, inverse_period, deviations_up, deviations_down] = constants;
    let mut sma_sum = *sma_sum_out;
    let mut moments_sum = *moments_sum_out;
    let mut moments_sum_squares = *moments_sum_squares_out;
    let mut last = BollingerBandsValue {
        upper: f64::NAN,
        middle: f64::NAN,
        lower: f64::NAN,
    };
    for i in period..inputs.len() {
        let input = inputs[i];
        let old = inputs[i - period];
        sma_sum -= old;
        sma_sum += input;
        moments_sum += input - old;
        moments_sum_squares += (input - old).mul_add(input + old, 0.0);
        let middle = sma_sum / period_f;
        let mean = moments_sum * inverse_period;
        let variance = moments_sum_squares * inverse_period - mean * mean;
        let deviation = variance.max(0.0).sqrt();
        last = BollingerBandsValue {
            upper: middle + deviations_up * deviation,
            middle,
            lower: middle - deviations_down * deviation,
        };
        upper_out.push(last.upper);
        middle_out.push(last.middle);
        lower_out.push(last.lower);
    }
    *sma_sum_out = sma_sum;
    *moments_sum_out = moments_sum;
    *moments_sum_squares_out = moments_sum_squares;
    last
}

/// One aligned upper, middle, and lower Bollinger Bands observation.
#[derive(Debug, Clone, Copy, PartialEq)]
/// Persistent Rust state or aligned output type for `BollingerBandsValue`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct BollingerBandsValue {
    pub upper: f64,
    pub middle: f64,
    pub lower: f64,
}

/// Shared-ring middle + deviation state for the default SMA matype.
///
/// BBANDS with an SMA middle band previously pushed every input into two
/// parallel rings (the SMA's window and the deviation moments' window).
/// This variant keeps ONE ring plus both scalar accumulator sets, using
/// exactly the arithmetic each component used before:
/// * SMA sum: `sum -= old; sum += input;` and `sum / period as f64`.
/// * Moments: `sum += input - old` / `(input-old).mul_add(input+old, 0.0)`
///   when full, `sum += input` / `input.mul_add(input, sum_squares)` while
///   warming, then `sum_squares/p - mean²` and `.max(0.0).sqrt() * nbdev`
///   with `nbdev = 1.0` (an exact identity, so the multiply is dropped).
struct SharedSmaCore {
    period: usize,
    inverse_period: f64,
    window: Window,
    sma_sum: f64,
    moments_sum: f64,
    moments_sum_squares: f64,
}

impl SharedSmaCore {
    fn new(period: usize) -> TaResult<Self> {
        if period < 2 {
            return Err(invalid_period("timeperiod", period, 2));
        }
        Ok(Self {
            period,
            inverse_period: 1.0 / period as f64,
            window: Window::new(period)?,
            sma_sum: 0.0,
            moments_sum: 0.0,
            moments_sum_squares: 0.0,
        })
    }

    /// One shared push; returns `(middle, deviation)` once warm.
    #[inline]
    fn append(&mut self, input: f64) -> Option<(f64, f64)> {
        let evicted = self.window.push(input);
        if let Some(old) = evicted {
            self.sma_sum -= old;
            self.sma_sum += input;
            self.moments_sum += input - old;
            self.moments_sum_squares += (input - old).mul_add(input + old, 0.0);
        } else {
            self.sma_sum += input;
            self.moments_sum += input;
            self.moments_sum_squares = input.mul_add(input, self.moments_sum_squares);
        }
        self.window.is_full().then(|| self.outputs())
    }

    #[inline]
    fn outputs(&self) -> (f64, f64) {
        let middle = self.sma_sum / self.period as f64;
        let mean = self.moments_sum * self.inverse_period;
        let variance = self.moments_sum_squares * self.inverse_period - mean * mean;
        (middle, variance.max(0.0).sqrt())
    }

    fn reset(&mut self) {
        self.window.clear();
        self.sma_sum = 0.0;
        self.moments_sum = 0.0;
        self.moments_sum_squares = 0.0;
    }
}

enum BollingerBandsCore {
    /// Default SMA middle band: one shared ring for mean and moments (M4).
    Sma(SharedSmaCore),
    /// Any other matype: dispatched MA plus a separate deviation state.
    Dispatch {
        middle: MovingAverageDispatcher,
        deviation: RollingStandardDeviation,
    },
}

/// Incremental Bollinger Bands with constant per-bar work.
/// Persistent Rust state or aligned output type for `BollingerBands`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct BollingerBands {
    core: BollingerBandsCore,
    deviations_up: f64,
    deviations_down: f64,
    value: Option<BollingerBandsValue>,
}

impl BollingerBands {
    /// Creates a BBANDS state for a period of at least two bars.
    pub fn new(
        period: usize,
        deviations_up: f64,
        deviations_down: f64,
        ma_type: MaType,
    ) -> TaResult<Self> {
        let core = if matches!(ma_type, MaType::SimpleMovingAverage) {
            BollingerBandsCore::Sma(SharedSmaCore::new(period)?)
        } else {
            BollingerBandsCore::Dispatch {
                middle: MovingAverageDispatcher::new(period, ma_type)?,
                deviation: RollingStandardDeviation::new(period, 1.0)?,
            }
        };
        Ok(Self {
            core,
            deviations_up,
            deviations_down,
            value: None,
        })
    }

    #[inline]
    fn bands(&self, middle: f64, deviation: f64) -> BollingerBandsValue {
        BollingerBandsValue {
            upper: middle + self.deviations_up * deviation,
            middle,
            lower: middle - self.deviations_down * deviation,
        }
    }

    /// Bulk kernel: O(1) add/evict recurrences over the shared ring's sums,
    /// indexing the input slice directly (SMA matype); other matypes fall
    /// back to per-bar appends. Bit-identical to [`Self::append`] in outputs
    /// and post-run streaming state.
    pub fn extend_slices_into(
        &mut self,
        inputs: &[f64],
        upper_out: &mut Vec<f64>,
        middle_out: &mut Vec<f64>,
        lower_out: &mut Vec<f64>,
    ) {
        let n = inputs.len();
        upper_out.reserve(n);
        middle_out.reserve(n);
        lower_out.reserve(n);
        let mut push = |value: Option<BollingerBandsValue>,
                        upper_out: &mut Vec<f64>,
                        middle_out: &mut Vec<f64>,
                        lower_out: &mut Vec<f64>| match value {
            Some(value) => {
                upper_out.push(value.upper);
                middle_out.push(value.middle);
                lower_out.push(value.lower);
            }
            None => {
                upper_out.push(f64::NAN);
                middle_out.push(f64::NAN);
                lower_out.push(f64::NAN);
            }
        };
        let period = match &self.core {
            BollingerBandsCore::Sma(core) => core.period,
            BollingerBandsCore::Dispatch { .. } => {
                for &input in inputs {
                    let value = self.append(input);
                    push(value, upper_out, middle_out, lower_out);
                }
                return;
            }
        };
        // Warm-up prologue: after `period` appends the shared ring holds
        // exactly `inputs[..period]`, regardless of prior state.
        let prologue = n.min(period);
        for &input in &inputs[..prologue] {
            let value = self.append(input);
            push(value, upper_out, middle_out, lower_out);
        }
        if n <= period {
            return;
        }
        let BollingerBandsCore::Sma(core) = &mut self.core else {
            unreachable!("SMA period resolved above");
        };
        // Steady loop: same accumulate/evict arithmetic as the shared-ring
        // append, with the evicted element read from the input slice.
        let mut sma_sum = core.sma_sum;
        let mut moments_sum = core.moments_sum;
        let mut moments_sum_squares = core.moments_sum_squares;
        let period_f = core.period as f64;
        let inverse_period = core.inverse_period;
        let (deviations_up, deviations_down) = (self.deviations_up, self.deviations_down);
        let last = bbands_steady_loop(
            inputs,
            period,
            [&mut sma_sum, &mut moments_sum, &mut moments_sum_squares],
            [period_f, inverse_period, deviations_up, deviations_down],
            upper_out,
            middle_out,
            lower_out,
        );
        core.sma_sum = sma_sum;
        core.moments_sum = moments_sum;
        core.moments_sum_squares = moments_sum_squares;
        // Rebuild the ring so subsequent appends continue bit-identically.
        core.window.clear();
        for &input in &inputs[n - period..] {
            core.window.push(input);
        }
        self.value = Some(last);
    }
}

impl StreamingIndicator for BollingerBands {
    type Output = BollingerBandsValue;

    fn append(&mut self, input: f64) -> Option<BollingerBandsValue> {
        let outputs = match &mut self.core {
            BollingerBandsCore::Sma(core) => core.append(input),
            BollingerBandsCore::Dispatch { middle, deviation } => {
                let middle = middle.append(input);
                let deviation = deviation.append(input);
                middle.zip(deviation)
            }
        };
        self.value = outputs.map(|(middle, deviation)| self.bands(middle, deviation));
        self.value
    }

    fn value(&self) -> Option<BollingerBandsValue> {
        self.value
    }

    fn reset(&mut self) {
        match &mut self.core {
            BollingerBandsCore::Sma(core) => core.reset(),
            BollingerBandsCore::Dispatch { middle, deviation } => {
                middle.reset();
                deviation.reset();
            }
        }
        self.value = None;
    }
}
