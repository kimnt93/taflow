"Descriptive, stateful Python interface for TAFlow."

__version__ = "0.1.2"

from .ma_type import MaType
from .indicators import AccelerationBands
from .indicators import AverageTrueRange
from .indicators import DoubleExponentialMovingAverage
from .indicators.kaufman_adaptive_moving_average import KaufmanAdaptiveMovingAverage
from .indicators import Aroon, AroonOscillator
from .indicators import AccumulationDistribution, AccumulationDistributionOscillator
from .indicators import BalanceOfPower, OnBalanceVolume
from .indicators.absolute_price_oscillator import AbsolutePriceOscillator
from .indicators import PercentagePriceOscillator
from .indicators.chande_momentum_oscillator import ChandeMomentumOscillator
from .indicators import Momentum
from .indicators import RateOfChange
from .indicators import RateOfChangePercent
from .indicators import RateOfChangeRatio
from .indicators import RateOfChangeRatioPercent
from .indicators import WilliamsPercentR
from .indicators import AverageDirectionalIndex, AverageDirectionalIndexRating
from .indicators.bollinger_bands import BollingerBands
from .indicators.commodity_channel_index import CommodityChannelIndex
from .indicators import DirectionalMovementIndex
from .indicators.candle_doji import CandleDoji
from .indicators.candle_takuri import CandleTakuri
from .indicators.candle_marubozu import CandleMarubozu
from .indicators.candle_closing_marubozu import CandleClosingMarubozu
from .indicators.candle_long_legged_doji import CandleLongLeggedDoji
from .indicators.candle_rickshawman import CandleRickshawman
from .indicators.candle_high_wave import CandleHighWave
from .indicators.candle_dragonfly_doji import CandleDragonflyDoji
from .indicators.candle_gravestone_doji import CandleGravestoneDoji
from .indicators.candle_short_line import CandleShortLine
from .indicators.candle_spinning_top import CandleSpinningTop
from .indicators.candle_long_line import CandleLongLine
from .indicators.candle_doji_star import CandleDojiStar
from .indicators.candle_belt_hold import CandleBeltHold
from .indicators.candle_engulfing import CandleEngulfing
from .indicators import ExponentialMovingAverage
from .indicators import FastStochasticOscillator
from .indicators import HilbertTransformTrendline
from .indicators.candle_hikkake import CandleHikkake
from .indicators.candle_hikkake_modified import CandleHikkakeModified
from .indicators.candle_hammer import CandleHammer
from .indicators.hilbert_transform_dominant_cycle_period import HilbertTransformDominantCyclePeriod
from .indicators.hilbert_transform_dominant_cycle_phase import HilbertTransformDominantCyclePhase
from .indicators.hilbert_transform_phasor import HilbertTransformPhasor
from .indicators.hilbert_transform_sine_wave import HilbertTransformSineWave
from .indicators.hilbert_transform_trend_mode import HilbertTransformTrendMode
from .indicators.intraday_momentum_index import IntradayMomentumIndex
from .indicators import MovingAverage
from .indicators import MinusDirectionalIndicator, MinusDirectionalMovement
from .indicators import MoneyFlowIndex
from .indicators import AveragePrice
from .indicators import MedianPrice, TypicalPrice, WeightedClose
from .indicators.moving_average_convergence_divergence_fixed import (
    MovingAverageConvergenceDivergenceFixed,
)
from .indicators.moving_average_convergence_divergence import (
    MovingAverageConvergenceDivergence,
)
from .indicators.moving_average_convergence_divergence_extended import (
    MovingAverageConvergenceDivergenceExtended,
)
from .indicators import ParabolicSar, ParabolicSarExtended
from .indicators import PlusDirectionalIndicator, PlusDirectionalMovement
from .indicators import StochasticOscillator, StochasticRelativeStrengthIndex
from .indicators.candle_stick_sandwich import CandleStickSandwich
from .indicators import TripleExponentialRateOfChange
from .indicators.candle_three_black_crows import CandleThreeBlackCrows
from .indicators.candle_three_inside import CandleThreeInside
from .indicators.candle_three_line_strike import CandleThreeLineStrike
from .indicators.candle_three_stars_in_south import CandleThreeStarsInSouth
from .indicators.candle_three_outside import CandleThreeOutside
from .indicators.candle_two_crows import CandleTwoCrows
from .indicators import UltimateOscillator
from .indicators import VariablePeriodMovingAverage
from .indicators.heikin_ashi import HeikinAshi
from .indicators import FibonacciRetracement
from .indicators.anchored_volume_weighted_average_price import (
    AnchoredVolumeWeightedAveragePrice,
)
from .indicators import VariableIndexDynamicAverage
from .indicators.laguerre_relative_strength_index import LaguerreRelativeStrengthIndex
from .indicators import SimpleMovingAverage
from .indicators import NormalizedAverageTrueRange, TrueRange
from .indicators import TriangularMovingAverage
from .indicators import TripleExponentialMovingAverage, TripleExponentialAverage
from .indicators import WeightedMovingAverage
from .indicators import RelativeStrengthIndex
from .indicators import RelativeMomentumIndex
from .indicators import SmoothedTrendChannel
from .indicators.jurik_moving_average import JurikMovingAverage
from .indicators import ParabolicMovingAverageStop
from .indicators import TomDeMarkSequential
from .indicators import EvenBetterSinewave
from .indicators import PremiumDiscount
from .indicators import OpeningRange
from .indicators import PivotPoints
from . import executions
from . import op
from .indicators import MathAbs, MathAcos
from .indicators import MathAcosh
from .indicators import MathAsin
from .indicators import MathAsinh
from .indicators import MathAtan
from .indicators import MathAtanh, MathCbrt, MathCeil
from .indicators import MathCos
from .indicators import MathCosh, MathCot, MathDegrees, MathExp, MathFloor
from .indicators import MathLn, MathLog10, MathLog1p, MathRadians
from .indicators import MathSin
from .indicators import MathSinh, MathSqrt, MathTan, MathTanh, MathAdd, MathSubtract
from .indicators.math_multiply import MathMultiply
from .indicators.math_divide import MathDivide
from .indicators import (
    RollingAverageDeviation,
    RollingMidpoint,
    RollingMidprice,
    RollingStandardDeviation,
    RollingVariance,
)
from .indicators import (
    RollingBeta,
    RollingCorrelation,
    RollingLinearRegression,
    RollingLinearRegressionAngle,
    RollingLinearRegressionIntercept,
    RollingLinearRegressionSlope,
    RollingTimeSeriesForecast,
)
from .indicators import MesaAdaptiveMovingAverage, RollingMinMax, RollingMinMaxIndex
from .indicators import KlingerVolumeOscillator
from .indicators import SessionVolumeLevels
from .indicators.candle_up_down_side_gap_three_methods import CandleUpDownSideGapThreeMethods
from .indicators.candle_three_white_soldiers import CandleThreeWhiteSoldiers
from .indicators.candle_abandoned_baby import CandleAbandonedBaby
from .indicators.candle_advance_block import CandleAdvanceBlock
from .indicators.candle_breakaway import CandleBreakaway
from .indicators.candle_conceal_baby_swall import CandleConcealBabySwall
from .indicators.candle_counter_attack import CandleCounterAttack
from .indicators.candle_dark_cloud_cover import CandleDarkCloudCover
from .indicators.candle_evening_doji_star import CandleEveningDojiStar
from .indicators.candle_evening_star import CandleEveningStar
from .indicators.candle_gap_side_side_white import CandleGapSideSideWhite
from .indicators.candle_hanging_man import CandleHangingMan
from .indicators.candle_harami import CandleHarami
from .indicators.candle_harami_cross import CandleHaramiCross
from .indicators.candle_homing_pigeon import CandleHomingPigeon
from .indicators.candle_identical_three_crows import CandleIdenticalThreeCrows
from .indicators.candle_in_neck import CandleInNeck
from .indicators.candle_inverted_hammer import CandleInvertedHammer
from .indicators.candle_kicking import CandleKicking
from .indicators.candle_kicking_by_length import CandleKickingByLength
from .indicators.candle_ladder_bottom import CandleLadderBottom
from .indicators.candle_matching_low import CandleMatchingLow
from .indicators.candle_mat_hold import CandleMatHold
from .indicators.candle_morning_doji_star import CandleMorningDojiStar
from .indicators.candle_morning_star import CandleMorningStar
from .indicators.candle_on_neck import CandleOnNeck
from .indicators.candle_piercing import CandlePiercing
from .indicators.candle_rise_fall_three_methods import CandleRiseFallThreeMethods
from .indicators.candle_separating_lines import CandleSeparatingLines
from .indicators.candle_shooting_star import CandleShootingStar
from .indicators.candle_stalled_pattern import CandleStalledPattern
from .indicators.candle_tasuki_gap import CandleTasukiGap
from .indicators.candle_thrusting import CandleThrusting
from .indicators.candle_tri_star import CandleTriStar
from .indicators.candle_unique_three_river import CandleUniqueThreeRiver
from .indicators.candle_upside_gap_two_crows import CandleUpsideGapTwoCrows
from .indicators import Lag
from .indicators import LogReturn
from .indicators import CumulativeSum
from .indicators import CumulativeProduct
from .indicators import CumulativeCount
from .indicators import RollingMedian
from .indicators import RollingMinimum, RollingMaximum
from .indicators import RollingSum
from .indicators import RollingMinimumIndex, RollingMaximumIndex
from .indicators import RollingMode
from .indicators import RollingQuantile
from .indicators import RollingPercentile
from .indicators import RollingRank
from .indicators import RollingZScore
from .indicators import RollingSkew, RollingKurtosis
from .indicators import RollingInterquartileRange
from .indicators import RollingCovariance
from .indicators import RollingWinsorize
from .indicators import ExponentiallyWeightedVariance
from .indicators import ExponentiallyWeightedSum
from .indicators import ExponentiallyWeightedStandardDeviation
from .indicators import ExponentiallyWeightedCovariance
from .indicators import ExponentiallyWeightedCorrelation
from .indicators.cumulative_maximum import CumulativeMaximum
from .indicators.cumulative_minimum import CumulativeMinimum
from .indicators import Drawdown
from .indicators import RollingSharpe
from .indicators import RollingSortino
from .indicators import RollingCalmar
from .indicators import HullMovingAverage
from .indicators import VolumeWeightedMovingAverage
from .indicators import ZeroLagExponentialMovingAverage
from .indicators import ArnaudLegouxMovingAverage
from .indicators import TrueStrengthIndex
from .indicators import AwesomeOscillator
from .indicators import FisherTransform
from .indicators.donchian import Donchian
from .indicators.ulcer_index import UlcerIndex
from .indicators.keltner_channels import KeltnerChannels
from .indicators.chaikin_volatility import ChaikinVolatility
from .indicators.crossover import Crossover
from .indicators.crossunder import Crossunder
from .indicators.cross import Cross
from .indicators.rising import Rising
from .indicators.falling import Falling
from .indicators.rolling_volume_weighted_average_price import RollingVolumeWeightedAveragePrice
from .indicators.force_index import ForceIndex
from .indicators.ease_of_movement import EaseOfMovement
from .indicators.higher_high import HigherHigh
from .indicators.lower_low import LowerLow
from .indicators.inside_bar import InsideBar
from .indicators.outside_bar import OutsideBar
from .indicators.gap_up import GapUp
from .indicators.gap_down import GapDown
from .indicators.bars_since import BarsSince
from .indicators.value_when import ValueWhen
from .indicators.highest_since import HighestSince
from .indicators.lowest_since import LowestSince
from .indicators.signal_delay import SignalDelay
from .indicators.position_hold import PositionHold
from .indicators.entry_exit import EntryExit
from .indicators.swing_high_low import SwingHighLow
from .indicators import Retracements
from .indicators import SessionExtrema
from .indicators import PreviousHighLow
from .indicators import Sessions
from .indicators.active_zone_list import ActiveZoneList
from .indicators.fair_value_gap import FairValueGap
from .indicators.break_of_structure_change_of_character import BreakOfStructureChangeOfCharacter
from .indicators.order_block import OrderBlock
from .indicators.liquidity import Liquidity
from .indicators.equal_highs_lows import EqualHighsLows
from .indicators.hedge_ratio import HedgeRatio
from .indicators import RollingEntropy
from .indicators import RollingAutocorr
from .indicators.hurst import Hurst
from .indicators.fractal_dimension import FractalDimension
from .indicators.rolling_alpha import RollingAlpha
from .indicators.rolling_information_ratio import RollingInformationRatio
from .indicators.close_to_close_sigma import CloseToCloseSigma
from .indicators import Parkinson
from .indicators.garman_klass import GarmanKlass
from .indicators import RogersSatchell
from .indicators.garman_klass_yang_zhang import GarmanKlassYangZhang
from .indicators import YangZhang
from .indicators.average_daily_dollar_value import AverageDailyDollarValue
from .indicators.amihud import Amihud
from .indicators import RollSpread, OrnsteinUhlenbeckHalfLife
from .indicators.cumulative_sum_control_chart import CumulativeSumControlChart
from .indicators.spread_z_score import SpreadZScore
from .indicators import FracDiff
from .indicators.kalman_hedge_ratio import KalmanHedgeRatio
from .indicators import Supertrend
from .indicators import Ichimoku
from .indicators import Squeeze, SqueezePro
from .indicators import SchaffTrendCycle
from .indicators import Vortex
from .indicators.know_sure_thing import KnowSureThing
from .indicators.mass_index import MassIndex
from .indicators.detrended_price_oscillator import DetrendedPriceOscillator
from .indicators.chaikin_money_flow import ChaikinMoneyFlow
from .indicators.volume_price_trend import VolumePriceTrend
from .indicators.negative_volume_index import NegativeVolumeIndex
from .indicators.positive_volume_index import PositiveVolumeIndex
from .indicators.mc_ginley_dynamic import McGinleyDynamic
from .indicators import DecayLinear
from .indicators import SignedPower
from .indicators import TimeSeriesRank

