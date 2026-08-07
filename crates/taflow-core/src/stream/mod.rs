//! Persistent technical indicators for bulk history and realtime continuation.
//!
//! Each TA implementation lives in its own module and retains only the bounded
//! recurrence state required to process newly appended bars.

use std::collections::VecDeque;

use crate::error::{TaError, TaResult};

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

pub use accbands::{AccelerationBands, AccbandsValue};
pub use adx::AverageDirectionalIndex;
pub use adxr::AverageDirectionalIndexRating;
pub use apo::AbsolutePriceOscillator;
pub use bbands::{Bbands, BbandsValue};
pub use cci::CommodityChannelIndex;
pub use cdl_doji::CandleDoji;
pub use cdl_dojistar::CandleDojiStar;
pub use cdl_dragonflydoji::CandleDragonflyDoji;
pub use cdl_2crows::Candle2Crows;
pub use cdl_3blackcrows::Candle3BlackCrows;
pub use cdl_3inside::Candle3Inside;
pub use cdl_3linestrike::Candle3LineStrike;
pub use cdl_3outside::Candle3Outside;
pub use cdl_3starsinsouth::Candle3StarsInSouth;
pub use cdl_3whitesoldiers::Candle3WhiteSoldiers;
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
pub use cdl_hikkakemod::CandleHikkakeMod;
pub use cdl_highwave::CandleHighWave;
pub use cdl_homingpigeon::CandleHomingPigeon;
pub use cdl_identical3crows::CandleIdentical3Crows;
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
pub use cdl_risefall3methods::CandleRiseFall3Methods;
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
pub use cdl_unique3river::CandleUnique3River;
pub use cdl_upsidegap2crows::CandleUpsideGap2Crows;
pub use cdl_xsidegap3methods::CandleXSideGap3Methods;
pub use cmo::ChandeMomentumOscillator;
pub use dema::DoubleExponentialMovingAverage;
pub use dx::DirectionalMovementIndex;
pub use ema::ExponentialMovingAverage;
pub use ht_trendline::HilbertTransformTrendline;
pub use ht_trendmode::HilbertTransformTrendMode;
pub use ht_dcperiod::HilbertTransformDominantCyclePeriod;
pub use ht_dcphase::HilbertTransformDominantCyclePhase;
pub use ht_phasor::{HilbertTransformPhasor, HtPhasorValue};
pub use ht_sine::{HilbertTransformSineWave, HtSineValue};
pub use imi::IntradayMomentumIndex;
pub use indicator::StreamingIndicator;
pub use kama::KaufmanAdaptiveMovingAverage;
pub use ma::MovingAverage;
pub use macd::{MovingAverageConvergenceDivergence, MacdValue};
pub use macdext::MovingAverageConvergenceDivergenceExtended;
pub use macdfix::MovingAverageConvergenceDivergenceFixed;
pub use mama::{MesaAdaptiveMovingAverage, MamaValue};
pub use mavp::MovingAverageVariablePeriod;
pub use mfi::MoneyFlowIndex;
pub use minus_di::MinusDirectionalIndicator;
pub use minus_dm::MinusDirectionalMovement;
pub use ppo::PercentagePriceOscillator;
pub use plus_di::PlusDirectionalIndicator;
pub use plus_dm::PlusDirectionalMovement;
pub use rsi::RelativeStrengthIndex;
pub use rolling_sum::RollingSum;
pub use rmi::RelativeMomentumIndex;
pub use laguerre_rsi::LaguerreRelativeStrengthIndex;
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
pub use sarext::ExtendedParabolicSar;
pub use sma::SimpleMovingAverage;
pub use stoch::{StochasticOscillator, StochValue};
pub use stochf::{FastStochasticOscillator, StochfValue};
pub use stochrsi::{StochasticRelativeStrengthIndex, StochrsiValue};
pub use t3::TripleExponentialAverage;
pub use tema::TripleExponentialMovingAverage;
pub use trix::TripleExponentialRateOfChange;
pub use trima::TriangularMovingAverage;
pub use ultosc::UltimateOscillator;
pub use crate::operators::{ActiveZoneList, AverageDailyDollarValue, ArnaudLegouxMovingAverage, Amihud, AwesomeOscillator, BarsSince, BosChoch, BosChochValue, ChaikinVolatility, ChaikinMoneyFlow, CloseToCloseSigma, Cross, Crossover, Crossunder, Cumsum, Cummax, Cummin, Cumprod, Cusum, Donchian, DonchianValue, DetrendedPriceOscillator, Drawdown, EaseOfMovement, EntryExit, EqualHighsLows, EqualHighsLowsValue, ExponentiallyWeightedCorrelation, ExponentiallyWeightedCovariance, ExponentiallyWeightedStandardDeviation, ExponentiallyWeightedVariance, Falling, FairValueGap, FairValueGapValue, FisherTransform, ForceIndex, FracDiff, GarmanKlass, GapDown, GapUp, GarmanKlassYangZhang, HedgeRatio, HighestSince, HullMovingAverage, HigherHigh, Hurst, Ichimoku, IchimokuValue, InsideBar, KalmanHedgeRatio, KeltnerChannels, KeltnerValue, KnowSureThing, KnowSureThingValue, Lag, LogReturn, LowerLow, LowestSince, Liquidity, LiquidityValue, MassIndex, McGinleyDynamic, NegativeVolumeIndex, OrderBlock, OrderBlockValue, OutsideBar, OrnsteinUhlenbeckHalfLife, Parkinson, PositionHold, PreviousHighLow, PreviousHighLowValue, PositiveVolumeIndex, Retracements, RetracementsValue, Rising, RollingAlpha, RollingAutocorr, RollingCalmar, RollingCov, RollingEntropy, RollingInformationRatio, RollingIqr, RollingKurtosis, RollingMedian, RollingMode, RollingQuantile, RollingRank, RollingSharpe, RollingSkew, RollingSortino, RollingWinsorize, RollingZscore, RogersSatchell, RollSpread, SessionExtrema, SessionExtremaValue, Sessions, SessionsValue, SignalDelay, SpreadZscore, Squeeze, SqueezePro, SqueezeProValue, SqueezeValue, SchaffTrendCycle, SchaffTrendCycleValue, Supertrend, SupertrendValue, Swing, SwingValue, TrueStrengthIndex, UlcerIndex, ValueWhen, Vortex, VortexValue, VolumePriceTrend, RollingVolumeWeightedAveragePrice, VolumeWeightedMovingAverage, YangZhang, ZeroLagExponentialMovingAverage};
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

