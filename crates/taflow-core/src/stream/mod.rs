//! Persistent technical indicators for bulk history and realtime continuation.
//!
//! Each TA implementation lives in its own module and retains only the bounded
//! recurrence state required to process newly appended bars.

#[cfg(test)]
pub(crate) mod tests_extrema_support {
    //! Shared adversarial datasets for the extrema-family bit-exactness tests.

    /// Random, monotonic increasing/decreasing, constant, and quantized
    /// (repeated equal extremes) series of the requested length.
    pub(crate) fn datasets(len: usize) -> Vec<Vec<f64>> {
        let mut state = 0x9E3779B97F4A7C15_u64;
        let random: Vec<f64> = (0..len)
            .map(|_| {
                state = state
                    .wrapping_mul(6364136223846793005)
                    .wrapping_add(1442695040888963407);
                ((state >> 33) % 997) as f64 / 7.0
            })
            .collect();
        let increasing: Vec<f64> = (0..len).map(|i| i as f64 * 0.5).collect();
        let decreasing: Vec<f64> = (0..len).map(|i| (len as f64) - i as f64 * 0.5).collect();
        let constant = vec![13.25_f64; len];
        let quantized: Vec<f64> = (0..len).map(|i| ((i * 7) % 5) as f64).collect();
        vec![random, increasing, decreasing, constant, quantized]
    }

    /// Periods 2/5/30/200 crossed with lengths 0, 1, p-1, p, p+1, 10_000.
    pub(crate) fn periods_and_lengths() -> Vec<(usize, usize)> {
        let mut cases = Vec::new();
        for &period in &[2usize, 5, 30, 200] {
            for &len in &[0usize, 1, period - 1, period, period + 1, 10_000] {
                cases.push((period, len));
            }
        }
        cases
    }
}

