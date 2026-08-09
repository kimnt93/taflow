#!/usr/bin/env python3
"""Compare taflow's Python interface with non-TA-Lib external oracles.

This complements ``verify.py`` (TA-Lib plus streaming/chunk invariance) with
the three independent sources requested for extension indicators/operators:

* pandas-ta-classic for modern technical indicators;
* Polars expressions for rolling/EWM/cumulative/math operators;
* smartmoneyconcepts for the SMC family, with explicit causal alignment.

Every comparison is named and reported.  Import/oracle failures are errors;
they are never silently converted into native self-consistency passes.
"""
from __future__ import annotations

import argparse
import importlib.metadata
import json
import os
from dataclasses import asdict, dataclass
from pathlib import Path
from typing import Iterable

import numpy as np
import pandas as pd
import pandas_ta_classic as pta
import polars as pl
import taflow

from registry import make_data

os.environ.setdefault("SMC_CREDIT", "0")
from smartmoneyconcepts import smc  # noqa: E402


RTOL = 1e-8
ATOL = 1e-10
HERE = Path(__file__).parent


@dataclass
class Result:
    oracle: str
    function: str
    output: str
    passed: bool
    max_abs_error: float
    nan_mismatches: int
    compared: int
    note: str = ""
    error: str | None = None
    expected_difference: bool = False


def _array(value) -> np.ndarray:
    if isinstance(value, (pd.Series, pd.Index)):
        value = value.to_numpy()
    elif isinstance(value, pl.Series):
        value = value.to_numpy()
    return np.asarray(value, dtype=np.float64)


def compare(
    rows: list[Result],
    oracle: str,
    function: str,
    output: str,
    actual,
    expected,
    *,
    start: int = 0,
    stop: int | None = None,
    rtol: float = RTOL,
    atol: float = ATOL,
    note: str = "",
    expected_difference: bool = False,
) -> None:
    try:
        actual = _array(actual)[start:stop]
        expected = _array(expected)[start:stop]
        if actual.shape != expected.shape:
            rows.append(Result(oracle, function, output, False, np.inf, 0, 0,
                               note, f"shape {actual.shape} != {expected.shape}",
                               expected_difference))
            return
        nan_actual = np.isnan(actual)
        nan_expected = np.isnan(expected)
        nan_mismatches = int(np.count_nonzero(nan_actual != nan_expected))
        valid = ~nan_actual & ~nan_expected
        max_error = (float(np.max(np.abs(actual[valid] - expected[valid])))
                     if np.any(valid) else 0.0)
        passed = nan_mismatches == 0 and bool(np.allclose(
            actual, expected, rtol=rtol, atol=atol, equal_nan=True))
        rows.append(Result(oracle, function, output, passed, max_error,
                           nan_mismatches, int(np.count_nonzero(valid)), note,
                           expected_difference=expected_difference))
    except Exception as exc:  # report the exact broken comparison
        rows.append(Result(oracle, function, output, False, np.inf, 0, 0,
                           note, f"{type(exc).__name__}: {exc}",
                           expected_difference))


def outputs(value) -> tuple[np.ndarray, ...]:
    return value if isinstance(value, tuple) else (value,)