#[derive(Debug, Clone)]
struct LaggedValue {
    period: usize,
    values: VecDeque<f64>,
}

impl LaggedValue {
    fn new(period: usize) -> TaResult<Self> {
        if period == 0 {
            return Err(invalid_period("timeperiod", period, 1));
        }
        Ok(Self {
            period,
            values: VecDeque::with_capacity(period),
        })
    }

    fn append(&mut self, input: f64) -> Option<(f64, f64)> {
        if self.values.len() < self.period {
            self.values.push_back(input);
            return None;
        }
        let previous = self.values.pop_front().expect("lag window is full");
        self.values.push_back(input);
        Some((input, previous))
    }

    fn reset(&mut self) {
        self.values.clear();
    }
}

macro_rules! lagged_indicator {
    ($name:ident, $calculate:expr) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            lag: LaggedValue,
            value: Option<f64>,
        }

        impl $name {
            /// Computes or updates `new` through the native Rust kernel.
            ///
            /// Parameters are the typed series and configuration values in the signature.
            ///
            /// Returns the computed value, aligned history, or a validation error.
            pub fn new(period: usize) -> TaResult<Self> {
                Ok(Self {
                    lag: LaggedValue::new(period)?,
                    value: None,
                })
            }
        }

        impl StreamingIndicator for $name {
            type Output = f64;

            fn append(&mut self, input: f64) -> Option<f64> {
                self.value = self
                    .lag
                    .append(input)
                    .map(|(current, previous)| $calculate(current, previous));
                self.value
            }

            fn value(&self) -> Option<f64> {
                self.value
            }

            fn reset(&mut self) {
                self.lag.reset();
                self.value = None;
            }
        }
    };
}

lagged_indicator!(Mom, |current: f64, previous: f64| current - previous);
lagged_indicator!(Roc, |current: f64, previous: f64| if previous != 0.0 {
    (current - previous) / previous * 100.0
} else {
    0.0
});
lagged_indicator!(Rocp, |current: f64, previous: f64| if previous != 0.0 {
    (current - previous) / previous
} else {
    0.0
});
lagged_indicator!(Rocr, |current: f64, previous: f64| if previous != 0.0 {
    current / previous
} else {
    0.0
});
lagged_indicator!(Rocr100, |current: f64, previous: f64| if previous != 0.0 {
    current / previous * 100.0
} else {
    0.0
});

#[derive(Debug, Clone)]
struct RollingExtrema {
    period: usize,
    index: usize,
    maximum: VecDeque<(usize, f64)>,
    minimum: VecDeque<(usize, f64)>,
}

impl RollingExtrema {
    fn new(period: usize) -> TaResult<Self> {
        if period == 0 {
            return Err(invalid_period("timeperiod", period, 1));
        }
        Ok(Self {
            period,
            index: 0,
            maximum: VecDeque::with_capacity(period),
            minimum: VecDeque::with_capacity(period),
        })
    }

    fn append_indexed(&mut self, input: f64) -> Option<((usize, f64), (usize, f64))> {
        let index = self.index;
        self.index += 1;
        while self
            .maximum
            .back()
            .is_some_and(|&(_, value)| value <= input)
        {
            self.maximum.pop_back();
        }
        while self
            .minimum
            .back()
            .is_some_and(|&(_, value)| value >= input)
        {
            self.minimum.pop_back();
        }
        self.maximum.push_back((index, input));
        self.minimum.push_back((index, input));
        let first_valid = index.saturating_add(1).saturating_sub(self.period);
        while self.maximum.front().is_some_and(|&(i, _)| i < first_valid) {
            self.maximum.pop_front();
        }
        while self.minimum.front().is_some_and(|&(i, _)| i < first_valid) {
            self.minimum.pop_front();
        }
        (index + 1 >= self.period).then(|| {
            (
                *self.maximum.front().expect("maximum queue is populated"),
                *self.minimum.front().expect("minimum queue is populated"),
            )
        })
    }