mod absolute_price_oscillator;
#[cfg(test)]
mod absolute_price_oscillator_test;
mod acceleration_bands;
#[cfg(test)]
mod acceleration_bands_test;
mod aroon;
mod aroon_rescan;
#[cfg(test)]
mod aroon_test;
mod average_directional_index;
mod average_directional_index_rating;
#[cfg(test)]
mod average_directional_index_rating_test;
#[cfg(test)]
mod average_directional_index_test;
mod bollinger_bands;
#[cfg(test)]
mod bollinger_bands_test;
mod candle_abandoned_baby;
#[cfg(test)]
mod candle_abandoned_baby_test;
mod candle_advance_block;
#[cfg(test)]
mod candle_advance_block_test;
mod candle_belt_hold;
#[cfg(test)]
mod candle_belt_hold_test;
mod candle_breakaway;
#[cfg(test)]
mod candle_breakaway_test;
mod candle_closing_marubozu;
#[cfg(test)]
mod candle_closing_marubozu_test;
mod candle_conceal_baby_swall;
#[cfg(test)]
mod candle_conceal_baby_swall_test;
mod candle_counter_attack;
#[cfg(test)]
mod candle_counter_attack_test;
mod candle_dark_cloud_cover;
#[cfg(test)]
mod candle_dark_cloud_cover_test;
mod candle_doji;
mod candle_doji_star;
#[cfg(test)]
mod candle_doji_star_test;
#[cfg(test)]
mod candle_doji_test;
mod candle_dragonfly_doji;
#[cfg(test)]
mod candle_dragonfly_doji_test;
mod candle_engulfing;
mod candle_evening_doji_star;
#[cfg(test)]
mod candle_evening_doji_star_test;
mod candle_evening_star;
#[cfg(test)]
mod candle_evening_star_test;
mod candle_gap_side_side_white;
#[cfg(test)]
mod candle_gap_side_side_white_test;
mod candle_gravestone_doji;
#[cfg(test)]
mod candle_gravestone_doji_test;
mod candle_hammer;
#[cfg(test)]
mod candle_hammer_test;
mod candle_hanging_man;
#[cfg(test)]
mod candle_hanging_man_test;
mod candle_harami;
mod candle_harami_cross;
#[cfg(test)]
mod candle_harami_cross_test;
#[cfg(test)]
mod candle_harami_test;
mod candle_high_wave;
#[cfg(test)]
mod candle_high_wave_test;
mod candle_hikkake;
mod candle_hikkake_modified;
#[cfg(test)]
mod candle_hikkake_modified_test;
#[cfg(test)]
mod candle_hikkake_test;
mod candle_homing_pigeon;
#[cfg(test)]
mod candle_homing_pigeon_test;
mod candle_identical_three_crows;
#[cfg(test)]
mod candle_identical_three_crows_test;
mod candle_in_neck;
#[cfg(test)]
mod candle_in_neck_test;
mod candle_inverted_hammer;
#[cfg(test)]
mod candle_inverted_hammer_test;
mod candle_kicking;
mod candle_kicking_by_length;
#[cfg(test)]
mod candle_kicking_by_length_test;
mod candle_ladder_bottom;
#[cfg(test)]
mod candle_ladder_bottom_test;
mod candle_long_legged_doji;
#[cfg(test)]
mod candle_long_legged_doji_test;
mod candle_long_line;
#[cfg(test)]
mod candle_long_line_test;
mod candle_marubozu;
#[cfg(test)]
mod candle_marubozu_test;
mod candle_mat_hold;
#[cfg(test)]
mod candle_mat_hold_test;
mod candle_matching_low;
#[cfg(test)]
mod candle_matching_low_test;
mod candle_morning_doji_star;
#[cfg(test)]
mod candle_morning_doji_star_test;
mod candle_morning_star;
#[cfg(test)]
mod candle_morning_star_test;
mod candle_on_neck;
#[cfg(test)]
mod candle_on_neck_test;
mod candle_piercing;
#[cfg(test)]
mod candle_piercing_test;
mod candle_rickshawman;
#[cfg(test)]
mod candle_rickshawman_test;
mod candle_rise_fall_three_methods;
#[cfg(test)]
mod candle_rise_fall_three_methods_test;
mod candle_separating_lines;
#[cfg(test)]
mod candle_separating_lines_test;
mod candle_shooting_star;
#[cfg(test)]
mod candle_shooting_star_test;
mod candle_short_line;
#[cfg(test)]
mod candle_short_line_test;
mod candle_spinning_top;
#[cfg(test)]
mod candle_spinning_top_test;
mod candle_stalled_pattern;
#[cfg(test)]
mod candle_stalled_pattern_test;
mod candle_stick_sandwich;
#[cfg(test)]
mod candle_stick_sandwich_test;
mod candle_takuri;
mod candle_tasuki_gap;
#[cfg(test)]
mod candle_tasuki_gap_test;
mod candle_three_black_crows;
#[cfg(test)]
mod candle_three_black_crows_test;
mod candle_three_inside;
#[cfg(test)]
mod candle_three_inside_test;
mod candle_three_line_strike;
#[cfg(test)]
mod candle_three_line_strike_test;
mod candle_three_outside;
#[cfg(test)]
mod candle_three_outside_test;
mod candle_three_stars_in_south;
#[cfg(test)]
mod candle_three_stars_in_south_test;
mod candle_three_white_soldiers;
#[cfg(test)]
mod candle_three_white_soldiers_test;
mod candle_thrusting;
#[cfg(test)]
mod candle_thrusting_test;
mod candle_tri_star;
#[cfg(test)]
mod candle_tri_star_test;
mod candle_two_crows;
#[cfg(test)]
mod candle_two_crows_test;
mod candle_unique_three_river;
#[cfg(test)]
mod candle_unique_three_river_test;
mod candle_up_down_side_gap_three_methods;
#[cfg(test)]
mod candle_up_down_side_gap_three_methods_test;
mod candle_upside_gap_two_crows;
#[cfg(test)]
mod candle_upside_gap_two_crows_test;
mod chande_momentum_oscillator;
#[cfg(test)]
mod chande_momentum_oscillator_test;
mod commodity_channel_index;
#[cfg(test)]
mod commodity_channel_index_test;
mod cycle;
mod directional;
mod directional_movement_index;
#[cfg(test)]
mod directional_movement_index_test;
mod double_exponential_moving_average;
#[cfg(test)]
mod double_exponential_moving_average_test;
mod exponential_moving_average;
#[cfg(test)]
mod exponential_moving_average_test;
mod hilbert_transform_dominant_cycle_period;
#[cfg(test)]
mod hilbert_transform_dominant_cycle_period_test;
mod hilbert_transform_dominant_cycle_phase;
#[cfg(test)]
mod hilbert_transform_dominant_cycle_phase_test;
mod hilbert_transform_phasor;
#[cfg(test)]
mod hilbert_transform_phasor_test;
mod hilbert_transform_sine_wave;
#[cfg(test)]
mod hilbert_transform_sine_wave_test;
mod hilbert_transform_trend_mode;
#[cfg(test)]
mod hilbert_transform_trend_mode_test;
mod hilbert_transform_trendline;
#[cfg(test)]
mod hilbert_transform_trendline_test;
mod indicator;
mod intraday_momentum_index;
#[cfg(test)]
mod intraday_momentum_index_test;
mod kaufman_adaptive_moving_average;
#[cfg(test)]
mod kaufman_adaptive_moving_average_test;
mod mama;
mod math_abs;
mod math_operator;
mod moving_average;
mod moving_average_convergence_divergence;
mod moving_average_convergence_divergence_extended;
#[cfg(test)]
mod moving_average_convergence_divergence_extended_test;
mod moving_average_convergence_divergence_fixed;
#[cfg(test)]
mod moving_average_convergence_divergence_fixed_test;
mod moving_average_convergence_divergence_helpers;
#[cfg(test)]
mod moving_average_convergence_divergence_test;
pub use math_abs::MathAbs;
#[cfg(test)]
mod math_abs_test;
mod math_acos;
pub use math_acos::MathAcos;
#[cfg(test)]
mod math_acos_test;
mod math_acosh;
pub use math_acosh::MathAcosh;
#[cfg(test)]
mod math_acosh_test;
mod math_asin;
pub use math_asin::MathAsin;
#[cfg(test)]
mod math_asin_test;
mod math_asinh;
pub use math_asinh::MathAsinh;
#[cfg(test)]
mod math_asinh_test;
mod math_atan;
pub use math_atan::MathAtan;
#[cfg(test)]
mod math_atan_test;
mod math_atanh;
pub use math_atanh::MathAtanh;
#[cfg(test)]
mod math_atanh_test;
mod math_cbrt;
pub use math_cbrt::MathCbrt;
#[cfg(test)]
mod math_cbrt_test;
mod math_ceil;
pub use math_ceil::MathCeil;
#[cfg(test)]
mod math_ceil_test;
mod math_cos;
pub use math_cos::MathCos;
#[cfg(test)]
mod math_cos_test;
mod math_cosh;
pub use math_cosh::MathCosh;
#[cfg(test)]
mod math_cosh_test;
mod math_cot;
pub use math_cot::MathCot;
#[cfg(test)]
mod math_cot_test;
mod math_degrees;
pub use math_degrees::MathDegrees;
#[cfg(test)]
mod math_degrees_test;
mod math_exp;
pub use math_exp::MathExp;
#[cfg(test)]
mod math_exp_test;
mod math_floor;
pub use math_floor::MathFloor;
#[cfg(test)]
mod math_floor_test;
mod math_ln;
pub use math_ln::MathLn;
#[cfg(test)]
mod math_ln_test;
mod math_log10;
pub use math_log10::MathLog10;
#[cfg(test)]
mod math_log10_test;
mod math_log1p;
pub use math_log1p::MathLog1p;
#[cfg(test)]
mod math_log1p_test;
mod math_radians;
pub use math_radians::MathRadians;
#[cfg(test)]
mod math_radians_test;
mod math_sin;
pub use math_sin::MathSin;
#[cfg(test)]
mod math_sin_test;
mod math_sinh;
pub use math_sinh::MathSinh;
#[cfg(test)]
mod math_sinh_test;
mod math_sqrt;
pub use math_sqrt::MathSqrt;
#[cfg(test)]
mod math_sqrt_test;
mod math_tan;
pub use math_tan::MathTan;
#[cfg(test)]
mod math_tan_test;
mod math_tanh;
pub use math_tanh::MathTanh;
mod accumulation_distribution_helper;
#[cfg(test)]
mod math_tanh_test;
mod minus_directional_indicator;
#[cfg(test)]
mod minus_directional_indicator_test;
mod minus_directional_movement;
#[cfg(test)]
mod minus_directional_movement_test;
mod money_flow_index;
#[cfg(test)]
mod money_flow_index_test;
mod moving_average_dispatcher;
#[cfg(test)]
mod moving_average_test;
mod pattern;
mod percentage_price_oscillator;
#[cfg(test)]
mod percentage_price_oscillator_test;
mod plus_directional_indicator;
#[cfg(test)]
mod plus_directional_indicator_test;
mod plus_directional_movement;
#[cfg(test)]
mod plus_directional_movement_test;
mod price_transform;
mod regression;
mod relative_strength_index;
#[cfg(test)]
mod relative_strength_index_test;
mod rolling_extrema;
mod rolling_median;
#[cfg(test)]
mod rolling_median_test;
mod rolling_mode;
mod rolling_price;
mod rolling_statistics;
mod rolling_sum;
#[cfg(test)]
mod rolling_sum_test;
mod session_flags;
pub(crate) mod sorted_ring;
mod statistic;
mod variable_period_moving_average;
#[cfg(test)]
mod variable_period_moving_average_test;
mod vhgw;
pub use session_flags::session_flags;
mod cumulative_count;
#[cfg(test)]
mod cumulative_count_test;
mod cumulative_maximum;
#[cfg(test)]
mod cumulative_maximum_test;
mod cumulative_minimum;
#[cfg(test)]
mod cumulative_minimum_test;
mod cumulative_product;
#[cfg(test)]
mod cumulative_product_test;
mod cumulative_sum;
#[cfg(test)]
mod cumulative_sum_test;
#[allow(unused_imports)]
pub use cumulative_count::CumulativeCount;
pub use cumulative_maximum::CumulativeMaximum;
pub use cumulative_minimum::CumulativeMinimum;
pub use cumulative_product::CumulativeProduct;
pub use cumulative_sum::CumulativeSum;
mod anchored_volume_weighted_average_price;
#[cfg(test)]
mod anchored_volume_weighted_average_price_test;
mod even_better_sinewave;
#[cfg(test)]
mod even_better_sinewave_test;
mod fibonacci_retracement;
#[cfg(test)]
mod fibonacci_retracement_test;
mod heikin_ashi;
#[cfg(test)]
mod heikin_ashi_test;
mod helpers;
mod jurik_moving_average;
#[cfg(test)]
mod jurik_moving_average_test;
mod klinger_volume_oscillator;
#[cfg(test)]
mod klinger_volume_oscillator_test;
mod lag;
#[cfg(test)]
mod lag_test;
mod lagged_common;
mod laguerre_relative_strength_index;
#[cfg(test)]
mod laguerre_relative_strength_index_test;
mod log_return;
#[cfg(test)]
mod log_return_test;
mod momentum;
#[cfg(test)]
mod momentum_test;
mod opening_range;
#[cfg(test)]
mod opening_range_test;
mod operator_states;
pub use operator_states::ActiveZoneList;
mod fast_stochastic_oscillator;
#[cfg(test)]
mod fast_stochastic_oscillator_test;
mod parabolic_moving_average_stop;
#[cfg(test)]
mod parabolic_moving_average_stop_test;
mod parabolic_sar;
mod parabolic_sar_extended;
#[cfg(test)]
mod parabolic_sar_extended_test;
#[cfg(test)]
mod parabolic_sar_test;
mod pivot_points;
#[cfg(test)]
mod pivot_points_test;
mod premium_discount;
#[cfg(test)]
mod premium_discount_test;
mod rate_of_change;
mod rate_of_change_percent;
#[cfg(test)]
mod rate_of_change_percent_test;
mod rate_of_change_ratio;
mod rate_of_change_ratio_percent;
#[cfg(test)]
mod rate_of_change_ratio_percent_test;
#[cfg(test)]
mod rate_of_change_ratio_test;
#[cfg(test)]
mod rate_of_change_test;
mod relative_momentum_index;
#[cfg(test)]
mod relative_momentum_index_test;
mod session_volume_levels;
#[cfg(test)]
mod session_volume_levels_test;
mod simple_moving_average;
#[cfg(test)]
mod simple_moving_average_test;
mod smoothed_trend_channel;
#[cfg(test)]
mod smoothed_trend_channel_test;
mod stochastic_oscillator;
#[cfg(test)]
mod stochastic_oscillator_test;
mod stochastic_relative_strength_index;
#[cfg(test)]
mod stochastic_relative_strength_index_test;
mod tom_de_mark_sequential;
#[cfg(test)]
mod tom_de_mark_sequential_test;
mod triangular_moving_average;
#[cfg(test)]
mod triangular_moving_average_test;
mod triple_exponential_average;
#[cfg(test)]
mod triple_exponential_average_test;
mod triple_exponential_moving_average;
#[cfg(test)]
mod triple_exponential_moving_average_test;
mod triple_exponential_rate_of_change;
#[cfg(test)]
mod triple_exponential_rate_of_change_test;
mod ultimate_oscillator;
#[cfg(test)]
mod ultimate_oscillator_test;
mod variable_index_dynamic_average;
#[cfg(test)]
mod variable_index_dynamic_average_test;
mod window;
#[allow(unused_imports)]
pub(crate) use helpers::invalid_period;
mod weighted_moving_average;
#[cfg(test)]
mod weighted_moving_average_test;

