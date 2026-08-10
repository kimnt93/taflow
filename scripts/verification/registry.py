"""Shared registry: map TA-Lib names to canonical taflow classes.

taflow no longer ships a TA-Lib compatibility module, so verification and
benchmarking drive the canonical CamelCase classes and translate:

  - names   — parsed from the master table in /CHECK.md (the maintained
              contract), resolved against the live module with synonym
              expansion (e.g. ``rolling_std`` -> ``RollingStandardDeviation``);
  - inputs  — TA-Lib input roles (from ``talib.abstract``) matched to the
              class ``extend`` signature positionally;
  - params  — TA-Lib parameter names mapped to constructor keywords via
              exact, synonym, or underscore-normalized matching.

Rows that cannot be resolved are reported (class missing / param mismatch),
never silently skipped — surfacing drift between CHECK.md and the code is a
feature.
"""

from __future__ import annotations

import inspect
import re
from dataclasses import dataclass, field
from pathlib import Path

import numpy as np

ROOT = Path(__file__).resolve().parents[2]
CHECK_MD = ROOT / "CHECK.md"
VERIFY_DIR = ROOT / "verify"
CORRECTNESS_EVIDENCE_DIR = VERIFY_DIR / "evidence" / "correctness"
BENCHMARK_EVIDENCE_DIR = VERIFY_DIR / "evidence" / "benchmark"


@dataclass(frozen=True)
class WickraBinding:
    """Describe one explicit canonical-class to Wickra oracle mapping.

    ``parameter_names`` are read from the TAFlow constructor and passed to the
    Wickra constructor in order. ``prepend_zero_close`` documents Wickra's
    two-input ``VolumeRsi`` batch API; TAFlow intentionally exposes only the
    volume series because Wickra does not use the close argument.
    """

    name: str
    parameter_names: tuple[str, ...]
    prepend_zero_close: bool = False
    cross_section: str | None = None
    input_mode: str | None = None
    actual_indices: tuple[int, ...] | None = None
    oracle_indices: tuple[int, ...] | None = None
    variant: str | None = None
    rtol: float = 1e-8
    atol: float = 1e-10


@dataclass(frozen=True)
class ExternalBinding:
    """Describe a non-TA-Lib batch oracle selected by explicit override."""

    source: str
    name: str
    variant: str | None = None


@dataclass(frozen=True)
class PandasTaBinding:
    """Describe an exact pandas-ta-classic callable and output projection."""

    name: str
    actual_indices: tuple[int, ...] | None = None
    oracle_indices: tuple[int, ...] | None = None
    extra_kwargs: tuple[tuple[str, object], ...] = ()
    variant: str | None = None
    rtol: float = 1e-8
    atol: float = 1e-10


