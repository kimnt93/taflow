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
mod accbands;
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
mod candle_3whitesoldiers;
mod candle_abandonedbaby;
mod candle_advanceblock;
mod candle_belthold;
mod candle_breakaway;
mod candle_closingmarubozu;
mod candle_concealbabyswall;
mod candle_counterattack;
mod candle_darkcloudcover;
mod candle_doji;
mod candle_dojistar;
mod candle_dragonflydoji;
mod candle_engulfing;
mod candle_eveningdojistar;
mod candle_eveningstar;
mod candle_gapsidesidewhite;
mod candle_gravestonedoji;
mod candle_hammer;
#[cfg(test)]
mod candle_hammer_test;
mod candle_hangingman;
mod candle_harami;
mod candle_haramicross;
mod candle_highwave;
mod candle_hikkake;
mod candle_hikkake_modified;
#[cfg(test)]
mod candle_hikkake_modified_test;
#[cfg(test)]
mod candle_hikkake_test;
mod candle_homingpigeon;
mod candle_identical3crows;
mod candle_inneck;
mod candle_invertedhammer;
mod candle_kicking;
mod candle_kickingbylength;
mod candle_ladderbottom;
mod candle_longleggeddoji;
mod candle_longline;
mod candle_marubozu;
mod candle_matchinglow;
mod candle_mathold;
mod candle_morningdojistar;
mod candle_morningstar;
mod candle_onneck;
mod candle_piercing;
mod candle_rickshawman;
mod candle_risefall3methods;
mod candle_separatinglines;
mod candle_shootingstar;
mod candle_shortline;
mod candle_spinningtop;
mod candle_stalledpattern;
mod candle_stick_sandwich;
#[cfg(test)]
mod candle_stick_sandwich_test;
mod candle_takuri;
mod candle_tasukigap;
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
mod candle_thrusting;
mod candle_tristar;
mod candle_two_crows;
#[cfg(test)]
mod candle_two_crows_test;
mod candle_unique3river;
mod candle_upsidegap2crows;
mod candle_xsidegap3methods;
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
mod imi;
#[cfg(test)]
mod imi_test;
mod indicator;
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
mod plus_di;
mod plus_dm;
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
mod sar;
mod sarext;
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
#[allow(unused_imports)]
pub(crate) use accbands::acceleration_bands;
pub use accbands::{AccelerationBands, AccelerationBandsValue};
pub use anchored_volume_weighted_average_price::{
    AnchoredVolumeWeightedAveragePrice, AnchoredVolumeWeightedAveragePriceValue,
};
#[allow(unused_imports)]
pub use average_directional_index::AverageDirectionalIndex;
#[allow(unused_imports)]
pub use average_directional_index_rating::AverageDirectionalIndexRating;