#[allow(unused_imports)]
pub use absolute_price_oscillator::AbsolutePriceOscillator;
pub use acceleration_bands::{AccelerationBands, AccelerationBandsValue};
pub use anchored_volume_weighted_average_price::{
    AnchoredVolumeWeightedAveragePrice, AnchoredVolumeWeightedAveragePriceValue,
};
#[allow(unused_imports)]
pub use average_directional_index::AverageDirectionalIndex;
#[allow(unused_imports)]
pub use average_directional_index_rating::AverageDirectionalIndexRating;

#[allow(unused_imports)]
pub use bollinger_bands::{BollingerBands, BollingerBandsValue};
pub use candle_abandoned_baby::CandleAbandonedBaby;
pub use candle_advance_block::CandleAdvanceBlock;
pub use candle_belt_hold::CandleBeltHold;
pub use candle_breakaway::CandleBreakaway;
pub use candle_closing_marubozu::CandleClosingMarubozu;
pub use candle_conceal_baby_swall::CandleConcealBabySwall;
pub use candle_counter_attack::CandleCounterAttack;
pub use candle_dark_cloud_cover::CandleDarkCloudCover;
pub use candle_doji::CandleDoji;
pub use candle_doji_star::CandleDojiStar;
pub use candle_dragonfly_doji::CandleDragonflyDoji;
pub use candle_engulfing::CandleEngulfing;
pub use candle_evening_doji_star::CandleEveningDojiStar;
pub use candle_evening_star::CandleEveningStar;
pub use candle_gap_side_side_white::CandleGapSideSideWhite;
pub use candle_gravestone_doji::CandleGravestoneDoji;
pub use candle_hammer::CandleHammer;
pub use candle_hanging_man::CandleHangingMan;
pub use candle_harami::CandleHarami;
pub use candle_harami_cross::CandleHaramiCross;
pub use candle_high_wave::CandleHighWave;
pub use candle_hikkake::CandleHikkake;
pub use candle_hikkake_modified::CandleHikkakeModified;
pub use candle_homing_pigeon::CandleHomingPigeon;
pub use candle_identical_three_crows::CandleIdenticalThreeCrows;
pub use candle_in_neck::CandleInNeck;
pub use candle_inverted_hammer::CandleInvertedHammer;
pub use candle_kicking::CandleKicking;
pub use candle_kicking_by_length::CandleKickingByLength;
pub use candle_ladder_bottom::CandleLadderBottom;
pub use candle_long_legged_doji::CandleLongLeggedDoji;
pub use candle_long_line::CandleLongLine;
pub use candle_marubozu::CandleMarubozu;
pub use candle_mat_hold::CandleMatHold;
pub use candle_matching_low::CandleMatchingLow;
pub use candle_morning_doji_star::CandleMorningDojiStar;
pub use candle_morning_star::CandleMorningStar;
pub use candle_on_neck::CandleOnNeck;
pub use candle_piercing::CandlePiercing;
pub use candle_rickshawman::CandleRickshawman;
pub use candle_rise_fall_three_methods::CandleRiseFallThreeMethods;
pub use candle_separating_lines::CandleSeparatingLines;
pub use candle_shooting_star::CandleShootingStar;
pub use candle_short_line::CandleShortLine;
pub use candle_spinning_top::CandleSpinningTop;
pub use candle_stalled_pattern::CandleStalledPattern;
pub use candle_stick_sandwich::CandleStickSandwich;
pub use candle_takuri::CandleTakuri;
pub use candle_tasuki_gap::CandleTasukiGap;
pub use candle_three_black_crows::CandleThreeBlackCrows;
pub use candle_three_inside::CandleThreeInside;
pub use candle_three_line_strike::CandleThreeLineStrike;
pub use candle_three_outside::CandleThreeOutside;
pub use candle_three_stars_in_south::CandleThreeStarsInSouth;
pub use candle_three_white_soldiers::CandleThreeWhiteSoldiers;
pub use candle_thrusting::CandleThrusting;
pub use candle_tri_star::CandleTriStar;
pub use candle_two_crows::CandleTwoCrows;
pub use candle_unique_three_river::CandleUniqueThreeRiver;
pub use candle_up_down_side_gap_three_methods::CandleUpDownSideGapThreeMethods;
pub use candle_upside_gap_two_crows::CandleUpsideGapTwoCrows;
pub use chande_momentum_oscillator::ChandeMomentumOscillator;
pub use commodity_channel_index::CommodityChannelIndex;
pub use directional_movement_index::DirectionalMovementIndex;
pub use double_exponential_moving_average::DoubleExponentialMovingAverage;
pub use even_better_sinewave::EvenBetterSinewave;
pub use exponential_moving_average::ExponentialMovingAverage;
pub use fibonacci_retracement::{FibonacciRetracement, FibonacciRetracementValue};
pub use heikin_ashi::{HeikinAshi, HeikinAshiValue};
pub use hilbert_transform_dominant_cycle_period::HilbertTransformDominantCyclePeriod;
pub use hilbert_transform_dominant_cycle_phase::HilbertTransformDominantCyclePhase;
pub use hilbert_transform_phasor::{HilbertTransformPhasor, HilbertTransformPhasorValue};
pub use hilbert_transform_sine_wave::{HilbertTransformSineWave, HilbertTransformSineWaveValue};
pub use hilbert_transform_trend_mode::HilbertTransformTrendMode;
pub use hilbert_transform_trendline::HilbertTransformTrendline;
pub use indicator::StreamingIndicator;
#[allow(unused_imports)]
pub use intraday_momentum_index::IntradayMomentumIndex;
pub use jurik_moving_average::JurikMovingAverage;
pub use kaufman_adaptive_moving_average::KaufmanAdaptiveMovingAverage;
pub use klinger_volume_oscillator::{KlingerVolumeOscillator, KlingerVolumeOscillatorValue};
pub use lag::Lag;
pub use laguerre_relative_strength_index::LaguerreRelativeStrengthIndex;
pub use log_return::LogReturn;
pub(crate) use mama::mesa_adaptive_moving_average;
pub use mama::{MesaAdaptiveMovingAverage, MesaAdaptiveMovingAverageValue};
#[allow(unused_imports)]
pub use minus_directional_indicator::MinusDirectionalIndicator;
pub use minus_directional_movement::MinusDirectionalMovement;
pub use momentum::Momentum;
pub use money_flow_index::MoneyFlowIndex;
pub use moving_average::MovingAverage;
pub use moving_average_convergence_divergence::{
    MovingAverageConvergenceDivergence, MovingAverageConvergenceDivergenceValue,
};
pub use moving_average_convergence_divergence_extended::MovingAverageConvergenceDivergenceExtended;
pub use moving_average_convergence_divergence_fixed::MovingAverageConvergenceDivergenceFixed;
pub use opening_range::{OpeningRange, OpeningRangeValue};
pub use parabolic_moving_average_stop::{
    ParabolicMovingAverageStop, ParabolicMovingAverageStopValue,
};
#[allow(unused_imports)]
pub use percentage_price_oscillator::PercentagePriceOscillator;
pub use pivot_points::{PivotPoints, PivotPointsValue};
pub use plus_directional_indicator::PlusDirectionalIndicator;
pub use plus_directional_movement::PlusDirectionalMovement;
pub use premium_discount::{PremiumDiscount, PremiumDiscountValue};
pub use rate_of_change::RateOfChange;
pub use rate_of_change_percent::RateOfChangePercent;
pub use rate_of_change_ratio::RateOfChangeRatio;
pub use rate_of_change_ratio_percent::RateOfChangeRatioPercent;
pub use relative_momentum_index::RelativeMomentumIndex;
pub use rolling_argmax::RollingArgmax;
pub use rolling_argmin::RollingArgmin;
#[allow(unused_imports)]
pub(crate) use rolling_extrema::{MonotonicMax, MonotonicMin, RollingExtrema};
pub use rolling_linear_regression::RollingLinearRegression;
pub use rolling_linear_regression_angle::RollingLinearRegressionAngle;
pub use rolling_linear_regression_intercept::RollingLinearRegressionIntercept;
pub use rolling_linear_regression_slope::RollingLinearRegressionSlope;
pub use rolling_max::RollingMax;
pub use rolling_median::RollingMedian;
pub use rolling_min::RollingMin;
pub use rolling_mode::RollingMode;
pub use rolling_time_series_forecast::RollingTimeSeriesForecast;
pub use variable_period_moving_average::VariablePeriodMovingAverage;