WICKRA_BINDINGS: dict[str, WickraBinding] = {
    "RollingMedian": WickraBinding("MedianMA", ("timeperiod",)),
    "RollingZScore": WickraBinding("ZScore", ("timeperiod",)),
    "RollingSkew": WickraBinding(
        "Skewness", ("timeperiod",), atol=1e-6
    ),
    "RollingKurtosis": WickraBinding(
        "Kurtosis", ("timeperiod",), atol=3e-4
    ),
    "RollingInterquartileRange": WickraBinding(
        "RollingIqr", ("timeperiod",)
    ),
    "RollingSharpe": WickraBinding("SharpeRatio", ("timeperiod",)),
    "RollingSortino": WickraBinding("SortinoRatio", ("timeperiod",)),
    "HullMovingAverage": WickraBinding("HMA", ("timeperiod",)),
    "VolumeWeightedMovingAverage": WickraBinding("VWMA", ("timeperiod",)),
    "ZeroLagExponentialMovingAverage": WickraBinding(
        "ZLEMA", ("timeperiod",)
    ),
    "ArnaudLegouxMovingAverage": WickraBinding(
        "ALMA", ("timeperiod", "offset", "sigma")
    ),
    "TrueStrengthIndex": WickraBinding("TSI", ("slow", "fast")),
    "KeltnerChannels": WickraBinding(
        "Keltner", ("timeperiod", "timeperiod", "multiplier")
    ),
    "RollingAutocorr": WickraBinding(
        "Autocorrelation", ("timeperiod",)
    ),
    "Hurst": WickraBinding("HurstExponent", ("timeperiod", "chunks")),
    "RollingAlpha": WickraBinding("Alpha", ("timeperiod",), atol=3e-8),
    "RollingInformationRatio": WickraBinding(
        "InformationRatio", ("timeperiod",)
    ),
    "KnowSureThing": WickraBinding(
        "KST",
        ("roc1", "roc2", "roc3", "roc4", "sma1", "sma2", "sma3", "sma4", "signal"),
    ),
    "NegativeVolumeIndex": WickraBinding("NVI", ()),
    "PositiveVolumeIndex": WickraBinding("PVI", ()),
    "Parkinson": WickraBinding("ParkinsonVolatility", ("timeperiod",)),
    "GarmanKlass": WickraBinding("GarmanKlassVolatility", ("timeperiod",)),
    "RogersSatchell": WickraBinding(
        "RogersSatchellVolatility", ("timeperiod",)
    ),
    "YangZhang": WickraBinding("YangZhangVolatility", ("timeperiod",)),
    "Amihud": WickraBinding(
        "AmihudIlliquidity", ("timeperiod",), input_mode="trade_pair"
    ),
    "TomDeMarkSequential": WickraBinding(
        "TDSequential", (), input_mode="triple_close"
    ),
    "LogReturn": WickraBinding("LogReturn", ("timeperiod",)),
    "RollingQuantile": WickraBinding(
        "RollingQuantile", ("timeperiod", "quantile")
    ),
    "RollingCovariance": WickraBinding("RollingCovariance", ("timeperiod",)),
    "AwesomeOscillator": WickraBinding("AwesomeOscillator", ("fast", "slow")),
    "Donchian": WickraBinding(
        "Donchian", ("timeperiod",), oracle_indices=(0, 2, 1)
    ),
    "ChaikinVolatility": WickraBinding(
        "ChaikinVolatility", ("timeperiod", "roc_period")
    ),
    "UlcerIndex": WickraBinding("UlcerIndex", ("timeperiod",)),
    "RollingVolumeWeightedAveragePrice": WickraBinding(
        "RollingVWAP", ("timeperiod",)
    ),
    "ForceIndex": WickraBinding("ForceIndex", ()),
    "EaseOfMovement": WickraBinding("EaseOfMovement", ()),
    "Vortex": WickraBinding("Vortex", ("window",)),
    "MassIndex": WickraBinding("MassIndex", ("ema_period", "sum_period")),
    "ChaikinMoneyFlow": WickraBinding("ChaikinMoneyFlow", ("period",)),
    "KlingerVolumeOscillator": WickraBinding(
        "KVO", ("fast", "slow"), actual_indices=(0,)
    ),
    "VolumePriceTrend": WickraBinding("VolumePriceTrend", ()),
    "McGinleyDynamic": WickraBinding("McGinleyDynamic", ("length",)),
    "VariableIndexDynamicAverage": WickraBinding(
        "VIDYA", ("length", "cmo_period")
    ),
    "LaguerreRelativeStrengthIndex": WickraBinding("LaguerreRSI", ("gamma",)),
    "HeikinAshi": WickraBinding("HeikinAshi", ()),
    "KalmanHedgeRatio": WickraBinding(
        "KalmanHedgeRatio",
        ("delta", "observation_variance"),
        input_mode="swap_pair",
        oracle_indices=(0,),
    ),
    "RelativeMomentumIndex": WickraBinding("RMI", ("timeperiod", "momentum")),
    "RollingMaximumDrawdown": WickraBinding("MaxDrawdown", ("timeperiod",)),
    "RollingOmegaRatio": WickraBinding("OmegaRatio", ("timeperiod", "threshold")),
    "RollingValueAtRisk": WickraBinding("ValueAtRisk", ("timeperiod", "confidence")),
    "RollingConditionalValueAtRisk": WickraBinding(
        "ConditionalValueAtRisk", ("timeperiod", "confidence")
    ),
    "RollingProfitFactor": WickraBinding("ProfitFactor", ("timeperiod",)),
    "RollingKellyCriterion": WickraBinding("KellyCriterion", ("timeperiod",)),
    "RollingTreynorRatio": WickraBinding("TreynorRatio", ("timeperiod",)),
    "VolumeOscillator": WickraBinding("VolumeOscillator", ("fast", "slow")),
    "VolumeZoneOscillator": WickraBinding("VZO", ("timeperiod",)),
    "DemandIndex": WickraBinding("DemandIndex", ("timeperiod",)),
    "VolumeRelativeStrengthIndex": WickraBinding(
        "VolumeRsi", ("period",), prepend_zero_close=True
    ),
    "RollingAverageDrawdown": WickraBinding("AverageDrawdown", ()),
    "RollingDrawdownDuration": WickraBinding("DrawdownDuration", ()),
    "RollingGainLossRatio": WickraBinding("GainLossRatio", ()),
    "RollingPainIndex": WickraBinding("PainIndex", ()),
    "RollingVarianceRatio": WickraBinding("VarianceRatio", ()),
    "RollingSpearmanCorrelation": WickraBinding("SpearmanCorrelation", ()),
    "RollingKendallRankCorrelation": WickraBinding("KendallTau", ()),
    "RollingCointegration": WickraBinding("Cointegration", ()),
    "RollingGrangerCausality": WickraBinding("GrangerCausality", ()),
    "RollingLeadLagCrossCorrelation": WickraBinding(
        "LeadLagCrossCorrelation", ()
    ),
    "RollingPairwiseBeta": WickraBinding("PairwiseBeta", ()),
    "RollingBetaNeutralSpread": WickraBinding("BetaNeutralSpread", ()),
    "RollingMedianAbsoluteDeviation": WickraBinding(
        "MedianAbsoluteDeviation", ()
    ),
    # Both implementations use the same O(1) regression recurrence. Near a
    # perfectly linear series, cancellation leaves sub-nanounit RSS noise.
    "RollingStandardError": WickraBinding(
        "StandardError", (), atol=1e-9
    ),
    "VolumeWeightedMovingAverageConvergenceDivergence": WickraBinding(
        "VolumeWeightedMacd", ()
    ),
    "BetterVolume": WickraBinding("BetterVolume", ()),
    "IntradayIntensity": WickraBinding("IntradayIntensity", ()),
    "TradeVolumeIndex": WickraBinding("TradeVolumeIndex", ()),
    "TwiggsMoneyFlow": WickraBinding("TwiggsMoneyFlow", ()),
    "WilliamsAccumulationDistribution": WickraBinding("Wad", ()),
    "MarketFacilitationIndex": WickraBinding("MarketFacilitationIndex", ()),
    "TimeSegmentedVolume": WickraBinding("TSV", ()),
    "MovingAverageEnvelope": WickraBinding("MaEnvelope", ()),
    "AverageTrueRangeBands": WickraBinding("AtrBands", ()),
    "LinearRegressionChannel": WickraBinding("LinRegChannel", ()),
    "StandardErrorBands": WickraBinding("StandardErrorBands", ()),
    "DoubleBollingerBands": WickraBinding("DoubleBollinger", ()),
    "HurstChannel": WickraBinding("HurstChannel", ()),
    "SuperSmoother": WickraBinding("SuperSmoother", ()),
    "InverseFisherTransform": WickraBinding("InverseFisherTransform", ()),
    "Decycler": WickraBinding("Decycler", ()),
    "DecyclerOscillator": WickraBinding("DecyclerOscillator", ()),
    "RoofingFilter": WickraBinding("RoofingFilter", ()),
    "CenterOfGravity": WickraBinding("CenterOfGravity", ()),
    "InstantaneousTrendline": WickraBinding("InstantaneousTrendline", ()),
    "AdaptiveCycle": WickraBinding("AdaptiveCycle", ()),
    "EhlersStochastic": WickraBinding("EhlersStochastic", ()),
    "HilbertDominantCycle": WickraBinding("HilbertDominantCycle", ()),
    "EmpiricalModeDecomposition": WickraBinding(
        "EmpiricalModeDecomposition", ()
    ),
    "ZigZag": WickraBinding("ZigZag", ()),
    "SessionVolumeWeightedAveragePrice": WickraBinding("SessionVwap", ()),
    "SessionRange": WickraBinding("SessionRange", ()),
    "OvernightGap": WickraBinding("OvernightGap", ()),
    "OvernightIntradayReturn": WickraBinding("OvernightIntradayReturn", ()),
    "AverageDailyRange": WickraBinding("AverageDailyRange", ()),
    "TimeOfDayReturnProfile": WickraBinding("TimeOfDayReturnProfile", ()),
    "DayOfWeekReturnProfile": WickraBinding("DayOfWeekProfile", ()),
    "IntradayVolatilityProfile": WickraBinding(
        "IntradayVolatilityProfile", ()
    ),
    "VolumeByTimeProfile": WickraBinding("VolumeByTimeProfile", ()),
    "QuartileBands": WickraBinding("QuartileBands", ()),
    "MedianChannel": WickraBinding("MedianChannel", ()),
    "AbsoluteBreadthIndex": WickraBinding(
        "AbsoluteBreadthIndex", (), cross_section="advance_decline"
    ),
    "CumulativeVolumeIndex": WickraBinding(
        "CumulativeVolumeIndex", (), cross_section="volume"
    ),
    "BullishPercentIndex": WickraBinding(
        "BullishPercentIndex", (), cross_section="buy_signal"
    ),
    "UpDownVolumeRatio": WickraBinding(
        "UpDownVolumeRatio", (), cross_section="volume"
    ),
    "PercentAboveMovingAverage": WickraBinding(
        "PercentAboveMa", (), cross_section="above_ma"
    ),
    "HighLowIndex": WickraBinding("HighLowIndex", (), cross_section="extrema"),
    "NewHighsNewLows": WickraBinding(
        "NewHighsNewLows", (), cross_section="extrema"
    ),
    "BreadthThrust": WickraBinding(
        "BreadthThrust", (), cross_section="advance_decline"
    ),
    "ArmsIndex": WickraBinding("Trin", (), cross_section="trin"),
    "McClellanSummationIndex": WickraBinding(
        "McClellanSummationIndex", (), cross_section="advance_decline"
    ),
    "McClellanOscillator": WickraBinding(
        "McClellanOscillator", (), cross_section="advance_decline"
    ),
    "CupAndHandle": WickraBinding("CupAndHandle", ()),
    "RectangleRange": WickraBinding("RectangleRange", ()),
    "FlagPennant": WickraBinding("FlagPennant", ()),
    "WedgePattern": WickraBinding("Wedge", ()),
    "TrianglePattern": WickraBinding("Triangle", ()),
    "HeadAndShoulders": WickraBinding("HeadAndShoulders", ()),
    "TripleTopBottom": WickraBinding("TripleTopBottom", ()),
    "ThreeDrives": WickraBinding("ThreeDrives", ()),
    "CypherPattern": WickraBinding("Cypher", ()),
    "SharkPattern": WickraBinding("Shark", ()),
    "CrabPattern": WickraBinding("Crab", ()),
    "BatPattern": WickraBinding("Bat", ()),
    "ButterflyPattern": WickraBinding("Butterfly", ()),
    "GartleyPattern": WickraBinding("Gartley", ()),
    "FourPointHarmonicPattern": WickraBinding("Abcd", ()),
    "FibonacciTimeZones": WickraBinding("FibTimeZones", ()),
    "FibonacciChannel": WickraBinding("FibChannel", ()),
    "FibonacciArcs": WickraBinding("FibArcs", ()),
    "FibonacciFan": WickraBinding("FibFan", ()),
    "FibonacciConfluence": WickraBinding("FibConfluence", ()),
    "GoldenPocket": WickraBinding("GoldenPocket", ()),
    "AutomaticFibonacci": WickraBinding("AutoFib", ()),
    "FibonacciProjection": WickraBinding("FibProjection", ()),
    "FibonacciExtension": WickraBinding("FibExtension", ()),
}

