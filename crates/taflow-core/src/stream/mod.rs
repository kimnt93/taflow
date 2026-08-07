//! Persistent technical indicators for bulk history and realtime continuation.
//!
//! Each TA implementation lives in its own module and retains only the bounded
//! recurrence state required to process newly appended bars.

use crate::error::TaError;

mod accbands;
mod math_operator;
mod math_transform;
mod price_transform;
mod statistic;
mod cycle;
mod pattern;
mod adx;
mod adxr;
mod apo;
mod aroon;
mod bbands;
mod cci;
mod candle_doji;
mod candle_dojistar;
mod candle_dragonflydoji;
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
mod candle_engulfing;
mod candle_eveningdojistar;
mod candle_eveningstar;
mod candle_gapsidesidewhite;
mod candle_gravestonedoji;
mod candle_hammer;
mod candle_hangingman;
mod candle_haramicross;
mod candle_harami;
mod candle_hikkake;
mod candle_hikkakemod;
mod candle_highwave;
mod candle_homingpigeon;
mod candle_identical3crows;
mod candle_inneck;
mod candle_invertedhammer;
mod candle_kickingbylength;
mod candle_kicking;
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
mod cmo;
mod dema;
mod directional;
mod dx;
mod ema;
mod ht_trendline;
mod ht_trendmode;
mod ht_dcperiod;
mod ht_dcphase;
mod ht_phasor;
mod ht_sine;
mod imi;
mod indicator;
mod kama;
mod ma;
mod macd;
mod macdext;
mod macdfix;
mod mama;
mod mavp;
mod mfi;
mod minus_di;
mod minus_dm;
mod moving_average;
mod ppo;
mod plus_di;
mod plus_dm;
mod rsi;
mod rolling_sum;
mod rolling_extrema;
mod rolling_median;
mod rolling_mode;
mod rolling_price;
mod volume_states;
mod aroon_true_range;
mod rolling_statistics;
mod regression;
mod math_price;
mod session_flags;
pub use session_flags::session_flags;
mod cumulative_sum;
mod cumulative_product;
mod cumulative_maximum;
mod cumulative_minimum;
pub use cumulative_sum::CumulativeSum;
pub use cumulative_product::CumulativeProduct;
pub use cumulative_maximum::CumulativeMaximum;
pub use cumulative_minimum::CumulativeMinimum;
mod lagged;
mod lagged_common;
mod momentum;
mod rate_of_change;
mod rate_of_change_percent;
mod rate_of_change_ratio;
mod rate_of_change_ratio_percent;
mod rmi;
mod laguerre_rsi;
mod even_better_sinewave;
mod jurik_moving_average;
mod ssl_channel;
mod premium_discount;
mod heikin_ashi;
mod fibonacci_retracement;
mod opening_range;
mod session_volume_levels;
mod klinger_volume_oscillator;
mod lag;
mod log_return;
mod parabolic_moving_average_stop;
mod tom_de_mark_sequential;
mod anchored_volume_weighted_average_price;
mod pivot_points;
mod sar;
mod sarext;
mod sma;
mod stoch;
mod stochf;
mod stochrsi;
mod t3;
mod tema;
mod trix;
mod trima;
mod ultosc;
mod window;
mod wma;
mod vidya;
mod operators;