#[allow(unused_imports)]
pub use fast_stochastic_oscillator::{FastStochasticOscillator, FastStochasticOscillatorValue};
pub use parabolic_sar::ParabolicSar;
pub use parabolic_sar_extended::ParabolicSarExtended;
#[allow(unused_imports)]
pub use relative_strength_index::RelativeStrengthIndex;
pub use rolling_sum::RollingSum;
pub use session_volume_levels::{SessionVolumeLevels, SessionVolumeLevelsValue};
pub use simple_moving_average::SimpleMovingAverage;
pub use smoothed_trend_channel::SmoothedTrendChannel;
#[allow(unused_imports)]
pub use stochastic_oscillator::{StochasticOscillator, StochasticOscillatorValue};
#[allow(unused_imports)]
pub use stochastic_relative_strength_index::{
    StochasticRelativeStrengthIndex, StochasticRelativeStrengthIndexValue,
};
pub use tom_de_mark_sequential::{TomDeMarkSequential, TomDeMarkSequentialValue};
pub use triangular_moving_average::TriangularMovingAverage;
#[allow(unused_imports)]
pub use triple_exponential_average::TripleExponentialAverage;
#[allow(unused_imports)]
pub use triple_exponential_moving_average::TripleExponentialMovingAverage;
pub use triple_exponential_rate_of_change::TripleExponentialRateOfChange;
#[allow(unused_imports)]
pub use ultimate_oscillator::UltimateOscillator;
pub use variable_index_dynamic_average::VariableIndexDynamicAverage;
pub use weighted_moving_average::WeightedMovingAverage;
pub use window::Window;

mod average_true_range;
#[cfg(test)]
mod average_true_range_test;
mod math_add;
mod normalized_average_true_range;
#[cfg(test)]
mod normalized_average_true_range_test;
mod true_range;
#[cfg(test)]
mod true_range_test;
pub use math_add::MathAdd;
#[cfg(test)]
mod math_add_test;
mod math_subtract;
pub use math_subtract::MathSubtract;
mod math_multiply;
#[cfg(test)]
mod math_subtract_test;
pub use math_multiply::MathMultiply;
mod math_divide;
#[cfg(test)]
mod math_multiply_test;
pub use math_divide::MathDivide;
mod average_price;
#[cfg(test)]
mod math_divide_test;
mod rolling_argmax;
#[cfg(test)]
mod rolling_argmax_test;
mod rolling_argmin;
#[cfg(test)]
mod rolling_argmin_test;
mod rolling_max;
#[cfg(test)]
mod rolling_max_test;
mod rolling_min;
mod rolling_min_max;
mod rolling_min_max_index;
#[cfg(test)]
mod rolling_min_max_index_test;
#[cfg(test)]
mod rolling_min_max_test;
#[cfg(test)]
mod rolling_min_test;
pub use average_price::AveragePrice;
#[cfg(test)]
mod average_price_test;
mod median_price;
pub use median_price::MedianPrice;
#[cfg(test)]
mod median_price_test;
mod typical_price;
pub use typical_price::TypicalPrice;
#[cfg(test)]
mod typical_price_test;
mod weighted_close;
pub use weighted_close::WeightedClose;
#[cfg(test)]
mod bars_since_test;
#[cfg(test)]
mod cross_test;
#[cfg(test)]
mod donchian_test;
#[cfg(test)]
mod drawdown_test;
#[cfg(test)]
mod entry_exit_test;
#[cfg(test)]
mod exponentially_weighted_correlation_test;
#[cfg(test)]
mod exponentially_weighted_covariance_test;
#[cfg(test)]
mod exponentially_weighted_standard_deviation_test;
#[cfg(test)]
mod exponentially_weighted_sum_test;
#[cfg(test)]
mod exponentially_weighted_variance_test;
#[cfg(test)]
mod gap_down_test;
#[cfg(test)]
mod gap_up_test;
#[cfg(test)]
mod highest_since_test;
#[cfg(test)]
mod keltner_channels_test;
#[cfg(test)]
mod lowest_since_test;
#[cfg(test)]
mod outside_bar_test;
#[cfg(test)]
mod position_hold_test;
mod rolling_average_deviation;
#[cfg(test)]
mod rolling_average_deviation_test;
mod rolling_beta;
#[cfg(test)]
mod rolling_beta_test;
#[cfg(test)]
mod rolling_calmar_test;
#[allow(unused_imports)]
mod rolling_correlation;
#[cfg(test)]
mod rolling_correlation_test;
mod rolling_linear_regression;
mod rolling_linear_regression_angle;
#[cfg(test)]
mod rolling_linear_regression_angle_test;
mod rolling_linear_regression_intercept;
#[cfg(test)]
mod rolling_linear_regression_intercept_test;
mod rolling_linear_regression_slope;
#[cfg(test)]
mod rolling_linear_regression_slope_test;
#[cfg(test)]
mod rolling_linear_regression_test;
mod rolling_midpoint;
#[cfg(test)]
mod rolling_midpoint_test;
mod rolling_midprice;
#[cfg(test)]
mod rolling_midprice_test;
#[cfg(test)]
mod rolling_sharpe_test;
#[cfg(test)]
mod rolling_sortino_test;
mod rolling_standard_deviation;
#[cfg(test)]
mod rolling_standard_deviation_test;
mod rolling_time_series_forecast;
#[cfg(test)]
mod rolling_time_series_forecast_test;
mod rolling_variance;
#[cfg(test)]
mod rolling_variance_test;
#[cfg(test)]
mod signal_delay_test;
#[cfg(test)]
mod smoothed_trend_channel_lifecycle_test;
#[cfg(test)]
mod value_when_test;
#[cfg(test)]
mod weighted_close_test;
#[allow(unused_imports)]
#[cfg(test)]
mod tests {
    use super::*;