NUMPY_BINDINGS: dict[str, ExternalBinding] = {
    "BreakOfStructureChangeOfCharacter": ExternalBinding(
        "NumPy", "causal BOS and CHOCH events"
    ),
    "OrderBlock": ExternalBinding("NumPy", "causal dual-scale order blocks"),
    "Liquidity": ExternalBinding("NumPy", "causal liquidity pools"),
    "EqualHighsLows": ExternalBinding("NumPy", "causal equal pivot levels"),
    "SwingHighLow": ExternalBinding("NumPy", "causal confirmed swing pivots"),
    "SmoothedTrendChannel": ExternalBinding("NumPy", "smoothed trend channel"),
    "PositionHold": ExternalBinding("NumPy", "nonzero position hold"),
    "EntryExit": ExternalBinding("NumPy", "entry-exit position state"),
    "SessionExtrema": ExternalBinding("NumPy", "explicit-session extrema"),
    "PreviousHighLow": ExternalBinding("NumPy", "previous-session high-low"),
    "Retracements": ExternalBinding("NumPy", "causal swing retracements"),
    "PremiumDiscount": ExternalBinding("NumPy", "rolling premium-discount zone"),
    "FibonacciRetracement": ExternalBinding("NumPy", "rolling Fibonacci levels"),
    "AnchoredVolumeWeightedAveragePrice": ExternalBinding(
        "NumPy", "anchored VWAP deviation bands"
    ),
    "PivotPoints": ExternalBinding("NumPy", "anchored classic pivot points"),
    "OpeningRange": ExternalBinding("NumPy", "anchored opening range"),
    "SessionVolumeLevels": ExternalBinding("NumPy", "anchored volume levels"),
    "HedgeRatio": ExternalBinding("NumPy", "rolling OLS hedge ratio"),
    "RollingEntropy": ExternalBinding("NumPy", "rolling Shannon entropy"),
    "FractalDimension": ExternalBinding("NumPy", "two-chunk rescaled-range dimension"),
    "OrnsteinUhlenbeckHalfLife": ExternalBinding("NumPy", "rolling OU half life"),
    "SpreadZScore": ExternalBinding("NumPy", "rolling hedged-spread z-score"),
    "CumulativeSumControlChart": ExternalBinding("NumPy", "CUSUM event filter"),
    "FracDiff": ExternalBinding("NumPy", "fixed-width fractional differencing"),
    "RollSpread": ExternalBinding("NumPy", "rolling Roll spread estimator"),
    "RollingPercentile": ExternalBinding("NumPy", "rolling percentile"),
    "Cross": ExternalBinding("NumPy", "causal cross event"),
    "GarmanKlassYangZhang": ExternalBinding(
        "NumPy", "annualized Garman-Klass-Yang-Zhang volatility"
    ),
    "CloseToCloseSigma": ExternalBinding(
        "NumPy", "annualized close-to-close volatility"
    ),
    "TimeSeriesRank": ExternalBinding("NumPy", "rolling percentile rank"),
    "DecayLinear": ExternalBinding("NumPy", "linear decay weighted mean"),
    "CumulativeCount": ExternalBinding("NumPy", "one-based cumulative count"),
    "ExponentiallyWeightedSum": ExternalBinding("NumPy", "exponentially weighted sum"),
    "AverageDailyDollarValue": ExternalBinding(
        "NumPy", "rolling average dollar volume"
    ),
    "LowerLow": ExternalBinding("NumPy", "lower low relation"),
    "InsideBar": ExternalBinding("NumPy", "inside bar relation"),
    "OutsideBar": ExternalBinding("NumPy", "outside bar relation"),
    "GapUp": ExternalBinding("NumPy", "gap up relation"),
    "GapDown": ExternalBinding("NumPy", "gap down relation"),
    "BarsSince": ExternalBinding("NumPy", "bars since condition"),
    "ValueWhen": ExternalBinding("NumPy", "last value when condition"),
    "HighestSince": ExternalBinding("NumPy", "highest since condition"),
    "LowestSince": ExternalBinding("NumPy", "lowest since condition"),
    "SignalDelay": ExternalBinding("NumPy", "signal delay"),
    "Drawdown": ExternalBinding("NumPy", "drawdown from cumulative maximum"),
    "CumulativeMaximum": ExternalBinding("NumPy", "numpy.maximum.accumulate"),
    "CumulativeMinimum": ExternalBinding("NumPy", "numpy.minimum.accumulate"),
    "RollingCoefficientOfDetermination": ExternalBinding(
        "NumPy", "rolling squared correlation"
    ),
    "ProjectionBands": ExternalBinding("NumPy", "rolling projection mean"),
    "Crossover": ExternalBinding("NumPy", "causal crossover"),
    "Crossunder": ExternalBinding("NumPy", "causal crossunder"),
    "Rising": ExternalBinding("NumPy", "period-over-period rising"),
    "Falling": ExternalBinding("NumPy", "period-over-period falling"),
    "HigherHigh": ExternalBinding("NumPy", "higher high relation"),
    "Lag": ExternalBinding("NumPy", "causal lag"),
    "CumulativeSum": ExternalBinding("NumPy", "numpy.cumsum"),
    "CumulativeProduct": ExternalBinding("NumPy", "numpy.cumprod"),
    "RollingMode": ExternalBinding("NumPy", "rolling mode"),
    "RollingRank": ExternalBinding("NumPy", "rolling percentile rank"),
    "RollingWinsorize": ExternalBinding("NumPy", "rolling winsorize"),
    "ExponentiallyWeightedVariance": ExternalBinding("NumPy", "ewm variance"),
    "ExponentiallyWeightedStandardDeviation": ExternalBinding(
        "NumPy", "ewm standard deviation"
    ),
    "ExponentiallyWeightedCovariance": ExternalBinding("NumPy", "ewm covariance"),
    "ExponentiallyWeightedCorrelation": ExternalBinding(
        "NumPy", "ewm correlation"
    ),
    "RollingCalmar": ExternalBinding("NumPy", "rolling calmar on equity"),
    "RollingRecoveryFactor": ExternalBinding(
        "NumPy", "rolling recovery factor on equity"
    ),
    "Ichimoku": ExternalBinding("NumPy", "causal ichimoku components"),
    "MathAbs": ExternalBinding("NumPy", "numpy.abs"),
    "MathAcosh": ExternalBinding("NumPy", "numpy.arccosh"),
    "MathAsinh": ExternalBinding("NumPy", "numpy.arcsinh"),
    "MathAtanh": ExternalBinding("NumPy", "numpy.arctanh"),
    "MathCbrt": ExternalBinding("NumPy", "numpy.cbrt"),
    "MathCot": ExternalBinding("NumPy", "numpy.tan reciprocal"),
    "MathDegrees": ExternalBinding("NumPy", "numpy.degrees"),
    "MathLog1p": ExternalBinding("NumPy", "numpy.log1p"),
    "MathRadians": ExternalBinding("NumPy", "numpy.radians"),
    "SignedPower": ExternalBinding("NumPy", "numpy.sign/abs/power"),
}

