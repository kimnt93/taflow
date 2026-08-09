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

mod accbands;
mod adx;
mod adxr;
mod apo;
mod aroon;
#[allow(unused_imports)]
pub(crate) use aroon::aroon;
mod aroon_true_range;
mod bbands;
mod candle_2crows;
mod candle_3blackcrows;
mod candle_3inside;
mod candle_3linestrike;
mod candle_3outside;
mod candle_3starsinsouth;
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
mod candle_hangingman;
mod candle_harami;
mod candle_haramicross;
mod candle_highwave;
mod candle_hikkake;
mod candle_hikkakemod;
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
mod candle_sticksandwich;
mod candle_takuri;
mod candle_tasukigap;
mod candle_thrusting;
mod candle_tristar;
mod candle_unique3river;
mod candle_upsidegap2crows;
mod candle_xsidegap3methods;
mod cci;
mod cmo;
mod cycle;
mod dema;
mod directional;
mod dx;
mod ema;
mod ht_dcperiod;
mod ht_dcphase;
mod ht_phasor;
mod ht_sine;
mod ht_trendline;
mod ht_trendmode;
mod imi;
mod indicator;
mod kama;
mod ma;
mod macd;
mod macdext;
mod macdfix;
mod mama;
mod math_abs;
mod math_operator;
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
#[cfg(test)]
mod math_tanh_test;
mod mfi;
mod minus_di;
mod minus_dm;
mod moving_average;
mod pattern;
mod plus_di;
mod plus_dm;
mod ppo;
mod price_transform;
mod regression;
mod rolling_extrema;
mod rolling_median;
mod rolling_mode;
mod rolling_price;
mod rolling_statistics;
mod rolling_sum;
mod rsi;
mod session_flags;
pub(crate) mod sorted_ring;
mod statistic;
mod variable_period_moving_average;
#[cfg(test)]
mod variable_period_moving_average_test;
mod vhgw;
mod volume_states;
pub use session_flags::session_flags;
mod cumulative_count;
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
pub(crate) use cumulative_count::cumulative_count;
pub use cumulative_count::CumulativeCount;
pub use cumulative_maximum::CumulativeMaximum;
pub use cumulative_minimum::CumulativeMinimum;
pub use cumulative_product::CumulativeProduct;
pub use cumulative_sum::CumulativeSum;
mod anchored_volume_weighted_average_price;
mod even_better_sinewave;
mod fibonacci_retracement;
mod heikin_ashi;
mod helpers;
mod jurik_moving_average;
mod klinger_volume_oscillator;
mod lag;
#[cfg(test)]
mod lag_test;
mod lagged;
mod lagged_common;
mod laguerre_rsi;
mod log_return;
#[cfg(test)]
mod log_return_test;
mod momentum;
mod opening_range;
mod operator_states;
pub use operator_states::ActiveZoneList;
mod parabolic_moving_average_stop;
mod pivot_points;
mod premium_discount;
mod rate_of_change;
mod rate_of_change_percent;
mod rate_of_change_ratio;
mod rate_of_change_ratio_percent;
mod rmi;
mod sar;
mod sarext;
mod session_volume_levels;
mod sma;
mod ssl_channel;
mod stoch;
mod stochf;
mod stochrsi;
mod t3;
mod tema;
mod tom_de_mark_sequential;
mod trima;
mod trix;
mod ultosc;
mod vidya;
mod window;
#[allow(unused_imports)]
pub(crate) use helpers::invalid_period;
mod wma;

#[allow(unused_imports)]
pub(crate) use accbands::acceleration_bands;
pub use accbands::{AccelerationBands, AccelerationBandsValue};
#[allow(unused_imports)]
pub(crate) use adx::average_directional_index;
pub use adx::AverageDirectionalIndex;
#[allow(unused_imports)]
pub(crate) use adxr::average_directional_index_rating;
pub use adxr::AverageDirectionalIndexRating;
pub use anchored_volume_weighted_average_price::AnchoredVolumeWeightedAveragePrice;
#[allow(unused_imports)]
pub(crate) use apo::absolute_price_oscillator;
pub use apo::AbsolutePriceOscillator;

