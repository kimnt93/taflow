//! Persistent technical indicators for bulk history and realtime continuation.
//!
//! Each TA implementation lives in its own module and retains only the bounded
//! recurrence state required to process newly appended bars.

use crate::error::TaError;

mod accbands;
mod adx;
mod adxr;
mod apo;
mod bbands;
mod cci;
mod cdl_doji;
mod cdl_dojistar;
mod cdl_dragonflydoji;
mod cdl_2crows;
mod cdl_3blackcrows;
mod cdl_3inside;
mod cdl_3linestrike;
mod cdl_3outside;
mod cdl_3starsinsouth;
mod cdl_3whitesoldiers;
mod cdl_abandonedbaby;
mod cdl_advanceblock;
mod cdl_belthold;
mod cdl_breakaway;
mod cdl_closingmarubozu;
mod cdl_concealbabyswall;
mod cdl_counterattack;
mod cdl_darkcloudcover;
mod cdl_engulfing;
mod cdl_eveningdojistar;
mod cdl_eveningstar;
mod cdl_gapsidesidewhite;
mod cdl_gravestonedoji;
mod cdl_hammer;
mod cdl_hangingman;
mod cdl_haramicross;
mod cdl_harami;
mod cdl_hikkake;
mod cdl_hikkakemod;
mod cdl_highwave;
mod cdl_homingpigeon;
mod cdl_identical3crows;
mod cdl_inneck;
mod cdl_invertedhammer;
mod cdl_kickingbylength;
mod cdl_kicking;
mod cdl_ladderbottom;
mod cdl_longleggeddoji;
mod cdl_longline;
mod cdl_marubozu;
mod cdl_matchinglow;
mod cdl_mathold;
mod cdl_morningdojistar;
mod cdl_morningstar;
mod cdl_onneck;
mod cdl_piercing;
mod cdl_rickshawman;
mod cdl_risefall3methods;
mod cdl_separatinglines;
mod cdl_shootingstar;
mod cdl_shortline;
mod cdl_spinningtop;
mod cdl_stalledpattern;
mod cdl_sticksandwich;
mod cdl_takuri;
mod cdl_tasukigap;
mod cdl_thrusting;
mod cdl_tristar;
mod cdl_unique3river;
mod cdl_upsidegap2crows;
mod cdl_xsidegap3methods;
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

