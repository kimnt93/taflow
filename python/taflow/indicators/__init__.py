"""Canonical namespace for persistent TAFlow indicators."""

from ..commodity_channel_index import CCI, CommodityChannelIndex
from ..doji import CDLDOJI, Doji
from ..takuri import CDLTAKURI, Takuri
from ..marubozu import CDLMARUBOZU, Marubozu
from ..closing_marubozu import CDLCLOSINGMARUBOZU, ClosingMarubozu
from ..long_legged_doji import CDLLONGLEGGEDDOJI, LongLeggedDoji
from ..rickshawman import CDLRICKSHAWMAN, Rickshawman
from ..high_wave import CDLHIGHWAVE, HighWave
from ..dragonfly_doji import CDLDRAGONFLYDOJI, DragonflyDoji
from ..gravestone_doji import CDLGRAVESTONEDOJI, GravestoneDoji
from ..short_line import CDLSHORTLINE, ShortLine
from ..spinning_top import CDLSPINNINGTOP, SpinningTop
from ..long_line import CDLLONGLINE, LongLine
from ..doji_star import CDLDOJISTAR, DojiStar
from ..belt_hold import CDLBELTHOLD, BeltHold
from ..engulfing import CDLENGULFING, Engulfing
from ..hammer import CDLHAMMER, Hammer
from ..hikkake import CDLHIKKAKE, Hikkake
from ..hikkake_modified import CDLHIKKAKEMOD, HikkakeModified
from ..exponential_moving_average import EMA, ExponentialMovingAverage
from ..hilbert_transform_dominant_cycle_period import HT_DCPERIOD, HilbertTransformDominantCyclePeriod
from ..hilbert_transform_dominant_cycle_phase import HT_DCPHASE, HilbertTransformDominantCyclePhase
from ..hilbert_transform_phasor import HT_PHASOR, HilbertTransformPhasor
from ..hilbert_transform_sine_wave import HT_SINE, HilbertTransformSineWave
from ..hilbert_transform_trend_mode import HT_TRENDMODE, HilbertTransformTrendMode
from ..money_flow_index import MFI, MoneyFlowIndex
from ..minus_directional_indicator import MINUS_DI, MinusDirectionalIndicator
from ..minus_directional_movement import MINUS_DM, MinusDirectionalMovement
from ..plus_directional_indicator import PLUS_DI, PlusDirectionalIndicator
from ..plus_directional_movement import PLUS_DM, PlusDirectionalMovement
from ..triple_exponential_rate_of_change import TRIX, TripleExponentialRateOfChange
from ..stick_sandwich import CDLSTICKSANDWICH, StickSandwich
from ..three_black_crows import CDL3BLACKCROWS, ThreeBlackCrows
from ..three_inside import CDL3INSIDE, ThreeInside
from ..three_line_strike import CDL3LINESTRIKE, ThreeLineStrike
from ..three_stars_in_south import CDL3STARSINSOUTH, ThreeStarsInSouth
from ..three_outside import CDL3OUTSIDE, ThreeOutside
from ..two_crows import CDL2CROWS, TwoCrows
from ..ultimate_oscillator import ULTOSC, UltimateOscillator
from ..up_down_side_gap_three_methods import CDLXSIDEGAP3METHODS, UpDownSideGapThreeMethods
from ..three_white_soldiers import CDL3WHITESOLDIERS, ThreeWhiteSoldiers
from ..abandoned_baby import CDLABANDONEDBABY, AbandonedBaby
from ..advance_block import CDLADVANCEBLOCK, AdvanceBlock
from ..breakaway import CDLBREAKAWAY, Breakaway
from ..conceal_baby_swall import CDLCONCEALBABYSWALL, ConcealBabySwall
from ..counter_attack import CDLCOUNTERATTACK, CounterAttack
from ..dark_cloud_cover import CDLDARKCLOUDCOVER, DarkCloudCover
from ..evening_doji_star import CDLEVENINGDOJISTAR, EveningDojiStar
from ..evening_star import CDLEVENINGSTAR, EveningStar
from ..gap_side_side_white import CDLGAPSIDESIDEWHITE, GapSideSideWhite
from ..hanging_man import CDLHANGINGMAN, HangingMan
from ..harami import CDLHARAMI, Harami
from ..harami_cross import CDLHARAMICROSS, HaramiCross
from ..homing_pigeon import CDLHOMINGPIGEON, HomingPigeon
from ..identical_three_crows import CDLIDENTICAL3CROWS, IdenticalThreeCrows
from ..in_neck import CDLINNECK, InNeck
from ..inverted_hammer import CDLINVERTEDHAMMER, InvertedHammer
from ..kicking import CDLKICKING, Kicking
from ..kicking_by_length import CDLKICKINGBYLENGTH, KickingByLength
from ..ladder_bottom import CDLLADDERBOTTOM, LadderBottom
from ..matching_low import CDLMATCHINGLOW, MatchingLow
from ..mat_hold import CDLMATHOLD, MatHold
from ..morning_doji_star import CDLMORNINGDOJISTAR, MorningDojiStar
from ..morning_star import CDLMORNINGSTAR, MorningStar
from ..on_neck import CDLONNECK, OnNeck
from ..piercing import CDLPIERCING, Piercing
from ..rise_fall_three_methods import CDLRISEFALL3METHODS, RiseFallThreeMethods
from ..separating_lines import CDLSEPARATINGLINES, SeparatingLines
from ..shooting_star import CDLSHOOTINGSTAR, ShootingStar
from ..stalled_pattern import CDLSTALLEDPATTERN, StalledPattern
from ..tasuki_gap import CDLTASUKIGAP, TasukiGap
from ..thrusting import CDLTHRUSTING, Thrusting
from ..tri_star import CDLTRISTAR, TriStar
from ..unique_three_river import CDLUNIQUE3RIVER, UniqueThreeRiver
from ..upside_gap_two_crows import CDLUPSIDEGAP2CROWS, UpsideGapTwoCrows
from ..lag import Lag
from ..log_return import LogReturn
from ..cumsum import Cumsum
from ..cumprod import Cumprod