#[allow(unused_imports)]
pub(crate) use bbands::bollinger_bands;
pub use bbands::{BollingerBands, BollingerBandsValue};
pub use candle_2crows::CandleTwoCrows;
pub use candle_3blackcrows::CandleThreeBlackCrows;
pub use candle_3inside::CandleThreeInside;
pub use candle_3linestrike::CandleThreeLineStrike;
pub use candle_3outside::CandleThreeOutside;
pub use candle_3starsinsouth::CandleThreeStarsInSouth;
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
pub use candle_hikkakemod::CandleHikkakeModified;
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
pub use candle_sticksandwich::CandleStickSandwich;
pub use candle_takuri::CandleTakuri;
pub use candle_tasukigap::CandleTasukiGap;
pub use candle_thrusting::CandleThrusting;
pub use candle_tristar::CandleTriStar;
pub use candle_unique3river::CandleUniqueThreeRiver;
pub use candle_upsidegap2crows::CandleUpsideGapTwoCrows;
pub use candle_xsidegap3methods::CandleUpDownSideGapThreeMethods;
#[allow(unused_imports)]
pub(crate) use cci::commodity_channel_index;
pub use cci::CommodityChannelIndex;
#[allow(unused_imports)]
pub(crate) use cmo::chande_momentum_oscillator;
pub use cmo::ChandeMomentumOscillator;
#[allow(unused_imports)]
pub(crate) use dema::double_exponential_moving_average;
pub use dema::DoubleExponentialMovingAverage;
#[allow(unused_imports)]
pub(crate) use dx::directional_movement_index;
pub use dx::DirectionalMovementIndex;
#[allow(unused_imports)]
pub(crate) use ema::exponential_moving_average;
pub use ema::ExponentialMovingAverage;
pub use even_better_sinewave::EvenBetterSinewave;
pub use fibonacci_retracement::FibonacciRetracement;
pub use heikin_ashi::HeikinAshi;
pub use ht_dcperiod::HilbertTransformDominantCyclePeriod;
pub use ht_dcphase::HilbertTransformDominantCyclePhase;
pub use ht_phasor::{HilbertTransformPhasor, HilbertTransformPhasorValue};
pub use ht_sine::{HilbertTransformSineWave, HilbertTransformSineWaveValue};
#[allow(unused_imports)]
pub(crate) use ht_trendline::hilbert_transform_trendline;
pub use ht_trendline::HilbertTransformTrendline;
pub use ht_trendmode::HilbertTransformTrendMode;
#[allow(unused_imports)]
pub(crate) use imi::intraday_momentum_index;
pub use imi::IntradayMomentumIndex;
pub use indicator::StreamingIndicator;
pub use jurik_moving_average::JurikMovingAverage;
#[allow(unused_imports)]
pub(crate) use kama::kaufman_adaptive_moving_average;
pub use kama::KaufmanAdaptiveMovingAverage;
pub use klinger_volume_oscillator::KlingerVolumeOscillator;
pub use lag::Lag;
#[allow(unused_imports)]
pub(crate) use lagged::{
    momentum, rate_of_change, rate_of_change_percent, rate_of_change_ratio,
    rate_of_change_ratio_percent,
};
pub use lagged::{
    Momentum, RateOfChange, RateOfChangePercent, RateOfChangeRatio, RateOfChangeRatioPercent,
};
pub use laguerre_rsi::LaguerreRelativeStrengthIndex;
pub use log_return::LogReturn;
#[allow(unused_imports)]
pub(crate) use ma::moving_average;
pub use ma::MovingAverage;
#[allow(unused_imports)]
pub(crate) use macd::moving_average_convergence_divergence;
pub use macd::{MovingAverageConvergenceDivergence, MovingAverageConvergenceDivergenceValue};
#[allow(unused_imports)]
pub(crate) use macdext::moving_average_convergence_divergence_extended;
pub use macdext::MovingAverageConvergenceDivergenceExtended;
#[allow(unused_imports)]
pub(crate) use macdfix::moving_average_convergence_divergence_fixed;
pub use macdfix::MovingAverageConvergenceDivergenceFixed;
#[allow(unused_imports)]
pub(crate) use mama::mesa_adaptive_moving_average;
pub use mama::{MesaAdaptiveMovingAverage, MesaAdaptiveMovingAverageValue};
#[allow(unused_imports)]
pub(crate) use math_operator::rolling_sum;
#[allow(unused_imports)]
pub(crate) use mfi::money_flow_index;
pub use mfi::MoneyFlowIndex;
#[allow(unused_imports)]
pub(crate) use minus_di::minus_directional_indicator;
pub use minus_di::MinusDirectionalIndicator;
#[allow(unused_imports)]
pub(crate) use minus_dm::minus_directional_movement;
pub use minus_dm::MinusDirectionalMovement;
pub use opening_range::OpeningRange;
pub use parabolic_moving_average_stop::ParabolicMovingAverageStop;
pub use pivot_points::PivotPoints;
#[allow(unused_imports)]
pub(crate) use plus_di::plus_directional_indicator;
pub use plus_di::PlusDirectionalIndicator;
#[allow(unused_imports)]
pub(crate) use plus_dm::plus_directional_movement;
pub use plus_dm::PlusDirectionalMovement;
#[allow(unused_imports)]
pub(crate) use ppo::percentage_price_oscillator;
pub use ppo::PercentagePriceOscillator;
pub use premium_discount::PremiumDiscount;
pub use regression::{Linearreg, LinearregAngle, LinearregIntercept, LinearregSlope, Tsf};
pub use rmi::RelativeMomentumIndex;
#[allow(unused_imports)]
pub(crate) use rolling_extrema::{MonotonicMax, MonotonicMin, RollingExtrema};
pub use rolling_extrema::{RollingArgmax, RollingArgmin, RollingMax, RollingMin};
pub use rolling_median::RollingMedian;
pub use rolling_mode::RollingMode;
pub use variable_period_moving_average::VariablePeriodMovingAverage;