pub use accbands::{AccelerationBands, AccelerationBandsValue};
pub use adx::AverageDirectionalIndex;
pub use adxr::AverageDirectionalIndexRating;
pub use apo::AbsolutePriceOscillator;
pub use bbands::{BollingerBands, BollingerBandsValue};
pub use cci::{CommodityChannelIndex, commodity_channel_index};
pub use cdl_doji::CandleDoji;
pub use cdl_dojistar::CandleDojiStar;
pub use cdl_dragonflydoji::CandleDragonflyDoji;
pub use cdl_2crows::CandleTwoCrows;
pub use cdl_3blackcrows::CandleThreeBlackCrows;
pub use cdl_3inside::CandleThreeInside;
pub use cdl_3linestrike::CandleThreeLineStrike;
pub use cdl_3outside::CandleThreeOutside;
pub use cdl_3starsinsouth::CandleThreeStarsInSouth;
pub use cdl_3whitesoldiers::CandleThreeWhiteSoldiers;
pub use cdl_abandonedbaby::CandleAbandonedBaby;
pub use cdl_advanceblock::CandleAdvanceBlock;
pub use cdl_belthold::CandleBeltHold;
pub use cdl_breakaway::CandleBreakaway;
pub use cdl_closingmarubozu::CandleClosingMarubozu;
pub use cdl_concealbabyswall::CandleConcealBabySwall;
pub use cdl_counterattack::CandleCounterAttack;
pub use cdl_darkcloudcover::CandleDarkCloudCover;
pub use cdl_engulfing::CandleEngulfing;
pub use cdl_eveningdojistar::CandleEveningDojiStar;
pub use cdl_eveningstar::CandleEveningStar;
pub use cdl_gapsidesidewhite::CandleGapSideSideWhite;
pub use cdl_gravestonedoji::CandleGravestoneDoji;
pub use cdl_hammer::CandleHammer;
pub use cdl_hangingman::CandleHangingMan;
pub use cdl_haramicross::CandleHaramiCross;
pub use cdl_harami::CandleHarami;
pub use cdl_hikkake::CandleHikkake;
pub use cdl_hikkakemod::CandleHikkakeModified;
pub use cdl_highwave::CandleHighWave;
pub use cdl_homingpigeon::CandleHomingPigeon;
pub use cdl_identical3crows::CandleIdenticalThreeCrows;
pub use cdl_inneck::CandleInNeck;
pub use cdl_invertedhammer::CandleInvertedHammer;
pub use cdl_kickingbylength::CandleKickingByLength;
pub use cdl_kicking::CandleKicking;
pub use cdl_ladderbottom::CandleLadderBottom;
pub use cdl_longleggeddoji::CandleLongLeggedDoji;
pub use cdl_longline::CandleLongLine;
pub use cdl_marubozu::CandleMarubozu;
pub use cdl_matchinglow::CandleMatchingLow;
pub use cdl_mathold::CandleMatHold;
pub use cdl_morningdojistar::CandleMorningDojiStar;
pub use cdl_morningstar::CandleMorningStar;
pub use cdl_onneck::CandleOnNeck;
pub use cdl_piercing::CandlePiercing;
pub use cdl_rickshawman::CandleRickshawman;
pub use cdl_risefall3methods::CandleRiseFallThreeMethods;
pub use cdl_separatinglines::CandleSeparatingLines;
pub use cdl_shootingstar::CandleShootingStar;
pub use cdl_shortline::CandleShortLine;
pub use cdl_spinningtop::CandleSpinningTop;
pub use cdl_stalledpattern::CandleStalledPattern;
pub use cdl_sticksandwich::CandleStickSandwich;
pub use cdl_takuri::CandleTakuri;
pub use cdl_tasukigap::CandleTasukiGap;
pub use cdl_thrusting::CandleThrusting;
pub use cdl_tristar::CandleTriStar;
pub use cdl_unique3river::CandleUniqueThreeRiver;
pub use cdl_upsidegap2crows::CandleUpsideGapTwoCrows;
pub use cdl_xsidegap3methods::CandleUpDownSideGapThreeMethods;
pub use cmo::ChandeMomentumOscillator;
pub use dema::DoubleExponentialMovingAverage;
pub use dx::DirectionalMovementIndex;
pub use ema::ExponentialMovingAverage;
pub use ht_trendline::HilbertTransformTrendline;
pub use ht_trendmode::HilbertTransformTrendMode;
pub use ht_dcperiod::HilbertTransformDominantCyclePeriod;
pub use ht_dcphase::HilbertTransformDominantCyclePhase;
pub use ht_phasor::{HilbertTransformPhasor, HilbertTransformPhasorValue};
pub use ht_sine::{HilbertTransformSineWave, HilbertTransformSineWaveValue};
pub use imi::IntradayMomentumIndex;
pub use indicator::StreamingIndicator;
pub use kama::KaufmanAdaptiveMovingAverage;
pub use ma::MovingAverage;
pub use macd::{MovingAverageConvergenceDivergence, MovingAverageConvergenceDivergenceValue};
pub use macdext::MovingAverageConvergenceDivergenceExtended;
pub use macdfix::MovingAverageConvergenceDivergenceFixed;
pub use mama::{MesaAdaptiveMovingAverage, MesaAdaptiveMovingAverageValue};
pub use mavp::VariablePeriodMovingAverage;
pub use mfi::MoneyFlowIndex;
pub use minus_di::MinusDirectionalIndicator;
pub use minus_dm::{MinusDirectionalMovement, minus_directional_movement};
pub use ppo::PercentagePriceOscillator;
pub use plus_di::PlusDirectionalIndicator;
pub use plus_dm::{PlusDirectionalMovement, plus_directional_movement};
pub use rsi::{RelativeStrengthIndex, relative_strength_index};
pub use rolling_sum::RollingSum;
pub use math_price::{Acos, Add, AveragePrice, Asin, Atan, Ceil, Cos, Cosh, Div, Exp, Floor, Ln, Log10, MedianPrice, Mult, Sin, Sinh, Sqrt, Sub, Tan, Tanh, TypicalPrice, WeightedClose};
pub use lagged::{Momentum, RateOfChange, RateOfChangePercent, RateOfChangeRatio, RateOfChangeRatioPercent, momentum, rate_of_change, rate_of_change_percent, rate_of_change_ratio, rate_of_change_ratio_percent};
pub use rolling_extrema::{RollingArgmax, RollingArgmin, RollingMax, RollingMin, RollingMinmax, RollingMinmaxIndex, RollingMinmaxIndexValue, RollingMinmaxValue};
pub use rolling_median::RollingMedian;
pub use rolling_mode::RollingMode;
use rolling_extrema::RollingExtrema;
pub use rolling_price::{RollingMidpoint, RollingMidprice};
pub use volume_states::{AccumulationDistribution, AccumulationDistributionOscillator, BalanceOfPower, OnBalanceVolume, WilliamsR};
pub use aroon_true_range::{Aroon, AroonOscillator, AroonValue, AverageTrueRange, NormalizedAverageTrueRange, TrueRange};
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
pub use sar::ParabolicSar;
pub use sarext::ParabolicSarExtended;
pub use sma::SimpleMovingAverage;
pub use stoch::{StochasticOscillator, StochasticOscillatorValue};
pub use stochf::{FastStochasticOscillator, FastStochasticOscillatorValue};
pub use stochrsi::{StochasticRelativeStrengthIndex, StochasticRelativeStrengthIndexValue};
pub use t3::TripleExponentialAverage;
pub use tema::TripleExponentialMovingAverage;
pub use trix::TripleExponentialRateOfChange;
pub use trima::TriangularMovingAverage;
pub use ultosc::UltimateOscillator;
pub use crate::operators::{ActiveZoneList, AverageDailyDollarValue, ArnaudLegouxMovingAverage, Amihud, AwesomeOscillator, BarsSince, BreakOfStructureChangeOfCharacter, BreakOfStructureChangeOfCharacterValue, ChaikinVolatility, ChaikinMoneyFlow, CloseToCloseSigma, Cross, Crossover, Crossunder, CumulativeSumControlChart, Donchian, DonchianValue, DetrendedPriceOscillator, Drawdown, EaseOfMovement, EntryExit, EqualHighsLows, EqualHighsLowsValue, ExponentiallyWeightedCorrelation, ExponentiallyWeightedCovariance, ExponentiallyWeightedStandardDeviation, ExponentiallyWeightedVariance, Falling, FairValueGap, FairValueGapValue, FisherTransform, ForceIndex, FracDiff, GarmanKlass, GapDown, GapUp, GarmanKlassYangZhang, HedgeRatio, HighestSince, HullMovingAverage, HigherHigh, Hurst, Ichimoku, IchimokuValue, InsideBar, KalmanHedgeRatio, KeltnerChannels, KeltnerValue, KnowSureThing, KnowSureThingValue, LowerLow, LowestSince, Liquidity, LiquidityValue, MassIndex, McGinleyDynamic, NegativeVolumeIndex, OrderBlock, OrderBlockValue, OutsideBar, OrnsteinUhlenbeckHalfLife, Parkinson, PositionHold, PreviousHighLow, PreviousHighLowValue, PositiveVolumeIndex, Retracements, RetracementsValue, Rising, RollingAlpha, RollingAutocorr, RollingCalmar, RollingCov, RollingEntropy, RollingInformationRatio, RollingInterquartileRange, RollingKurtosis, RollingQuantile, RollingRank, RollingSharpe, RollingSkew, RollingSortino, RollingWinsorize, RollingZScore, RogersSatchell, RollSpread, SessionExtrema, SessionExtremaValue, Sessions, SessionsValue, SignalDelay, SpreadZScore, Squeeze, SqueezePro, SqueezeProValue, SqueezeValue, SchaffTrendCycle, SchaffTrendCycleValue, Supertrend, SupertrendValue, SwingHighLow, SwingValue, TrueStrengthIndex, UlcerIndex, ValueWhen, Vortex, VortexValue, VolumePriceTrend, RollingVolumeWeightedAveragePrice, VolumeWeightedMovingAverage, YangZhang, ZeroLagExponentialMovingAverage};
pub use window::Window;
pub use wma::WeightedMovingAverage;
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
    use crate::{
        math_operator, math_transform, momentum, overlap, price_transform, statistic, volatility,
        volume,
    };

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
        let sma_batch = overlap::simple_moving_average(&input, 7).unwrap();
        let ema_batch = overlap::exponential_moving_average(&input, 7).unwrap();
        let wma_batch = overlap::weighted_moving_average(&input, 7).unwrap();
        let dema_batch = overlap::double_exponential_moving_average(&input, 7).unwrap();
        let tema_batch = overlap::triple_exponential_moving_average(&input, 7).unwrap();
        let trima_batch = overlap::triangular_moving_average(&input, 7).unwrap();
        let kama_batch = overlap::kaufman_adaptive_moving_average(&input, 7).unwrap();
        let midpoint_batch = overlap::midpoint(&input, 7).unwrap();
        let rsi_batch = momentum::relative_strength_index(&input, 14).unwrap();
        let cmo_batch = momentum::chande_momentum_oscillator(&input, 14).unwrap();
        let mom_batch = momentum::momentum(&input, 7).unwrap();
        let roc_batch = momentum::rate_of_change(&input, 7).unwrap();
        let rocp_batch = momentum::rate_of_change_percent(&input, 7).unwrap();
        let rocr_batch = momentum::rate_of_change_ratio(&input, 7).unwrap();
        let rocr100_batch = momentum::rate_of_change_ratio_percent(&input, 7).unwrap();
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
        let expected = overlap::midprice(&high, &low, 7).unwrap();
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
        check_unary!(Acos, math_transform::acos);
        check_unary!(Asin, math_transform::asin);
        check_unary!(Atan, math_transform::atan);
        check_unary!(Ceil, math_transform::ceil);
        check_unary!(Cos, math_transform::cos);
        check_unary!(Cosh, math_transform::cosh);
        check_unary!(Exp, math_transform::exp);
        check_unary!(Floor, math_transform::floor);
        check_unary!(Ln, math_transform::ln);
        check_unary!(Log10, math_transform::log10);
        check_unary!(Sin, math_transform::sin);
        check_unary!(Sinh, math_transform::sinh);
        check_unary!(Sqrt, math_transform::sqrt);
        check_unary!(Tan, math_transform::tan);
        check_unary!(Tanh, math_transform::tanh);

        macro_rules! check_binary {
            ($state:ty, $batch:path) => {{
                let expected = $batch(&input, &other).unwrap();
                let mut state = <$state>::new();
                for index in 0..input.len() {
                    assert_eq!(state.append(input[index], other[index]), expected[index]);
                }
            }};
        }
        check_binary!(Add, math_operator::add);
        check_binary!(Sub, math_operator::sub);
        check_binary!(Mult, math_operator::mult);
        check_binary!(Div, math_operator::div);

        let open = &input;
        let high: Vec<_> = input.iter().map(|value| value + 0.2).collect();
        let low: Vec<_> = input.iter().map(|value| value - 0.1).collect();
        let close = &other;
        let avg = price_transform::average_price(open, &high, &low, close).unwrap();
        let med = price_transform::median_price(&high, &low).unwrap();
        let typ = price_transform::typical_price(&high, &low, close).unwrap();
        let wcl = price_transform::weighted_close(&high, &low, close).unwrap();
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
        let max_expected = math_operator::max(&input, period).unwrap();
        let min_expected = math_operator::min(&input, period).unwrap();
        let sum_expected = math_operator::sum(&input, period).unwrap();
        let maxindex_expected = math_operator::maxindex(&input, period).unwrap();
        let minindex_expected = math_operator::minindex(&input, period).unwrap();
        let (minmax_min, minmax_max) = math_operator::minmax(&input, period).unwrap();
        let (minidx, maxidx) = math_operator::minmaxindex(&input, period).unwrap();
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
        let avgdev_expected = statistic::avgdev(&input, period).unwrap();
        let var_expected = statistic::var(&input, period, 2.0).unwrap();
        let stddev_expected = statistic::stddev(&input, period, 2.0).unwrap();
        let mut avgdev = RollingAverageDeviation::new(period).unwrap();
        let mut var = RollingVariance::new(period, 2.0).unwrap();
        let mut stddev = RollingStandardDeviation::new(period, 2.0).unwrap();
        for index in 0..input.len() {
            assert_optional_eq(avgdev.append(input[index]), avgdev_expected[index]);
            assert_optional_eq(var.append(input[index]), var_expected[index]);
            assert_optional_eq(stddev.append(input[index]), stddev_expected[index]);
        }

        let constant = vec![42.0; 30];
        let expected = statistic::stddev(&constant, 5, 3.0).unwrap();
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
        let beta_expected = statistic::beta(&market, &asset, period).unwrap();
        let correl_expected = statistic::correl(&market, &asset, period).unwrap();
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
        let linearreg_expected = statistic::linearreg(&input, period).unwrap();
        let slope_expected = statistic::linearreg_slope(&input, period).unwrap();
        let intercept_expected = statistic::linearreg_intercept(&input, period).unwrap();
        let angle_expected = statistic::linearreg_angle(&input, period).unwrap();
        let tsf_expected = statistic::tsf(&input, period).unwrap();
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
        let ad_expected = volume::accumulation_distribution(&high, &low, &close, &volumes).unwrap();
        let adosc_expected = volume::accumulation_distribution_oscillator(&high, &low, &close, &volumes, 3, 10).unwrap();
        let obv_expected = volume::on_balance_volume(&close, &volumes).unwrap();
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
        let bop_expected = momentum::balance_of_power(&open, &high, &low, &close).unwrap();
        let willr_expected = momentum::williams_r(&high, &low, &close, period).unwrap();
        let (down_expected, up_expected) = momentum::aroon(&high, &low, period).unwrap();
        let osc_expected = momentum::aroon_oscillator(&high, &low, period).unwrap();
        let mut bop = BalanceOfPower::new();
        let mut willr = WilliamsR::new(period).unwrap();
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
        let atr_batch = volatility::average_true_range(&high, &low, &close, 14).unwrap();
        let trange_batch = volatility::true_range(&high, &low, &close).unwrap();
        let natr_batch = volatility::normalized_average_true_range(&high, &low, &close, 14).unwrap();
        let (macd_batch, signal_batch, histogram_batch) =
            momentum::moving_average_convergence_divergence(&close, 12, 26, 9).unwrap();
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