#[allow(unused_imports)]
pub use bollinger_bands::{BollingerBands, BollingerBandsValue};
pub use candle_3whitesoldiers::CandleThreeWhiteSoldiers;
pub use candle_abandonedbaby::CandleAbandonedBaby;
pub use candle_advanceblock::CandleAdvanceBlock;
pub use candle_belthold::CandleBeltHold;
pub use candle_breakaway::CandleBreakaway;
pub use candle_closingmarubozu::CandleClosingMarubozu;
pub use candle_concealbabyswall::CandleConcealBabySwall;
pub use candle_counterattack::CandleCounterAttack;
pub use candle_darkcloudcover::CandleDarkCloudCover;
pub use candle_doji::CandleDoji;
pub use candle_dojistar::CandleDojiStar;
pub use candle_dragonflydoji::CandleDragonflyDoji;
pub use candle_engulfing::CandleEngulfing;
pub use candle_eveningdojistar::CandleEveningDojiStar;
pub use candle_eveningstar::CandleEveningStar;
pub use candle_gapsidesidewhite::CandleGapSideSideWhite;
pub use candle_gravestonedoji::CandleGravestoneDoji;
pub use candle_hammer::CandleHammer;
pub use candle_hangingman::CandleHangingMan;
pub use candle_harami::CandleHarami;
pub use candle_haramicross::CandleHaramiCross;
pub use candle_highwave::CandleHighWave;
pub use candle_hikkake::CandleHikkake;
pub use candle_hikkake_modified::CandleHikkakeModified;
pub use candle_homingpigeon::CandleHomingPigeon;
pub use candle_identical3crows::CandleIdenticalThreeCrows;
pub use candle_inneck::CandleInNeck;
pub use candle_invertedhammer::CandleInvertedHammer;
pub use candle_kicking::CandleKicking;
pub use candle_kickingbylength::CandleKickingByLength;
pub use candle_ladderbottom::CandleLadderBottom;
pub use candle_longleggeddoji::CandleLongLeggedDoji;
pub use candle_longline::CandleLongLine;
pub use candle_marubozu::CandleMarubozu;
pub use candle_matchinglow::CandleMatchingLow;
pub use candle_mathold::CandleMatHold;
pub use candle_morningdojistar::CandleMorningDojiStar;
pub use candle_morningstar::CandleMorningStar;
pub use candle_onneck::CandleOnNeck;
pub use candle_piercing::CandlePiercing;
pub use candle_rickshawman::CandleRickshawman;
pub use candle_risefall3methods::CandleRiseFallThreeMethods;
pub use candle_separatinglines::CandleSeparatingLines;
pub use candle_shootingstar::CandleShootingStar;
pub use candle_shortline::CandleShortLine;
pub use candle_spinningtop::CandleSpinningTop;
pub use candle_stalledpattern::CandleStalledPattern;
pub use candle_stick_sandwich::CandleStickSandwich;
pub use candle_takuri::CandleTakuri;
pub use candle_tasukigap::CandleTasukiGap;
pub use candle_three_black_crows::CandleThreeBlackCrows;
pub use candle_three_inside::CandleThreeInside;
pub use candle_three_line_strike::CandleThreeLineStrike;
pub use candle_three_outside::CandleThreeOutside;
pub use candle_three_stars_in_south::CandleThreeStarsInSouth;
pub use candle_thrusting::CandleThrusting;
pub use candle_tristar::CandleTriStar;
pub use candle_two_crows::CandleTwoCrows;
pub use candle_unique3river::CandleUniqueThreeRiver;
pub use candle_upsidegap2crows::CandleUpsideGapTwoCrows;
pub use candle_xsidegap3methods::CandleUpDownSideGapThreeMethods;
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
#[allow(unused_imports)]
pub use imi::IntradayMomentumIndex;
pub use indicator::StreamingIndicator;
pub use jurik_moving_average::JurikMovingAverage;
pub use kaufman_adaptive_moving_average::KaufmanAdaptiveMovingAverage;
pub use klinger_volume_oscillator::{KlingerVolumeOscillator, KlingerVolumeOscillatorValue};
pub use lag::Lag;
pub use laguerre_relative_strength_index::LaguerreRelativeStrengthIndex;
pub use log_return::LogReturn;
#[allow(unused_imports)]
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
#[allow(unused_imports)]
pub(crate) use plus_di::plus_directional_indicator;
pub use plus_di::PlusDirectionalIndicator;
#[allow(unused_imports)]
pub(crate) use plus_dm::plus_directional_movement;
pub use plus_dm::PlusDirectionalMovement;
pub use premium_discount::{PremiumDiscount, PremiumDiscountValue};
pub use rate_of_change::RateOfChange;
pub use rate_of_change_percent::RateOfChangePercent;
pub use rate_of_change_ratio::RateOfChangeRatio;
pub use rate_of_change_ratio_percent::RateOfChangeRatioPercent;
pub use regression::{Linearreg, LinearregAngle, LinearregIntercept, LinearregSlope, Tsf};
pub use relative_momentum_index::RelativeMomentumIndex;
#[allow(unused_imports)]
pub(crate) use rolling_extrema::{MonotonicMax, MonotonicMin, RollingExtrema};
pub use rolling_extrema::{RollingArgmax, RollingArgmin, RollingMax, RollingMin};
pub use rolling_median::RollingMedian;
pub use rolling_mode::RollingMode;
pub use variable_period_moving_average::VariablePeriodMovingAverage;

