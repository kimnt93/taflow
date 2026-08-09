"Descriptive, stateful Python interface for TAFlow."

__version__ = "0.1.2"

from .ma_type import MaType
from .acceleration_bands import AccelerationBands
from .average_true_range import AverageTrueRange
from .double_exponential_moving_average import DoubleExponentialMovingAverage
from .kaufman_adaptive_moving_average import KaufmanAdaptiveMovingAverage
from .aroon import Aroon
from .aroon_oscillator import AroonOscillator
from .accumulation_distribution import AccumulationDistribution
from .accumulation_distribution_oscillator import AccumulationDistributionOscillator
from .balance_of_power import BalanceOfPower
from .on_balance_volume import OnBalanceVolume
from .absolute_price_oscillator import AbsolutePriceOscillator
from .percentage_price_oscillator import PercentagePriceOscillator
from .chande_momentum_oscillator import ChandeMomentumOscillator
from .momentum import Momentum
from .rate_of_change import RateOfChange
from .rate_of_change_percent import RateOfChangePercent
from .rate_of_change_ratio import RateOfChangeRatio
from .rate_of_change_ratio_percent import RateOfChangeRatioPercent
from .williams_percent_r import WilliamsPercentR
from .average_directional_index import AverageDirectionalIndex
from .average_directional_index_rating import AverageDirectionalIndexRating
from .bollinger_bands import BollingerBands
from .commodity_channel_index import CommodityChannelIndex
from .directional_movement_index import DirectionalMovementIndex
from .candle_doji import CandleDoji
from .candle_takuri import CandleTakuri
from .candle_marubozu import CandleMarubozu
from .candle_closing_marubozu import CandleClosingMarubozu
from .candle_long_legged_doji import CandleLongLeggedDoji
from .candle_rickshawman import CandleRickshawman
from .candle_high_wave import CandleHighWave
from .candle_dragonfly_doji import CandleDragonflyDoji
from .candle_gravestone_doji import CandleGravestoneDoji
from .candle_short_line import CandleShortLine
from .candle_spinning_top import CandleSpinningTop
from .candle_long_line import CandleLongLine
from .candle_doji_star import CandleDojiStar
from .candle_belt_hold import CandleBeltHold
from .candle_engulfing import CandleEngulfing
from .exponential_moving_average import ExponentialMovingAverage
from .fast_stochastic_oscillator import FastStochasticOscillator
from .hilbert_transform_trendline import HilbertTransformTrendline
from .candle_hikkake import CandleHikkake
from .candle_hikkake_modified import CandleHikkakeModified
from .candle_hammer import CandleHammer
from .hilbert_transform_dominant_cycle_period import HilbertTransformDominantCyclePeriod
from .hilbert_transform_dominant_cycle_phase import HilbertTransformDominantCyclePhase
from .hilbert_transform_phasor import HilbertTransformPhasor
from .hilbert_transform_sine_wave import HilbertTransformSineWave
from .hilbert_transform_trend_mode import HilbertTransformTrendMode
from .intraday_momentum_index import IntradayMomentumIndex
from .moving_average import MovingAverage
from .minus_directional_indicator import MinusDirectionalIndicator
from .minus_directional_movement import MinusDirectionalMovement
from .money_flow_index import MoneyFlowIndex
from .average_price import AveragePrice
from .median_price import MedianPrice
from .typical_price import TypicalPrice
from .weighted_close import WeightedClose
from .moving_average_convergence_divergence_fixed import (
    MovingAverageConvergenceDivergenceFixed,
)
from .moving_average_convergence_divergence import (
    MovingAverageConvergenceDivergence,
)
from .moving_average_convergence_divergence_extended import (
    MovingAverageConvergenceDivergenceExtended,
)
from .parabolic_sar import ParabolicSar
from .parabolic_sar_extended import ParabolicSarExtended
from .plus_directional_indicator import PlusDirectionalIndicator
from .plus_directional_movement import PlusDirectionalMovement
from .stochastic_oscillator import StochasticOscillator
from .stochastic_relative_strength_index import StochasticRelativeStrengthIndex
from .candle_stick_sandwich import CandleStickSandwich
from .triple_exponential_rate_of_change import TripleExponentialRateOfChange
from .candle_three_black_crows import CandleThreeBlackCrows
from .candle_three_inside import CandleThreeInside
from .candle_three_line_strike import CandleThreeLineStrike
from .candle_three_stars_in_south import CandleThreeStarsInSouth
from .candle_three_outside import CandleThreeOutside
from .candle_two_crows import CandleTwoCrows
from .ultimate_oscillator import UltimateOscillator
from .variable_period_moving_average import VariablePeriodMovingAverage
from .heikin_ashi import HeikinAshi
from .fibonacci_retracement import FibonacciRetracement
from .anchored_volume_weighted_average_price import (
    AnchoredVolumeWeightedAveragePrice,
)
from .variable_index_dynamic_average import VariableIndexDynamicAverage
from .laguerre_relative_strength_index import LaguerreRelativeStrengthIndex
from .simple_moving_average import SimpleMovingAverage
from .normalized_average_true_range import NormalizedAverageTrueRange
from .true_range import TrueRange
from .triangular_moving_average import TriangularMovingAverage
from .triple_exponential_moving_average import TripleExponentialMovingAverage
from .triple_exponential_average import TripleExponentialAverage
from .weighted_moving_average import WeightedMovingAverage
from .relative_strength_index import RelativeStrengthIndex
from .relative_momentum_index import RelativeMomentumIndex
from .indicators import SmoothedTrendChannel
from .jurik_moving_average import JurikMovingAverage
from .indicators import ParabolicMovingAverageStop
from .indicators import TomDeMarkSequential
from .indicators import EvenBetterSinewave
from .indicators import PremiumDiscount
from .indicators import OpeningRange
from .indicators import PivotPoints
from . import executions
from . import op
from .math_abs import MathAbs
from .math_acos import MathAcos
from .math_acosh import MathAcosh
from .math_asin import MathAsin
from .math_asinh import MathAsinh
from .math_atan import MathAtan
from .math_atanh import MathAtanh
from .math_cbrt import MathCbrt
from .math_ceil import MathCeil
from .math_cos import MathCos
from .math_cosh import MathCosh
from .math_cot import MathCot
from .math_degrees import MathDegrees
from .math_exp import MathExp
from .math_floor import MathFloor
from .math_ln import MathLn
from .math_log10 import MathLog10
from .math_log1p import MathLog1p
from .math_radians import MathRadians
from .math_sin import MathSin
from .math_sinh import MathSinh
from .math_sqrt import MathSqrt
from .math_tan import MathTan
from .math_tanh import MathTanh
from .math_add import MathAdd
from .math_subtract import MathSubtract
from .math_multiply import MathMultiply
from .math_divide import MathDivide
from .rolling_standard_deviation import RollingStandardDeviation
from .rolling_average_deviation import RollingAverageDeviation
from .rolling_midpoint import RollingMidpoint
from .rolling_midprice import RollingMidprice
from .rolling_variance import RollingVariance
from .rolling_linear_regression import RollingLinearRegression
from .rolling_linear_regression_angle import RollingLinearRegressionAngle
from .rolling_linear_regression_intercept import RollingLinearRegressionIntercept
from .rolling_linear_regression_slope import RollingLinearRegressionSlope
from .rolling_time_series_forecast import RollingTimeSeriesForecast
from .rolling_beta import RollingBeta
from .rolling_correlation import RollingCorrelation
from .mesa_adaptive_moving_average import MesaAdaptiveMovingAverage
from .rolling_min_max import RollingMinMax
from .rolling_min_max_index import RollingMinMaxIndex
from .indicators import KlingerVolumeOscillator
from .indicators import SessionVolumeLevels
from .candle_up_down_side_gap_three_methods import CandleUpDownSideGapThreeMethods
from .candle_three_white_soldiers import CandleThreeWhiteSoldiers
from .candle_abandoned_baby import CandleAbandonedBaby
from .candle_advance_block import CandleAdvanceBlock
from .candle_breakaway import CandleBreakaway
from .candle_conceal_baby_swall import CandleConcealBabySwall
from .candle_counter_attack import CandleCounterAttack
from .candle_dark_cloud_cover import CandleDarkCloudCover
from .candle_evening_doji_star import CandleEveningDojiStar
from .candle_evening_star import CandleEveningStar
from .candle_gap_side_side_white import CandleGapSideSideWhite
from .candle_hanging_man import CandleHangingMan
from .candle_harami import CandleHarami
from .candle_harami_cross import CandleHaramiCross
from .candle_homing_pigeon import CandleHomingPigeon
from .candle_identical_three_crows import CandleIdenticalThreeCrows
from .candle_in_neck import CandleInNeck
from .candle_inverted_hammer import CandleInvertedHammer
from .candle_kicking import CandleKicking
from .candle_kicking_by_length import CandleKickingByLength
from .candle_ladder_bottom import CandleLadderBottom
from .candle_matching_low import CandleMatchingLow
from .candle_mat_hold import CandleMatHold
from .candle_morning_doji_star import CandleMorningDojiStar
from .candle_morning_star import CandleMorningStar
from .candle_on_neck import CandleOnNeck
from .candle_piercing import CandlePiercing
from .candle_rise_fall_three_methods import CandleRiseFallThreeMethods
from .candle_separating_lines import CandleSeparatingLines
from .candle_shooting_star import CandleShootingStar
from .candle_stalled_pattern import CandleStalledPattern
from .candle_tasuki_gap import CandleTasukiGap
from .candle_thrusting import CandleThrusting
from .candle_tri_star import CandleTriStar
from .candle_unique_three_river import CandleUniqueThreeRiver
from .candle_upside_gap_two_crows import CandleUpsideGapTwoCrows
from .lag import Lag
from .log_return import LogReturn
from .cumulative_sum import CumulativeSum
from .cumulative_product import CumulativeProduct
from .indicators import CumulativeCount
from .rolling_median import RollingMedian
from .rolling_min import RollingMin
from .rolling_max import RollingMax
from .rolling_sum import RollingSum
from .rolling_argmin import RollingArgmin
from .rolling_argmax import RollingArgmax
from .rolling_mode import RollingMode
from .rolling_quantile import RollingQuantile
from .rolling_percentile import RollingPercentile
from .rolling_rank import RollingRank
from .rolling_z_score import RollingZScore
from .rolling_skew import RollingSkew
from .rolling_kurtosis import RollingKurtosis
from .rolling_interquartile_range import RollingInterquartileRange
from .rolling_covariance import RollingCovariance
from .rolling_winsorize import RollingWinsorize
from .exponentially_weighted_variance import ExponentiallyWeightedVariance
from .exponentially_weighted_sum import ExponentiallyWeightedSum
from .exponentially_weighted_standard_deviation import ExponentiallyWeightedStandardDeviation
from .exponentially_weighted_covariance import ExponentiallyWeightedCovariance
from .exponentially_weighted_correlation import ExponentiallyWeightedCorrelation
from .cumulative_maximum import CumulativeMaximum
from .cumulative_minimum import CumulativeMinimum
from .drawdown import Drawdown
from .rolling_sharpe import RollingSharpe
from .rolling_sortino import RollingSortino
from .rolling_calmar import RollingCalmar
from .hull_moving_average import HullMovingAverage
from .volume_weighted_moving_average import VolumeWeightedMovingAverage
from .zero_lag_exponential_moving_average import ZeroLagExponentialMovingAverage
from .arnaud_legoux_moving_average import ArnaudLegouxMovingAverage
from .true_strength_index import TrueStrengthIndex
from .awesome_oscillator import AwesomeOscillator
from .fisher_transform import FisherTransform
from .donchian import Donchian
from .ulcer_index import UlcerIndex
from .keltner_channels import KeltnerChannels
from .chaikin_volatility import ChaikinVolatility
from .crossover import Crossover
from .crossunder import Crossunder
from .cross import Cross
from .rising import Rising
from .falling import Falling
from .rolling_volume_weighted_average_price import RollingVolumeWeightedAveragePrice
from .force_index import ForceIndex
from .ease_of_movement import EaseOfMovement
from .higher_high import HigherHigh
from .lower_low import LowerLow
from .inside_bar import InsideBar
from .outside_bar import OutsideBar
from .gap_up import GapUp
from .gap_down import GapDown
from .bars_since import BarsSince
from .value_when import ValueWhen
from .highest_since import HighestSince
from .lowest_since import LowestSince
from .signal_delay import SignalDelay
from .position_hold import PositionHold
from .entry_exit import EntryExit
from .swing_high_low import SwingHighLow
from .swing_high import SwingHigh
from .swing_low import SwingLow
from .retracements import Retracements
from .session_extrema import SessionExtrema
from .previous_high_low import PreviousHighLow
from .sessions import Sessions
from .active_zone_list import ActiveZoneList
from .fair_value_gap import FairValueGap
from .donchian_channels import DonchianChannels
from .swing_highs_lows import SwingHighsLows
from .break_of_structure_change_of_character import BreakOfStructureChangeOfCharacter
from .order_block import OrderBlock
from .liquidity import Liquidity
from .equal_highs_lows import EqualHighsLows
from .hedge_ratio import HedgeRatio
from .rolling_entropy import RollingEntropy
from .rolling_autocorr import RollingAutocorr
from .hurst import Hurst
from .fractal_dimension import FractalDimension
from .rolling_alpha import RollingAlpha
from .rolling_information_ratio import RollingInformationRatio
from .close_to_close_sigma import CloseToCloseSigma
from .parkinson import Parkinson
from .garman_klass import GarmanKlass
from .rogers_satchell import RogersSatchell
from .garman_klass_yang_zhang import GarmanKlassYangZhang
from .yang_zhang import YangZhang
from .average_daily_dollar_value import AverageDailyDollarValue
from .amihud import Amihud
from .roll_spread import RollSpread
from .ornstein_uhlenbeck_half_life import OrnsteinUhlenbeckHalfLife
from .cumulative_sum_control_chart import CumulativeSumControlChart
from .spread_z_score import SpreadZScore
from .frac_diff import FracDiff
from .kalman_hedge_ratio import KalmanHedgeRatio
from .supertrend import Supertrend
from .ichimoku import Ichimoku
from .squeeze import Squeeze
from .squeeze_pro import SqueezePro
from .schaff_trend_cycle import SchaffTrendCycle
from .vortex import Vortex
from .know_sure_thing import KnowSureThing
from .mass_index import MassIndex
from .detrended_price_oscillator import DetrendedPriceOscillator
from .chaikin_money_flow import ChaikinMoneyFlow
from .volume_price_trend import VolumePriceTrend
from .negative_volume_index import NegativeVolumeIndex
from .positive_volume_index import PositiveVolumeIndex
from .mc_ginley_dynamic import McGinleyDynamic
from .decay_linear import DecayLinear
from .signed_power import SignedPower
from .time_series_rank import TimeSeriesRank

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
    "RollingMin",
    "RollingMax",
    "RollingSum",
    "RollingArgmin",
    "RollingArgmax",
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
    "SwingHigh",
    "SwingLow",
    "Retracements",
    "SessionExtrema",
    "PreviousHighLow",
    "Sessions",
    "ActiveZoneList",
    "FairValueGap",
    "DonchianChannels",
    "SwingHighsLows",
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