    fn append(&mut self, input: f64) -> Option<(f64, f64)> {
        self.append_indexed(input)
            .map(|(maximum, minimum)| (maximum.1, minimum.1))
    }

    fn reset(&mut self) {
        self.index = 0;
        self.maximum.clear();
        self.minimum.clear();
    }
}

macro_rules! rolling_extrema_indicator {
    ($name:ident, $select:expr) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            extrema: RollingExtrema,
            value: Option<f64>,
        }

        impl $name {
            /// Computes or updates `new` through the native Rust kernel.
            ///
            /// Parameters are the typed series and configuration values in the signature.
            ///
            /// Returns the computed value, aligned history, or a validation error.
            pub fn new(period: usize) -> TaResult<Self> {
                Ok(Self {
                    extrema: RollingExtrema::new(period)?,
                    value: None,
                })
            }
        }

        impl StreamingIndicator for $name {
            type Output = f64;

            fn append(&mut self, input: f64) -> Option<f64> {
                self.value = self.extrema.append(input).map($select);
                self.value
            }

            fn value(&self) -> Option<f64> {
                self.value
            }

            fn reset(&mut self) {
                self.extrema.reset();
                self.value = None;
            }
        }
    };
}

rolling_extrema_indicator!(RollingMax, |(maximum, _)| maximum);
rolling_extrema_indicator!(RollingMin, |(_, minimum)| minimum);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RollingMinmaxValue {
    pub minimum: f64,
    pub maximum: f64,
}

#[derive(Debug, Clone)]
pub struct RollingMinmax {
    extrema: RollingExtrema,
    value: Option<RollingMinmaxValue>,
}

impl RollingMinmax {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(period: usize) -> TaResult<Self> {
        Ok(Self {
            extrema: RollingExtrema::new(period)?,
            value: None,
        })
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, input: f64) -> Option<RollingMinmaxValue> {
        self.value = self
            .extrema
            .append(input)
            .map(|(maximum, minimum)| RollingMinmaxValue { minimum, maximum });
        self.value
    }

    /// Computes or updates `value` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn value(&self) -> Option<RollingMinmaxValue> {
        self.value
    }

    /// Computes or updates `reset` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn reset(&mut self) {
        self.extrema.reset();
        self.value = None;
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RollingMinmaxIndexValue {
    pub minimum: usize,
    pub maximum: usize,
}

#[derive(Debug, Clone)]
struct RollingIndexExtrema {
    period: usize,
    index: usize,
    window: VecDeque<(usize, f64)>,
    maximum: Option<(usize, f64)>,
    minimum: Option<(usize, f64)>,
}

impl RollingIndexExtrema {
    fn new(period: usize) -> TaResult<Self> {
        if period == 0 {
            return Err(invalid_period("timeperiod", period, 1));
        }
        Ok(Self {
            period,
            index: 0,
            window: VecDeque::with_capacity(period),
            maximum: None,
            minimum: None,
        })
    }

    fn append(&mut self, input: f64) -> RollingMinmaxIndexValue {
        let index = self.index;
        self.index += 1;
        if self.window.len() == self.period {
            self.window.pop_front();
        }
        self.window.push_back((index, input));
        if self.window.len() < self.period {
            return RollingMinmaxIndexValue {
                minimum: 0,
                maximum: 0,
            };
        }

        let first_valid = index + 1 - self.period;
        if self.maximum.is_none() || self.maximum.is_some_and(|(i, _)| i < first_valid) {
            self.maximum =
                self.window
                    .iter()
                    .copied()
                    .reduce(|best, current| if current.1 > best.1 { current } else { best });
        } else if self.maximum.is_some_and(|(_, value)| input >= value) {
            self.maximum = Some((index, input));
        }
        if self.minimum.is_none() || self.minimum.is_some_and(|(i, _)| i < first_valid) {
            self.minimum =
                self.window
                    .iter()
                    .copied()
                    .reduce(|best, current| if current.1 < best.1 { current } else { best });
        } else if self.minimum.is_some_and(|(_, value)| input <= value) {
            self.minimum = Some((index, input));
        }
        RollingMinmaxIndexValue {
            minimum: self.minimum.expect("full window has a minimum").0,
            maximum: self.maximum.expect("full window has a maximum").0,
        }
    }

    fn reset(&mut self) {
        self.index = 0;
        self.window.clear();
        self.maximum = None;
        self.minimum = None;
    }
}

macro_rules! rolling_index_indicator {
    ($name:ident, $select:expr) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            extrema: RollingIndexExtrema,
            value: Option<f64>,
        }

        impl $name {
            /// Computes or updates `new` through the native Rust kernel.
            ///
            /// Parameters are the typed series and configuration values in the signature.
            ///
            /// Returns the computed value, aligned history, or a validation error.
            pub fn new(period: usize) -> TaResult<Self> {
                Ok(Self {
                    extrema: RollingIndexExtrema::new(period)?,
                    value: None,
                })
            }
        }

        impl StreamingIndicator for $name {
            type Output = f64;

            fn append(&mut self, input: f64) -> Option<f64> {
                let indices = self.extrema.append(input);
                self.value = Some($select(indices) as f64);
                self.value
            }

            fn value(&self) -> Option<f64> {
                self.value
            }

            fn reset(&mut self) {
                self.extrema.reset();
                self.value = None;
            }
        }
    };
}

