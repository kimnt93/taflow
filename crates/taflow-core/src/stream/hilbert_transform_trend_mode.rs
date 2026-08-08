//! Batch implementation for `hilbert_transform_trend_mode`.

use super::cycle::*;
use crate::error::{TaError, TaResult};

/// HT_TRENDMODE - Hilbert Transform - Trend vs Cycle Mode
///
/// Returns 1 (trend) or 0 (cycle).
/// Compute the hilbert transform trend mode result for the supplied aligned series.
///
/// # Parameters
///
/// * `input` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn hilbert_transform_trend_mode(input: &[f64]) -> TaResult<Vec<i32>> {
    let len = input.len();
    let lookback: usize = 63;

    if len <= lookback {
        return Err(TaError::InsufficientData {
            need: lookback + 1,
            got: len,
        });
    }

    // We need to replicate C TA-Lib exactly, including the trendline computation
    // and the smoothPrice circular buffer access. We'll do a full inline computation
    // rather than reusing ht_dc_phase_core, because trendmode needs:
    // - smoothPrice buffer for trendline comparison
    // - iTrend1/2/3 for WMA trendline
    // - prevDCPhase for phase rate-of-change check

    let start_idx = lookback;

    let trailing_wma_start = start_idx - lookback;
    let (mut wma, mut today) = WmaState::init(input, trailing_wma_start);

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

    let mut smooth_price = [0.0_f64; SMOOTH_PRICE_SIZE];
    let mut smooth_price_idx: usize = 0;

    let mut dc_phase: f64 = 0.0;
    #[allow(unused_assignments)]
    let mut prev_dc_phase: f64 = 0.0;

    // Trend mode specific variables
    let mut i_trend1: f64 = 0.0;
    let mut i_trend2: f64 = 0.0;
    let mut i_trend3: f64 = 0.0;
    let mut days_in_trend: i32 = 0;
    #[allow(unused_assignments)]
    let mut prev_sine: f64 = 0.0;
    #[allow(unused_assignments)]
    let mut prev_lead_sine: f64 = 0.0;
    let mut sine: f64 = 0.0;
    let mut lead_sine: f64 = 0.0;

    let mut output = vec![0_i32; len];

    while today < len {
        let adjusted_prev_period = 0.075 * period + 0.54;

        let today_value = input[today];
        let smoothed_value = wma.next(input, today_value);

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
        prev_dc_phase = dc_phase;
        let dc_period = smooth_period + 0.5;
        let dc_period_int = dc_period as i32;
        let mut real_part = 0.0_f64;
        let mut imag_part = 0.0_f64;

        let mut idx = smooth_price_idx;
        for &(sin_angle, cos_angle) in dc_sin_cos(dc_period_int.max(0) as usize) {
            let price = smooth_price[idx];
            real_part += sin_angle * price;
            imag_part += cos_angle * price;
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

        dc_phase += 360.0 / smooth_period;
        if imag_part < 0.0 {
            dc_phase += 180.0;
        }
        if dc_phase > 315.0 {
            dc_phase -= 360.0;
        }

        prev_sine = sine;
        prev_lead_sine = lead_sine;
        sine = (dc_phase * DEG2RAD).sin();
        lead_sine = ((dc_phase + 45.0) * DEG2RAD).sin();

        // Compute Trendline
        let dc_period2 = smooth_period + 0.5;
        let dc_period_int2 = dc_period2 as i32;

        let mut temp = 0.0_f64;
        let mut price_idx = today;
        for _ in 0..dc_period_int2 {
            temp += input[price_idx];
            if price_idx == 0 {
                break;
            }
            price_idx -= 1;
        }

        if dc_period_int2 > 0 {
            temp /= dc_period_int2 as f64;
        }

        let trendline = (4.0 * temp + 3.0 * i_trend1 + 2.0 * i_trend2 + i_trend3) / 10.0;
        i_trend3 = i_trend2;
        i_trend2 = i_trend1;
        i_trend1 = temp;

        // Compute trend mode (assume trend by default)
        let mut trend = 1_i32;

        // Measure days in trend from last crossing of SineWave indicator lines
        if (sine > lead_sine && prev_sine <= prev_lead_sine)
            || (sine < lead_sine && prev_sine >= prev_lead_sine)
        {
            days_in_trend = 0;
            trend = 0;
        }

        days_in_trend += 1;

        if (days_in_trend as f64) < 0.5 * smooth_period {
            trend = 0;
        }

        let phase_change = dc_phase - prev_dc_phase;
        if smooth_period != 0.0
            && phase_change > 0.67 * 360.0 / smooth_period
            && phase_change < 1.5 * 360.0 / smooth_period
        {
            trend = 0;
        }

        let current_smooth_price = smooth_price[smooth_price_idx];
        if trendline != 0.0 && ((current_smooth_price - trendline) / trendline).abs() >= 0.015 {
            trend = 1;
        }

        if today >= start_idx {
            output[today] = trend;
        }

        // Advance circular buffer
        smooth_price_idx += 1;
        if smooth_price_idx >= SMOOTH_PRICE_SIZE {
            smooth_price_idx = 0;
        }

        today += 1;
    }

    Ok(output)
}