def run_polars(data: dict[str, np.ndarray], rows: list[Result]) -> None:
    """Exact overlap with Polars' documented expression semantics."""
    n = 14
    q = 0.25
    close, other = data["close"], data["close2"]
    standardized = (close - close.mean()) / close.std()
    frame = pl.DataFrame({"close": close, "other": other,
                          "standardized": standardized})

    expressions = {
        "sum": pl.col("close").rolling_sum(n),
        "min": pl.col("close").rolling_min(n),
        "max": pl.col("close").rolling_max(n),
        "mean": pl.col("close").rolling_mean(n),
        "median": pl.col("close").rolling_median(n),
        "quantile": pl.col("close").rolling_quantile(
            q, interpolation="linear", window_size=n),
        "variance": pl.col("close").rolling_var(n, ddof=0),
        "stddev": pl.col("close").rolling_std(n, ddof=0),
        # Center/scale first: skew and kurtosis are invariant to this affine
        # transform, while Polars' raw-moment rolling kernel otherwise loses
        # precision for price series centered near 100.
        "skew": pl.col("standardized").rolling_skew(n, bias=True),
        "kurtosis": pl.col("standardized").rolling_kurtosis(
            n, fisher=True, bias=True),
        "covariance": pl.rolling_cov("close", "other", window_size=n, ddof=0),
        "correlation": pl.rolling_corr("close", "other", window_size=n),
        "ewm_variance": pl.col("close").ewm_var(
            span=n, adjust=False, bias=True),
        "ewm_stddev": pl.col("close").ewm_std(
            span=n, adjust=False, bias=True),
        "cumulative_sum": pl.col("close").cum_sum(),
        "cumulative_product": (pl.col("close") / 100.0).cum_prod(),
        "cumulative_minimum": pl.col("close").cum_min(),
        "cumulative_maximum": pl.col("close").cum_max(),
        "abs": pl.col("other").sub(100.0).abs(),
        "floor": pl.col("close").floor(),
        "ceil": pl.col("close").ceil(),
        "sqrt": pl.col("close").sqrt(),
        "ln": pl.col("close").log(),
        "exp": (pl.col("close") / 100.0).exp(),
        "sin": pl.col("close").sin(),
        "cos": pl.col("close").cos(),
        "tan": pl.col("close").tan(),
    }
    oracle = frame.select(*(expr.alias(name) for name, expr in expressions.items()))

    actual = {
        "sum": taflow.RollingSum(close, timeperiod=n).compute(),
        "min": taflow.RollingMin(close, timeperiod=n).compute(),
        "max": taflow.RollingMax(close, timeperiod=n).compute(),
        "mean": taflow.SimpleMovingAverage(close, timeperiod=n).compute(),
        "median": taflow.RollingMedian(n, close).compute(),
        "quantile": taflow.RollingQuantile(n, q, close).compute(),
        "variance": taflow.RollingVariance(close, timeperiod=n).compute(),
        "stddev": taflow.RollingStandardDeviation(close, timeperiod=n).compute(),
        "skew": taflow.RollingSkew(n, standardized).compute(),
        "kurtosis": taflow.RollingKurtosis(n, standardized).compute(),
        "covariance": taflow.RollingCov(n, close, other).compute(),
        "correlation": taflow.RollingCorrelation(close, other, timeperiod=n).compute(),
        "ewm_variance": taflow.ExponentiallyWeightedVariance(n, close).compute(),
        "ewm_stddev": taflow.ExponentiallyWeightedStandardDeviation(n, close).compute(),
        "cumulative_sum": taflow.CumulativeSum(close).compute(),
        "cumulative_product": taflow.CumulativeProduct(close / 100.0).compute(),
        "cumulative_minimum": taflow.CumulativeMinimum(close).compute(),
        "cumulative_maximum": taflow.CumulativeMaximum(close).compute(),
        "abs": taflow.MathAbs(other - 100.0).compute(),
        "floor": taflow.MathFloor(close).compute(),
        "ceil": taflow.MathCeil(close).compute(),
        "sqrt": taflow.MathSqrt(close).compute(),
        "ln": taflow.MathLn(close).compute(),
        "exp": taflow.MathExp(close / 100.0).compute(),
        "sin": taflow.MathSin(close).compute(),
        "cos": taflow.MathCos(close).compute(),
        "tan": taflow.MathTan(close).compute(),
    }
    for name, value in actual.items():
        tolerance = {"skew": 1e-6, "kurtosis": 1e-4}.get(name, ATOL)
        note = ("Polars raw-moment kernel tolerance after centering/scaling"
                if name in {"skew", "kurtosis"} else "")
        compare(rows, "Polars", name, name, value, oracle[name],
                atol=tolerance, note=note)