    fn assert_optional_eq(actual: Option<f64>, expected: f64) {
        if expected.is_nan() {
            assert_eq!(actual, None);
        } else {
            assert!((actual.expect("expected a warm value") - expected).abs() < 1e-12);
        }
    }

    #[test]
    fn window_evicts_without_growing() {
        let mut window = Window::new(2).unwrap();
        assert_eq!(window.push(1.0), None);
        assert_eq!(window.push(2.0), None);
        assert_eq!(window.push(3.0), Some(1.0));
        assert_eq!(window.len(), 2);
    }

    #[test]
    fn scalar_states_match_batch_for_every_bar() {
        let input: Vec<f64> = (0..80)
            .map(|i| 100.0 + (i as f64 * 0.37).sin() * 8.0 + i as f64 * 0.05)
            .collect();
        let mut sma_batch_state = SimpleMovingAverage::new(7).unwrap();
        let mut sma_batch = Vec::new();
        sma_batch_state.extend_slice_into(&input, &mut sma_batch);
        let mut ema_batch_state = ExponentialMovingAverage::new(7).unwrap();
        let mut ema_batch = Vec::new();
        ema_batch_state.extend_slice_into(&input, &mut ema_batch);
        let mut wma_batch_state = WeightedMovingAverage::new(7).unwrap();
        let mut wma_batch = Vec::new();
        wma_batch_state.extend_slice_into(&input, &mut wma_batch);
        let mut dema_batch_state = DoubleExponentialMovingAverage::new(7).unwrap();
        let mut dema_batch = Vec::new();
        dema_batch_state.extend_slice_into(&input, &mut dema_batch);
        let mut tema_batch_state = TripleExponentialMovingAverage::new(7).unwrap();
        let mut tema_batch = Vec::new();
        tema_batch_state.extend_slice_into(&input, &mut tema_batch);
        let mut trima_batch_state = TriangularMovingAverage::new(7).unwrap();
        let mut trima_batch = Vec::new();
        trima_batch_state.extend_slice_into(&input, &mut trima_batch);
        let mut kama_batch_state = KaufmanAdaptiveMovingAverage::new(7).unwrap();
        let mut kama_batch = Vec::new();
        kama_batch_state.extend_slice_into(&input, &mut kama_batch);
        let mut midpoint_batch_state = RollingMidpoint::new(7).unwrap();
        let mut midpoint_batch = Vec::new();
        midpoint_batch_state.extend_slice_into(&input, &mut midpoint_batch);
        let mut cmo_batch_state = ChandeMomentumOscillator::new(14).unwrap();
        let mut cmo_batch = Vec::new();
        cmo_batch_state.extend_slice_into(&input, &mut cmo_batch);
        let mut sma = SimpleMovingAverage::new(7).unwrap();
        let mut ema = ExponentialMovingAverage::new(7).unwrap();
        let mut wma = WeightedMovingAverage::new(7).unwrap();
        let mut dema = DoubleExponentialMovingAverage::new(7).unwrap();
        let mut tema = TripleExponentialMovingAverage::new(7).unwrap();
        let mut trima = TriangularMovingAverage::new(7).unwrap();
        let mut kama = KaufmanAdaptiveMovingAverage::new(7).unwrap();
        let mut midpoint = RollingMidpoint::new(7).unwrap();
        let mut cmo = ChandeMomentumOscillator::new(14).unwrap();

        for index in 0..input.len() {
            let value = input[index];
            assert_optional_eq(sma.append(value), sma_batch[index]);
            assert_optional_eq(ema.append(value), ema_batch[index]);
            assert_optional_eq(wma.append(value), wma_batch[index]);
            assert_optional_eq(dema.append(value), dema_batch[index]);
            assert_optional_eq(tema.append(value), tema_batch[index]);
            assert_optional_eq(trima.append(value), trima_batch[index]);
            assert_optional_eq(kama.append(value), kama_batch[index]);
            assert_optional_eq(midpoint.append(value), midpoint_batch[index]);
            assert_optional_eq(cmo.append(value), cmo_batch[index]);
        }
    }

    #[test]
    fn midprice_matches_batch_for_every_bar() {
        let close: Vec<f64> = (0..80)
            .map(|i| 100.0 + (i as f64 * 0.31).sin() * 5.0)
            .collect();
        let high: Vec<f64> = close
            .iter()
            .enumerate()
            .map(|(i, value)| value + 1.0 + (i % 3) as f64 * 0.2)
            .collect();
        let low: Vec<f64> = close
            .iter()
            .enumerate()
            .map(|(i, value)| value - 0.8 - (i % 4) as f64 * 0.15)
            .collect();
        let mut batch_state = RollingMidprice::new(7).unwrap();
        let mut expected = Vec::new();
        batch_state
            .extend_slices_into(&high, &low, &mut expected)
            .unwrap();
        let mut state = RollingMidprice::new(7).unwrap();
        for index in 0..close.len() {
            assert_optional_eq(state.append(high[index], low[index]), expected[index]);
        }
    }

