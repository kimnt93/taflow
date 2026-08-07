"""Descriptive, stateful Python interface for TAFlow."""

__version__ = "0.1.2"

from . import talib
from .acceleration_bands import AccelerationBands
from .average_directional_index import AverageDirectionalIndex
from .average_directional_index_rating import AverageDirectionalIndexRating
from .bollinger_bands import BollingerBands
from .commodity_channel_index import CCI, CommodityChannelIndex
from .directional_movement_index import DirectionalMovementIndex
from .doji import CDLDOJI, Doji
from .takuri import CDLTAKURI, Takuri
from .marubozu import CDLMARUBOZU, Marubozu
from .closing_marubozu import CDLCLOSINGMARUBOZU, ClosingMarubozu
from .long_legged_doji import CDLLONGLEGGEDDOJI, LongLeggedDoji
from .rickshawman import CDLRICKSHAWMAN, Rickshawman
from .high_wave import CDLHIGHWAVE, HighWave
from .dragonfly_doji import CDLDRAGONFLYDOJI, DragonflyDoji
from .gravestone_doji import CDLGRAVESTONEDOJI, GravestoneDoji
from .short_line import CDLSHORTLINE, ShortLine
from .spinning_top import CDLSPINNINGTOP, SpinningTop
from .long_line import CDLLONGLINE, LongLine
from .doji_star import CDLDOJISTAR, DojiStar
from .belt_hold import CDLBELTHOLD, BeltHold
from .engulfing import CDLENGULFING, Engulfing
from .exponential_moving_average import EMA, ExponentialMovingAverage
from .fast_stochastic_oscillator import FastStochasticOscillator
from .hilbert_transform_trendline import HilbertTransformTrendline
from .hikkake import CDLHIKKAKE, Hikkake
from .hikkake_modified import CDLHIKKAKEMOD, HikkakeModified
from .hammer import CDLHAMMER, Hammer
from .hilbert_transform_dominant_cycle_period import HT_DCPERIOD, HilbertTransformDominantCyclePeriod
from .hilbert_transform_dominant_cycle_phase import HT_DCPHASE, HilbertTransformDominantCyclePhase
from .hilbert_transform_phasor import HT_PHASOR, HilbertTransformPhasor
from .hilbert_transform_sine_wave import HT_SINE, HilbertTransformSineWave
from .hilbert_transform_trend_mode import HT_TRENDMODE, HilbertTransformTrendMode
from .intraday_momentum_index import IntradayMomentumIndex
from .moving_average import MovingAverage
from .minus_directional_indicator import MINUS_DI, MinusDirectionalIndicator
from .minus_directional_movement import MINUS_DM, MinusDirectionalMovement
from .money_flow_index import MFI, MoneyFlowIndex
from .moving_average_convergence_divergence_fixed import (
    MovingAverageConvergenceDivergenceFixed,
)
from .moving_average_convergence_divergence_extended import (
    MovingAverageConvergenceDivergenceExtended,
)
from .parabolic_sar import ParabolicSar
from .parabolic_sar_extended import ParabolicSarExtended
from .plus_directional_indicator import PLUS_DI, PlusDirectionalIndicator
from .plus_directional_movement import PLUS_DM, PlusDirectionalMovement
from .stochastic_oscillator import StochasticOscillator
from .stochastic_relative_strength_index import StochasticRelativeStrengthIndex
from .stick_sandwich import CDLSTICKSANDWICH, StickSandwich
from .triple_exponential_rate_of_change import TRIX, TripleExponentialRateOfChange
from .three_black_crows import CDL3BLACKCROWS, ThreeBlackCrows
from .three_inside import CDL3INSIDE, ThreeInside
from .three_line_strike import CDL3LINESTRIKE, ThreeLineStrike
from .three_stars_in_south import CDL3STARSINSOUTH, ThreeStarsInSouth
from .three_outside import CDL3OUTSIDE, ThreeOutside
from .two_crows import CDL2CROWS, TwoCrows
from .ultimate_oscillator import ULTOSC, UltimateOscillator
from .variable_period_moving_average import VariablePeriodMovingAverage
from .up_down_side_gap_three_methods import CDLXSIDEGAP3METHODS, UpDownSideGapThreeMethods
from .three_white_soldiers import CDL3WHITESOLDIERS, ThreeWhiteSoldiers
from .abandoned_baby import CDLABANDONEDBABY, AbandonedBaby
from .advance_block import CDLADVANCEBLOCK, AdvanceBlock
from .breakaway import CDLBREAKAWAY, Breakaway
from .conceal_baby_swall import CDLCONCEALBABYSWALL, ConcealBabySwall
from .counter_attack import CDLCOUNTERATTACK, CounterAttack
from .dark_cloud_cover import CDLDARKCLOUDCOVER, DarkCloudCover
from .evening_doji_star import CDLEVENINGDOJISTAR, EveningDojiStar
from .evening_star import CDLEVENINGSTAR, EveningStar
from .gap_side_side_white import CDLGAPSIDESIDEWHITE, GapSideSideWhite
from .hanging_man import CDLHANGINGMAN, HangingMan
from .harami import CDLHARAMI, Harami
from .harami_cross import CDLHARAMICROSS, HaramiCross
from .homing_pigeon import CDLHOMINGPIGEON, HomingPigeon
from .identical_three_crows import CDLIDENTICAL3CROWS, IdenticalThreeCrows
from .in_neck import CDLINNECK, InNeck
from .inverted_hammer import CDLINVERTEDHAMMER, InvertedHammer
from .kicking import CDLKICKING, Kicking
from .kicking_by_length import CDLKICKINGBYLENGTH, KickingByLength
from .ladder_bottom import CDLLADDERBOTTOM, LadderBottom
from .matching_low import CDLMATCHINGLOW, MatchingLow
from .mat_hold import CDLMATHOLD, MatHold
from .morning_doji_star import CDLMORNINGDOJISTAR, MorningDojiStar
from .morning_star import CDLMORNINGSTAR, MorningStar
from .on_neck import CDLONNECK, OnNeck
from .piercing import CDLPIERCING, Piercing
from .rise_fall_three_methods import CDLRISEFALL3METHODS, RiseFallThreeMethods
from .separating_lines import CDLSEPARATINGLINES, SeparatingLines
from .shooting_star import CDLSHOOTINGSTAR, ShootingStar
from .stalled_pattern import CDLSTALLEDPATTERN, StalledPattern
from .tasuki_gap import CDLTASUKIGAP, TasukiGap
from .thrusting import CDLTHRUSTING, Thrusting
from .tri_star import CDLTRISTAR, TriStar
from .unique_three_river import CDLUNIQUE3RIVER, UniqueThreeRiver
from .upside_gap_two_crows import CDLUPSIDEGAP2CROWS, UpsideGapTwoCrows
from .lag import Lag
from .log_return import LogReturn
from .cumsum import Cumsum
from .cumprod import Cumprod
from .rolling_median import RollingMedian
from .rolling_mode import RollingMode
from .rolling_quantile import RollingQuantile
from .rolling_percentile import RollingPercentile
from .rolling_rank import RollingRank
from .rolling_zscore import RollingZscore
from .rolling_skew import RollingSkew
from .rolling_kurtosis import RollingKurtosis
from .rolling_iqr import RollingIqr
from .rolling_cov import RollingCov
from .rolling_winsorize import RollingWinsorize
from .rolling_apply import rolling_apply
from .ewm_var import EwmVar
from .ewm_std import EwmStd
from .ewm_cov import EwmCov
from .ewm_corr import EwmCorr
from .cummax import Cummax
from .cummin import Cummin
from .drawdown import Drawdown
from .rolling_sharpe import RollingSharpe
from .rolling_sortino import RollingSortino
from .rolling_calmar import RollingCalmar
from .hma import Hma
from .vwma import Vwma
from .zlema import Zlema
from .alma import Alma
from .tsi import Tsi
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
from .vwap import Vwap
from .force_index import ForceIndex
from .ease_of_movement import EaseOfMovement
from .bar_helpers import HigherHigh, LowerLow, InsideBar, OutsideBar, GapUp, GapDown
from .state_helpers import BarsSince, ValueWhen, HighestSince, LowestSince, SignalDelay, PositionHold, EntryExit
from .swing import SwingHighLow, SwingHigh, SwingLow
from .retracements import Retracements
from .session import SessionExtrema, session_flags
from .previous_high_low import PreviousHighLow
from .sessions import Sessions
from .zones import ActiveZoneList
from .fvg import Fvg
from .donchian_channels import DonchianChannels
from .rolling_vwap import RollingVwap
from .swing_highs_lows import SwingHighsLows
from .bos_choch import BosChoch
from .ob import Ob
from .liquidity import Liquidity
from .equal_highs_lows import EqualHighsLows
from .hedge_ratio import HedgeRatio
from .rolling_entropy import RollingEntropy
from .rolling_autocorr import RollingAutocorr
from .hurst import Hurst
from .fractal_dimension import FractalDimension
from .rolling_alpha import RollingAlpha, RollingInformationRatio
from .close_to_close_sigma import CloseToCloseSigma
from .parkinson import Parkinson
from .garman_klass import GarmanKlass
from .rogers_satchell import RogersSatchell
from .gk_yang_zhang import GkYangZhang
from .yang_zhang import YangZhang
from .adv import Adv
from .amihud import Amihud
from .roll_spread import RollSpread
from .ou_half_life import OuHalfLife
from .cusum import Cusum
from .spread_zscore import SpreadZscore
from .frac_diff import FracDiff
from .kalman_hedge_ratio import KalmanHedgeRatio

