// Hilbert Transform Cycle Indicators
//
// Faithful port of C TA-Lib Hilbert Transform indicators.
// All 5 indicators (HT_DCPERIOD, HT_DCPHASE, HT_PHASOR, HT_SINE, HT_TRENDMODE)
// use the same even/odd alternating Hilbert transform algorithm.
//
// Key implementation details matching C TA-Lib:
// - WMA(4) price smoother using running periodWMASub/periodWMASum
// - Even/odd alternating Hilbert transform with 3-element circular buffers
// - I1 = detrender delayed 3 bars via I1ForOddPrev2/3 and I1ForEvenPrev2/3
// - Period = 360 / (atan(Im/Re) * rad2Deg)
// - DC Phase uses circular buffer of 50 smoothed prices

use crate::error::{TaError, TaResult};

pub use super::hilbert_transform_dominant_cycle_period::hilbert_transform_dominant_cycle_period;
pub use super::hilbert_transform_dominant_cycle_phase::hilbert_transform_dominant_cycle_phase;
pub use super::hilbert_transform_phasor::hilbert_transform_phasor;
pub use super::hilbert_transform_sine_wave::hilbert_transform_sine_wave;
pub use super::hilbert_transform_trend_mode::hilbert_transform_trend_mode;

pub(crate) const RAD2DEG: f64 = 180.0 / std::f64::consts::PI; // 45.0 / atan(1)
pub(crate) const DEG2RAD: f64 = std::f64::consts::PI / 180.0;
pub(crate) const CONST_DEG2RAD_BY360: f64 = 2.0 * std::f64::consts::PI; // atan(1) * 8

const A: f64 = 0.0962;
const B: f64 = 0.5769;

pub(crate) const SMOOTH_PRICE_SIZE: usize = 50;

// ============================================================
// Hilbert Transform variables for one signal (even/odd buffers)
// ============================================================

pub(crate) struct HilbertVars {
    pub(crate) odd: [f64; 3],
    pub(crate) even: [f64; 3],
    pub(crate) prev_odd: f64,
    pub(crate) prev_even: f64,
    pub(crate) prev_input_odd: f64,
    pub(crate) prev_input_even: f64,
}

impl HilbertVars {
    pub(crate) fn new() -> Self {
        Self {
            odd: [0.0; 3],
            even: [0.0; 3],
            prev_odd: 0.0,
            prev_even: 0.0,
            prev_input_odd: 0.0,
            prev_input_even: 0.0,
        }
    }
}

/// Perform the Hilbert transform for an even bar.
/// Returns the transformed value.
#[inline(always)]
pub(crate) fn do_hilbert_even(
    vars: &mut HilbertVars,
    input: f64,
    hilbert_idx: usize,
    adjusted_prev_period: f64,
) -> f64 {
    let hilbert_temp_real = A * input;
    let mut result = -vars.even[hilbert_idx];
    vars.even[hilbert_idx] = hilbert_temp_real;
    result += hilbert_temp_real;
    result -= vars.prev_even;
    vars.prev_even = B * vars.prev_input_even;
    result += vars.prev_even;
    vars.prev_input_even = input;
    result *= adjusted_prev_period;
    result
}

/// Perform the Hilbert transform for an odd bar.
/// Returns the transformed value.
#[inline(always)]
pub(crate) fn do_hilbert_odd(
    vars: &mut HilbertVars,
    input: f64,
    hilbert_idx: usize,
    adjusted_prev_period: f64,
) -> f64 {
    let hilbert_temp_real = A * input;
    let mut result = -vars.odd[hilbert_idx];
    vars.odd[hilbert_idx] = hilbert_temp_real;
    result += hilbert_temp_real;
    result -= vars.prev_odd;
    vars.prev_odd = B * vars.prev_input_odd;
    result += vars.prev_odd;
    vars.prev_input_odd = input;
    result *= adjusted_prev_period;
    result
}

// ============================================================
// WMA price smoother (running implementation matching C TA-Lib)
// ============================================================

pub(crate) struct WmaState {
    pub(crate) period_wma_sub: f64,
    pub(crate) period_wma_sum: f64,
    pub(crate) trailing_wma_value: f64,
    pub(crate) trailing_wma_idx: usize,
}