#[allow(unused_imports)]
pub use fast_stochastic_oscillator::{FastStochasticOscillator, FastStochasticOscillatorValue};
#[allow(unused_imports)]
pub use relative_strength_index::RelativeStrengthIndex;
pub use rolling_sum::RollingSum;
#[allow(unused_imports)]
pub(crate) use sar::parabolic_sar;
pub use sar::ParabolicSar;
#[allow(unused_imports)]
pub(crate) use sarext::parabolic_sar_extended;
pub use sarext::ParabolicSarExtended;
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
#[cfg(test)]
mod math_divide_test;
mod rolling_max;
#[allow(unused_imports)]
pub(crate) use rolling_max::rolling_max;
mod rolling_argmax;
#[allow(unused_imports)]
pub(crate) use rolling_argmax::rolling_argmax;
mod rolling_min;
#[allow(unused_imports)]
pub(crate) use rolling_min::rolling_min;
mod rolling_argmin;
#[allow(unused_imports)]
pub(crate) use rolling_argmin::rolling_argmin;
mod rolling_minmax;
#[allow(unused_imports)]
pub(crate) use rolling_minmax::rolling_minmax;
mod rolling_minmax_index;
#[allow(unused_imports)]
pub(crate) use rolling_minmax_index::rolling_minmax_index;
mod average_price;
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
mod rolling_average_deviation;
#[cfg(test)]
mod rolling_average_deviation_test;
mod rolling_beta;
mod rolling_standard_deviation;
#[cfg(test)]
mod rolling_standard_deviation_test;
mod rolling_variance;
#[cfg(test)]
mod rolling_variance_test;
#[cfg(test)]
mod weighted_close_test;
#[allow(unused_imports)]
pub(crate) use rolling_beta::rolling_beta;
mod rolling_corr;
#[allow(unused_imports)]
pub(crate) use rolling_corr::rolling_corr;
mod rolling_linreg;
#[allow(unused_imports)]
pub(crate) use rolling_linreg::rolling_linreg;
mod rolling_linreg_slope;
#[allow(unused_imports)]
pub(crate) use rolling_linreg_slope::rolling_linreg_slope;
mod rolling_linreg_intercept;
#[allow(unused_imports)]
pub(crate) use rolling_linreg_intercept::rolling_linreg_intercept;
mod rolling_linreg_angle;
#[allow(unused_imports)]
pub(crate) use rolling_linreg_angle::rolling_linreg_angle;
mod rolling_tsf;
#[allow(unused_imports)]
pub(crate) use rolling_tsf::rolling_tsf;
#[cfg(test)]
mod bars_since_test;
#[cfg(test)]
mod cross_test;
#[cfg(test)]
mod donchian_test;
#[cfg(test)]
mod entry_exit_test;
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
mod rolling_midpoint;
#[cfg(test)]
mod rolling_midpoint_test;
mod rolling_midprice;
#[cfg(test)]
mod rolling_midprice_test;
#[cfg(test)]
mod signal_delay_test;
#[cfg(test)]
mod smoothed_trend_channel_lifecycle_test;
#[cfg(test)]
mod value_when_test;
#[allow(unused_imports)]
pub(crate) use candle_3whitesoldiers::candle_three_white_soldiers;
#[allow(unused_imports)]
pub(crate) use candle_abandonedbaby::candle_abandoned_baby;
#[allow(unused_imports)]
pub(crate) use candle_advanceblock::candle_advance_block;
#[allow(unused_imports)]
pub(crate) use candle_belthold::candle_belt_hold;
#[allow(unused_imports)]
pub(crate) use candle_breakaway::candle_breakaway;
#[allow(unused_imports)]
pub(crate) use candle_closingmarubozu::candle_closing_marubozu;
#[allow(unused_imports)]
pub(crate) use candle_concealbabyswall::candle_conceal_baby_swall;
#[allow(unused_imports)]
pub(crate) use candle_counterattack::candle_counterattack;
#[allow(unused_imports)]
pub(crate) use candle_darkcloudcover::candle_dark_cloud_cover;
#[allow(unused_imports)]
pub(crate) use candle_doji::candle_doji;
#[allow(unused_imports)]
pub(crate) use candle_dojistar::candle_doji_star;
#[allow(unused_imports)]
pub(crate) use candle_dragonflydoji::candle_dragonfly_doji;
#[allow(unused_imports)]
pub(crate) use candle_engulfing::candle_engulfing;
#[allow(unused_imports)]
pub(crate) use candle_eveningdojistar::candle_evening_doji_star;
#[allow(unused_imports)]
pub(crate) use candle_eveningstar::candle_evening_star;
#[allow(unused_imports)]
pub(crate) use candle_gapsidesidewhite::candle_gap_side_side_white;
#[allow(unused_imports)]
pub(crate) use candle_gravestonedoji::candle_gravestone_doji;
#[allow(unused_imports)]
pub(crate) use candle_hangingman::candle_hanging_man;
#[allow(unused_imports)]
pub(crate) use candle_harami::candle_harami;
#[allow(unused_imports)]
pub(crate) use candle_haramicross::candle_harami_cross;
#[allow(unused_imports)]
pub(crate) use candle_highwave::candle_high_wave;
#[allow(unused_imports)]
pub(crate) use candle_homingpigeon::candle_homing_pigeon;
#[allow(unused_imports)]
pub(crate) use candle_identical3crows::candle_identical_three_crows;
#[allow(unused_imports)]
pub(crate) use candle_inneck::candle_in_neck;
#[allow(unused_imports)]
pub(crate) use candle_invertedhammer::candle_inverted_hammer;
#[allow(unused_imports)]
pub(crate) use candle_kicking::candle_kicking;
#[allow(unused_imports)]
pub(crate) use candle_kickingbylength::candle_kicking_by_length;
#[allow(unused_imports)]
pub(crate) use candle_ladderbottom::candle_ladder_bottom;
#[allow(unused_imports)]
pub(crate) use candle_longleggeddoji::candle_long_legged_doji;
#[allow(unused_imports)]
pub(crate) use candle_longline::candle_long_line;
#[allow(unused_imports)]
pub(crate) use candle_marubozu::candle_marubozu;
#[allow(unused_imports)]
pub(crate) use candle_matchinglow::candle_matching_low;
#[allow(unused_imports)]
pub(crate) use candle_mathold::candle_mat_hold;
#[allow(unused_imports)]
pub(crate) use candle_morningdojistar::candle_morning_doji_star;
#[allow(unused_imports)]
pub(crate) use candle_morningstar::candle_morning_star;
#[allow(unused_imports)]
pub(crate) use candle_onneck::candle_on_neck;
#[allow(unused_imports)]
pub(crate) use candle_piercing::candle_piercing;
#[allow(unused_imports)]
pub(crate) use candle_rickshawman::candle_rickshawman;
#[allow(unused_imports)]
pub(crate) use candle_risefall3methods::candle_rise_fall_three_methods;
#[allow(unused_imports)]
pub(crate) use candle_separatinglines::candle_separating_lines;
#[allow(unused_imports)]
pub(crate) use candle_shootingstar::candle_shooting_star;
#[allow(unused_imports)]
pub(crate) use candle_shortline::candle_short_line;
#[allow(unused_imports)]
pub(crate) use candle_spinningtop::candle_spinningtop;
#[allow(unused_imports)]
pub(crate) use candle_stalledpattern::candle_stalled_pattern;
#[allow(unused_imports)]
pub(crate) use candle_takuri::candle_takuri;
#[allow(unused_imports)]
pub(crate) use candle_tasukigap::candle_tasuki_gap;
#[allow(unused_imports)]
pub(crate) use candle_thrusting::candle_thrusting;
#[allow(unused_imports)]
pub(crate) use candle_tristar::candle_tri_star;
#[allow(unused_imports)]
pub(crate) use candle_unique3river::candle_unique_three_river;
#[allow(unused_imports)]
pub(crate) use candle_upsidegap2crows::candle_upside_gap_two_crows;
#[allow(unused_imports)]
pub(crate) use candle_xsidegap3methods::candle_xside_gap_three_methods;

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
        let max_expected = crate::stream::rolling_max(&input, period).unwrap();
        let min_expected = crate::stream::rolling_min(&input, period).unwrap();
        let mut sum_batch_state = RollingSum::new(period).unwrap();
        let mut sum_expected = Vec::new();
        sum_batch_state.extend_slice_into(&input, &mut sum_expected);
        let maxindex_expected = crate::stream::rolling_argmax(&input, period).unwrap();
        let minindex_expected = crate::stream::rolling_argmin(&input, period).unwrap();
        let (minmax_min, minmax_max) = crate::stream::rolling_minmax(&input, period).unwrap();
        let (minidx, maxidx) = crate::stream::rolling_minmax_index(&input, period).unwrap();
        let mut max = RollingMax::new(period).unwrap();
        let mut min = RollingMin::new(period).unwrap();
        let mut sum = RollingSum::new(period).unwrap();
        let mut maxindex = RollingArgmax::new(period).unwrap();
        let mut minindex = RollingArgmin::new(period).unwrap();
        let mut minmax = RollingMinmax::new(period).unwrap();
        let mut minmaxindex = RollingMinmaxIndex::new(period).unwrap();

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
        let beta_expected = crate::stream::rolling_beta(&market, &asset, period).unwrap();
        let correl_expected = crate::stream::rolling_corr(&market, &asset, period).unwrap();
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
        let linearreg_expected = crate::stream::rolling_linreg(&input, period).unwrap();
        let slope_expected = crate::stream::rolling_linreg_slope(&input, period).unwrap();
        let intercept_expected = crate::stream::rolling_linreg_intercept(&input, period).unwrap();
        let angle_expected = crate::stream::rolling_linreg_angle(&input, period).unwrap();
        let tsf_expected = crate::stream::rolling_tsf(&input, period).unwrap();
        let mut linearreg = Linearreg::new(period).unwrap();
        let mut slope = LinearregSlope::new(period).unwrap();
        let mut intercept = LinearregIntercept::new(period).unwrap();
        let mut angle = LinearregAngle::new(period).unwrap();
        let mut tsf = Tsf::new(period).unwrap();
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
mod williams_percent_r;
#[cfg(test)]
mod williams_percent_r_test;
#[allow(unused_imports)]
pub(crate) use drawdown::drawdown;
mod rolling_sharpe;
#[allow(unused_imports)]
pub(crate) use rolling_sharpe::rolling_sharpe;
pub use rolling_sharpe::RollingSharpe;
mod rolling_sortino;
#[allow(unused_imports)]
pub(crate) use rolling_sortino::rolling_sortino;
pub use rolling_sortino::RollingSortino;
mod rolling_calmar;
#[allow(unused_imports)]
pub(crate) use rolling_calmar::rolling_calmar;
mod hull_moving_average;
#[allow(unused_imports)]
pub(crate) use hull_moving_average::hull_moving_average;
mod volume_weighted_moving_average;
#[allow(unused_imports)]
pub(crate) use volume_weighted_moving_average::volume_weighted_moving_average;
mod zero_lag_exponential_moving_average;
#[allow(unused_imports)]
pub(crate) use zero_lag_exponential_moving_average::zero_lag_exponential_moving_average;
mod arnaud_legoux_moving_average;
#[allow(unused_imports)]
pub(crate) use arnaud_legoux_moving_average::arnaud_legoux_moving_average;
mod true_strength_index;
#[allow(unused_imports)]
pub(crate) use true_strength_index::true_strength_index;
mod awesome_oscillator;
#[allow(unused_imports)]
pub(crate) use awesome_oscillator::awesome_oscillator;
mod fisher_transform;
pub use fisher_transform::FisherTransform;
mod ulcer_index;
#[allow(unused_imports)]
pub(crate) use ulcer_index::ulcer_index;
mod chaikin_volatility;
#[allow(unused_imports)]
pub(crate) use chaikin_volatility::chaikin_volatility;
mod rolling_volume_weighted_average_price;
#[allow(unused_imports)]
pub(crate) use rolling_volume_weighted_average_price::rolling_volume_weighted_average_price;
mod force_index;
#[allow(unused_imports)]
pub(crate) use force_index::force_index;
mod ease_of_movement;
#[allow(unused_imports)]
pub(crate) use ease_of_movement::ease_of_movement;
mod rising;
#[allow(unused_imports)]
pub(crate) use rising::rising;
pub use rising::Rising;
mod falling;
#[allow(unused_imports)]
pub(crate) use falling::falling;
pub use falling::Falling;
mod rolling_entropy;
#[allow(unused_imports)]
pub(crate) use rolling_entropy::rolling_entropy;
mod rolling_autocorr;
#[allow(unused_imports)]
pub(crate) use rolling_autocorr::rolling_autocorr;
mod hurst;
#[allow(unused_imports)]
pub(crate) use hurst::hurst;
mod fractal_dimension;
#[allow(unused_imports)]
pub(crate) use fractal_dimension::fractal_dimension;
mod rolling_alpha;
#[allow(unused_imports)]
pub(crate) use rolling_alpha::rolling_alpha;
mod rolling_information_ratio;
#[allow(unused_imports)]
pub(crate) use rolling_information_ratio::rolling_information_ratio;
mod hedge_ratio;
#[allow(unused_imports)]
pub(crate) use hedge_ratio::hedge_ratio;
mod session_extrema;
#[allow(unused_imports)]
pub(crate) use session_extrema::session_extrema;
mod fair_value_gap;
#[allow(unused_imports)]
pub(crate) use fair_value_gap::fair_value_gap;
mod break_of_structure_change_of_character;
#[allow(unused_imports)]
pub(crate) use break_of_structure_change_of_character::break_of_structure_change_of_character;
mod order_block;
#[allow(unused_imports)]
pub(crate) use order_block::order_block;
mod liquidity;
#[allow(unused_imports)]
pub(crate) use liquidity::liquidity;
mod equal_highs_lows;
#[allow(unused_imports)]
pub(crate) use equal_highs_lows::equal_highs_lows;
mod previous_high_low;
#[allow(unused_imports)]
pub(crate) use previous_high_low::previous_high_low;
mod sessions;
#[allow(unused_imports)]
pub(crate) use sessions::sessions;
mod retracements;
#[allow(unused_imports)]
pub(crate) use retracements::retracements;
mod close_to_close_sigma;
#[allow(unused_imports)]
pub(crate) use close_to_close_sigma::close_to_close_sigma;
mod parkinson;
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
mod swing_highs_lows;
#[allow(unused_imports)]
pub(crate) use swing_highs_lows::swing_highs_lows;
mod rolling_quantile;
#[allow(unused_imports)]
pub(crate) use rolling_quantile::rolling_quantile;
mod rolling_percentile;
#[allow(unused_imports)]
pub(crate) use rolling_percentile::rolling_percentile;
mod rolling_rank;
#[allow(unused_imports)]
pub(crate) use rolling_rank::rolling_rank;
mod rolling_zscore;
#[allow(unused_imports)]
pub(crate) use rolling_zscore::rolling_zscore;
mod rolling_skew;
#[allow(unused_imports)]
pub(crate) use rolling_skew::rolling_skew;
pub use rolling_skew::RollingSkew;
mod rolling_kurtosis;
#[allow(unused_imports)]
pub(crate) use rolling_kurtosis::rolling_kurtosis;
pub use rolling_kurtosis::RollingKurtosis;
mod rolling_iqr;
#[allow(unused_imports)]
pub(crate) use rolling_iqr::rolling_iqr;
mod rolling_cov;
#[allow(unused_imports)]
pub(crate) use rolling_cov::rolling_cov;
mod rolling_winsorize;
#[allow(unused_imports)]
pub(crate) use rolling_winsorize::rolling_winsorize;
mod ewm_var;
#[allow(unused_imports)]
pub(crate) use ewm_var::ewm_var;
mod ewm_sum;
#[allow(unused_imports)]
pub(crate) use ewm_sum::ewm_sum;
pub use ewm_sum::ExponentiallyWeightedSum;
mod ewm_std;
#[allow(unused_imports)]
pub(crate) use ewm_std::ewm_std;
mod ewm_cov;
#[allow(unused_imports)]
pub(crate) use ewm_cov::ewm_cov;
mod ewm_corr;
#[allow(unused_imports)]
pub(crate) use ewm_corr::ewm_corr;
mod mass_index;
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
#[allow(unused_imports)]
#[allow(unused_imports)]
pub(crate) use rolling_mode::rolling_mode;
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
pub use ewm_corr::ExponentiallyWeightedCorrelation;
pub use ewm_cov::ExponentiallyWeightedCovariance;
pub use ewm_std::ExponentiallyWeightedStandardDeviation;
pub use ewm_var::ExponentiallyWeightedVariance;
pub use fair_value_gap::{FairValueGap, FairValueGapValue};
pub use frac_diff::FracDiff;
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
pub use rogers_satchell::RogersSatchell;
pub use roll_spread::RollSpread;
pub use rolling_alpha::RollingAlpha;
pub use rolling_autocorr::RollingAutocorr;
pub use rolling_cov::RollingCov;
pub use rolling_entropy::RollingEntropy;
pub use rolling_information_ratio::RollingInformationRatio;
pub use rolling_iqr::RollingInterquartileRange;
pub use rolling_quantile::RollingQuantile;
pub use rolling_rank::RollingRank;
pub use rolling_winsorize::RollingWinsorize;
pub use rolling_zscore::RollingZScore;
pub use schaff_trend_cycle::{SchaffTrendCycle, SchaffTrendCycleValue};
pub use session_extrema::{SessionExtrema, SessionExtremaValue};
pub use sessions::{Sessions, SessionsValue};
pub use spread_zscore::SpreadZScore;
pub use squeeze::{Squeeze, SqueezeValue};
pub use squeeze_pro::{SqueezePro, SqueezeProValue};
pub use supertrend::{Supertrend, SupertrendValue};
pub use swing_highs_lows::{SwingHighLow, SwingValue};
pub use true_strength_index::TrueStrengthIndex;
pub use value_when::ValueWhen;
pub use volume_weighted_moving_average::VolumeWeightedMovingAverage;
pub use vortex::{Vortex, VortexValue};
pub use yang_zhang::YangZhang;
pub use zero_lag_exponential_moving_average::ZeroLagExponentialMovingAverage;
mod donchian;
pub use donchian::{Donchian, DonchianValue};
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
pub use rolling_corr::RollingCorrelation;
pub use rolling_midpoint::RollingMidpoint;
pub use rolling_midprice::RollingMidprice;
pub use rolling_minmax::{RollingMinmax, RollingMinmaxValue};
pub use rolling_minmax_index::{RollingMinmaxIndex, RollingMinmaxIndexValue};
pub use rolling_standard_deviation::RollingStandardDeviation;
pub use rolling_variance::RollingVariance;
pub use true_range::TrueRange;
pub use volume_price_trend::VolumePriceTrend;
pub use williams_percent_r::WilliamsPercentR;