__all__ = [
    "MovingAverage",
    "MinusDirectionalIndicator",
    "MinusDirectionalMovement",
    "MoneyFlowIndex",
    "AveragePrice",
    "MedianPrice",
    "TypicalPrice",
    "WeightedClose",
    "MovingAverageConvergenceDivergenceFixed",
    "MovingAverageConvergenceDivergence",
    "MovingAverageConvergenceDivergenceExtended",
    "BollingerBands",
    "CommodityChannelIndex",
    "FastStochasticOscillator",
    "HilbertTransformTrendline",
    "CandleHikkake",
    "CandleHikkakeModified",
    "CandleHammer",
    "HilbertTransformDominantCyclePeriod",
    "HilbertTransformDominantCyclePhase",
    "HilbertTransformPhasor",
    "HilbertTransformSineWave",
    "HilbertTransformTrendMode",
    "StochasticOscillator",
    "StochasticRelativeStrengthIndex",
    "CandleStickSandwich",
    "TripleExponentialRateOfChange",
    "CandleThreeBlackCrows",
    "CandleThreeInside",
    "CandleThreeLineStrike",
    "CandleThreeStarsInSouth",
    "CandleThreeOutside",
    "CandleTwoCrows",
    "UltimateOscillator",
    "VariablePeriodMovingAverage",
    "HeikinAshi",
    "FibonacciRetracement",
    "AnchoredVolumeWeightedAveragePrice",
    "VariableIndexDynamicAverage",
    "LaguerreRelativeStrengthIndex",
    "RelativeStrengthIndex",
    "RelativeMomentumIndex",
    "SmoothedTrendChannel",
    "JurikMovingAverage",
    "ParabolicMovingAverageStop",
    "TomDeMarkSequential",
    "EvenBetterSinewave",
    "PremiumDiscount",
    "OpeningRange",
    "PivotPoints",
    "KlingerVolumeOscillator",
    "SessionVolumeLevels",
    "CandleUpDownSideGapThreeMethods",
    "IntradayMomentumIndex",
    "AccelerationBands",
    "AverageTrueRange",
    "DoubleExponentialMovingAverage",
    "KaufmanAdaptiveMovingAverage",
    "SimpleMovingAverage",
    "NormalizedAverageTrueRange",
    "TrueRange",
    "TriangularMovingAverage",
    "TripleExponentialMovingAverage",
    "TripleExponentialAverage",
    "WeightedMovingAverage",
    "Aroon",
    "AroonOscillator",
    "AccumulationDistribution",
    "AccumulationDistributionOscillator",
    "BalanceOfPower",
    "OnBalanceVolume",
    "AbsolutePriceOscillator",
    "PercentagePriceOscillator",
    "ChandeMomentumOscillator",
    "Momentum",
    "RateOfChange",
    "RateOfChangePercent",
    "RateOfChangeRatio",
    "RateOfChangeRatioPercent",
    "WilliamsPercentR",
    "AverageDirectionalIndex",
    "AverageDirectionalIndexRating",
    "DirectionalMovementIndex",
    "CandleDoji",
    "CandleTakuri",
    "CandleMarubozu",
    "CandleClosingMarubozu",
    "CandleLongLeggedDoji",
    "CandleRickshawman",
    "CandleHighWave",
    "CandleDragonflyDoji",
    "CandleGravestoneDoji",
    "CandleShortLine",
    "CandleSpinningTop",
    "CandleLongLine",
    "CandleDojiStar",
    "CandleBeltHold",
    "CandleEngulfing",
    "ExponentialMovingAverage",
    "ParabolicSar",
    "ParabolicSarExtended",
    "PlusDirectionalIndicator",
    "PlusDirectionalMovement",
    "CandleThreeWhiteSoldiers",
    "CandleAbandonedBaby",
    "CandleAdvanceBlock",
    "CandleBreakaway",
    "CandleConcealBabySwall",
    "CandleCounterAttack",
    "CandleDarkCloudCover",
    "CandleEveningDojiStar",
    "CandleEveningStar",
    "CandleGapSideSideWhite",
    "CandleHangingMan",
    "CandleHarami",
    "CandleHaramiCross",
    "CandleHomingPigeon",
    "CandleIdenticalThreeCrows",
    "CandleInNeck",
    "CandleInvertedHammer",
    "CandleKicking",
    "CandleKickingByLength",
    "CandleLadderBottom",
    "CandleMatchingLow",
    "CandleMatHold",
    "CandleMorningDojiStar",
    "CandleMorningStar",
    "CandleOnNeck",
    "CandlePiercing",
    "CandleRiseFallThreeMethods",
    "CandleSeparatingLines",
    "CandleShootingStar",
    "CandleStalledPattern",
    "CandleTasukiGap",
    "CandleThrusting",
    "CandleTriStar",
    "CandleUniqueThreeRiver",
    "CandleUpsideGapTwoCrows",
    "Lag",
    "LogReturn",
    "CumulativeSum",
    "CumulativeProduct",
    "CumulativeCount",
    "RollingMedian",
    "RollingMinimum",
    "RollingMaximum",
    "RollingSum",
    "RollingMinimumIndex",
    "RollingMaximumIndex",
    "RollingMode",
    "RollingQuantile",
    "RollingPercentile",
    "RollingRank",
    "RollingZScore",
    "RollingSkew",
    "RollingKurtosis",
    "RollingInterquartileRange",
    "RollingCovariance",
    "RollingWinsorize",
    "RollingStandardDeviation",
    "RollingAverageDeviation", "RollingMidpoint", "RollingMidprice",
    "RollingVariance", "RollingLinearRegression", "RollingLinearRegressionAngle",
    "RollingLinearRegressionIntercept", "RollingLinearRegressionSlope",
    "RollingTimeSeriesForecast", "RollingBeta", "RollingCorrelation",
    "MesaAdaptiveMovingAverage",
    "RollingMinMax", "RollingMinMaxIndex",
    "MathAbs", "MathAcos", "MathAcosh", "MathAsin", "MathAsinh",
    "MathAtan", "MathAtanh", "MathCbrt", "MathCeil", "MathCos",
    "MathCosh", "MathCot", "MathDegrees", "MathExp", "MathFloor",
    "MathLn", "MathLog10", "MathLog1p", "MathRadians", "MathSin",
    "MathSinh", "MathSqrt", "MathTan", "MathTanh",
    "MathAdd", "MathSubtract", "MathMultiply", "MathDivide",
    "ExponentiallyWeightedVariance",
    "ExponentiallyWeightedSum",
    "ExponentiallyWeightedStandardDeviation",
    "ExponentiallyWeightedCovariance",
    "ExponentiallyWeightedCorrelation",
    "CumulativeMaximum",
    "CumulativeMinimum",
    "Drawdown",
    "RollingSharpe",
    "RollingSortino",
    "RollingCalmar",
    "HullMovingAverage",
    "VolumeWeightedMovingAverage",
    "ZeroLagExponentialMovingAverage",
    "ArnaudLegouxMovingAverage",
    "TrueStrengthIndex",
    "AwesomeOscillator",
    "FisherTransform",
    "Donchian",
    "UlcerIndex",
    "KeltnerChannels",
    "ChaikinVolatility",
    "Crossover",
    "Crossunder",
    "Cross",
    "Rising",
    "Falling",
    "RollingVolumeWeightedAveragePrice",
    "ForceIndex",
    "EaseOfMovement",
    "HigherHigh",
    "LowerLow",
    "InsideBar",
    "OutsideBar",
    "GapUp",
    "GapDown",
    "BarsSince",
    "ValueWhen",
    "HighestSince",
    "LowestSince",
    "SignalDelay",
    "PositionHold",
    "EntryExit",
    "SwingHighLow",
    "Retracements",
    "SessionExtrema",
    "PreviousHighLow",
    "Sessions",
    "ActiveZoneList",
    "FairValueGap",
    "BreakOfStructureChangeOfCharacter",
    "OrderBlock",
    "Liquidity",
    "EqualHighsLows",
    "HedgeRatio",
    "RollingEntropy",
    "RollingAutocorr",
    "Hurst",
    "FractalDimension",
    "RollingAlpha",
    "RollingInformationRatio",
    "CloseToCloseSigma",
    "Parkinson",
    "GarmanKlass",
    "RogersSatchell",
    "GarmanKlassYangZhang",
    "YangZhang",
    "AverageDailyDollarValue",
    "Amihud",
    "RollSpread",
    "OrnsteinUhlenbeckHalfLife",
    "CumulativeSumControlChart",
    "SpreadZScore",
    "FracDiff",
    "KalmanHedgeRatio",
    "Supertrend",
    "Ichimoku",
    "Squeeze",
    "SqueezePro",
    "SchaffTrendCycle",
    "Vortex",
    "KnowSureThing",
    "MassIndex",
    "DetrendedPriceOscillator",
    "ChaikinMoneyFlow",
    "VolumePriceTrend",
    "NegativeVolumeIndex",
    "PositiveVolumeIndex",
    "McGinleyDynamic",
    "DecayLinear",
    "SignedPower",
    "TimeSeriesRank",
    "op",
    "__version__",
]

# Public adapters share a native-backed lifecycle.  A few older extension
# classes do not yet export Rust's inexpensive length method, so install the
# common adapter implementation once after the complete public API is known.
from ._adapter_protocol import install_adapter_protocol as _install_adapter_protocol

_install_adapter_protocol(globals(), __all__)
del _install_adapter_protocol