rolling_index_indicator!(RollingArgmax, |value: RollingMinmaxIndexValue| value.maximum);
rolling_index_indicator!(RollingArgmin, |value: RollingMinmaxIndexValue| value.minimum);

#[derive(Debug, Clone)]
pub struct RollingMinmaxIndex {
    extrema: RollingIndexExtrema,
    value: Option<RollingMinmaxIndexValue>,
}

impl RollingMinmaxIndex {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(period: usize) -> TaResult<Self> {
        Ok(Self {
            extrema: RollingIndexExtrema::new(period)?,
            value: None,
        })
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, input: f64) -> RollingMinmaxIndexValue {
        let value = self.extrema.append(input);
        self.value = Some(value);
        value
    }

    /// Computes or updates `value` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn value(&self) -> Option<RollingMinmaxIndexValue> {
        self.value
    }

    /// Computes or updates `reset` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn reset(&mut self) {
        self.extrema.reset();
        self.value = None;
    }
}

/// Stateful midpoint of the rolling highest and lowest input values.
#[derive(Debug, Clone)]
pub struct RollingMidpoint {
    extrema: RollingExtrema,
    value: Option<f64>,
}

impl RollingMidpoint {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(period: usize) -> TaResult<Self> {
        Ok(Self {
            extrema: RollingExtrema::new(period)?,
            value: None,
        })
    }
}

impl StreamingIndicator for RollingMidpoint {
    type Output = f64;

    fn append(&mut self, input: f64) -> Option<f64> {
        self.value = self
            .extrema
            .append(input)
            .map(|(maximum, minimum)| (maximum + minimum) * 0.5);
        self.value
    }

    fn value(&self) -> Option<f64> {
        self.value
    }

    fn reset(&mut self) {
        self.extrema.reset();
        self.value = None;
    }
}

/// Stateful midpoint of rolling high maxima and low minima.
#[derive(Debug, Clone)]
pub struct RollingMidprice {
    highs: RollingExtrema,
    lows: RollingExtrema,
    value: Option<f64>,
}

impl RollingMidprice {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(period: usize) -> TaResult<Self> {
        Ok(Self {
            highs: RollingExtrema::new(period)?,
            lows: RollingExtrema::new(period)?,
            value: None,
        })
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, high: f64, low: f64) -> Option<f64> {
        let maximum = self.highs.append(high).map(|values| values.0);
        let minimum = self.lows.append(low).map(|values| values.1);
        self.value = maximum.zip(minimum).map(|(high, low)| (high + low) * 0.5);
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

    /// Computes or updates `reset` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn reset(&mut self) {
        self.highs.reset();
        self.lows.reset();
        self.value = None;
    }
}