pub use accbands::{AccelerationBands, AccelerationBandsValue, acceleration_bands};
pub use adx::{AverageDirectionalIndex, average_directional_index};
pub use adxr::{AverageDirectionalIndexRating, average_directional_index_rating};
pub use apo::{AbsolutePriceOscillator, absolute_price_oscillator};
pub use math_operator::{add, sub, mult, div, rolling_max, rolling_argmax, rolling_min, rolling_argmin, rolling_sum, rolling_minmax, rolling_minmax_index};
pub use math_transform::{acos, asin, atan, ceil, cos, cosh, exp, floor, ln, log10, sin, sinh, sqrt, tan, tanh};
pub use price_transform::{average_price, median_price, typical_price, weighted_close};
pub use statistic::{rolling_avgdev, rolling_std, rolling_var, rolling_beta, rolling_corr, rolling_linreg, rolling_linreg_angle, rolling_linreg_intercept, rolling_linreg_slope, rolling_tsf};
pub use cycle::{hilbert_transform_dominant_cycle_period, hilbert_transform_dominant_cycle_phase, hilbert_transform_phasor, hilbert_transform_sine_wave, hilbert_transform_trend_mode};
pub use pattern::{candle_doji, candle_hammer, candle_engulfing, candle_closing_marubozu, candle_dragonfly_doji, candle_gravestone_doji, candle_high_wave, candle_long_legged_doji, candle_long_line, candle_marubozu, candle_rickshawman, candle_short_line, candle_spinningtop, candle_takuri, candle_two_crows, candle_counterattack, candle_dark_cloud_cover, candle_doji_star, candle_hanging_man, candle_harami, candle_harami_cross, candle_hikkake, candle_hikkake_modified, candle_homing_pigeon, candle_in_neck, candle_inverted_hammer, candle_kicking, candle_kicking_by_length, candle_matching_low, candle_on_neck, candle_piercing, candle_separating_lines, candle_shooting_star, candle_stick_sandwich, candle_thrusting, candle_belt_hold, candle_three_black_crows, candle_three_inside, candle_three_line_strike, candle_three_outside, candle_three_stars_in_south, candle_three_white_soldiers, candle_abandoned_baby, candle_advance_block, candle_breakaway, candle_conceal_baby_swall, candle_evening_doji_star, candle_evening_star, candle_gap_side_side_white, candle_identical_three_crows, candle_ladder_bottom, candle_mat_hold, candle_morning_doji_star, candle_morning_star, candle_rise_fall_three_methods, candle_stalled_pattern, candle_tasuki_gap, candle_tri_star, candle_unique_three_river, candle_upside_gap_two_crows, candle_xside_gap_three_methods};
pub use aroon::{aroon, aroon_oscillator};
pub use bbands::{BollingerBands, BollingerBandsValue, bollinger_bands};
pub use cci::{CommodityChannelIndex, commodity_channel_index};
pub use candle_doji::CandleDoji;
pub use candle_dojistar::CandleDojiStar;
pub use candle_dragonflydoji::CandleDragonflyDoji;
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
pub use candle_engulfing::CandleEngulfing;
pub use candle_eveningdojistar::CandleEveningDojiStar;
pub use candle_eveningstar::CandleEveningStar;
pub use candle_gapsidesidewhite::CandleGapSideSideWhite;
pub use candle_gravestonedoji::CandleGravestoneDoji;
pub use candle_hammer::CandleHammer;
pub use candle_hangingman::CandleHangingMan;
pub use candle_haramicross::CandleHaramiCross;
pub use candle_harami::CandleHarami;
pub use candle_hikkake::CandleHikkake;
pub use candle_hikkakemod::CandleHikkakeModified;
pub use candle_highwave::CandleHighWave;
pub use candle_homingpigeon::CandleHomingPigeon;
pub use candle_identical3crows::CandleIdenticalThreeCrows;
pub use candle_inneck::CandleInNeck;
pub use candle_invertedhammer::CandleInvertedHammer;
pub use candle_kickingbylength::CandleKickingByLength;
pub use candle_kicking::CandleKicking;
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
pub use cmo::{ChandeMomentumOscillator, chande_momentum_oscillator};
pub use dema::{DoubleExponentialMovingAverage, double_exponential_moving_average};
pub use dx::{DirectionalMovementIndex, directional_movement_index};
pub use ema::{ExponentialMovingAverage, exponential_moving_average};
pub use ht_trendline::{HilbertTransformTrendline, hilbert_transform_trendline};
pub use ht_trendmode::HilbertTransformTrendMode;
pub use ht_dcperiod::HilbertTransformDominantCyclePeriod;
pub use ht_dcphase::HilbertTransformDominantCyclePhase;
pub use ht_phasor::{HilbertTransformPhasor, HilbertTransformPhasorValue};
pub use ht_sine::{HilbertTransformSineWave, HilbertTransformSineWaveValue};
pub use imi::{IntradayMomentumIndex, intraday_momentum_index};
pub use indicator::StreamingIndicator;
pub use kama::{KaufmanAdaptiveMovingAverage, kaufman_adaptive_moving_average};
pub use ma::{MovingAverage, moving_average};
pub use macd::{MovingAverageConvergenceDivergence, MovingAverageConvergenceDivergenceValue, moving_average_convergence_divergence};
pub use macdext::{MovingAverageConvergenceDivergenceExtended, moving_average_convergence_divergence_extended};
pub use macdfix::{MovingAverageConvergenceDivergenceFixed, moving_average_convergence_divergence_fixed};
pub use mama::{MesaAdaptiveMovingAverage, MesaAdaptiveMovingAverageValue, mesa_adaptive_moving_average};
pub use mavp::{VariablePeriodMovingAverage, moving_average_variable_period};
pub use mfi::{MoneyFlowIndex, money_flow_index};
pub use minus_di::{MinusDirectionalIndicator, minus_directional_indicator};
pub use minus_dm::{MinusDirectionalMovement, minus_directional_movement};
pub use ppo::{PercentagePriceOscillator, percentage_price_oscillator};
pub use plus_di::{PlusDirectionalIndicator, plus_directional_indicator};
pub use plus_dm::{PlusDirectionalMovement, plus_directional_movement};
pub use rsi::{RelativeStrengthIndex, relative_strength_index};
pub use rolling_sum::RollingSum;
pub use math_price::{Acos, Add, AveragePrice, Asin, Atan, Ceil, Cos, Cosh, Div, Exp, Floor, Ln, Log10, MedianPrice, Mult, Sin, Sinh, Sqrt, Sub, Tan, Tanh, TypicalPrice, WeightedClose};
pub use lagged::{Momentum, RateOfChange, RateOfChangePercent, RateOfChangeRatio, RateOfChangeRatioPercent, momentum, rate_of_change, rate_of_change_percent, rate_of_change_ratio, rate_of_change_ratio_percent};
pub use rolling_extrema::{RollingArgmax, RollingArgmin, RollingMax, RollingMin, RollingMinmax, RollingMinmaxIndex, RollingMinmaxIndexValue, RollingMinmaxValue};
pub use rolling_median::RollingMedian;
pub use rolling_mode::RollingMode;
use rolling_extrema::RollingExtrema;
pub use rolling_price::{RollingMidpoint, RollingMidprice, midpoint, midprice};
pub use volume_states::{AccumulationDistribution, AccumulationDistributionOscillator, BalanceOfPower, OnBalanceVolume, WilliamsPercentR, accumulation_distribution, accumulation_distribution_oscillator, balance_of_power, on_balance_volume, williams_percent_r};
pub use aroon_true_range::{Aroon, AroonOscillator, AroonValue, AverageTrueRange, NormalizedAverageTrueRange, TrueRange, average_true_range, normalized_average_true_range, true_range};
pub use rolling_statistics::{RollingAverageDeviation, RollingBeta, RollingCorrelation, RollingStandardDeviation, RollingVariance};
pub use regression::{Linearreg, LinearregAngle, LinearregIntercept, LinearregSlope, Tsf};
pub use rmi::RelativeMomentumIndex;
pub use laguerre_rsi::LaguerreRelativeStrengthIndex;
pub use lag::Lag;
pub use log_return::LogReturn;
pub use even_better_sinewave::EvenBetterSinewave;
pub use jurik_moving_average::JurikMovingAverage;
pub use ssl_channel::SmoothedTrendChannel;
pub use premium_discount::PremiumDiscount;
pub use heikin_ashi::HeikinAshi;
pub use fibonacci_retracement::FibonacciRetracement;
pub use opening_range::OpeningRange;
pub use session_volume_levels::SessionVolumeLevels;
pub use klinger_volume_oscillator::KlingerVolumeOscillator;
pub use parabolic_moving_average_stop::ParabolicMovingAverageStop;
pub use tom_de_mark_sequential::TomDeMarkSequential;
pub use anchored_volume_weighted_average_price::AnchoredVolumeWeightedAveragePrice;
pub use pivot_points::PivotPoints;
pub use sar::{ParabolicSar, parabolic_sar};
pub use sarext::{ParabolicSarExtended, extended_parabolic_sar};
pub use sma::{SimpleMovingAverage, simple_moving_average};
pub use stoch::{StochasticOscillator, StochasticOscillatorValue, stochastic_oscillator};
pub use stochf::{FastStochasticOscillator, FastStochasticOscillatorValue, fast_stochastic_oscillator};
pub use stochrsi::{StochasticRelativeStrengthIndex, StochasticRelativeStrengthIndexValue, stochastic_relative_strength_index};
pub use t3::{TripleExponentialAverage, triple_exponential_average};
pub use tema::{TripleExponentialMovingAverage, triple_exponential_moving_average};
pub use trix::{TripleExponentialRateOfChange, triple_exponential_rate_of_change};
pub use trima::{TriangularMovingAverage, triangular_moving_average};
pub use ultosc::{UltimateOscillator, ultimate_oscillator};
pub use operators::{ActiveZoneList, AverageDailyDollarValue, ArnaudLegouxMovingAverage, Amihud, AwesomeOscillator, BarsSince, BreakOfStructureChangeOfCharacter, BreakOfStructureChangeOfCharacterValue, ChaikinVolatility, ChaikinMoneyFlow, CloseToCloseSigma, Cross, Crossover, Crossunder, CumulativeSumControlChart, Donchian, DonchianValue, DetrendedPriceOscillator, Drawdown, EaseOfMovement, EntryExit, EqualHighsLows, EqualHighsLowsValue, ExponentiallyWeightedCorrelation, ExponentiallyWeightedCovariance, ExponentiallyWeightedStandardDeviation, ExponentiallyWeightedVariance, Falling, FairValueGap, FairValueGapValue, FisherTransform, ForceIndex, FracDiff, GarmanKlass, GapDown, GapUp, GarmanKlassYangZhang, HedgeRatio, HighestSince, HullMovingAverage, HigherHigh, Hurst, Ichimoku, IchimokuValue, InsideBar, KalmanHedgeRatio, KeltnerChannels, KeltnerValue, KnowSureThing, KnowSureThingValue, LowerLow, LowestSince, Liquidity, LiquidityValue, MassIndex, McGinleyDynamic, NegativeVolumeIndex, OrderBlock, OrderBlockValue, OutsideBar, OrnsteinUhlenbeckHalfLife, Parkinson, PositionHold, PreviousHighLow, PreviousHighLowValue, PositiveVolumeIndex, Retracements, RetracementsValue, Rising, RollingAlpha, RollingAutocorr, RollingCalmar, RollingCov, RollingEntropy, RollingInformationRatio, RollingInterquartileRange, RollingKurtosis, RollingQuantile, RollingRank, RollingSharpe, RollingSkew, RollingSortino, RollingWinsorize, RollingZScore, RogersSatchell, RollSpread, SessionExtrema, SessionExtremaValue, Sessions, SessionsValue, SignalDelay, SpreadZScore, Squeeze, SqueezePro, SqueezeProValue, SqueezeValue, SchaffTrendCycle, SchaffTrendCycleValue, Supertrend, SupertrendValue, SwingHighLow, SwingValue, TrueStrengthIndex, UlcerIndex, ValueWhen, Vortex, VortexValue, VolumePriceTrend, RollingVolumeWeightedAveragePrice, VolumeWeightedMovingAverage, YangZhang, ZeroLagExponentialMovingAverage};
pub use window::Window;
pub use wma::{WeightedMovingAverage, weighted_moving_average};
pub use vidya::VariableIndexDynamicAverage;