impl WmaState {
    /// Initialize WMA with first 3 prices (unrolled, matching C TA-Lib).
    /// `start` is the index of the first price bar.
    pub(crate) fn init(input: &[f64], start: usize) -> (Self, usize) {
        // Match C TA-Lib's exact step-by-step accumulation order.
        // Floating-point addition is NOT associative; the order matters
        // for bit-identical results over hundreds of running iterations.
        let p0 = input[start];
        let p1 = input[start + 1];
        let p2 = input[start + 2];

        let mut period_wma_sub = p0;
        period_wma_sub += p1;
        period_wma_sub += p2;

        let mut period_wma_sum = p0;
        period_wma_sum += p1 * 2.0;
        period_wma_sum += p2 * 3.0;

        let state = WmaState {
            period_wma_sub,
            period_wma_sum,
            trailing_wma_value: 0.0,
            trailing_wma_idx: start,
        };
        (state, start + 3) // next today index
    }

    /// Compute next smoothed value (DO_PRICE_WMA macro equivalent).
    #[inline(always)]
    pub(crate) fn next(&mut self, input: &[f64], new_price: f64) -> f64 {
        self.period_wma_sub += new_price;
        self.period_wma_sub -= self.trailing_wma_value;
        self.period_wma_sum += new_price * 4.0;
        self.trailing_wma_value = input[self.trailing_wma_idx];
        self.trailing_wma_idx += 1;
        let smoothed = self.period_wma_sum * 0.1;
        self.period_wma_sum -= self.period_wma_sub;
        smoothed
    }
}

// ============================================================
// HT_DCPERIOD
// ============================================================

// ============================================================
// HT_PHASOR
// ============================================================

// ============================================================
// Common core for HT_DCPHASE, HT_SINE, HT_TRENDMODE
// (all require smoothPeriod + DCPhase + smoothPrice circular buffer)
// ============================================================

/// Results from the DC phase core computation.
pub(crate) struct DcPhaseResult {
    /// dc_phase[i] for each bar (NAN before lookback)
    pub(crate) dc_phase: Vec<f64>,
    /// The index of the first valid output
    pub(crate) first_valid: usize,
}