pub use rolling_sum::RollingSum;
#[allow(unused_imports)]
pub(crate) use rsi::relative_strength_index;
pub use rsi::RelativeStrengthIndex;
#[allow(unused_imports)]
pub(crate) use sar::parabolic_sar;
pub use sar::ParabolicSar;
#[allow(unused_imports)]
pub(crate) use sarext::parabolic_sar_extended;
pub use sarext::ParabolicSarExtended;
pub use session_volume_levels::SessionVolumeLevels;
#[allow(unused_imports)]
pub(crate) use sma::simple_moving_average;
pub use sma::SimpleMovingAverage;
pub use ssl_channel::SmoothedTrendChannel;
#[allow(unused_imports)]
pub(crate) use stoch::stochastic_oscillator;
pub use stoch::{StochasticOscillator, StochasticOscillatorValue};
#[allow(unused_imports)]
pub(crate) use stochf::fast_stochastic_oscillator;
pub use stochf::{FastStochasticOscillator, FastStochasticOscillatorValue};
#[allow(unused_imports)]
pub(crate) use stochrsi::stochastic_relative_strength_index;
pub use stochrsi::{StochasticRelativeStrengthIndex, StochasticRelativeStrengthIndexValue};
#[allow(unused_imports)]
pub(crate) use t3::triple_exponential_average;
pub use t3::TripleExponentialAverage;
#[allow(unused_imports)]
pub(crate) use tema::triple_exponential_moving_average;
pub use tema::TripleExponentialMovingAverage;
pub use tom_de_mark_sequential::TomDeMarkSequential;
#[allow(unused_imports)]
pub(crate) use trima::triangular_moving_average;
pub use trima::TriangularMovingAverage;
#[allow(unused_imports)]
pub(crate) use trix::triple_exponential_rate_of_change;
pub use trix::TripleExponentialRateOfChange;
#[allow(unused_imports)]
pub(crate) use ultosc::ultimate_oscillator;
pub use ultosc::UltimateOscillator;
pub use vidya::VariableIndexDynamicAverage;
pub use window::Window;
#[allow(unused_imports)]
pub(crate) use wma::weighted_moving_average;
pub use wma::WeightedMovingAverage;