NUMPY_DOMAINS = {
    "BreakOfStructureChangeOfCharacter": "prices",
    "OrderBlock": "prices",
    "Liquidity": "prices",
    "EqualHighsLows": "prices",
    "SwingHighLow": "prices",
    "SmoothedTrendChannel": "prices",
    "PositionHold": "centered",
    "EntryExit": "prices",
    "SessionExtrema": "prices",
    "PreviousHighLow": "prices",
    "Retracements": "prices",
    "PremiumDiscount": "prices",
    "FibonacciRetracement": "prices",
    "AnchoredVolumeWeightedAveragePrice": "prices",
    "PivotPoints": "prices",
    "OpeningRange": "prices",
    "SessionVolumeLevels": "prices",
    "HedgeRatio": "centered",
    "RollingEntropy": "centered",
    "FractalDimension": "prices",
    "OrnsteinUhlenbeckHalfLife": "prices",
    "SpreadZScore": "centered",
    "CumulativeSumControlChart": "centered",
    "FracDiff": "centered",
    "RollSpread": "prices",
    "RollingPercentile": "centered",
    "Cross": "centered",
    "GarmanKlassYangZhang": "prices",
    "CloseToCloseSigma": "prices",
    "TimeSeriesRank": "centered",
    "DecayLinear": "centered",
    "CumulativeCount": "centered",
    "ExponentiallyWeightedSum": "centered",
    "AverageDailyDollarValue": "prices",
    "LowerLow": "prices",
    "InsideBar": "prices",
    "OutsideBar": "prices",
    "GapUp": "prices",
    "GapDown": "prices",
    "BarsSince": "prices",
    "ValueWhen": "prices",
    "HighestSince": "prices",
    "LowestSince": "prices",
    "SignalDelay": "centered",
    "Drawdown": "equity",
    "CumulativeMaximum": "centered",
    "CumulativeMinimum": "centered",
    "RollingCoefficientOfDetermination": "centered",
    "ProjectionBands": "prices",
    "Crossover": "centered",
    "Crossunder": "centered",
    "Rising": "centered",
    "Falling": "centered",
    "HigherHigh": "prices",
    "Lag": "centered",
    "CumulativeSum": "centered",
    "CumulativeProduct": "unit",
    "RollingMode": "centered",
    "RollingRank": "centered",
    "RollingWinsorize": "centered",
    "ExponentiallyWeightedVariance": "centered",
    "ExponentiallyWeightedStandardDeviation": "centered",
    "ExponentiallyWeightedCovariance": "centered",
    "ExponentiallyWeightedCorrelation": "centered",
    "RollingCalmar": "equity",
    "RollingRecoveryFactor": "equity",
    "Ichimoku": "prices",
    "MathAbs": "centered",
    "MathAcosh": "positive",
    "MathAsinh": "centered",
    "MathAtanh": "unit",
    "MathCbrt": "centered",
    "MathCot": "angle",
    "MathDegrees": "angle",
    "MathLog1p": "log_domain",
    "MathRadians": "angle",
    "SignedPower": "centered",
}