macro_rules! unary_indicator {
    ($name:ident, $operation:expr) => {
        #[derive(Debug, Clone, Default)]
        pub struct $name {
            value: Option<f64>,
        }

        impl $name {
            /// Computes or updates `new` through the native Rust kernel.
            ///
            /// Parameters are the typed series and configuration values in the signature.
            ///
            /// Returns the computed value, aligned history, or a validation error.
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl StreamingIndicator for $name {
            type Output = f64;

            fn append(&mut self, input: f64) -> Option<f64> {
                self.value = Some($operation(input));
                self.value
            }

            fn value(&self) -> Option<f64> {
                self.value
            }

            fn reset(&mut self) {
                self.value = None;
            }
        }
    };
}

unary_indicator!(Acos, f64::acos);
unary_indicator!(Asin, f64::asin);
unary_indicator!(Atan, f64::atan);
unary_indicator!(Ceil, f64::ceil);
unary_indicator!(Cos, f64::cos);
unary_indicator!(Cosh, f64::cosh);
unary_indicator!(Exp, f64::exp);
unary_indicator!(Floor, f64::floor);
unary_indicator!(Ln, f64::ln);
unary_indicator!(Log10, f64::log10);
unary_indicator!(Sin, f64::sin);
unary_indicator!(Sinh, f64::sinh);
unary_indicator!(Sqrt, f64::sqrt);
unary_indicator!(Tan, f64::tan);
unary_indicator!(Tanh, f64::tanh);

macro_rules! binary_indicator {
    ($name:ident, $operation:expr) => {
        #[derive(Debug, Clone, Default)]
        pub struct $name {
            value: Option<f64>,
        }

        impl $name {
            /// Computes or updates `new` through the native Rust kernel.
            ///
            /// Parameters are the typed series and configuration values in the signature.
            ///
            /// Returns the computed value, aligned history, or a validation error.
            pub fn new() -> Self {
                Self::default()
            }

            /// Computes or updates `append` through the native Rust kernel.
            ///
            /// Parameters are the typed series and configuration values in the signature.
            ///
            /// Returns the computed value, aligned history, or a validation error.
            pub fn append(&mut self, left: f64, right: f64) -> f64 {
                let value = $operation(left, right);
                self.value = Some(value);
                value
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
                self.value = None;
            }
        }
    };
}

binary_indicator!(Add, |left: f64, right: f64| left + right);
binary_indicator!(Sub, |left: f64, right: f64| left - right);
binary_indicator!(Mult, |left: f64, right: f64| left * right);
binary_indicator!(Div, |left: f64, right: f64| left / right);

/// Stateful average price `(open + high + low + close) / 4`.
#[derive(Debug, Clone, Default)]
pub struct AveragePrice {
    value: Option<f64>,
}

impl AveragePrice {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new() -> Self {
        Self::default()
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, open: f64, high: f64, low: f64, close: f64) -> f64 {
        let value = (open + high + low + close) * 0.25;
        self.value = Some(value);
        value
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
        self.value = None;
    }
}

binary_indicator!(Medprice, |high: f64, low: f64| (high + low) * 0.5);

macro_rules! price3_indicator {
    ($name:ident, $operation:expr) => {
        #[derive(Debug, Clone, Default)]
        pub struct $name {
            value: Option<f64>,
        }

        impl $name {
            /// Computes or updates `new` through the native Rust kernel.
            ///
            /// Parameters are the typed series and configuration values in the signature.
            ///
            /// Returns the computed value, aligned history, or a validation error.
            pub fn new() -> Self {
                Self::default()
            }

            /// Computes or updates `append` through the native Rust kernel.
            ///
            /// Parameters are the typed series and configuration values in the signature.
            ///
            /// Returns the computed value, aligned history, or a validation error.
            pub fn append(&mut self, high: f64, low: f64, close: f64) -> f64 {
                let value = $operation(high, low, close);
                self.value = Some(value);
                value
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
                self.value = None;
            }
        }
    };
}

price3_indicator!(Typprice, |high: f64, low: f64, close: f64| (high
    + low
    + close)
    * (1.0 / 3.0));
price3_indicator!(Wclprice, |high: f64, low: f64, close: f64| (high
    + low
    + close
    + close)
    * 0.25);

#[derive(Debug, Clone)]
struct RollingMoments {
    inverse_period: f64,
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
            inverse_period: 1.0 / period as f64,
            window: Window::new(period)?,
            sum: 0.0,
            sum_squares: 0.0,
        })
    }

    fn append(&mut self, input: f64) -> Option<f64> {
        if self.window.is_full() {
            let old = self.window.push(input).expect("full moments window evicts");
            self.sum += input - old;
            self.sum_squares += (input - old).mul_add(input + old, 0.0);
        } else {
            self.window.push(input);
            self.sum += input;
            self.sum_squares = input.mul_add(input, self.sum_squares);
        }
        self.window.is_full().then(|| {
            let mean = self.sum * self.inverse_period;
            self.sum_squares * self.inverse_period - mean * mean
        })
    }

    fn reset(&mut self) {
        self.window.clear();
        self.sum = 0.0;
        self.sum_squares = 0.0;
    }
}

/// Stateful population variance. TA-Lib accepts but ignores `nbdev` for VAR.
#[derive(Debug, Clone)]
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

    fn append(&mut self, input: f64) -> Option<f64> {
        self.value = self
            .moments
            .append(input)
            .map(|variance| variance.max(0.0).sqrt() * self.nbdev);
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

#[derive(Debug, Clone, Copy)]
struct PairMoments {
    sx: f64,
    sy: f64,
    sxx: f64,
    syy: f64,
    sxy: f64,
}

#[derive(Debug, Clone)]
struct RollingPairMoments {
    period: usize,
    window: VecDeque<(f64, f64)>,
    moments: PairMoments,
}

impl RollingPairMoments {
    fn new(period: usize) -> TaResult<Self> {
        if period < 2 {
            return Err(invalid_period("timeperiod", period, 2));
        }
        Ok(Self {
            period,
            window: VecDeque::with_capacity(period),
            moments: PairMoments {
                sx: 0.0,
                sy: 0.0,
                sxx: 0.0,
                syy: 0.0,
                sxy: 0.0,
            },
        })
    }

    fn append(&mut self, x: f64, y: f64) -> Option<PairMoments> {
        if self.window.len() == self.period {
            let (old_x, old_y) = self.window.pop_front().expect("pair window is full");
            self.moments.sx += x - old_x;
            self.moments.sy += y - old_y;
            self.moments.sxx += x * x - old_x * old_x;
            self.moments.syy += y * y - old_y * old_y;
            self.moments.sxy += x * y - old_x * old_y;
        } else {
            self.moments.sx += x;
            self.moments.sy += y;
            self.moments.sxx += x * x;
            self.moments.syy += y * y;
            self.moments.sxy += x * y;
        }
        self.window.push_back((x, y));
        (self.window.len() == self.period).then_some(self.moments)
    }

    fn reset(&mut self) {
        self.window.clear();
        self.moments = PairMoments {
            sx: 0.0,
            sy: 0.0,
            sxx: 0.0,
            syy: 0.0,
            sxy: 0.0,
        };
    }

    fn reseed_linear_sums_with_batch_order(&mut self) -> PairMoments {
        let x: Vec<f64> = self.window.iter().map(|value| value.0).collect();
        let y: Vec<f64> = self.window.iter().map(|value| value.1).collect();
        self.moments.sx = crate::simd::sum_f64(&x);
        self.moments.sy = crate::simd::sum_f64(&y);
        self.moments
    }
}

/// Stateful Pearson correlation over paired observations.
#[derive(Debug, Clone)]
pub struct RollingCorrelation {
    period: f64,
    moments: RollingPairMoments,
    seeded: bool,
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
            seeded: false,
            value: None,
        })
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, x: f64, y: f64) -> Option<f64> {
        let moments = self.moments.append(x, y).map(|moments| {
            if self.seeded {
                moments
            } else {
                self.seeded = true;
                self.moments.reseed_linear_sums_with_batch_order()
            }
        });
        self.value = moments.map(|m| {
            let numerator = self.period * m.sxy - m.sx * m.sy;
            let denominator =
                ((self.period * m.sxx - m.sx * m.sx) * (self.period * m.syy - m.sy * m.sy)).sqrt();
            if denominator > 0.0 {
                numerator / denominator
            } else {
                0.0
            }
        });
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

    /// Computes or updates `reset` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn reset(&mut self) {
        self.moments.reset();
        self.seeded = false;
        self.value = None;
    }
}