/// Core computation shared by HT_DCPHASE, HT_SINE, HT_TRENDMODE.
/// lookback = 63 (31 skip + 32 warmup).
pub(crate) fn ht_dc_phase_core(input: &[f64]) -> DcPhaseResult {
    let len = input.len();
    let lookback: usize = 63;
    let start_idx = lookback;

    let mut dc_phase_out = vec![0.0_f64; len];
    dc_phase_out[..lookback.min(len)].fill(f64::NAN);

    if len <= lookback {
        return DcPhaseResult {
            dc_phase: dc_phase_out,
            first_valid: len, // no valid output
        };
    }

    let trailing_wma_start = start_idx - lookback; // = 0
    let (mut wma, mut today) = WmaState::init(input, trailing_wma_start);

    // Warm up WMA: 34 iterations for DC phase functions
    // (63 - 32 + 3 = 34, accounts for the extra 31 skip)
    for _ in 0..34 {
        let val = input[today];
        today += 1;
        let _ = wma.next(input, val);
    }

    let mut hilbert_idx: usize = 0;
    let mut detrender_vars = HilbertVars::new();
    let mut q1_vars = HilbertVars::new();
    let mut ji_vars = HilbertVars::new();
    let mut jq_vars = HilbertVars::new();

    let mut period: f64 = 0.0;
    let mut smooth_period: f64 = 0.0;

    let mut prev_i2: f64 = 0.0;
    let mut prev_q2: f64 = 0.0;
    let mut re: f64 = 0.0;
    let mut im: f64 = 0.0;

    let mut i1_for_odd_prev2: f64 = 0.0;
    let mut i1_for_odd_prev3: f64 = 0.0;
    let mut i1_for_even_prev2: f64 = 0.0;
    let mut i1_for_even_prev3: f64 = 0.0;

    // Circular buffer for smooth prices
    let mut smooth_price = [0.0_f64; SMOOTH_PRICE_SIZE];
    let mut smooth_price_idx: usize = 0;

    let mut dc_phase: f64 = 0.0;

    while today < len {
        let adjusted_prev_period = 0.075 * period + 0.54;

        let today_value = input[today];
        let smoothed_value = wma.next(input, today_value);

        // Store in circular buffer
        smooth_price[smooth_price_idx] = smoothed_value;

        let (detrender, q1, i2, q2);

        if today % 2 == 0 {
            detrender = do_hilbert_even(
                &mut detrender_vars,
                smoothed_value,
                hilbert_idx,
                adjusted_prev_period,
            );
            q1 = do_hilbert_even(&mut q1_vars, detrender, hilbert_idx, adjusted_prev_period);
            let _ji = do_hilbert_even(
                &mut ji_vars,
                i1_for_even_prev3,
                hilbert_idx,
                adjusted_prev_period,
            );
            let _jq = do_hilbert_even(&mut jq_vars, q1, hilbert_idx, adjusted_prev_period);
            hilbert_idx += 1;
            if hilbert_idx == 3 {
                hilbert_idx = 0;
            }

            q2 = 0.2 * (q1 + _ji) + 0.8 * prev_q2;
            i2 = 0.2 * (i1_for_even_prev3 - _jq) + 0.8 * prev_i2;

            i1_for_odd_prev3 = i1_for_odd_prev2;
            i1_for_odd_prev2 = detrender;
        } else {
            detrender = do_hilbert_odd(
                &mut detrender_vars,
                smoothed_value,
                hilbert_idx,
                adjusted_prev_period,
            );
            q1 = do_hilbert_odd(&mut q1_vars, detrender, hilbert_idx, adjusted_prev_period);
            let _ji = do_hilbert_odd(
                &mut ji_vars,
                i1_for_odd_prev3,
                hilbert_idx,
                adjusted_prev_period,
            );
            let _jq = do_hilbert_odd(&mut jq_vars, q1, hilbert_idx, adjusted_prev_period);

            q2 = 0.2 * (q1 + _ji) + 0.8 * prev_q2;
            i2 = 0.2 * (i1_for_odd_prev3 - _jq) + 0.8 * prev_i2;

            i1_for_even_prev3 = i1_for_even_prev2;
            i1_for_even_prev2 = detrender;
        }

        // Adjust period
        re = 0.2 * (i2 * prev_i2 + q2 * prev_q2) + 0.8 * re;
        im = 0.2 * (i2 * prev_q2 - q2 * prev_i2) + 0.8 * im;
        prev_q2 = q2;
        prev_i2 = i2;

        let temp_real = period;
        if im != 0.0 && re != 0.0 {
            period = 360.0 / ((im / re).atan() * RAD2DEG);
        }
        let temp_real2 = 1.5 * temp_real;
        if period > temp_real2 {
            period = temp_real2;
        }
        let temp_real2 = 0.67 * temp_real;
        if period < temp_real2 {
            period = temp_real2;
        }
        if period < 6.0 {
            period = 6.0;
        } else if period > 50.0 {
            period = 50.0;
        }
        period = 0.2 * period + 0.8 * temp_real;

        smooth_period = 0.33 * period + 0.67 * smooth_period;

        // Compute DC Phase
        let dc_period = smooth_period + 0.5;
        let dc_period_int = dc_period as i32;
        let mut real_part = 0.0_f64;
        let mut imag_part = 0.0_f64;

        let mut idx = smooth_price_idx;
        for i in 0..dc_period_int {
            let angle = (i as f64 * CONST_DEG2RAD_BY360) / dc_period_int as f64;
            let price = smooth_price[idx];
            real_part += angle.sin() * price;
            imag_part += angle.cos() * price;
            if idx == 0 {
                idx = SMOOTH_PRICE_SIZE - 1;
            } else {
                idx -= 1;
            }
        }

        let abs_imag = imag_part.abs();
        if abs_imag > 0.0 {
            dc_phase = (real_part / imag_part).atan() * RAD2DEG;
        } else if abs_imag <= 0.01 {
            if real_part < 0.0 {
                dc_phase -= 90.0;
            } else if real_part > 0.0 {
                dc_phase += 90.0;
            }
        }
        dc_phase += 90.0;

        // Compensate for one bar lag of the weighted moving average
        dc_phase += 360.0 / smooth_period;
        if imag_part < 0.0 {
            dc_phase += 180.0;
        }
        if dc_phase > 315.0 {
            dc_phase -= 360.0;
        }

        if today >= start_idx {
            dc_phase_out[today] = dc_phase;
        }

        // Advance circular buffer
        smooth_price_idx += 1;
        if smooth_price_idx >= SMOOTH_PRICE_SIZE {
            smooth_price_idx = 0;
        }

        today += 1;
    }

    DcPhaseResult {
        dc_phase: dc_phase_out,
        first_valid: start_idx,
    }
}