mod true_range;
#[allow(unused_imports)]
pub(crate) use true_range::true_range;
mod average_true_range;
#[allow(unused_imports)]
pub(crate) use average_true_range::average_true_range;
mod normalized_average_true_range;
#[allow(unused_imports)]
pub(crate) use normalized_average_true_range::normalized_average_true_range;
mod math_add;
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
mod rolling_avgdev;
#[cfg(test)]
mod weighted_close_test;
#[allow(unused_imports)]
pub(crate) use rolling_avgdev::rolling_avgdev;
mod rolling_std;
#[allow(unused_imports)]
pub(crate) use rolling_std::rolling_std;
mod rolling_var;
#[allow(unused_imports)]
pub(crate) use rolling_var::rolling_var;
mod rolling_beta;
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
mod hilbert_transform_dominant_cycle_period;
#[allow(unused_imports)]
pub(crate) use hilbert_transform_dominant_cycle_period::hilbert_transform_dominant_cycle_period;
mod hilbert_transform_phasor;
#[allow(unused_imports)]
pub(crate) use hilbert_transform_phasor::hilbert_transform_phasor;
mod hilbert_transform_dominant_cycle_phase;
#[allow(unused_imports)]
pub(crate) use hilbert_transform_dominant_cycle_phase::hilbert_transform_dominant_cycle_phase;
mod hilbert_transform_sine_wave;
#[allow(unused_imports)]
pub(crate) use hilbert_transform_sine_wave::hilbert_transform_sine_wave;
mod hilbert_transform_trend_mode;
#[allow(unused_imports)]
pub(crate) use hilbert_transform_trend_mode::hilbert_transform_trend_mode;
mod rolling_midpoint;
#[allow(unused_imports)]
pub(crate) use rolling_midpoint::rolling_midpoint;
mod rolling_midprice;
#[allow(unused_imports)]
pub(crate) use candle_2crows::candle_two_crows;
#[allow(unused_imports)]
pub(crate) use candle_3blackcrows::candle_three_black_crows;
#[allow(unused_imports)]
pub(crate) use candle_3inside::candle_three_inside;
#[allow(unused_imports)]
pub(crate) use candle_3linestrike::candle_three_line_strike;
#[allow(unused_imports)]
pub(crate) use candle_3outside::candle_three_outside;
#[allow(unused_imports)]
pub(crate) use candle_3starsinsouth::candle_three_stars_in_south;
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
pub(crate) use candle_hammer::candle_hammer;
#[allow(unused_imports)]
pub(crate) use candle_hangingman::candle_hanging_man;
#[allow(unused_imports)]
pub(crate) use candle_harami::candle_harami;
#[allow(unused_imports)]
pub(crate) use candle_haramicross::candle_harami_cross;
#[allow(unused_imports)]
pub(crate) use candle_highwave::candle_high_wave;
#[allow(unused_imports)]
pub(crate) use candle_hikkake::candle_hikkake;
#[allow(unused_imports)]
pub(crate) use candle_hikkakemod::candle_hikkake_modified;
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
pub(crate) use candle_sticksandwich::candle_stick_sandwich;
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
#[allow(unused_imports)]
pub(crate) use rolling_midprice::rolling_midprice;

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
        let sma_batch = simple_moving_average(&input, 7).unwrap();
        let ema_batch = exponential_moving_average(&input, 7).unwrap();
        let wma_batch = weighted_moving_average(&input, 7).unwrap();
        let dema_batch = double_exponential_moving_average(&input, 7).unwrap();
        let tema_batch = triple_exponential_moving_average(&input, 7).unwrap();
        let trima_batch = triangular_moving_average(&input, 7).unwrap();
        let kama_batch = kaufman_adaptive_moving_average(&input, 7).unwrap();
        let midpoint_batch = rolling_midpoint(&input, 7).unwrap();
        let rsi_batch = relative_strength_index(&input, 14).unwrap();
        let cmo_batch = chande_momentum_oscillator(&input, 14).unwrap();
        let mom_batch = momentum(&input, 7).unwrap();
        let roc_batch = rate_of_change(&input, 7).unwrap();
        let rocp_batch = rate_of_change_percent(&input, 7).unwrap();
        let rocr_batch = rate_of_change_ratio(&input, 7).unwrap();
        let rocr100_batch = rate_of_change_ratio_percent(&input, 7).unwrap();
        let mut sma = SimpleMovingAverage::new(7).unwrap();
        let mut ema = ExponentialMovingAverage::new(7).unwrap();
        let mut wma = WeightedMovingAverage::new(7).unwrap();
        let mut dema = DoubleExponentialMovingAverage::new(7).unwrap();
        let mut tema = TripleExponentialMovingAverage::new(7).unwrap();
        let mut trima = TriangularMovingAverage::new(7).unwrap();
        let mut kama = KaufmanAdaptiveMovingAverage::new(7).unwrap();
        let mut midpoint = RollingMidpoint::new(7).unwrap();
        let mut rsi = RelativeStrengthIndex::new(14).unwrap();
        let mut cmo = ChandeMomentumOscillator::new(14).unwrap();
        let mut mom = Momentum::new(7).unwrap();
        let mut roc = RateOfChange::new(7).unwrap();
        let mut rocp = RateOfChangePercent::new(7).unwrap();
        let mut rocr = RateOfChangeRatio::new(7).unwrap();
        let mut rocr100 = RateOfChangeRatioPercent::new(7).unwrap();

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
            assert_optional_eq(rsi.append(value), rsi_batch[index]);
            assert_optional_eq(cmo.append(value), cmo_batch[index]);
            assert_optional_eq(mom.append(value), mom_batch[index]);
            assert_optional_eq(roc.append(value), roc_batch[index]);
            assert_optional_eq(rocp.append(value), rocp_batch[index]);
            assert_optional_eq(rocr.append(value), rocr_batch[index]);
            assert_optional_eq(rocr100.append(value), rocr100_batch[index]);
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
        let expected = rolling_midprice(&high, &low, 7).unwrap();
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
        let sum_expected = crate::stream::rolling_sum(&input, period).unwrap();
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
        let avgdev_expected = crate::stream::rolling_avgdev(&input, period).unwrap();
        let var_expected = crate::stream::rolling_var(&input, period, 2.0).unwrap();
        let stddev_expected = crate::stream::rolling_std(&input, period, 2.0).unwrap();
        let mut avgdev = RollingAverageDeviation::new(period).unwrap();
        let mut var = RollingVariance::new(period, 2.0).unwrap();
        let mut stddev = RollingStandardDeviation::new(period, 2.0).unwrap();
        for index in 0..input.len() {
            assert_optional_eq(avgdev.append(input[index]), avgdev_expected[index]);
            assert_optional_eq(var.append(input[index]), var_expected[index]);
            assert_optional_eq(stddev.append(input[index]), stddev_expected[index]);
        }

        let constant = vec![42.0; 30];
        let expected = crate::stream::rolling_std(&constant, 5, 3.0).unwrap();
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

    #[test]
    fn volume_states_match_batch_for_every_bar() {
        let close: Vec<f64> = (0..100)
            .map(|index| 50.0 + index as f64 * 0.06 + (index as f64 * 0.25).sin() * 3.0)
            .collect();
        let mut high: Vec<f64> = close.iter().map(|value| value + 1.2).collect();
        let mut low: Vec<f64> = close.iter().map(|value| value - 0.8).collect();
        for index in (11..close.len()).step_by(19) {
            high[index] = close[index];
            low[index] = close[index];
        }
        let volumes: Vec<f64> = (0..close.len())
            .map(|index| 1_000.0 + (index % 13) as f64 * 37.0)
            .collect();
        let ad_expected = accumulation_distribution(&high, &low, &close, &volumes).unwrap();
        let adosc_expected =
            accumulation_distribution_oscillator(&high, &low, &close, &volumes, 3, 10).unwrap();
        let mut ad = AccumulationDistribution::new();
        let mut adosc = AccumulationDistributionOscillator::new(3, 10).unwrap();
        for index in 0..close.len() {
            assert_eq!(
                ad.append(high[index], low[index], close[index], volumes[index]),
                ad_expected[index]
            );
            assert_optional_eq(
                adosc.append(high[index], low[index], close[index], volumes[index]),
                adosc_expected[index],
            );
        }
    }

    #[test]
    fn rolling_ohlc_momentum_states_match_batch_for_every_bar() {
        let close: Vec<f64> = (0..100)
            .map(|index| 70.0 + index as f64 * 0.04 + (index as f64 * 0.27).sin() * 4.0)
            .collect();
        let open: Vec<f64> = close
            .iter()
            .enumerate()
            .map(|(index, close)| close + (index as f64 * 0.11).cos() * 0.7)
            .collect();
        let mut high: Vec<f64> = open
            .iter()
            .zip(&close)
            .map(|(open, close)| open.max(*close) + 1.0)
            .collect();
        let mut low: Vec<f64> = open
            .iter()
            .zip(&close)
            .map(|(open, close)| open.min(*close) - 0.8)
            .collect();
        for index in (9..close.len()).step_by(17) {
            high[index] = close[index];
            low[index] = close[index];
        }
        let period = 14;
        let willr_expected = williams_percent_r(&high, &low, &close, period).unwrap();
        let (down_expected, up_expected) = crate::stream::aroon(&high, &low, period).unwrap();
        let osc_expected = crate::stream::aroon_oscillator(&high, &low, period).unwrap();
        let mut willr = WilliamsPercentR::new(period).unwrap();
        let mut aroon = Aroon::new(period).unwrap();
        let mut oscillator = AroonOscillator::new(period).unwrap();
        for index in 0..close.len() {
            assert_optional_eq(
                willr.append(high[index], low[index], close[index]),
                willr_expected[index],
            );
            match aroon.append(high[index], low[index]) {
                Some(value) => {
                    assert_eq!(value.down, down_expected[index]);
                    assert_eq!(value.up, up_expected[index]);
                }
                None => {
                    assert!(down_expected[index].is_nan());
                    assert!(up_expected[index].is_nan());
                }
            }
            assert_optional_eq(
                oscillator.append(high[index], low[index]),
                osc_expected[index],
            );
        }
    }

    #[test]
    fn atr_and_macd_match_batch_for_every_bar() {
        let close: Vec<f64> = (0..90)
            .map(|i| 100.0 + (i as f64 * 0.21).cos() * 4.0 + i as f64 * 0.1)
            .collect();
        let high: Vec<f64> = close.iter().map(|v| v + 1.5).collect();
        let low: Vec<f64> = close.iter().map(|v| v - 1.0).collect();
        let atr_batch = average_true_range(&high, &low, &close, 14).unwrap();
        let trange_batch = true_range(&high, &low, &close).unwrap();
        let natr_batch = normalized_average_true_range(&high, &low, &close, 14).unwrap();
        let (macd_batch, signal_batch, histogram_batch) =
            crate::stream::moving_average_convergence_divergence(&close, 12, 26, 9).unwrap();
        let mut atr = AverageTrueRange::new(14).unwrap();
        let mut trange = TrueRange::new();
        let mut natr = NormalizedAverageTrueRange::new(14).unwrap();
        let mut macd = MovingAverageConvergenceDivergence::new(12, 26, 9).unwrap();

        for i in 0..close.len() {
            assert_optional_eq(atr.append(high[i], low[i], close[i]), atr_batch[i]);
            assert_optional_eq(trange.append(high[i], low[i], close[i]), trange_batch[i]);
            assert_optional_eq(natr.append(high[i], low[i], close[i]), natr_batch[i]);
            let actual = macd.append(close[i]);
            if macd_batch[i].is_nan() {
                assert_eq!(actual, None);
            } else {
                let actual = actual.expect("expected a warm MACD value");
                assert!((actual.macd - macd_batch[i]).abs() < 1e-12);
                assert!((actual.signal - signal_batch[i]).abs() < 1e-12);
                assert!((actual.histogram - histogram_batch[i]).abs() < 1e-12);
            }
        }
    }

    #[test]
    fn reset_replays_identically() {
        let input = [1.0, 2.0, 4.0, 8.0, 16.0];
        let mut state = ExponentialMovingAverage::new(3).unwrap();
        let first: Vec<_> = input.iter().map(|&v| state.append(v)).collect();
        state.reset();
        let second: Vec<_> = input.iter().map(|&v| state.append(v)).collect();
        assert_eq!(first, second);
    }
}
mod on_balance_volume;
#[cfg(test)]
mod on_balance_volume_test;