    #[test]
    fn rolling_math_states_match_batch_including_ties() {
        let input = vec![
            4.0, 2.0, 2.0, 5.0, 3.0, 3.0, 5.0, 1.0, 1.0, 4.0, 4.0, 2.0, 6.0, 6.0, 0.0, 0.0, 5.0,
        ];
        let period = 4;
        let mut max_expected_state = RollingMax::new(period).unwrap();
        let mut max_expected = Vec::new();
        max_expected_state.extend_slice_into(&input, &mut max_expected);
        let mut min_expected_state = RollingMin::new(period).unwrap();
        let mut min_expected = Vec::new();
        min_expected_state.extend_slice_into(&input, &mut min_expected);
        let mut sum_batch_state = RollingSum::new(period).unwrap();
        let mut sum_expected = Vec::new();
        sum_batch_state.extend_slice_into(&input, &mut sum_expected);
        let mut maxindex_expected_state = RollingArgmax::new(period).unwrap();
        let mut maxindex_expected = Vec::new();
        maxindex_expected_state.extend_slice_into(&input, &mut maxindex_expected);
        let mut minindex_expected_state = RollingArgmin::new(period).unwrap();
        let mut minindex_expected = Vec::new();
        minindex_expected_state.extend_slice_into(&input, &mut minindex_expected);
        let mut minmax_expected_state = RollingMinMax::new(period).unwrap();
        let mut minmax_min = Vec::new();
        let mut minmax_max = Vec::new();
        minmax_expected_state.extend_slices_into(&input, &mut minmax_min, &mut minmax_max);
        let mut minmaxindex_expected_state = RollingMinMaxIndex::new(period).unwrap();
        let mut minidx = Vec::new();
        let mut maxidx = Vec::new();
        minmaxindex_expected_state.extend_slices_into(&input, &mut minidx, &mut maxidx);
        let mut max = RollingMax::new(period).unwrap();
        let mut min = RollingMin::new(period).unwrap();
        let mut sum = RollingSum::new(period).unwrap();
        let mut maxindex = RollingArgmax::new(period).unwrap();
        let mut minindex = RollingArgmin::new(period).unwrap();
        let mut minmax = RollingMinMax::new(period).unwrap();
        let mut minmaxindex = RollingMinMaxIndex::new(period).unwrap();

        for index in 0..input.len() {
            assert_optional_eq(max.append(input[index]), max_expected[index]);
            assert_optional_eq(min.append(input[index]), min_expected[index]);
            assert_optional_eq(sum.append(input[index]), sum_expected[index]);
            assert_optional_eq(maxindex.append(input[index]), maxindex_expected[index]);
            assert_optional_eq(minindex.append(input[index]), minindex_expected[index]);
            match minmax.append(input[index]) {
                Some(value) => {
                    assert_eq!(value.minimum, minmax_min[index]);
                    assert_eq!(value.maximum, minmax_max[index]);
                }
                None => {
                    assert!(minmax_min[index].is_nan());
                    assert!(minmax_max[index].is_nan());
                }
            }
            let indices = minmaxindex.append(input[index]);
            assert_eq!(indices.minimum as f64, minidx[index]);
            assert_eq!(indices.maximum as f64, maxidx[index]);
        }
    }

    #[test]
    fn rolling_statistic_states_match_batch_for_every_bar() {
        let input: Vec<f64> = (0..96)
            .map(|index| {
                1_000_000.0 + (index as f64 * 0.23).sin() * 11.0 + (index % 5) as f64 * 0.125
            })
            .collect();
        let period = 12;
        let mut avgdev_batch = RollingAverageDeviation::new(period).unwrap();
        let mut avgdev_expected = Vec::new();
        avgdev_batch.extend_slice_into(&input, &mut avgdev_expected);
        let mut var_batch = RollingVariance::new(period, 2.0).unwrap();
        let mut var_expected = Vec::new();
        var_batch.extend_slice_into(&input, &mut var_expected);
        let mut stddev_batch = RollingStandardDeviation::new(period, 2.0).unwrap();
        let mut stddev_expected = Vec::new();
        stddev_batch.extend_slice_into(&input, &mut stddev_expected);
        let mut avgdev = RollingAverageDeviation::new(period).unwrap();
        let mut var = RollingVariance::new(period, 2.0).unwrap();
        let mut stddev = RollingStandardDeviation::new(period, 2.0).unwrap();
        for index in 0..input.len() {
            assert_optional_eq(avgdev.append(input[index]), avgdev_expected[index]);
            assert_optional_eq(var.append(input[index]), var_expected[index]);
            assert_optional_eq(stddev.append(input[index]), stddev_expected[index]);
        }

        let constant = vec![42.0; 30];
        let mut expected_state = RollingStandardDeviation::new(5, 3.0).unwrap();
        let mut expected = Vec::new();
        expected_state.extend_slice_into(&constant, &mut expected);
        let mut state = RollingStandardDeviation::new(5, 3.0).unwrap();
        for (input, expected) in constant.into_iter().zip(expected) {
            assert_optional_eq(state.append(input), expected);
        }
    }

    #[test]
    fn bivariate_statistic_states_match_batch_for_every_bar() {
        let market: Vec<f64> = (0..100)
            .map(|index| 80.0 + index as f64 * 0.08 + (index as f64 * 0.17).sin() * 3.0)
            .collect();
        let asset: Vec<f64> = market
            .iter()
            .enumerate()
            .map(|(index, market)| market * 1.3 + (index as f64 * 0.29).cos() * 2.0)
            .collect();
        let period = 10;
        let mut beta_expected_state = RollingBeta::new(period).unwrap();
        let mut beta_expected = Vec::new();
        beta_expected_state
            .extend_slices_into(&market, &asset, &mut beta_expected)
            .unwrap();
        let mut correl_expected_state = RollingCorrelation::new(period).unwrap();
        let mut correl_expected = Vec::new();
        correl_expected_state
            .extend_slices_into(&market, &asset, &mut correl_expected)
            .unwrap();
        let mut beta = RollingBeta::new(period).unwrap();
        let mut correl = RollingCorrelation::new(period).unwrap();
        for index in 0..market.len() {
            let beta_actual = beta.append(market[index], asset[index]);
            let correl_actual = correl.append(market[index], asset[index]);
            if beta_expected[index].is_nan() {
                assert_eq!(beta_actual, None);
            } else {
                assert!(
                    (beta_actual.unwrap() - beta_expected[index]).abs() < 1e-12,
                    "BETA differs at {index}: {:?} vs {}",
                    beta_actual,
                    beta_expected[index]
                );
            }
            if correl_expected[index].is_nan() {
                assert_eq!(correl_actual, None);
            } else {
                assert!(
                    (correl_actual.unwrap() - correl_expected[index]).abs() < 1e-12,
                    "CORREL differs at {index}: {:?} vs {}",
                    correl_actual,
                    correl_expected[index]
                );
            }
        }
    }

    #[test]
    fn regression_states_match_batch_for_every_bar() {
        let input: Vec<f64> = (0..100)
            .map(|index| 200.0 + index as f64 * 0.4 + (index as f64 * 0.19).sin() * 7.0)
            .collect();
        let period = 14;
        let mut linearreg_expected = Vec::new();
        RollingLinearRegression::new(period)
            .unwrap()
            .extend_slice_into(&input, &mut linearreg_expected);
        let mut slope_expected = Vec::new();
        RollingLinearRegressionSlope::new(period)
            .unwrap()
            .extend_slice_into(&input, &mut slope_expected);
        let mut intercept_expected = Vec::new();
        RollingLinearRegressionIntercept::new(period)
            .unwrap()
            .extend_slice_into(&input, &mut intercept_expected);
        let mut angle_expected = Vec::new();
        RollingLinearRegressionAngle::new(period)
            .unwrap()
            .extend_slice_into(&input, &mut angle_expected);
        let mut tsf_expected = Vec::new();
        RollingTimeSeriesForecast::new(period)
            .unwrap()
            .extend_slice_into(&input, &mut tsf_expected);
        let mut linearreg = RollingLinearRegression::new(period).unwrap();
        let mut slope = RollingLinearRegressionSlope::new(period).unwrap();
        let mut intercept = RollingLinearRegressionIntercept::new(period).unwrap();
        let mut angle = RollingLinearRegressionAngle::new(period).unwrap();
        let mut tsf = RollingTimeSeriesForecast::new(period).unwrap();
        for index in 0..input.len() {
            assert_optional_eq(linearreg.append(input[index]), linearreg_expected[index]);
            assert_optional_eq(slope.append(input[index]), slope_expected[index]);
            assert_optional_eq(intercept.append(input[index]), intercept_expected[index]);
            assert_optional_eq(angle.append(input[index]), angle_expected[index]);
            assert_optional_eq(tsf.append(input[index]), tsf_expected[index]);
        }
    }
}
mod on_balance_volume;
#[cfg(test)]
mod on_balance_volume_test;