PANDAS_TA_BINDINGS: dict[str, PandasTaBinding] = {
    "FisherTransform": PandasTaBinding("fisher", oracle_indices=(0,)),
    "Supertrend": PandasTaBinding(
        "supertrend", actual_indices=(2, 3), oracle_indices=(2, 3)
    ),
    "SchaffTrendCycle": PandasTaBinding("stc", atol=2e-6),
    "DetrendedPriceOscillator": PandasTaBinding(
        "dpo", extra_kwargs=(("centered", False),)
    ),
    "JurikMovingAverage": PandasTaBinding("jma"),
    "EvenBetterSinewave": PandasTaBinding("ebsw"),
    "Squeeze": PandasTaBinding("squeeze"),
    "SqueezePro": PandasTaBinding("squeeze_pro"),
    "ParabolicMovingAverageStop": PandasTaBinding("pmax", actual_indices=(0,)),
}

SMC_BINDINGS: dict[str, ExternalBinding] = {
    "FairValueGap": ExternalBinding("SMC", "smartmoneyconcepts.smc.fvg"),
    "Sessions": ExternalBinding("SMC", "smartmoneyconcepts.smc.sessions"),
}

# Whole-name replacements for table rows whose class names expand
# differently than token-by-token camel-casing.
WHOLE_NAME = {
    "acos": "math_acos", "asin": "math_asin", "atan": "math_atan",
    "ceil": "math_ceil", "cos": "math_cos", "cosh": "math_cosh",
    "exp": "math_exp", "floor": "math_floor", "ln": "math_ln",
    "log10": "math_log10", "sin": "math_sin", "sinh": "math_sinh",
    "sqrt": "math_sqrt", "tan": "math_tan", "tanh": "math_tanh",
    "add": "math_add", "sub": "math_subtract",
    "mult": "math_multiply", "div": "math_divide",
    "avg_price": "average_price",
    "kvo": "klinger_volume_oscillator",
    "mcginley": "mcginley_dynamic",
    "vidya": "variable_index_dynamic_average",
    "laguerre_rsi": "laguerre_relative_strength_index",
    "rmi": "relative_momentum_index",
    "jma": "jurik_moving_average",
    "td_sequential": "tom_de_mark_sequential",
    "fib_retracement": "fibonacci_retracement",
    "anchored_vwap": "anchored_volume_weighted_average_price",
    "ssl_channel": "smoothed_trend_channel",
    "pmax": "parabolic_moving_average_stop",
}