mod accumulation_distribution;
#[allow(unused_imports)]
pub(crate) use accumulation_distribution::accumulation_distribution;
mod accumulation_distribution_oscillator;
#[allow(unused_imports)]
pub(crate) use accumulation_distribution_oscillator::accumulation_distribution_oscillator;
mod balance_of_power;
#[cfg(test)]
mod balance_of_power_test;
mod williams_percent_r;
#[allow(unused_imports)]
pub(crate) use williams_percent_r::williams_percent_r;
mod drawdown;
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
pub(crate) use rolling_median::rolling_median;
#[allow(unused_imports)]
pub(crate) use rolling_mode::rolling_mode;
mod aroon_oscillator;
#[allow(unused_imports)]
pub(crate) use aroon_oscillator::aroon_oscillator;

mod bar_relation;
mod bars_since;
mod gap_down;
mod gap_up;
mod higher_high;
mod highest_since;
mod inside_bar;
mod lower_low;
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
pub use crossover::Crossover;
mod crossunder;
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
pub use rolling_avgdev::RollingAverageDeviation;
pub use rolling_beta::RollingBeta;
pub use rolling_calmar::RollingCalmar;
pub use rolling_corr::RollingCorrelation;
pub use rolling_midpoint::RollingMidpoint;
pub use rolling_midprice::RollingMidprice;
pub use rolling_minmax::{RollingMinmax, RollingMinmaxValue};
pub use rolling_minmax_index::{RollingMinmaxIndex, RollingMinmaxIndexValue};
pub use rolling_std::RollingStandardDeviation;
pub use rolling_var::RollingVariance;
pub use true_range::TrueRange;
pub use volume_price_trend::VolumePriceTrend;
pub use williams_percent_r::WilliamsPercentR;
