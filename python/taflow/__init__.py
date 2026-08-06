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
from .engulfing import CDLENGULFING, Engulfing
from .exponential_moving_average import EMA, ExponentialMovingAverage
from .fast_stochastic_oscillator import FastStochasticOscillator
from .hilbert_transform_trendline import HilbertTransformTrendline
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
from .triple_exponential_rate_of_change import TRIX, TripleExponentialRateOfChange
from .ultimate_oscillator import ULTOSC, UltimateOscillator
from .variable_period_moving_average import VariablePeriodMovingAverage

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
    "TripleExponentialRateOfChange",
    "TRIX",
    "UltimateOscillator",
    "ULTOSC",
    "VariablePeriodMovingAverage",
    "IntradayMomentumIndex",
    "AccelerationBands",
    "AverageDirectionalIndex",
    "AverageDirectionalIndexRating",
    "DirectionalMovementIndex",
    "Doji",
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
    "__version__",
]