# Token-level synonym expansion applied inside snake names.
TOKEN_SYNONYMS = {
    "std": "standard_deviation",
    "var": "variance",
    "corr": "correlation",
    "cov": "covariance",
    "avgdev": "average_deviation",
    "linreg": "linear_regression",
    "tsf": "time_series_forecast",
    "ewm": "exponentially_weighted",
    "vwap": "volume_weighted_average_price",
}

# TA-Lib parameter name -> candidate constructor keyword names (tried in
# order, before falling back to underscore-normalized equality).
PARAM_SYNONYMS = {
    "timeperiod": ("timeperiod", "period", "time_period"),
    "nbdevup": ("deviations_up", "nbdevup"),
    "nbdevdn": ("deviations_down", "nbdevdn"),
    "matype": ("moving_average_type", "matype", "average_type"),
    "fastmatype": ("fast_average_type", "fast_moving_average_type"),
    "slowmatype": ("slow_average_type", "slow_moving_average_type"),
    "signalmatype": ("signal_average_type", "signal_moving_average_type"),
    "fastd_matype": ("fast_d_average_type",),
    "slowk_matype": ("slow_k_average_type",),
    "slowd_matype": ("slow_d_average_type",),
    "vfactor": ("volume_factor", "v_factor"),
    "startvalue": ("start_value",),
    "offsetonreverse": ("offset_on_reverse",),
    "accelerationinitlong": ("acceleration_init_long",),
    "accelerationlong": ("acceleration_long",),
    "accelerationmaxlong": ("acceleration_max_long",),
    "accelerationinitshort": ("acceleration_init_short",),
    "accelerationshort": ("acceleration_short",),
    "accelerationmaxshort": ("acceleration_max_short",),
    "minperiod": ("min_period",),
    "maxperiod": ("max_period",),
    "fastlimit": ("fast_limit",),
    "slowlimit": ("slow_limit",),
}

# Input domains narrower than a price series.
INPUT_DOMAIN_OVERRIDES = {"ACOS": "unit", "ASIN": "unit"}

# Canonical-only pointwise operations with a restricted real-valued domain.
SNAKE_DOMAIN_OVERRIDES = {"math_atanh": "unit"}

# Series-typed constructor/extend parameter names (never mapped as params).
SERIES_PARAM_NAMES = {
    "_input", "input", "values", "close", "high", "low", "open", "_open",
    "volume", "left", "right", "periods", "period", "real", "price", "column",
    "x", "y", "benchmark", "condition", "new_session", "anchor", "entry",
    "_exit", "input0", "input1", "_input0", "_input1", "h", "l", "change",
    "value", "equity",
}


def _norm(name: str) -> str:
    return name.replace("_", "").lower()


def _snake_case(name: str) -> str:
    """Convert a canonical CamelCase class name to its module spelling."""
    first = re.sub(r"(.)([A-Z][a-z]+)", r"\1_\2", name)
    return re.sub(r"([a-z0-9])([A-Z])", r"\1_\2", first).lower()


def scalar_default(name: str, default=inspect.Parameter.empty):
    """Deterministic constructor value for required non-series parameters."""
    if default is not inspect.Parameter.empty:
        return default
    if name in {"quantile", "alpha"}:
        return 0.5
    if name == "percentile":
        return 50.0
    if name in {"gamma", "phase"}:
        return 0.5
    if name in {"stdev", "value_area"}:
        return 1.0
    if "average_type" in name or name == "matype":
        return 0
    if name == "fastlimit":
        return 0.5
    if name == "slowlimit":
        return 0.05
    if name in {"factor", "scalar", "multiplier"} or "factor" in name:
        return 0.7
    return 5


def parse_master_table() -> list[tuple[str, str]]:
    """Return (snake_name, talib_name_or_'_') rows from CHECK.md §2.5."""
    text = CHECK_MD.read_text()
    return [
        (m.group(1), m.group(2))
        for m in re.finditer(
            r"^\| \[(?:x| )\] \| ([a-z0-9_]+) \| [a-z0-9_]+ "
            r"\| ([A-Z0-9_]+|_) \|", text, re.M)
    ]


def resolve_class(snake: str):
    """Resolve a snake-case table name to a live taflow class (or None)."""
    import taflow

    by_norm = {_norm(c): c for c in dir(taflow) if c[:1].isupper()}
    candidates = [snake]
    if snake in WHOLE_NAME:
        candidates.append(WHOLE_NAME[snake])
    candidates.append(
        "_".join(TOKEN_SYNONYMS.get(tok, tok) for tok in snake.split("_")))
    for cand in candidates:
        hit = by_norm.get(_norm(cand))
        if hit:
            return getattr(taflow, hit)
    return None