mod accumulation_distribution;
mod accumulation_distribution_oscillator;
#[cfg(test)]
mod accumulation_distribution_oscillator_test;
#[cfg(test)]
mod accumulation_distribution_test;
mod balance_of_power;
#[cfg(test)]
mod balance_of_power_test;
mod drawdown;
mod rolling_sharpe;
mod williams_percent_r;
#[cfg(test)]
mod williams_percent_r_test;
pub use rolling_sharpe::RollingSharpe;
mod rolling_sortino;
pub use rolling_sortino::RollingSortino;
mod arnaud_legoux_moving_average;
#[cfg(test)]
mod arnaud_legoux_moving_average_test;
mod awesome_oscillator;
#[cfg(test)]
mod awesome_oscillator_test;
mod fisher_transform;
#[cfg(test)]
mod fisher_transform_test;
mod hull_moving_average;
#[cfg(test)]
mod hull_moving_average_test;
mod rolling_calmar;
mod true_strength_index;
#[cfg(test)]
mod true_strength_index_test;
mod volume_weighted_moving_average;
#[cfg(test)]
mod volume_weighted_moving_average_test;
mod zero_lag_exponential_moving_average;
#[cfg(test)]
mod zero_lag_exponential_moving_average_test;
pub use fisher_transform::FisherTransform;
mod chaikin_volatility;
#[cfg(test)]
mod chaikin_volatility_test;
mod ease_of_movement;
#[cfg(test)]
mod ease_of_movement_test;
mod falling;
#[cfg(test)]
mod falling_test;
mod force_index;
#[cfg(test)]
mod force_index_test;
mod fractal_dimension;
#[cfg(test)]
mod fractal_dimension_test;
mod hurst;
#[cfg(test)]
mod hurst_test;
mod rising;
#[cfg(test)]
mod rising_test;
mod rolling_alpha;
#[cfg(test)]
mod rolling_alpha_test;
mod rolling_autocorr;
#[cfg(test)]
mod rolling_autocorr_test;
mod rolling_entropy;
#[cfg(test)]
mod rolling_entropy_test;
mod rolling_information_ratio;
mod rolling_volume_weighted_average_price;
#[cfg(test)]
mod rolling_volume_weighted_average_price_test;
mod ulcer_index;
#[cfg(test)]
mod ulcer_index_test;
#[allow(unused_imports)]
pub(crate) use rolling_information_ratio::rolling_information_ratio;
mod break_of_structure_change_of_character;
mod fair_value_gap;
#[cfg(test)]
mod fair_value_gap_test;
mod hedge_ratio;
#[cfg(test)]
mod hedge_ratio_test;
mod session_extrema;
#[cfg(test)]
mod session_extrema_test;
#[allow(unused_imports)]
pub(crate) use break_of_structure_change_of_character::break_of_structure_change_of_character;
mod order_block;
#[allow(unused_imports)]
pub(crate) use order_block::order_block;
mod liquidity;
#[allow(unused_imports)]
pub(crate) use liquidity::liquidity;
mod close_to_close_sigma;
#[cfg(test)]
mod close_to_close_sigma_test;
mod equal_highs_lows;
#[cfg(test)]
mod equal_highs_lows_test;
mod parkinson;
mod previous_high_low;
#[cfg(test)]
mod previous_high_low_test;
mod retracements;
#[cfg(test)]
mod retracements_test;
mod sessions;
#[cfg(test)]
mod sessions_test;
#[allow(unused_imports)]
pub(crate) use parkinson::parkinson;
mod garman_klass;
#[allow(unused_imports)]
pub(crate) use garman_klass::garman_klass;
mod rogers_satchell;
#[allow(unused_imports)]
pub(crate) use rogers_satchell::rogers_satchell;
mod garman_klass_yang_zhang;
#[allow(unused_imports)]
pub(crate) use garman_klass_yang_zhang::garman_klass_yang_zhang;
mod yang_zhang;
#[allow(unused_imports)]
pub(crate) use yang_zhang::yang_zhang;
mod time_series_rank;
#[allow(unused_imports)]
pub(crate) use time_series_rank::time_series_rank;
pub use time_series_rank::TimeSeriesRank;
mod signed_power;
#[allow(unused_imports)]
pub(crate) use signed_power::signed_power;
pub use signed_power::SignedPower;
mod decay_linear;
#[allow(unused_imports)]
pub(crate) use decay_linear::decay_linear;
pub use decay_linear::DecayLinear;
mod average_daily_dollar_value;
#[allow(unused_imports)]
pub(crate) use average_daily_dollar_value::average_daily_dollar_value;
mod amihud;
#[allow(unused_imports)]
pub(crate) use amihud::amihud;
mod roll_spread;
#[allow(unused_imports)]
pub(crate) use roll_spread::roll_spread;
mod ornstein_uhlenbeck_half_life;
#[allow(unused_imports)]
pub(crate) use ornstein_uhlenbeck_half_life::ornstein_uhlenbeck_half_life;
mod cumulative_sum_control_chart;
#[allow(unused_imports)]
pub(crate) use cumulative_sum_control_chart::cumulative_sum_control_chart;
mod spread_zscore;
#[allow(unused_imports)]
pub(crate) use spread_zscore::spread_zscore;
mod frac_diff;
#[allow(unused_imports)]
pub(crate) use frac_diff::frac_diff;
mod kalman_hedge_ratio;
#[allow(unused_imports)]
pub(crate) use kalman_hedge_ratio::kalman_hedge_ratio;
mod supertrend;
#[allow(unused_imports)]
pub(crate) use supertrend::supertrend;
mod ichimoku;
#[allow(unused_imports)]
pub(crate) use ichimoku::ichimoku;
mod squeeze;
#[allow(unused_imports)]
pub(crate) use squeeze::squeeze;
mod squeeze_pro;
#[allow(unused_imports)]
pub(crate) use squeeze_pro::squeeze_pro;
mod schaff_trend_cycle;
#[allow(unused_imports)]
pub(crate) use schaff_trend_cycle::schaff_trend_cycle;
mod vortex;
#[allow(unused_imports)]
pub(crate) use vortex::vortex;
mod know_sure_thing;
#[allow(unused_imports)]
pub(crate) use know_sure_thing::know_sure_thing;
mod swing_high_low;
mod swing_highs_lows;
pub use swing_high_low::{SwingHighLow, SwingValue};
mod swing_highs_lows_indicator;
#[cfg(test)]
mod swing_highs_lows_indicator_test;
#[cfg(test)]
mod swing_highs_lows_test;
pub use swing_highs_lows_indicator::SwingHighsLows;
mod swing_high;
#[cfg(test)]
mod swing_high_test;
pub use swing_high::SwingHigh;
mod swing_low;
#[cfg(test)]
mod swing_low_test;
pub use swing_low::SwingLow;
mod rolling_percentile;
#[cfg(test)]
mod rolling_percentile_test;
mod rolling_quantile;
#[cfg(test)]
mod rolling_quantile_test;
mod rolling_rank;
#[cfg(test)]
mod rolling_rank_test;
mod rolling_skew;
#[cfg(test)]
mod rolling_skew_test;
mod rolling_z_score;
#[cfg(test)]
mod rolling_z_score_test;
pub use rolling_skew::RollingSkew;
mod rolling_kurtosis;
#[cfg(test)]
mod rolling_kurtosis_test;
pub use rolling_kurtosis::RollingKurtosis;
mod exponentially_weighted_correlation;
mod exponentially_weighted_covariance;
mod exponentially_weighted_standard_deviation;
mod exponentially_weighted_sum;
mod exponentially_weighted_variance;
mod mass_index;
mod rolling_covariance;
#[cfg(test)]
mod rolling_covariance_test;
mod rolling_interquartile_range;
#[cfg(test)]
mod rolling_interquartile_range_test;
#[allow(unused_imports)]
mod rolling_winsorize;
#[cfg(test)]
mod rolling_winsorize_test;
#[allow(unused_imports)]
pub(crate) use mass_index::mass_index;
mod detrended_price_oscillator;
#[allow(unused_imports)]
pub(crate) use detrended_price_oscillator::detrended_price_oscillator;
mod chaikin_money_flow;
#[allow(unused_imports)]
pub(crate) use chaikin_money_flow::chaikin_money_flow;
mod volume_price_trend;
#[allow(unused_imports)]
pub(crate) use volume_price_trend::volume_price_trend;
mod negative_volume_index;
#[allow(unused_imports)]
pub(crate) use negative_volume_index::negative_volume_index;
mod positive_volume_index;
#[allow(unused_imports)]
pub(crate) use positive_volume_index::positive_volume_index;
mod mcginley_dynamic;
#[allow(unused_imports)]
pub(crate) use mcginley_dynamic::mcginley_dynamic;
mod aroon_oscillator;
#[cfg(test)]
mod aroon_oscillator_test;