__all__ = [
    "talib",
    "MovingAverage",
    "MinusDirectionalIndicator",
    "MINUS_DI",
    "MinusDirectionalMovement",
    "MINUS_DM",
    "MoneyFlowIndex",
    "MFI",
    "MovingAverageConvergenceDivergenceFixed",
    "MovingAverageConvergenceDivergenceExtended",
    "BollingerBands",
    "CommodityChannelIndex",
    "CCI",
    "FastStochasticOscillator",
    "HilbertTransformTrendline",
    "Hikkake",
    "CDLHIKKAKE",
    "HikkakeModified",
    "CDLHIKKAKEMOD",
    "Hammer",
    "CDLHAMMER",
    "HilbertTransformDominantCyclePeriod",
    "HT_DCPERIOD",
    "HilbertTransformDominantCyclePhase",
    "HT_DCPHASE",
    "HilbertTransformPhasor",
    "HT_PHASOR",
    "HilbertTransformSineWave",
    "HT_SINE",
    "HilbertTransformTrendMode",
    "HT_TRENDMODE",
    "StochasticOscillator",
    "StochasticRelativeStrengthIndex",
    "StickSandwich",
    "CDLSTICKSANDWICH",
    "TripleExponentialRateOfChange",
    "TRIX",
    "ThreeBlackCrows",
    "CDL3BLACKCROWS",
    "ThreeInside",
    "CDL3INSIDE",
    "ThreeLineStrike",
    "CDL3LINESTRIKE",
    "ThreeStarsInSouth",
    "CDL3STARSINSOUTH",
    "ThreeOutside",
    "CDL3OUTSIDE",
    "TwoCrows",
    "CDL2CROWS",
    "UltimateOscillator",
    "ULTOSC",
    "VariablePeriodMovingAverage",
    "UpDownSideGapThreeMethods",
    "CDLXSIDEGAP3METHODS",
    "IntradayMomentumIndex",
    "AccelerationBands",
    "AverageDirectionalIndex",
    "AverageDirectionalIndexRating",
    "DirectionalMovementIndex",
    "Doji",
    "Takuri",
    "CDLTAKURI",
    "Marubozu",
    "CDLMARUBOZU",
    "ClosingMarubozu",
    "CDLCLOSINGMARUBOZU",
    "LongLeggedDoji",
    "CDLLONGLEGGEDDOJI",
    "Rickshawman",
    "CDLRICKSHAWMAN",
    "HighWave",
    "CDLHIGHWAVE",
    "DragonflyDoji",
    "CDLDRAGONFLYDOJI",
    "GravestoneDoji",
    "CDLGRAVESTONEDOJI",
    "ShortLine", "CDLSHORTLINE",
    "SpinningTop", "CDLSPINNINGTOP",
    "LongLine", "CDLLONGLINE",
    "DojiStar", "CDLDOJISTAR",
    "BeltHold", "CDLBELTHOLD",
    "CDLDOJI",
    "Engulfing",
    "CDLENGULFING",
    "ExponentialMovingAverage",
    "EMA",
    "ParabolicSar",
    "ParabolicSarExtended",
    "PlusDirectionalIndicator",
    "PLUS_DI",
    "PlusDirectionalMovement",
    "PLUS_DM",
    "ThreeWhiteSoldiers", "CDL3WHITESOLDIERS",
    "AbandonedBaby", "CDLABANDONEDBABY",
    "AdvanceBlock", "CDLADVANCEBLOCK",
    "Breakaway", "CDLBREAKAWAY",
    "ConcealBabySwall", "CDLCONCEALBABYSWALL",
    "CounterAttack", "CDLCOUNTERATTACK",
    "DarkCloudCover", "CDLDARKCLOUDCOVER",
    "EveningDojiStar", "CDLEVENINGDOJISTAR",
    "EveningStar", "CDLEVENINGSTAR",
    "GapSideSideWhite", "CDLGAPSIDESIDEWHITE",
    "HangingMan", "CDLHANGINGMAN",
    "Harami", "CDLHARAMI",
    "HaramiCross", "CDLHARAMICROSS",
    "HomingPigeon", "CDLHOMINGPIGEON",
    "IdenticalThreeCrows", "CDLIDENTICAL3CROWS",
    "InNeck", "CDLINNECK",
    "InvertedHammer", "CDLINVERTEDHAMMER",
    "Kicking", "CDLKICKING",
    "KickingByLength", "CDLKICKINGBYLENGTH",
    "LadderBottom", "CDLLADDERBOTTOM",
    "MatchingLow", "CDLMATCHINGLOW",
    "MatHold", "CDLMATHOLD",
    "MorningDojiStar", "CDLMORNINGDOJISTAR",
    "MorningStar", "CDLMORNINGSTAR",
    "OnNeck", "CDLONNECK",
    "Piercing", "CDLPIERCING",
    "RiseFallThreeMethods", "CDLRISEFALL3METHODS",
    "SeparatingLines", "CDLSEPARATINGLINES",
    "ShootingStar", "CDLSHOOTINGSTAR",
    "StalledPattern", "CDLSTALLEDPATTERN",
    "TasukiGap", "CDLTASUKIGAP",
    "Thrusting", "CDLTHRUSTING",
    "TriStar", "CDLTRISTAR",
    "UniqueThreeRiver", "CDLUNIQUE3RIVER",
    "UpsideGapTwoCrows", "CDLUPSIDEGAP2CROWS",
    "Lag", "LogReturn", "Cumsum", "Cumprod", "RollingMedian", "RollingMode",
    "RollingQuantile", "RollingPercentile", "RollingRank", "RollingZscore",
    "RollingSkew", "RollingKurtosis", "RollingIqr",
    "RollingCov", "RollingWinsorize",
    "rolling_apply",
    "EwmVar", "EwmStd", "EwmCov", "EwmCorr",
    "Cummax", "Cummin", "Drawdown",
    "RollingSharpe", "RollingSortino", "RollingCalmar",
    "Hma", "Vwma", "Zlema", "Alma",
    "Tsi", "AwesomeOscillator", "FisherTransform",
    "Donchian", "UlcerIndex",
    "KeltnerChannels", "ChaikinVolatility",
    "Crossover", "Crossunder", "Cross", "Rising", "Falling",
    "Vwap", "ForceIndex", "EaseOfMovement",
    "HigherHigh", "LowerLow", "InsideBar", "OutsideBar", "GapUp", "GapDown",
    "BarsSince", "ValueWhen", "HighestSince", "LowestSince",
    "SignalDelay",
    "PositionHold",
    "EntryExit",
    "SwingHighLow", "SwingHigh", "SwingLow",
    "Retracements",
    "SessionExtrema", "session_flags",
    "PreviousHighLow", "Sessions",
    "ActiveZoneList",
    "Fvg",
    "DonchianChannels", "RollingVwap", "SwingHighsLows",
    "BosChoch",
    "Ob",
    "Liquidity",
    "EqualHighsLows",
    "HedgeRatio",
    "RollingEntropy", "RollingAutocorr",
    "Hurst", "FractalDimension",
    "RollingAlpha", "RollingInformationRatio",
    "CloseToCloseSigma", "Parkinson", "GarmanKlass", "RogersSatchell", "GkYangZhang", "YangZhang",
    "Adv", "Amihud", "RollSpread", "OuHalfLife", "Cusum",
    "SpreadZscore",
    "FracDiff",
    "KalmanHedgeRatio",
    "__version__",
]