def run_pandas_ta(data: dict[str, np.ndarray], rows: list[Result]) -> None:
    """Modern indicator overlap at explicit pandas-ta-classic parameters."""
    high, low, close, volume = (data[k] for k in ("high", "low", "close", "volume"))
    open_ = data["open"]
    h, l, c, v, o = map(pd.Series, (high, low, close, volume, open_))
    c2 = pd.Series(data["close2"])

    def many(function: str, actual, expected, names: Iterable[str], **kwargs) -> None:
        expected_columns = (tuple(expected.iloc[:, i] for i in range(expected.shape[1]))
                            if isinstance(expected, pd.DataFrame) else (expected,))
        for name, got, want in zip(names, outputs(actual), expected_columns, strict=True):
            compare(rows, "pandas-ta-classic", function, name, got, want, **kwargs)

    many("awesome_oscillator", taflow.AwesomeOscillator(high, low).compute(),
         pta.ao(h, l, fast=5, slow=34), ("ao",))
    many("log_return", taflow.LogReturn(1, close).compute(),
         pta.log_return(c, length=1), ("log_return",))
    many("force_index", taflow.ForceIndex(close, volume).compute(),
         pta.efi(c, v, length=1, mamode="ema"), ("force_index",),
         note="taflow exposes the unsmoothed one-bar force; pandas-ta EFI length=1")
    many("crossover", taflow.Crossover(close, data["close2"]).compute(),
         pta.cross(c, c2, above=True), ("crossover",))
    many("crossunder", taflow.Crossunder(close, data["close2"]).compute(),
         pta.cross(c, c2, above=False), ("crossunder",))
    many("hull_moving_average", taflow.HullMovingAverage(10, close).compute(),
         pta.hma(c, length=10), ("hma",))
    many("volume_weighted_moving_average",
         taflow.VolumeWeightedMovingAverage(10, close, volume).compute(),
         pta.vwma(c, v, length=10), ("vwma",))
    many("zero_lag_exponential_moving_average",
         taflow.ZeroLagExponentialMovingAverage(10, close).compute(),
         pta.zlma(c, length=10, mamode="ema", talib=False), ("zlema",),
         note="force pandas-ta's native EMA; TA-Lib rejects leading ZLMA NaNs")
    for length in (1, 2, 21):
        many("zero_lag_exponential_moving_average",
             taflow.ZeroLagExponentialMovingAverage(length, close).compute(),
             pta.zlma(c, length=length, mamode="ema", talib=False),
             (f"zlema[length={length}]",),
             note="parameter matrix; force pandas-ta's native EMA")
    donchian = pta.donchian(h, l, lower_length=20, upper_length=20)
    donchian = donchian.iloc[:, [2, 0, 1]]
    many("donchian_channels", taflow.DonchianChannels(high, low, 20).compute(),
         donchian, ("upper", "lower", "mid"))
    many("fisher_transform", taflow.FisherTransform(high, low, 10).compute(),
         pta.fisher(h, l, length=10).iloc[:, 0], ("fisher",))
    # pandas-ta-classic coerces lengths below 10 back to its default of 9.
    for length in (11, 21):
        many("fisher_transform", taflow.FisherTransform(high, low, length).compute(),
             pta.fisher(h, l, length=length).iloc[:, 0],
             (f"fisher[length={length}]",))
    many("chaikin_money_flow",
         taflow.ChaikinMoneyFlow(high, low, close, volume, 20).compute(),
         pta.cmf(h, l, c, v, length=20), ("cmf",))
    many("detrended_price_oscillator",
         taflow.DetrendedPriceOscillator(close, 20).compute(),
         pta.dpo(c, length=20, centered=False), ("dpo",))
    many("mcginley_dynamic", taflow.McGinleyDynamic(close, 10, 1.0).compute(),
         pta.mcgd(c, length=10, c=1.0), ("mcginley",))
    many("variable_index_dynamic_average",
         taflow.VariableIndexDynamicAverage(close, 14).compute(),
         pta.vidya(c, length=14), ("vidya",))
    for length in (1, 2, 30):
        many("variable_index_dynamic_average",
             taflow.VariableIndexDynamicAverage(close, length).compute(),
             pta.vidya(c, length=length), (f"vidya[length={length}]",))
    many("jurik_moving_average", taflow.JurikMovingAverage(close, 7, 0).compute(),
         pta.jma(c, length=7, phase=0), ("jma",))
    for length, phase in ((1, 0), (2, -100), (7, 100), (21, 35)):
        with np.errstate(divide="ignore", invalid="ignore"):
            expected_jma = pta.jma(c, length=length, phase=phase)
        many("jurik_moving_average",
             taflow.JurikMovingAverage(close, length, phase).compute(),
             expected_jma, (f"jma[length={length},phase={phase}]",))
    with np.errstate(over="ignore", invalid="ignore"):
        expected_ebsw_40 = pta.ebsw(c, length=40)
        expected_ebsw_60 = pta.ebsw(c, length=60)
    many("even_better_sinewave", taflow.EvenBetterSinewave(close, 40).compute(),
         expected_ebsw_40, ("ebsw",))
    many("even_better_sinewave", taflow.EvenBetterSinewave(close, 60).compute(),
         expected_ebsw_60, ("ebsw[length=60]",))
    many("vortex", taflow.Vortex(high, low, close, 14).compute(),
         pta.vortex(h, l, c, length=14), ("plus", "minus"))
    many("know_sure_thing", taflow.KnowSureThing(close).compute(),
         pta.kst(c), ("kst", "signal"), expected_difference=True,
         note="taflow follows the bukosabino/ta KST scaling; pandas-ta multiplies by an extra 100")
    many("mass_index", taflow.MassIndex(high, low, 9, 25).compute(),
         pta.massi(h, l, fast=9, slow=25), ("mass",),
         expected_difference=True,
         note="taflow follows bukosabino/ta EMA initialization")
    many("negative_volume_index", taflow.NegativeVolumeIndex(close, volume).compute(),
         pta.nvi(c, v, initial=1000), ("nvi",), expected_difference=True,
         note="taflow uses the standard multiplicative index; pandas-ta uses a cumulative volume-weighted ROC")
    many("positive_volume_index", taflow.PositiveVolumeIndex(close, volume).compute(),
         pta.pvi(c, v, initial=1000), ("pvi",), expected_difference=True,
         note="taflow uses the standard multiplicative index; pandas-ta uses a cumulative volume-weighted ROC")

    many("supertrend", taflow.Supertrend(high, low, close, 7, 3.0).compute(),
         pta.supertrend(h, l, c, length=7, multiplier=3.0),
         ("trend", "direction", "long", "short"), start=6,
         note="pandas-ta seeds pre-ATR rows; compare from length-1")
    many("squeeze", taflow.Squeeze(high, low, close).compute(),
         pta.squeeze(h, l, c, use_tr=True, mamode="sma"),
         ("momentum", "on", "off", "no"))
    many("squeeze_pro", taflow.SqueezePro(high, low, close).compute(),
         pta.squeeze_pro(h, l, c, use_tr=True, mamode="sma"),
         ("momentum", "on_wide", "on_normal", "on_narrow", "off", "no"))
    many("schaff_trend_cycle", taflow.SchaffTrendCycle(close).compute(),
         pta.stc(c), ("stc", "macd", "stochastic"), atol=1e-5,
         note="stream-safe epsilon convention; documented tolerance 1e-5")
    many("klinger_volume_oscillator",
         taflow.KlingerVolumeOscillator(high, low, close, volume).compute(),
         pta.kvo(h, l, c, v, fast=34, slow=55, signal=13),
         ("kvo", "signal"))
    many("klinger_volume_oscillator",
         taflow.KlingerVolumeOscillator(
             high, low, close, volume, fast=5, slow=8, signal=3).compute(),
         pta.kvo(h, l, c, v, fast=5, slow=8, signal=3),
         ("kvo[fast=5,slow=8,signal=3]", "signal[fast=5,slow=8,signal=3]"))
    td = pta.td_seq(c, asint=True)
    # taflow exposes the conventional capped-nine buy/sell setup pair.  The
    # package columns are sell(up), buy(down) and continue through count 13.
    td_expected = pd.DataFrame({
        "buy": np.minimum(td.iloc[:, 1], 9),
        "sell": np.minimum(td.iloc[:, 0], 9),
    })
    many("tom_de_mark_sequential", taflow.TomDeMarkSequential(close).compute(),
         td_expected, ("buy", "sell"),
         note="pandas-ta columns reordered and capped at the DeMark setup count of nine")

    # pandas-ta shifts leading spans forward and the chikou span backward for
    # plotting.  taflow intentionally emits all five causal values at the bar
    # where they are known; undo only those presentation shifts.
    ichi = pta.ichimoku(h, l, c, tenkan=9, kijun=26, senkou=52)[0]
    ichi_expected = (
        ichi.iloc[:, 2], ichi.iloc[:, 3], ichi.iloc[:, 0].shift(-26),
        ichi.iloc[:, 1].shift(-26), c,
    )
    for name, got, want in zip(
            ("tenkan", "kijun", "span_a", "span_b", "chikou"),
            taflow.Ichimoku(high, low, close).compute(), ichi_expected, strict=True):
        stop = -26 if name in {"span_a", "span_b"} else None
        compare(rows, "pandas-ta-classic", "ichimoku", name, got, want,
                stop=stop, note="causal alignment; plotting displacement removed")

    ha = pta.ha(o, h, l, c)
    actual_ha = taflow.HeikinAshi(open_, high, low, close).compute()
    many("heikin_ashi", actual_ha, ha, ("open", "high", "low", "close"))