/// Stateful TA-Lib BETA over percentage returns of two input series.
#[derive(Debug, Clone)]
pub struct RollingBeta {
    period: f64,
    previous: Option<(f64, f64)>,
    returns: RollingPairMoments,
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
            returns: RollingPairMoments::new(period)?,
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
        let x = (input0 - previous0) / previous0;
        let y = (input1 - previous1) / previous1;
        self.value = self.returns.append(x, y).map(|m| {
            let denominator = self.period * m.sxx - m.sx * m.sx;
            if denominator > 0.0 {
                (self.period * m.sxy - m.sx * m.sy) / denominator
            } else {
                0.0
            }
        });
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

#[derive(Debug, Clone, Copy)]
struct RegressionValue {
    slope: f64,
    intercept: f64,
}

#[derive(Debug, Clone)]
struct RegressionCore {
    period: usize,
    period_f: f64,
    sum_x: f64,
    denominator: f64,
    window: Window,
    sum_y: f64,
    weighted_sum: f64,
    seeded: bool,
}

impl RegressionCore {
    fn new(period: usize) -> TaResult<Self> {
        if period < 2 {
            return Err(invalid_period("timeperiod", period, 2));
        }
        let period_f = period as f64;
        let sum_x = period_f * (period_f - 1.0) / 2.0;
        let sum_x2 = period_f * (period_f - 1.0) * (2.0 * period_f - 1.0) / 6.0;
        Ok(Self {
            period,
            period_f,
            sum_x,
            denominator: period_f * sum_x2 - sum_x * sum_x,
            window: Window::new(period)?,
            sum_y: 0.0,
            weighted_sum: 0.0,
            seeded: false,
        })
    }

    fn append(&mut self, input: f64) -> Option<RegressionValue> {
        if !self.seeded {
            self.window.push(input);
            if !self.window.is_full() {
                return None;
            }
            let values: Vec<f64> = self.window.iter().copied().collect();
            self.sum_y = crate::simd::sum_f64(&values);
            self.weighted_sum = values
                .iter()
                .enumerate()
                .map(|(index, value)| index as f64 * value)
                .sum();
            self.seeded = true;
        } else {
            let old = self.window.push(input).expect("regression window is full");
            self.weighted_sum =
                self.weighted_sum - self.sum_y + old + (self.period_f - 1.0) * input;
            self.sum_y = self.sum_y - old + input;
        }
        let slope =
            (self.period_f * self.weighted_sum - self.sum_x * self.sum_y) / self.denominator;
        let intercept = (self.sum_y - slope * self.sum_x) / self.period_f;
        Some(RegressionValue { slope, intercept })
    }

    fn reset(&mut self) {
        self.window.clear();
        self.sum_y = 0.0;
        self.weighted_sum = 0.0;
        self.seeded = false;
    }
}

macro_rules! regression_indicator {
    ($name:ident, $calculate:expr) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            core: RegressionCore,
            value: Option<f64>,
        }

        impl $name {
            /// Computes or updates `new` through the native Rust kernel.
            ///
            /// Parameters are the typed series and configuration values in the signature.
            ///
            /// Returns the computed value, aligned history, or a validation error.
            pub fn new(period: usize) -> TaResult<Self> {
                Ok(Self {
                    core: RegressionCore::new(period)?,
                    value: None,
                })
            }
        }

        impl StreamingIndicator for $name {
            type Output = f64;

            fn append(&mut self, input: f64) -> Option<f64> {
                let period = self.core.period;
                self.value = self
                    .core
                    .append(input)
                    .map(|regression| $calculate(regression, period));
                self.value
            }

            fn value(&self) -> Option<f64> {
                self.value
            }

            fn reset(&mut self) {
                self.core.reset();
                self.value = None;
            }
        }
    };
}

regression_indicator!(Linearreg, |value: RegressionValue, period: usize| value
    .intercept
    + value.slope * (period - 1) as f64);
regression_indicator!(LinearregSlope, |value: RegressionValue, _| value.slope);
regression_indicator!(LinearregIntercept, |value: RegressionValue, _| value
    .intercept);