pub(super) fn invalid_period(name: &'static str, period: usize, minimum: usize) -> TaError {
    TaError::InvalidParameter {
        name,
        value: period.to_string(),
        reason: if minimum == 1 {
            "must be >= 1"
        } else {
            "must be >= 2"
        },
    }
}

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
        let midpoint_batch = midpoint(&input, 7).unwrap();
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
        let expected = midprice(&high, &low, 7).unwrap();
        let mut state = RollingMidprice::new(7).unwrap();
        for index in 0..close.len() {
            assert_optional_eq(state.append(high[index], low[index]), expected[index]);
        }
    }

    #[test]
    fn stateless_states_match_batch_for_every_bar() {
        let input: Vec<f64> = (0..40).map(|i| 0.1 + i as f64 / 50.0).collect();
        let other: Vec<f64> = (0..40).map(|i| 1.0 + i as f64 * 0.03).collect();

        macro_rules! check_unary {
            ($state:ty, $batch:path) => {{
                let expected = $batch(&input);
                let mut state = <$state>::new();
                for (value, expected) in input.iter().zip(expected) {
                    assert_optional_eq(state.append(*value), expected);
                }
            }};
        }
        check_unary!(Acos, crate::stream::acos);
        check_unary!(Asin, crate::stream::asin);
        check_unary!(Atan, crate::stream::atan);
        check_unary!(Ceil, crate::stream::ceil);
        check_unary!(Cos, crate::stream::cos);
        check_unary!(Cosh, crate::stream::cosh);
        check_unary!(Exp, crate::stream::exp);
        check_unary!(Floor, crate::stream::floor);
        check_unary!(Ln, crate::stream::ln);
        check_unary!(Log10, crate::stream::log10);
        check_unary!(Sin, crate::stream::sin);
        check_unary!(Sinh, crate::stream::sinh);
        check_unary!(Sqrt, crate::stream::sqrt);
        check_unary!(Tan, crate::stream::tan);
        check_unary!(Tanh, crate::stream::tanh);

        macro_rules! check_binary {
            ($state:ty, $batch:path) => {{
                let expected = $batch(&input, &other).unwrap();
                let mut state = <$state>::new();
                for index in 0..input.len() {
                    assert_eq!(state.append(input[index], other[index]), expected[index]);
                }
            }};
        }
        check_binary!(Add, crate::stream::add);
        check_binary!(Sub, crate::stream::sub);
        check_binary!(Mult, crate::stream::mult);
        check_binary!(Div, crate::stream::div);

        let open = &input;
        let high: Vec<_> = input.iter().map(|value| value + 0.2).collect();
        let low: Vec<_> = input.iter().map(|value| value - 0.1).collect();
        let close = &other;
        let avg = crate::stream::average_price(open, &high, &low, close).unwrap();
        let med = crate::stream::median_price(&high, &low).unwrap();
        let typ = crate::stream::typical_price(&high, &low, close).unwrap();
        let wcl = crate::stream::weighted_close(&high, &low, close).unwrap();
        let mut avg_state = AveragePrice::new();
        let mut med_state = MedianPrice::new();
        let mut typ_state = TypicalPrice::new();
        let mut wcl_state = WeightedClose::new();
        for index in 0..input.len() {
            assert_eq!(
                avg_state.append(open[index], high[index], low[index], close[index]),
                avg[index]
            );
            assert_eq!(med_state.append(high[index], low[index]), med[index]);
            assert_eq!(
                typ_state.append(high[index], low[index], close[index]),
                typ[index]
            );
            assert_eq!(
                wcl_state.append(high[index], low[index], close[index]),
                wcl[index]
            );
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
        let adosc_expected = accumulation_distribution_oscillator(&high, &low, &close, &volumes, 3, 10).unwrap();
        let obv_expected = on_balance_volume(&close, &volumes).unwrap();
        let mut ad = AccumulationDistribution::new();
        let mut adosc = AccumulationDistributionOscillator::new(3, 10).unwrap();
        let mut obv = OnBalanceVolume::new();
        for index in 0..close.len() {
            assert_eq!(
                ad.append(high[index], low[index], close[index], volumes[index]),
                ad_expected[index]
            );
            assert_optional_eq(
                adosc.append(high[index], low[index], close[index], volumes[index]),
                adosc_expected[index],
            );
            assert_eq!(
                obv.append(close[index], volumes[index]),
                obv_expected[index]
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
        let bop_expected = balance_of_power(&open, &high, &low, &close).unwrap();
        let willr_expected = williams_percent_r(&high, &low, &close, period).unwrap();
        let (down_expected, up_expected) = crate::stream::aroon(&high, &low, period).unwrap();
        let osc_expected = crate::stream::aroon_oscillator(&high, &low, period).unwrap();
        let mut bop = BalanceOfPower::new();
        let mut willr = WilliamsPercentR::new(period).unwrap();
        let mut aroon = Aroon::new(period).unwrap();
        let mut oscillator = AroonOscillator::new(period).unwrap();
        for index in 0..close.len() {
            assert_eq!(
                bop.append(open[index], high[index], low[index], close[index]),
                bop_expected[index]
            );
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