@dataclass
class Spec:
    """One function: taflow class + TA-Lib translation."""

    snake: str
    talib_name: str | None            # None => taflow-only
    cls: type | None
    ctor_kwargs: dict = field(default_factory=dict)
    series_args: tuple[str, ...] = ()  # extend arg names, in call order
    input_roles: tuple[str, ...] = ()  # TA-Lib roles, same order
    domain: str = "prices"
    error: str | None = None
    warnings: list = field(default_factory=list)
    lookback: int = 0
    wickra: WickraBinding | None = None
    pandas_ta: PandasTaBinding | None = None
    numpy: ExternalBinding | None = None
    smc: ExternalBinding | None = None

    # -- construction ------------------------------------------------------

    @classmethod
    def build(cls, snake: str, talib_name: str | None) -> "Spec":
        spec = cls(snake=snake, talib_name=talib_name,
                   cls=resolve_class(snake))
        if spec.cls is None:
            spec.error = "no matching taflow class"
            return spec
        spec.wickra = WICKRA_BINDINGS.get(spec.cls.__name__)
        spec.pandas_ta = PANDAS_TA_BINDINGS.get(spec.cls.__name__)
        spec.numpy = NUMPY_BINDINGS.get(spec.cls.__name__)
        spec.smc = SMC_BINDINGS.get(spec.cls.__name__)
        if spec.numpy:
            spec.domain = NUMPY_DOMAINS[spec.cls.__name__]
        try:
            sig = inspect.signature(spec.cls.__init__)
        except (TypeError, ValueError):
            spec.error = "constructor signature unavailable"
            return spec
        ctor_params = {p: value for p, value in sig.parameters.items()
                       if p != "self" and value.kind not in
                       (value.VAR_POSITIONAL, value.VAR_KEYWORD)}

        try:
            ext_sig = inspect.signature(spec.cls.extend)
            # Ignore optional dataframe-column selector kwargs.
            spec.series_args = tuple(
                p for p in ext_sig.parameters
                if p != "self" and not p.endswith("column"))
        except (AttributeError, TypeError, ValueError):
            spec.error = "no extend method"
            return spec

        if talib_name:
            spec._bind_talib(talib_name, set(ctor_params))
        else:
            spec.input_roles = tuple(
                "close" if a in ("_input", "input", "values") else a
                for a in spec.series_args)
            spec.domain = SNAKE_DOMAIN_OVERRIDES.get(snake, spec.domain)
        for name, parameter in ctor_params.items():
            if (_norm(name) not in {_norm(item) for item in spec.series_args}
                    and name not in spec.ctor_kwargs
                    and parameter.default is inspect.Parameter.empty):
                spec.ctor_kwargs[name] = scalar_default(name)
        return spec

    @property
    def oracle_source(self) -> str | None:
        """Return the selected source in TA-Lib/Wickra/NumPy/SMC priority."""
        if self.talib_name:
            return "TA-Lib"
        if self.wickra:
            return "Wickra"
        if self.pandas_ta:
            return "pandas-ta-classic"
        if self.numpy:
            return "NumPy"
        if self.smc:
            return "SMC"
        return None

    @property
    def oracle_name(self) -> str | None:
        """Return the selected external callable name."""
        if self.talib_name:
            return self.talib_name
        binding = self.wickra or self.pandas_ta or self.numpy or self.smc
        return binding.name if binding else None

    @property
    def oracle_variant(self) -> str | None:
        """Return a documented semantic difference for the selected oracle."""
        binding = self.wickra or self.pandas_ta or self.numpy or self.smc
        return binding.variant if binding else None

    def _bind_talib(self, name: str, ctor_params: set[str]) -> None:
        from talib import abstract

        info = abstract.Function(name).info
        self.lookback = int(abstract.Function(name).lookback)
        self.domain = INPUT_DOMAIN_OVERRIDES.get(name, "prices")

        roles: list[str] = []
        for role, value in info["input_names"].items():
            if isinstance(value, (list, tuple)):
                roles.extend(value)
            else:
                roles.append(role if role.startswith("price") else value)
        if len(roles) != len(self.series_args):
            self.error = (f"input arity mismatch: talib {roles} vs "
                          f"extend{self.series_args}")
            return
        self.input_roles = tuple(roles)

        norm_ctor = {_norm(p): p for p in ctor_params
                     if p not in SERIES_PARAM_NAMES}
        for talib_param, value in info["parameters"].items():
            target = None
            for cand in PARAM_SYNONYMS.get(talib_param, ()):
                if cand in ctor_params:
                    target = cand
                    break
            if target is None:
                target = norm_ctor.get(_norm(talib_param))
            if target is None:
                # Parameter absent from the canonical class (e.g. TA-Lib's
                # `penetration` / `nbdev`). At TA-Lib defaults the comparison
                # is still valid; record a warning instead of failing.
                self.warnings.append(
                    f"talib param {talib_param!r} (default {value!r}) has no "
                    "constructor counterpart — compared at defaults only")
                continue
            self.ctor_kwargs[target] = value

    # -- data --------------------------------------------------------------

    def arrays(self, data: dict, n: int) -> list[np.ndarray]:
        if self.wickra and self.wickra.cross_section:
            index = np.arange(n, dtype=np.int64)
            advancers = (1 + index % 4).astype(np.float64)
            decliners = (1 + (index * 3) % 3).astype(np.float64)
            mode = self.wickra.cross_section
            if mode == "volume":
                return [
                    np.ascontiguousarray(advancers * (10 + index % 5)),
                    np.ascontiguousarray(decliners * (8 + (index * 2) % 7)),
                ]
            if mode in {"buy_signal", "above_ma"}:
                return [
                    np.ascontiguousarray((index % 9).astype(np.float64)),
                    np.full(n, 8.0, dtype=np.float64),
                ]
            if mode == "extrema":
                return [
                    np.ascontiguousarray((index % 4).astype(np.float64)),
                    np.ascontiguousarray(((index * 2) % 4).astype(np.float64)),
                ]
            if mode == "trin":
                return [
                    np.ascontiguousarray(advancers),
                    np.ascontiguousarray(decliners),
                    np.ascontiguousarray(advancers * (10 + index % 5)),
                    np.ascontiguousarray(decliners * (8 + (index * 2) % 7)),
                ]
            return [np.ascontiguousarray(advancers), np.ascontiguousarray(decliners)]

        out = []
        for role in self.input_roles:
            if (self.cls and self.cls.__name__ == "Sessions"
                    and role == "new_session"):
                key = "one_session"
            elif role in ("_input", "input", "values", "price", "real",
                        "close", "change", "value", "equity"):
                key = self.domain if self.domain in data else "close"
            elif role in ("price0", "left", "x", "input0", "_input0"):
                key = "close"
            elif role in ("price1", "right", "y", "benchmark", "input1",
                          "_input1"):
                key = "close2"
            elif role in ("period", "periods"):
                key = "periods"
            elif role == "h":
                key = "high"
            elif role == "l":
                key = "low"
            elif role in data:
                key = role
            else:
                key = "close"
            out.append(np.ascontiguousarray(data[key][:n]))
        return out

    # -- state adapters (handle fluent and value-returning APIs) -----------

    def new_state(self):
        boolean_roles = {
            "condition", "new_session", "anchor", "entry", "_exit",
            "on_buy_signal", "above_moving_average",
        }
        empty_series = [
            np.empty(0, dtype=np.bool_ if role in boolean_roles else np.float64)
            for role in self.input_roles
        ]
        return self.cls(*empty_series, **self.ctor_kwargs)

    @staticmethod
    def extend(state, arrays) -> tuple[np.ndarray, ...]:
        result = state.extend(*arrays)
        if result is state:
            result = state.compute()
        return result if isinstance(result, tuple) else (result,)

    @staticmethod
    def append_value(state, bar):
        result = state.append(*bar)
        if result is state:
            result = getattr(state, "value", None)
        return result