def _event_flags(frame: pd.DataFrame, signal: str, event_index: str) -> np.ndarray:
    result = np.full(len(frame), np.nan)
    for direction, index in zip(frame[signal], frame[event_index]):
        if not np.isnan(direction) and not np.isnan(index) and int(index) > 0:
            result[int(index)] = direction
    return result


def _event_values(frame: pd.DataFrame, value: str, event_index: str) -> np.ndarray:
    """Move a package value from its historical marker to its known event bar."""
    result = np.full(len(frame), np.nan)
    for item, index in zip(frame[value], frame[event_index]):
        if not np.isnan(item) and not np.isnan(index) and int(index) > 0:
            result[int(index)] = item
    return result


def run_smc(data: dict[str, np.ndarray], rows: list[Result]) -> None:
    """Compare SMC outputs after converting lookahead/index columns to events."""
    bars = len(data["close"])
    index = pd.date_range("2024-01-01", periods=bars, freq="15min")
    ohlcv = pd.DataFrame({k: data[k] for k in ("open", "high", "low", "close", "volume")},
                         index=index)

    reference_fvg = smc.fvg(ohlcv.copy(), join_consecutive=False)
    actual_fvg = taflow.FairValueGap(
        data["open"], data["high"], data["low"], data["close"]).compute()
    for name, got, source in zip(("fvg", "top", "bottom"), actual_fvg[:3],
                                 ("FVG", "Top", "Bottom"), strict=True):
        compare(rows, "smartmoneyconcepts", "fair_value_gap", name, got,
                reference_fvg[source].shift(1),
                note="package marker at middle candle; shifted to causal detection bar")
    compare(rows, "smartmoneyconcepts", "fair_value_gap", "mitigated",
            actual_fvg[3], _event_flags(reference_fvg, "FVG", "MitigatedIndex"),
            note="package future index converted to flag at mitigation bar")

    swing_length = 5
    reference_swing = smc.swing_highs_lows(ohlcv.copy(), swing_length=swing_length)
    actual_swing = taflow.SwingHighsLows(
        data["high"], data["low"], swing_length=swing_length).compute()
    # Synthetic endpoint markers and the final unconfirmable lookahead region
    # are package presentation artifacts, so compare only the confirmable core.
    expected_signal = reference_swing["HighLow"].shift(swing_length)
    expected_level = reference_swing["Level"].shift(swing_length)
    compare(rows, "smartmoneyconcepts", "swing_highs_lows", "signal",
            actual_swing[0], expected_signal, start=swing_length + 1,
            stop=-(swing_length + 1),
            note="package also removes markers retroactively; causal taflow cannot retract emitted events",
            expected_difference=True)
    compare(rows, "smartmoneyconcepts", "swing_highs_lows", "level",
            actual_swing[1], expected_level, start=swing_length + 1,
            stop=-(swing_length + 1),
            note="package also removes markers retroactively; causal taflow cannot retract emitted events",
            expected_difference=True)

    # SMC writes BOS/CHOCH markers back onto an earlier pivot only after a
    # later break.  Compare taflow's causal break event with the package's
    # BrokenIndex projected forward, and retain the earlier setup outputs as
    # an explicit contract variant rather than claiming raw parity.
    reference_structure = smc.bos_choch(
        ohlcv.copy(), reference_swing, close_break=True)
    actual_structure = taflow.BreakOfStructureChangeOfCharacter(
        data["high"], data["low"], data["close"],
        swing_length=swing_length).compute()
    structure_note = (
        "package retrospectively keeps only broken structures at their pivot; "
        "taflow emits causal setup and break events and cannot retract history"
    )
    compare(rows, "smartmoneyconcepts", "break_of_structure_change_of_character",
            "bos", actual_structure[0], reference_structure["BOS"].shift(swing_length),
            note=structure_note, expected_difference=True)
    compare(rows, "smartmoneyconcepts", "break_of_structure_change_of_character",
            "choch", actual_structure[1], reference_structure["CHOCH"].shift(swing_length),
            note=structure_note, expected_difference=True)
    compare(rows, "smartmoneyconcepts", "break_of_structure_change_of_character",
            "level", actual_structure[2],
            _event_values(reference_structure, "Level", "BrokenIndex"),
            note=structure_note, expected_difference=True)
    broken_direction = np.where(
        reference_structure["BOS"].notna(), reference_structure["BOS"],
        reference_structure["CHOCH"],
    )
    broken_frame = reference_structure.copy()
    broken_frame["Direction"] = broken_direction
    compare(rows, "smartmoneyconcepts", "break_of_structure_change_of_character",
            "broken", actual_structure[3],
            _event_flags(broken_frame, "Direction", "BrokenIndex"),
            note=structure_note, expected_difference=True)

    # The upstream OB definition marks the historical lowest/highest candle
    # before a structure break and later deletes invalidated blocks.  taflow's
    # streaming definition confirms dual-scale pivots and excludes ATR outlier
    # bars, so all exposed fields are deliberately reported as variants.
    reference_ob = smc.ob(ohlcv.copy(), reference_swing, close_mitigation=False)
    actual_ob = taflow.OrderBlock(
        data["high"], data["low"], data["close"], data["volume"],
        swing_length=swing_length, internal_length=3, atr_period=14,
        threshold=2.0).compute()
    ob_note = (
        "different published definitions: package retroactively marks and deletes "
        "historical blocks; taflow is causal, dual-pivot, and ATR-filtered"
    )
    for name, got, source in zip(
            ("ob", "top", "bottom", "ob_volume"), actual_ob[:4],
            ("OB", "Top", "Bottom", "OBVolume"), strict=True):
        compare(rows, "smartmoneyconcepts", "order_block", name, got,
                reference_ob[source].shift(swing_length), note=ob_note,
                expected_difference=True)
    compare(rows, "smartmoneyconcepts", "order_block", "mitigated", actual_ob[4],
            _event_flags(reference_ob, "OB", "MitigatedIndex"), note=ob_note,
            expected_difference=True)

    # Project the package's second-touch and future sweep indices to their
    # event bars.  Its tolerance is a percentage of the full-series range;
    # taflow uses a causal percentage of each pool level.
    reference_liquidity = smc.liquidity(
        ohlcv.copy(), reference_swing, range_percent=0.01)
    actual_liquidity = taflow.Liquidity(
        data["high"], data["low"], swing_length=swing_length,
        range_percent=0.01).compute()
    liquidity_note = (
        "package uses full-series range and retroactive group starts; taflow uses "
        "causal level-relative tolerance and emits second-touch/sweep events"
    )
    compare(rows, "smartmoneyconcepts", "liquidity", "liquidity",
            actual_liquidity[0],
            _event_values(reference_liquidity, "Liquidity", "End"),
            note=liquidity_note, expected_difference=True)
    compare(rows, "smartmoneyconcepts", "liquidity", "level", actual_liquidity[1],
            _event_values(reference_liquidity, "Level", "End"),
            note=liquidity_note, expected_difference=True)
    compare(rows, "smartmoneyconcepts", "liquidity", "swept", actual_liquidity[2],
            _event_flags(reference_liquidity, "Liquidity", "Swept"),
            note=liquidity_note, expected_difference=True)

    reference_retracements = smc.retracements(ohlcv.copy(), reference_swing)
    actual_retracements = taflow.Retracements(
        data["high"], data["low"], data["close"],
        swing_length=swing_length).compute()
    retracement_note = (
        "package is lookahead-aligned, rounded, and uses candle extremes; taflow "
        "confirms swings causally and measures the current close"
    )
    for name, got, source in zip(
            ("direction", "current_retracement_pct", "deepest_retracement_pct"),
            actual_retracements,
            ("Direction", "CurrentRetracement%", "DeepestRetracement%"), strict=True):
        compare(rows, "smartmoneyconcepts", "retracements", name, got,
                reference_retracements[source].shift(swing_length),
                note=retracement_note, expected_difference=True)

    # A full-day custom session makes every 15-minute bar active.  Session
    # starts are supplied explicitly to taflow's timestamp-free Rust core.
    reference_session = smc.sessions(
        ohlcv.copy(), "Custom", "00:00", "23:59", "UTC")
    day_boundary = np.asarray(index.normalize().to_series().diff().notna()).copy()
    day_boundary[0] = True
    one_session = np.zeros(bars, dtype=bool)
    one_session[0] = True
    actual_session = taflow.Sessions(one_session, data["high"], data["low"]).compute()
    for name, got, source in zip(("active", "high", "low"), actual_session,
                                 ("Active", "High", "Low"), strict=True):
        compare(rows, "smartmoneyconcepts", "sessions", name, got,
                reference_session[source], atol=1e-5,
                note="all-day custom session; boundaries passed as flags")

    reference_previous = smc.previous_high_low(ohlcv.copy(), time_frame="1D")
    actual_previous = taflow.PreviousHighLow(
        day_boundary, data["high"], data["low"]).compute()
    for name, got, source in zip(
            ("previous_high", "previous_low", "broken_high", "broken_low"),
            actual_previous,
            ("PreviousHigh", "PreviousLow", "BrokenHigh", "BrokenLow"), strict=True):
        compare(rows, "smartmoneyconcepts", "previous_high_low", name, got,
                reference_previous[source], atol=1e-5,
                note="SMC 0.0.27 selects periods_before-2 (two-day lag); taflow exposes the immediately previous day",
                expected_difference=True)