// ============================================================
// HT_DCPHASE
// ============================================================

// ============================================================
// HT_SINE
// ============================================================

// ============================================================
// HT_TRENDMODE
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// Generate test data with periodic fluctuation
    fn make_test_data(n: usize) -> Vec<f64> {
        (0..n)
            .map(|i| 50.0 + 10.0 * (i as f64 * 0.2).sin())
            .collect()
    }

    #[test]
    fn test_ht_dcperiod_basic() {
        let input = make_test_data(200);
        let result = hilbert_transform_dominant_cycle_period(&input).unwrap();
        assert!(result[31].is_nan());
        assert!(!result[32].is_nan());
        for i in 32..200 {
            assert!(
                result[i] >= 6.0 && result[i] <= 50.0,
                "period out of range at {}: {}",
                i,
                result[i]
            );
        }
    }

    #[test]
    fn test_ht_dcperiod_insufficient_data() {
        let input = vec![1.0; 32];
        assert!(hilbert_transform_dominant_cycle_period(&input).is_err());
    }

    #[test]
    fn test_ht_dcphase_basic() {
        let input = make_test_data(200);
        let result = hilbert_transform_dominant_cycle_phase(&input).unwrap();
        assert!(result[62].is_nan());
        assert!(!result[63].is_nan());
    }

    #[test]
    fn test_ht_dcphase_insufficient_data() {
        let input = vec![1.0; 63];
        assert!(hilbert_transform_dominant_cycle_phase(&input).is_err());
    }

    #[test]
    fn test_ht_phasor_basic() {
        let input = make_test_data(200);
        let (inphase, quadrature) = hilbert_transform_phasor(&input).unwrap();
        assert!(inphase[31].is_nan());
        assert!(!inphase[32].is_nan());
        assert!(quadrature[31].is_nan());
        assert!(!quadrature[32].is_nan());
    }

    #[test]
    fn test_ht_phasor_insufficient_data() {
        let input = vec![1.0; 32];
        assert!(hilbert_transform_phasor(&input).is_err());
    }

    #[test]
    fn test_ht_sine_basic() {
        let input = make_test_data(200);
        let (sine, leadsine) = hilbert_transform_sine_wave(&input).unwrap();
        assert!(sine[62].is_nan());
        assert!(!sine[63].is_nan());
        assert!(!leadsine[63].is_nan());
        for i in 63..200 {
            assert!(
                sine[i] >= -1.0 && sine[i] <= 1.0,
                "sine out of range at {}: {}",
                i,
                sine[i]
            );
            assert!(
                leadsine[i] >= -1.0 && leadsine[i] <= 1.0,
                "leadsine out of range at {}: {}",
                i,
                leadsine[i]
            );
        }
    }

    #[test]
    fn test_ht_sine_insufficient_data() {
        let input = vec![1.0; 63];
        assert!(hilbert_transform_sine_wave(&input).is_err());
    }

    #[test]
    fn test_ht_trendmode_basic() {
        let input = make_test_data(200);
        let result = hilbert_transform_trend_mode(&input).unwrap();
        for i in 0..63 {
            assert_eq!(result[i], 0);
        }
        for i in 63..200 {
            assert!(
                result[i] == 0 || result[i] == 1,
                "trendmode must be 0 or 1 at {}",
                i
            );
        }
    }

    #[test]
    fn test_ht_trendmode_insufficient_data() {
        let input = vec![1.0; 63];
        assert!(hilbert_transform_trend_mode(&input).is_err());
    }
}