def build_registry() -> dict[str, Spec]:
    """Return every public lifecycle class, including CHECK.md mappings."""
    import taflow

    specs: dict[str, Spec] = {}
    for snake, talib_name in parse_master_table():
        key = talib_name if talib_name != "_" else snake
        specs[key] = Spec.build(snake,
                                talib_name if talib_name != "_" else None)
    registered = {spec.cls for spec in specs.values() if spec.cls is not None}
    for name in getattr(taflow, "__all__", ()):
        candidate = getattr(taflow, name, None)
        if (not isinstance(candidate, type) or candidate in registered
                or not all(hasattr(candidate, method)
                           for method in ("append", "extend", "compute", "reset"))):
            continue
        snake = _snake_case(name)
        specs[snake] = Spec.build(snake, None)
    return specs


def constructor_value(spec: Spec, name: str):
    """Return the configured or documented default constructor value."""
    if name in spec.ctor_kwargs:
        return spec.ctor_kwargs[name]
    parameter = inspect.signature(spec.cls.__init__).parameters[name]
    return scalar_default(name, parameter.default)


def resolve_specs(names: list[str], registry: dict[str, Spec]) -> tuple[list[Spec], list[str]]:
    """Resolve TA-Lib, snake-case, or canonical class names without aliases."""
    indexes: dict[str, Spec] = {}
    for key, spec in registry.items():
        for candidate in (key, spec.snake,
                          spec.talib_name or "",
                          spec.cls.__name__ if spec.cls else ""):
            if candidate:
                indexes[_norm(candidate)] = spec
    resolved, unknown = [], []
    for name in names:
        spec = indexes.get(_norm(name))
        if spec is None:
            unknown.append(name)
        elif spec not in resolved:
            resolved.append(spec)
    return resolved, unknown


# ---------------------------------------------------------------------------
# Shared deterministic data generator (mean-reverting log-price OHLCV)
# ---------------------------------------------------------------------------

def make_data(n: int, seed: int = 42) -> dict[str, np.ndarray]:
    def ar1(offset: int) -> np.ndarray:
        rng = np.random.default_rng(seed + offset)
        noise = rng.normal(0.0, 0.02, n)
        decay = 1.0 - 0.001
        block = 4096
        pows = decay ** np.arange(block)
        inv_pows = decay ** -np.arange(block)
        x = np.empty(n)
        carry = 0.0
        for start in range(0, n, block):
            chunk = noise[start:start + block]
            m = len(chunk)
            conv = pows[:m] * np.cumsum(chunk * inv_pows[:m])
            x[start:start + m] = conv + carry * decay * pows[:m]
            carry = x[start + m - 1]
        return 100.0 * np.exp(x)

    close = ar1(0)
    rng = np.random.default_rng(seed + 1000)
    spread = close * 0.01
    high = close + rng.uniform(0.0, 1.0, n) * spread
    low = close - rng.uniform(0.0, 1.0, n) * spread
    open_ = low + rng.uniform(0.0, 1.0, n) * (high - low)
    unit_noise = np.random.default_rng(seed + 2000).normal(0.0, 0.05, n)
    centered = (ar1(5000) - 100.0) / 10.0
    return {
        "open": open_, "high": high, "low": low, "close": close,
        "volume": rng.uniform(1e5, 1e6, n),
        "close2": ar1(3000),
        # TA-Lib's MAVP Python binding requires a float64 periods array even
        # though the values represent integral periods.
        "periods": np.random.default_rng(seed + 4000).integers(
            2, 31, n).astype(np.float64),
        "unit": np.clip(np.cumsum(unit_noise) % 1.8 - 0.9, -0.99, 0.99),
        "centered": centered,
        "positive": np.abs(centered) + 1.0,
        "angle": centered + 0.25,
        "log_domain": np.abs(centered),
        "condition": np.arange(n) % 11 == 0,
        "on_buy_signal": np.arange(n) % 7 == 0,
        "above_moving_average": close > np.mean(close),
        # TAFlow's public timestamp contract is Unix nanoseconds. Oracle
        # adapters convert units when an external library uses milliseconds.
        "timestamp": 1_700_000_000_000_000_000
        + np.arange(n, dtype=np.int64) * 3_600_000_000_000,
        "new_session": np.arange(n) % 32 == 0,
        "one_session": np.arange(n) == 0,
        "anchor": np.arange(n) % 64 == 0,
        "entry": np.arange(n) % 17 == 0,
        "_exit": np.arange(n) % 19 == 0,
    }