mod bar_relation;
mod bars_since;
mod gap_down;
mod gap_up;
mod higher_high;
#[cfg(test)]
mod higher_high_test;
mod highest_since;
mod inside_bar;
#[cfg(test)]
mod inside_bar_test;
mod lower_low;
#[cfg(test)]
mod lower_low_test;
mod lowest_since;
mod outside_bar;
mod value_when;
pub use amihud::Amihud;
pub use arnaud_legoux_moving_average::ArnaudLegouxMovingAverage;
pub use average_daily_dollar_value::AverageDailyDollarValue;
pub use awesome_oscillator::AwesomeOscillator;
pub use bars_since::BarsSince;
pub use break_of_structure_change_of_character::{
    BreakOfStructureChangeOfCharacter, BreakOfStructureChangeOfCharacterValue,
};
pub use close_to_close_sigma::CloseToCloseSigma;
pub use cumulative_sum_control_chart::CumulativeSumControlChart;
pub use drawdown::Drawdown;
pub use equal_highs_lows::{EqualHighsLows, EqualHighsLowsValue};
pub use exponentially_weighted_correlation::ExponentiallyWeightedCorrelation;
pub use exponentially_weighted_covariance::ExponentiallyWeightedCovariance;
pub use exponentially_weighted_standard_deviation::ExponentiallyWeightedStandardDeviation;
pub use exponentially_weighted_sum::ExponentiallyWeightedSum;
pub use exponentially_weighted_variance::ExponentiallyWeightedVariance;
pub use fair_value_gap::{FairValueGap, FairValueGapValue};
pub use falling::Falling;
pub use frac_diff::FracDiff;
pub use fractal_dimension::FractalDimension;
pub use gap_down::GapDown;
pub use gap_up::GapUp;
pub use garman_klass::GarmanKlass;
pub use garman_klass_yang_zhang::GarmanKlassYangZhang;
pub use hedge_ratio::HedgeRatio;
pub use higher_high::HigherHigh;
pub use highest_since::HighestSince;
pub use hull_moving_average::HullMovingAverage;
pub use hurst::Hurst;
pub use ichimoku::{Ichimoku, IchimokuValue};
pub use inside_bar::InsideBar;
pub use kalman_hedge_ratio::KalmanHedgeRatio;
pub use know_sure_thing::{KnowSureThing, KnowSureThingValue};
pub use liquidity::{Liquidity, LiquidityValue};
pub use lower_low::LowerLow;
pub use lowest_since::LowestSince;
pub use order_block::{OrderBlock, OrderBlockValue};
pub use ornstein_uhlenbeck_half_life::OrnsteinUhlenbeckHalfLife;
pub use outside_bar::OutsideBar;
pub use parkinson::Parkinson;
pub use previous_high_low::{PreviousHighLow, PreviousHighLowValue};
pub use retracements::{Retracements, RetracementsValue};
pub use rising::Rising;
pub use rogers_satchell::RogersSatchell;
pub use roll_spread::RollSpread;
pub use rolling_alpha::RollingAlpha;
pub use rolling_autocorr::RollingAutocorr;
pub use rolling_covariance::RollingCovariance;
pub use rolling_entropy::RollingEntropy;
pub use rolling_information_ratio::RollingInformationRatio;
pub use rolling_interquartile_range::RollingInterquartileRange;
pub use rolling_percentile::RollingPercentile;
pub use rolling_quantile::RollingQuantile;
pub use rolling_rank::RollingRank;
pub use rolling_winsorize::RollingWinsorize;
pub use rolling_z_score::RollingZScore;
pub use schaff_trend_cycle::{SchaffTrendCycle, SchaffTrendCycleValue};
pub use session_extrema::{SessionExtrema, SessionExtremaValue};
pub use sessions::{Sessions, SessionsValue};
pub use spread_zscore::SpreadZScore;
pub use squeeze::{Squeeze, SqueezeValue};
pub use squeeze_pro::{SqueezePro, SqueezeProValue};
pub use supertrend::{Supertrend, SupertrendValue};
pub use true_strength_index::TrueStrengthIndex;
pub use value_when::ValueWhen;
pub use volume_weighted_moving_average::VolumeWeightedMovingAverage;
pub use vortex::{Vortex, VortexValue};
pub use yang_zhang::YangZhang;
pub use zero_lag_exponential_moving_average::ZeroLagExponentialMovingAverage;
mod donchian;
pub use donchian::{Donchian, DonchianValue};
mod donchian_channels;
#[cfg(test)]
mod donchian_channels_test;
pub use donchian_channels::DonchianChannels;
pub use ulcer_index::UlcerIndex;
mod keltner_channels;
pub use chaikin_volatility::ChaikinVolatility;
pub use ease_of_movement::EaseOfMovement;
pub use force_index::ForceIndex;
pub use keltner_channels::{KeltnerChannels, KeltnerValue};
pub use rolling_volume_weighted_average_price::RollingVolumeWeightedAveragePrice;
mod signal_delay;
pub use signal_delay::SignalDelay;
mod position_hold;
pub use position_hold::PositionHold;
mod entry_exit;
pub use entry_exit::EntryExit;
mod crossover;
#[cfg(test)]
mod crossover_test;
pub use crossover::Crossover;
mod crossunder;
#[cfg(test)]
mod crossunder_test;
pub use crossunder::Crossunder;
mod cross;
pub use accumulation_distribution::AccumulationDistribution;
pub use accumulation_distribution_oscillator::AccumulationDistributionOscillator;
pub use aroon::{Aroon, AroonValue};
pub use aroon_oscillator::AroonOscillator;
pub use average_true_range::AverageTrueRange;
pub use balance_of_power::BalanceOfPower;
pub use chaikin_money_flow::ChaikinMoneyFlow;
pub use cross::Cross;
pub use detrended_price_oscillator::DetrendedPriceOscillator;
pub use mass_index::MassIndex;
pub use mcginley_dynamic::McGinleyDynamic;
pub use negative_volume_index::NegativeVolumeIndex;
pub use normalized_average_true_range::NormalizedAverageTrueRange;
pub use on_balance_volume::OnBalanceVolume;
pub use positive_volume_index::PositiveVolumeIndex;
pub use rolling_average_deviation::RollingAverageDeviation;
pub use rolling_beta::RollingBeta;
pub use rolling_calmar::RollingCalmar;
pub use rolling_correlation::RollingCorrelation;
pub use rolling_midpoint::RollingMidpoint;
pub use rolling_midprice::RollingMidprice;
pub use rolling_min_max::{RollingMinMax, RollingMinMaxValue};
pub use rolling_min_max_index::{RollingMinMaxIndex, RollingMinMaxIndexValue};
pub use rolling_standard_deviation::RollingStandardDeviation;
pub use rolling_variance::RollingVariance;
pub use true_range::TrueRange;
pub use volume_price_trend::VolumePriceTrend;
pub use williams_percent_r::WilliamsPercentR;