regression_indicator!(LinearregAngle, |value: RegressionValue, _| value
    .slope
    .atan()
    .to_degrees());
regression_indicator!(Tsf, |value: RegressionValue, period: usize| value.intercept
    + value.slope * period as f64);

fn ad_increment(high: f64, low: f64, close: f64, volume: f64) -> f64 {
    let range = high - low;
    if range > 0.0 {
        ((close - low) - (high - close)) / range * volume
    } else {
        0.0
    }
}

/// Stateful Chaikin accumulation/distribution line.
#[derive(Debug, Clone, Default)]
pub struct AccumulationDistribution {
    total: f64,
    value: Option<f64>,
}

impl AccumulationDistribution {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new() -> Self {
        Self::default()
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, high: f64, low: f64, close: f64, volume: f64) -> f64 {
        self.total += ad_increment(high, low, close, volume);
        self.value = Some(self.total);
        self.total
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
        self.total = 0.0;
        self.value = None;
    }
}

/// Stateful Chaikin A/D oscillator with first-value EMA seeds.
#[derive(Debug, Clone)]
pub struct AccumulationDistributionOscillator {
    lookback: usize,
    index: usize,
    fast_k: f64,
    slow_k: f64,
    ad: f64,
    fast_ema: Option<f64>,
    slow_ema: Option<f64>,
    value: Option<f64>,
}

impl AccumulationDistributionOscillator {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(fast_period: usize, slow_period: usize) -> TaResult<Self> {
        if fast_period < 2 || slow_period < 2 {
            return Err(TaError::InvalidParameter {
                name: "fastperiod/slowperiod",
                value: format!("{fast_period}/{slow_period}"),
                reason: "both periods must be >= 2",
            });
        }
        Ok(Self {
            lookback: fast_period.max(slow_period) - 1,
            index: 0,
            fast_k: 2.0 / (fast_period as f64 + 1.0),
            slow_k: 2.0 / (slow_period as f64 + 1.0),
            ad: 0.0,
            fast_ema: None,
            slow_ema: None,
            value: None,
        })
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, high: f64, low: f64, close: f64, volume: f64) -> Option<f64> {
        self.ad += ad_increment(high, low, close, volume);
        match (self.fast_ema, self.slow_ema) {
            (Some(fast), Some(slow)) => {
                self.fast_ema = Some(self.fast_k.mul_add(self.ad - fast, fast));
                self.slow_ema = Some(self.slow_k.mul_add(self.ad - slow, slow));
            }
            _ => {
                self.fast_ema = Some(self.ad);
                self.slow_ema = Some(self.ad);
            }
        }
        if self.index >= self.lookback {
            self.value = Some(
                self.fast_ema.expect("fast EMA is initialized")
                    - self.slow_ema.expect("slow EMA is initialized"),
            );
        }
        self.index += 1;
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

    /// Computes or updates `reset` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn reset(&mut self) {
        self.index = 0;
        self.ad = 0.0;
        self.fast_ema = None;
        self.slow_ema = None;
        self.value = None;
    }
}

/// Stateful on-balance volume.
#[derive(Debug, Clone, Default)]
pub struct OnBalanceVolume {
    previous_close: Option<f64>,
    total: f64,
    value: Option<f64>,
}

impl OnBalanceVolume {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new() -> Self {
        Self::default()
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, close: f64, volume: f64) -> f64 {
        match self.previous_close.replace(close) {
            None => self.total = volume,
            Some(previous) if close > previous => self.total += volume,
            Some(previous) if close < previous => self.total -= volume,
            Some(_) => {}
        }
        self.value = Some(self.total);
        self.total
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
        self.previous_close = None;
        self.total = 0.0;
        self.value = None;
    }
}

/// Stateful balance of power.
#[derive(Debug, Clone, Default)]
pub struct BalanceOfPower {
    value: Option<f64>,
}

impl BalanceOfPower {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new() -> Self {
        Self::default()
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, open: f64, high: f64, low: f64, close: f64) -> f64 {
        let range = high - low;
        let value = if range > 0.0 {
            (close - open) / range
        } else {
            0.0
        };
        self.value = Some(value);
        value
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
        self.value = None;
    }
}

/// Stateful Williams %R.
#[derive(Debug, Clone)]
pub struct WilliamsPercentR {
    highs: RollingExtrema,
    lows: RollingExtrema,
    value: Option<f64>,
}