__all__ = [
    "CommodityChannelIndex", "CCI", "ExponentialMovingAverage", "EMA",
    "Doji", "CDLDOJI",
    "Takuri", "CDLTAKURI",
    "Marubozu", "CDLMARUBOZU",
    "ClosingMarubozu", "CDLCLOSINGMARUBOZU",
    "LongLeggedDoji", "CDLLONGLEGGEDDOJI",
    "Rickshawman", "CDLRICKSHAWMAN",
    "HighWave", "CDLHIGHWAVE",
    "DragonflyDoji", "CDLDRAGONFLYDOJI",
    "GravestoneDoji", "CDLGRAVESTONEDOJI",
    "ShortLine", "CDLSHORTLINE",
    "SpinningTop", "CDLSPINNINGTOP",
    "LongLine", "CDLLONGLINE",
    "DojiStar", "CDLDOJISTAR",
    "BeltHold", "CDLBELTHOLD",
    "Engulfing", "CDLENGULFING",
    "Hammer", "CDLHAMMER",
    "Hikkake", "CDLHIKKAKE",
    "HikkakeModified", "CDLHIKKAKEMOD",
    "HilbertTransformDominantCyclePeriod", "HT_DCPERIOD",
    "HilbertTransformDominantCyclePhase", "HT_DCPHASE",
    "HilbertTransformPhasor", "HT_PHASOR",
    "HilbertTransformSineWave", "HT_SINE",
    "HilbertTransformTrendMode", "HT_TRENDMODE",
    "MoneyFlowIndex", "MFI", "MinusDirectionalIndicator", "MINUS_DI",
    "MinusDirectionalMovement", "MINUS_DM", "PlusDirectionalIndicator", "PLUS_DI",
    "PlusDirectionalMovement", "PLUS_DM", "TripleExponentialRateOfChange", "TRIX",
    "UltimateOscillator", "ULTOSC",
    "UpDownSideGapThreeMethods", "CDLXSIDEGAP3METHODS",
    "ThreeBlackCrows", "CDL3BLACKCROWS",
    "ThreeInside", "CDL3INSIDE",
    "ThreeLineStrike", "CDL3LINESTRIKE",
    "ThreeStarsInSouth", "CDL3STARSINSOUTH",
    "ThreeOutside", "CDL3OUTSIDE",
    "StickSandwich", "CDLSTICKSANDWICH",
    "TwoCrows", "CDL2CROWS",
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
    "Lag", "LogReturn", "Cumsum", "Cumprod",
]