def package_versions() -> dict[str, str]:
    return {
        name: importlib.metadata.version(name)
        for name in ("taflow", "pandas-ta-classic", "polars", "smartmoneyconcepts")
    }


def write_report(rows: list[Result], report: Path, bars: int) -> None:
    versions = package_versions()
    matched = sum(row.passed for row in rows)
    variants = sum(not row.passed and row.expected_difference for row in rows)
    failed = len(rows) - matched - variants
    lines = [
        "# External correctness oracles", "",
        f"Bars: **{bars:,}** | Matches: **{matched}** | "
        f"Documented variants: **{variants}** | Failures: **{failed}** | "
        f"rtol={RTOL}, atol={ATOL}", "",
        "Versions: " + ", ".join(f"{name} {version}" for name, version in versions.items()),
        "", "| Oracle | Function | Output | Verdict | Max error | NaN mismatches | Note |",
        "|---|---|---|---:|---:|---:|---|",
    ]
    for row in sorted(rows, key=lambda item: (item.passed, item.oracle, item.function, item.output)):
        verdict = ("MATCH" if row.passed else
                   "VARIANT" if row.expected_difference else "**FAIL**")
        error = row.error or row.note
        lines.append(f"| {row.oracle} | `{row.function}` | `{row.output}` | {verdict} | "
                     f"`{row.max_abs_error:.3e}` | {row.nan_mismatches} | {error} |")
    report.write_text("\n".join(lines) + "\n")
    report.with_suffix(".json").write_text(json.dumps({
        "bars": bars, "versions": versions, "rows": [asdict(row) for row in rows],
    }, indent=2) + "\n")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--bars", type=int, default=2_000)
    parser.add_argument("--oracle", choices=("all", "pandas-ta", "polars", "smc"),
                        default="all")
    parser.add_argument("--report", type=Path, default=HERE / "EXTERNAL_ORACLES.md")
    args = parser.parse_args()
    data = make_data(args.bars)
    rows: list[Result] = []
    runners = {
        "pandas-ta": run_pandas_ta,
        "polars": run_polars,
        "smc": run_smc,
    }
    selected = runners.values() if args.oracle == "all" else (runners[args.oracle],)
    for runner in selected:
        try:
            runner(data, rows)
        except Exception as exc:
            rows.append(Result(runner.__name__, "runner", "runner", False,
                               np.inf, 0, 0, error=f"{type(exc).__name__}: {exc}"))
    write_report(rows, args.report, args.bars)
    for row in rows:
        verdict = ("MATCH" if row.passed else
                   "VARIANT" if row.expected_difference else "FAIL")
        print(f"{row.oracle:23} {row.function:40} {row.output:18} "
              f"{verdict}")
    failed = sum(not row.passed and not row.expected_difference for row in rows)
    variants = sum(not row.passed and row.expected_difference for row in rows)
    print(f"\nwrote {args.report}: {len(rows) - failed - variants}/{len(rows)} matched, "
          f"{variants} documented variants, {failed} failures")
    return int(failed != 0)


if __name__ == "__main__":
    raise SystemExit(main())