impl WilliamsPercentR {
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
            highs: RollingExtrema::new(period)?,
            lows: RollingExtrema::new(period)?,
            value: None,
        })
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, high: f64, low: f64, close: f64) -> Option<f64> {
        let maximum = self.highs.append(high).map(|value| value.0);
        let minimum = self.lows.append(low).map(|value| value.1);
        self.value = maximum.zip(minimum).map(|(maximum, minimum)| {
            let range = maximum - minimum;
            if range > 0.0 {
                -100.0 * (maximum - close) / range
            } else {
                0.0
            }
        });
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

    /// Computes or updates `reset` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn reset(&mut self) {
        self.highs.reset();
        self.lows.reset();
        self.value = None;
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AroonValue {
    pub down: f64,
    pub up: f64,
}

/// Stateful Aroon down/up pair over a `period + 1` bar window.
#[derive(Debug, Clone)]
pub struct Aroon {
    period: usize,
    inverse_period: f64,
    index: usize,
    highs: RollingExtrema,
    lows: RollingExtrema,
    value: Option<AroonValue>,
}

impl Aroon {
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
            inverse_period: 100.0 / period as f64,
            index: 0,
            highs: RollingExtrema::new(period + 1)?,
            lows: RollingExtrema::new(period + 1)?,
            value: None,
        })
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, high: f64, low: f64) -> Option<AroonValue> {
        let current = self.index;
        self.index += 1;
        let highest = self.highs.append_indexed(high).map(|value| value.0 .0);
        let lowest = self.lows.append_indexed(low).map(|value| value.1 .0);
        self.value = highest.zip(lowest).map(|(highest, lowest)| AroonValue {
            down: (self.period - (current - lowest)) as f64 * self.inverse_period,
            up: (self.period - (current - highest)) as f64 * self.inverse_period,
        });
        self.value
    }

    /// Computes or updates `value` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn value(&self) -> Option<AroonValue> {
        self.value
    }

    /// Computes or updates `reset` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn reset(&mut self) {
        self.index = 0;
        self.highs.reset();
        self.lows.reset();
        self.value = None;
    }
}

#[derive(Debug, Clone)]
pub struct AroonOscillator {
    aroon: Aroon,
    value: Option<f64>,
}

impl AroonOscillator {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(period: usize) -> TaResult<Self> {
        Ok(Self {
            aroon: Aroon::new(period)?,
            value: None,
        })
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, high: f64, low: f64) -> Option<f64> {
        self.value = self
            .aroon
            .append(high, low)
            .map(|value| value.up - value.down);
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

    /// Computes or updates `reset` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn reset(&mut self) {
        self.aroon.reset();
        self.value = None;
    }
}

/// Stateful Average True Range.  Each appended bar is `(high, low, close)`.
#[derive(Debug, Clone)]
pub struct AverageTrueRange {
    period: usize,
    previous_close: Option<f64>,
    tr_count: usize,
    tr_sum: f64,
    value: Option<f64>,
}

/// Stateful true range. The first bar has no previous close and is not warm.
#[derive(Debug, Clone)]
pub struct TrueRange {
    previous_close: Option<f64>,
    value: Option<f64>,
}

impl TrueRange {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new() -> Self {
        Self {
            previous_close: None,
            value: None,
        }
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, high: f64, low: f64, close: f64) -> Option<f64> {
        let previous = self.previous_close.replace(close)?;
        self.value = Some(
            (high - low)
                .max((high - previous).abs())
                .max((low - previous).abs()),
        );
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

    /// Computes or updates `reset` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn reset(&mut self) {
        self.previous_close = None;
        self.value = None;
    }
}

impl Default for TrueRange {
    fn default() -> Self {
        Self::new()
    }
}

/// Stateful normalized ATR, matching `NATR = ATR / close * 100`.
#[derive(Debug, Clone)]
pub struct NormalizedAverageTrueRange {
    atr: AverageTrueRange,
    value: Option<f64>,
}

impl NormalizedAverageTrueRange {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(period: usize) -> TaResult<Self> {
        Ok(Self {
            atr: AverageTrueRange::new(period)?,
            value: None,
        })
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, high: f64, low: f64, close: f64) -> Option<f64> {
        self.value = self.atr.append(high, low, close).map(|atr| {
            if close == 0.0 {
                0.0
            } else {
                atr / close * 100.0
            }
        });
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

    /// Computes or updates `reset` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn reset(&mut self) {
        self.atr.reset();
        self.value = None;
    }
}

impl AverageTrueRange {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(period: usize) -> TaResult<Self> {
        if period == 0 {
            return Err(invalid_period("timeperiod", period, 1));
        }
        Ok(Self {
            period,
            previous_close: None,
            tr_count: 0,
            tr_sum: 0.0,
            value: None,
        })
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, high: f64, low: f64, close: f64) -> Option<f64> {
        let Some(previous_close) = self.previous_close.replace(close) else {
            return None;
        };
        let true_range = (high - low)
            .max((high - previous_close).abs())
            .max((low - previous_close).abs());
        self.tr_count += 1;

        if self.tr_count < self.period {
            self.tr_sum += true_range;
            return None;
        }

        if self.tr_count == self.period {
            self.value = Some((self.tr_sum + true_range) / self.period as f64);
        } else if let Some(previous) = self.value {
            let period = self.period as f64;
            self.value = Some((previous * (period - 1.0) + true_range) / period);
        }
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

    /// Computes or updates `reset` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn reset(&mut self) {
        self.previous_close = None;
        self.tr_count = 0;
        self.tr_sum = 0.0;
        self.value = None;
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
        let mut mom = Mom::new(7).unwrap();
        let mut roc = Roc::new(7).unwrap();
        let mut rocp = Rocp::new(7).unwrap();
        let mut rocr = Rocr::new(7).unwrap();
        let mut rocr100 = Rocr100::new(7).unwrap();

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
        let mut med_state = Medprice::new();
        let mut typ_state = Typprice::new();
        let mut wcl_state = Wclprice::new();
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
