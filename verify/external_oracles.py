#!/usr/bin/env python3
"""Compare taflow's Python interface with non-TA-Lib external oracles.

This complements ``verify.py`` (TA-Lib plus streaming/chunk invariance) with
the three independent sources requested for extension indicators/operators:

* NumPy for pointwise mathematical transforms;
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


def run_numpy(data: dict[str, np.ndarray], rows: list[Result]) -> None:
    """Exact pointwise comparisons against documented NumPy ufuncs."""
    centered = (data["close2"] - 100.0) / 10.0
    positive = np.abs(centered) + 1.0
    unit = np.tanh(centered / 4.0)
    log_domain = np.abs(centered)
    angle = centered + 0.25

    cases = {
        "math_abs": (taflow.MathAbs(centered).compute(), np.abs(centered),
                     "numpy.abs"),
        "math_acosh": (taflow.MathAcosh(positive).compute(), np.arccosh(positive),
                       "numpy.arccosh"),
        "math_asinh": (taflow.MathAsinh(centered).compute(), np.arcsinh(centered),
                       "numpy.arcsinh"),
        "math_atanh": (taflow.MathAtanh(unit).compute(), np.arctanh(unit),
                       "numpy.arctanh"),
        "math_cbrt": (taflow.MathCbrt(centered).compute(), np.cbrt(centered),
                      "numpy.cbrt"),
        "math_cot": (taflow.MathCot(angle).compute(), 1.0 / np.tan(angle),
                     "numpy.tan reciprocal"),
        "math_degrees": (taflow.MathDegrees(angle).compute(), np.degrees(angle),
                         "numpy.degrees"),
        "math_log1p": (taflow.MathLog1p(log_domain).compute(), np.log1p(log_domain),
                       "numpy.log1p"),
        "math_radians": (taflow.MathRadians(angle).compute(), np.radians(angle),
                         "numpy.radians"),
        "signed_power": (
            taflow.SignedPower(centered, exponent=2.0).compute(),
            np.sign(centered) * np.abs(centered) ** 2.0,
            "numpy.sign/numpy.abs/numpy.power",
        ),
    }
    for function, (actual, expected, api) in cases.items():
        compare(rows, "NumPy", function, "all", actual, expected, note=api)


def run_pandas(data: dict[str, np.ndarray], rows: list[Result]) -> None:
    """Independent causal operator comparisons built from pandas windows."""
    n = 20
    close = pd.Series(data["close"])
    other = pd.Series(data["close2"])
    high, low, volume = (pd.Series(data[key]) for key in ("high", "low", "volume"))
    condition = close > other

    def check(function: str, actual, expected, api: str, *, atol: float = ATOL) -> None:
        compare(rows, "pandas", function, "all", actual, expected,
                atol=atol, note=api)

    check("lag", taflow.Lag(close, n).compute(), close.shift(n), "Series.shift")
    check("signal_delay", taflow.SignalDelay(close, n).compute(), close.shift(n),
          "Series.shift")
    check("rolling_mode", taflow.RollingMode(close, n).compute(),
          close.rolling(n).apply(lambda x: x.value_counts(sort=False).idxmax()),
          "Series.rolling.apply/value_counts")
    rank = close.rolling(n).apply(
        lambda x: ((x.iloc[:-1] < x.iloc[-1]).sum()
                   + (x == x.iloc[-1]).sum()) / len(x), raw=False)
    check("rolling_rank", taflow.RollingRank(close, n).compute(), rank,
          "Series.rolling.apply")
    check("time_series_rank", taflow.TimeSeriesRank(close, n).compute(), rank,
          "Series.rolling.apply")
    winsor = close.rolling(n).apply(
        lambda x: np.clip(x.iloc[-1], x.quantile(0.05), x.quantile(0.95)),
        raw=False)
    check("rolling_winsorize", taflow.RollingWinsorize(close, n).compute(), winsor,
          "Series.rolling.quantile/numpy.clip")

    ewm_left = close.ewm(span=n, adjust=False)
    check("ewm_cov", taflow.ExponentiallyWeightedCovariance(close, other, n).compute(),
          ewm_left.cov(other, bias=True), "ExponentialMovingWindow.cov(bias=True)",
          atol=1e-9)
    check("ewm_corr", taflow.ExponentiallyWeightedCorrelation(close, other, n).compute(),
          ewm_left.corr(other).fillna(0.0), "ExponentialMovingWindow.corr", atol=1e-9)
    check("ewm_sum", taflow.ExponentiallyWeightedSum(close, n).compute(),
          close.ewm(span=n, adjust=True).sum(), "ExponentialMovingWindow.sum")

    check("drawdown", taflow.Drawdown(close).compute(),
          close / close.cummax() - 1.0, "Series.cummax")
    mean = close.rolling(n).mean()
    std = close.rolling(n).std(ddof=0)
    check("rolling_sharpe", taflow.RollingSharpe(close, n).compute(),
          (mean / std).where(std > 0, 0.0).where(mean.notna()),
          "Rolling.mean/Rolling.std(ddof=0)", atol=1e-8)
    sortino = close.rolling(n).apply(
        lambda x: x.mean() / np.sqrt(np.mean(np.minimum(x, 0.0) ** 2))
        if np.any(x < 0.0) else 0.0, raw=True)
    check("rolling_sortino", taflow.RollingSortino(close, n).compute(), sortino,
          "Series.rolling.apply")
    calmar = close.rolling(n).apply(
        lambda x: x.mean() / -np.min(x / np.maximum.accumulate(x) - 1.0)
        if np.min(x / np.maximum.accumulate(x) - 1.0) < 0.0 else 0.0,
        raw=True)
    check("rolling_calmar", taflow.RollingCalmar(close, n).compute(), calmar,
          "Series.rolling.apply")

    check("rising", taflow.Rising(close, n).compute(),
          (close > close.shift(n)).astype(float).where(close.shift(n).notna()),
          "Series.shift comparison")
    check("falling", taflow.Falling(close, n).compute(),
          (close < close.shift(n)).astype(float).where(close.shift(n).notna()),
          "Series.shift comparison")
    relations = {
        "higher_high": (taflow.HigherHigh(high, low).compute(), high > high.shift()),
        "lower_low": (taflow.LowerLow(high, low).compute(), low < low.shift()),
        "inside_bar": (taflow.InsideBar(high, low).compute(),
                       (high < high.shift()) & (low > low.shift())),
        "outside_bar": (taflow.OutsideBar(high, low).compute(),
                        (high > high.shift()) & (low < low.shift())),
        "gap_up": (taflow.GapUp(high, low).compute(), low > high.shift()),
        "gap_down": (taflow.GapDown(high, low).compute(), high < low.shift()),
    }
    for function, (actual, expected) in relations.items():
        check(function, actual, expected.astype(float).where(high.shift().notna()),
              "Series.shift comparison")

    groups = condition.astype(int).cumsum()
    check("bars_since", taflow.BarsSince(condition).compute(),
          condition.groupby(groups).cumcount().astype(float), "Series.groupby.cumcount")
    check("value_when", taflow.ValueWhen(condition, close).compute(),
          close.where(condition).ffill(), "Series.where/ffill")
    check("highest_since", taflow.HighestSince(condition, close).compute(),
          close.groupby(groups).cummax(), "Series.groupby.cummax")
    check("lowest_since", taflow.LowestSince(condition, close).compute(),
          close.groupby(groups).cummin(), "Series.groupby.cummin")

    covariance = close.rolling(n).cov(other, ddof=0)
    variance_x = close.rolling(n).var(ddof=0)
    variance_benchmark = other.rolling(n).var(ddof=0)
    beta = covariance / variance_x
    check("hedge_ratio", taflow.HedgeRatio(close, other, n).compute(),
          beta.where(variance_x > 0, 0.0).where(variance_x.notna()),
          "Rolling.cov/Rolling.var(ddof=0)", atol=2e-9)
    autocorr = close.rolling(n).apply(
        lambda x: pd.Series(x[:-1]).corr(pd.Series(x[1:])), raw=True)
    check("rolling_autocorr", taflow.RollingAutocorr(close, n).compute(), autocorr,
          "Series.rolling.apply/Series.corr", atol=1e-9)
    beta_benchmark = covariance / variance_benchmark
    alpha = close.rolling(n).mean() - beta_benchmark * other.rolling(n).mean()
    check("rolling_alpha", taflow.RollingAlpha(close, other, n).compute(), alpha,
          "Rolling.cov/Rolling.var/Rolling.mean", atol=1e-9)
    active = close - other
    information = active.rolling(n).mean() / active.rolling(n).std(ddof=0)
    check("rolling_information_ratio",
          taflow.RollingInformationRatio(close, other, n).compute(), information,
          "Rolling.mean/Rolling.std(ddof=0)", atol=1e-9)
    entropy = close.round(1).rolling(n).apply(
        lambda x: -(x.value_counts(normalize=True) *
                    np.log(x.value_counts(normalize=True))).sum(), raw=False)
    check("rolling_entropy", taflow.RollingEntropy(close.round(1), n).compute(), entropy,
          "Series.rolling.apply/value_counts")

    check("average_daily_dollar_value",
          taflow.AverageDailyDollarValue(close, volume, n).compute(),
          (close * volume).rolling(n).mean(), "Series.rolling.mean")
    typical = (high + low + close) / 3.0
    check("rolling_vwap",
          taflow.RollingVolumeWeightedAveragePrice(high, low, close, volume, n).compute(),
          (typical * volume).rolling(n).sum() / volume.rolling(n).sum(),
          "Series.rolling.sum")
    check("cumulative_count", taflow.CumulativeCount(close).compute(),
          pd.Series(np.arange(1, len(close) + 1, dtype=float)), "Series.size/arange")
    weights = np.arange(1, n + 1, dtype=float)
    check("decay_linear", taflow.DecayLinear(close, n).compute(),
          close.rolling(n).apply(lambda x: np.dot(x, weights) / weights.sum(), raw=True),
          "Series.rolling.apply/numpy.dot")

    # Published volatility and market-microstructure definitions, expressed
    # independently with pandas rolling windows.
    open_ = pd.Series(data["open"])
    log_return = np.log(close / close.shift())
    check("close_to_close_sigma", taflow.CloseToCloseSigma(close, n).compute(),
          log_return.rolling(n).std(ddof=0), "Series.rolling.std(log returns)")
    park_term = np.log(high / low) ** 2 / (4.0 * np.log(2.0))
    check("parkinson", taflow.Parkinson(high, low, n).compute(),
          np.sqrt(park_term.rolling(n).mean()), "Parkinson estimator via Rolling.mean")
    gk_term = (0.5 * np.log(high / low) ** 2
               - (2.0 * np.log(2.0) - 1.0) * np.log(close / open_) ** 2)
    check("garman_klass", taflow.GarmanKlass(open_, high, low, close, n).compute(),
          np.sqrt(gk_term.rolling(n).mean()), "Garman-Klass via Rolling.mean")
    rs_term = (np.log(high / close) * np.log(high / open_)
               + np.log(low / close) * np.log(low / open_))
    check("rogers_satchell", taflow.RogersSatchell(open_, high, low, close, n).compute(),
          np.sqrt(rs_term.rolling(n).mean()), "Rogers-Satchell via Rolling.mean")
    overnight = np.log(open_ / close.shift()) ** 2
    check("garman_klass_yang_zhang",
          taflow.GarmanKlassYangZhang(open_, high, low, close, n).compute(),
          np.sqrt((gk_term + overnight).rolling(n).mean()),
          "Garman-Klass-Yang-Zhang via Rolling.mean")
    k = 0.34 / (1.34 + (n + 1.0) / (n - 1.0))
    yz = np.sqrt((overnight.rolling(n).mean()
                  + k * (np.log(close / open_) ** 2).rolling(n).mean()
                  + (1.0 - k) * rs_term.rolling(n).mean()).clip(lower=0.0))
    check("yang_zhang", taflow.YangZhang(open_, high, low, close, n).compute(), yz,
          "Yang-Zhang via Rolling.mean")

    amihud = (close.pct_change(fill_method=None).abs() / (close * volume)).rolling(n).mean()
    check("amihud", taflow.Amihud(close, volume, n).compute(), amihud,
          "Series.pct_change/rolling.mean")
    delta = close.diff()
    previous_delta = delta.shift()
    previous_delta.iloc[1] = 0.0
    roll_cov = delta.rolling(n).cov(previous_delta, ddof=1)
    check("roll_spread", taflow.RollSpread(close, n).compute(),
          2.0 * np.sqrt((-roll_cov).clip(lower=0.0)),
          "Series.diff/Rolling.cov(ddof=1)", atol=1e-8)

    spread_values = np.full(len(close), np.nan)
    x_values, y_values = close.to_numpy(), other.to_numpy()
    for index in range(n - 1, len(close)):
        xw = x_values[index - n + 1:index + 1]
        yw = y_values[index - n + 1:index + 1]
        vx = np.mean((xw - xw.mean()) ** 2)
        beta_value = (np.mean((xw - xw.mean()) * (yw - yw.mean())) / vx
                      if vx > 0 else 0.0)
        window_spread = yw - beta_value * xw
        spread_std = window_spread.std(ddof=0)
        spread_values[index] = ((window_spread[-1] - window_spread.mean()) / spread_std
                                if spread_std > 0 else 0.0)
    check("spread_zscore", taflow.SpreadZScore(close, other, n).compute(), spread_values,
          "numpy rolling OLS/z-score", atol=1e-8)

    lagged = close.shift()
    change = close.diff()
    covariance_ou = change.rolling(n).cov(lagged, ddof=1)
    variance_ou = lagged.rolling(n).var(ddof=1)
    mean_reversion = -covariance_ou / variance_ou
    half_life = (np.log(2.0) / mean_reversion).where(mean_reversion > 0.0)
    check("ornstein_uhlenbeck_half_life",
          taflow.OrnsteinUhlenbeckHalfLife(close, n).compute(), half_life,
          "Rolling.cov/Rolling.var OU regression", atol=1e-8)

    changes = close.diff().fillna(0.0).to_numpy()
    cusum = np.zeros(len(close))
    positive = negative = 0.0
    for index, change_value in enumerate(changes):
        positive = max(0.0, positive + change_value)
        negative = max(0.0, negative - change_value)
        if positive > 1.0:
            positive, cusum[index] = 0.0, 1.0
        elif negative > 1.0:
            negative, cusum[index] = 0.0, -1.0
    check("cumulative_sum_control_chart",
          taflow.CumulativeSumControlChart(changes, 1.0).compute(), cusum,
          "NumPy implementation of AFML CUSUM")

    average_high = high.rolling(10).mean()
    average_low = low.rolling(10).mean()
    ssl_low = np.full(len(close), np.nan)
    ssl_high = np.full(len(close), np.nan)
    side = 1
    for index in range(9, len(close)):
        if close.iloc[index] > average_high.iloc[index]:
            side = 1
        elif close.iloc[index] < average_low.iloc[index]:
            side = -1
        if side > 0:
            ssl_low[index], ssl_high[index] = average_low.iloc[index], average_high.iloc[index]
        else:
            ssl_low[index], ssl_high[index] = average_high.iloc[index], average_low.iloc[index]
    ssl_actual = taflow.SmoothedTrendChannel(high, low, close, 10).compute()
    compare(rows, "pandas", "ssl_channel", "lower", ssl_actual[0], ssl_low,
            note="Rolling.mean SSL recurrence")
    compare(rows, "pandas", "ssl_channel", "upper", ssl_actual[1], ssl_high,
            note="Rolling.mean SSL recurrence")

    # Anchored and rolling level operators.
    anchor = np.asarray(data["anchor"], dtype=bool)
    anchor_groups = pd.Series(anchor, index=close.index).cumsum()
    anchored_typical = (high + low + close) / 3.0
    anchored_volume = volume.groupby(anchor_groups, sort=False).cumsum()
    anchored_weighted = (anchored_typical * volume).groupby(
        anchor_groups, sort=False).cumsum()
    anchored_weighted_square = (anchored_typical * anchored_typical * volume).groupby(
        anchor_groups, sort=False).cumsum()
    anchored_center = anchored_weighted / anchored_volume
    anchored_variance = (
        anchored_weighted_square / anchored_volume - anchored_center * anchored_center
    ).clip(lower=0.0)
    anchored_deviation = anchored_variance.pow(0.5)
    anchored = (
        anchored_center,
        anchored_center + anchored_deviation,
        anchored_center - anchored_deviation,
    )
    anchored_actual = taflow.AnchoredVolumeWeightedAveragePrice(
        high, low, close, volume, anchor, 1.0).compute()
    for output, actual, expected in zip(("vwap", "upper", "lower"),
                                        anchored_actual, anchored, strict=True):
        compare(rows, "pandas", "anchored_vwap", output, actual, expected,
                atol=1e-8, note="pandas grouped cumulative weighted moments")

    fib_high = close.rolling(120, min_periods=1).max()
    fib_low = close.rolling(120, min_periods=1).min()
    fib_span = fib_high - fib_low
    fib_actual = taflow.FibonacciRetracement(close, 120).compute()
    for ratio, actual in zip((0.0, 0.236, 0.382, 0.5, 0.618, 0.786, 1.0),
                             fib_actual, strict=True):
        compare(rows, "pandas", "fibonacci_retracement", f"{ratio:g}", actual,
                fib_high - fib_span * ratio, note="Rolling.min/Rolling.max")

    premium_high = close.rolling(n, min_periods=1).max()
    premium_low = close.rolling(n, min_periods=1).min()
    equilibrium = (premium_high + premium_low) / 2.0
    zone = pd.Series(np.select((close > equilibrium, close < equilibrium),
                               (1.0, -1.0), default=0.0))
    premium_actual = taflow.PremiumDiscount(close, n).compute()
    compare(rows, "pandas", "premium_discount", "zone", premium_actual[0], zone,
            note="Rolling.min/Rolling.max")
    compare(rows, "pandas", "premium_discount", "equilibrium", premium_actual[1],
            equilibrium, note="Rolling.min/Rolling.max")

    pivot_expected = [np.full(len(close), np.nan) for _ in range(5)]
    previous_high = previous_low = previous_close = None
    for index in range(len(close)):
        if anchor[index]:
            if previous_high is not None:
                pivot = (previous_high + previous_low + previous_close) / 3.0
                bar_range = previous_high - previous_low
                levels = (pivot, 2 * pivot - previous_low, 2 * pivot - previous_high,
                          pivot - bar_range, pivot + bar_range)
                for output, value in zip(pivot_expected, levels):
                    output[index:] = value
            previous_high, previous_low, previous_close = high[index], low[index], close[index]
        else:
            previous_high = max(previous_high, high[index])
            previous_low = min(previous_low, low[index])
            previous_close = close[index]
    pivot_actual = taflow.PivotPoints(high, low, close, anchor).compute()
    for name, actual, expected in zip(("pivot", "r1", "s1", "s2", "r2"),
                                      pivot_actual, pivot_expected, strict=True):
        compare(rows, "pandas", "pivot_points", name, actual, expected,
                note="NumPy anchored OHLC pivot definition")

    opening_expected = [np.empty(len(close)), np.empty(len(close)), np.empty(len(close))]
    opening_count, opening_high, opening_low = 0, -np.inf, np.inf
    for index in range(len(close)):
        if anchor[index]:
            opening_count, opening_high, opening_low = 0, -np.inf, np.inf
        if opening_count < 30:
            opening_high = max(opening_high, high[index])
            opening_low = min(opening_low, low[index])
            opening_count += 1
        opening_expected[0][index] = opening_high
        opening_expected[1][index] = opening_low
        opening_expected[2][index] = (1.0 if close[index] > opening_high else
                                      -1.0 if close[index] < opening_low else 0.0)
    opening_actual = taflow.OpeningRange(high, low, close, anchor, 30).compute()
    for name, actual, expected in zip(("high", "low", "breakout"), opening_actual,
                                      opening_expected, strict=True):
        compare(rows, "pandas", "opening_range", name, actual, expected,
                note="NumPy anchored opening-range definition")

    # Fixed-width fractional differentiation from AFML.
    weights_fd = [1.0]
    order, threshold = 0.5, 1e-5
    while True:
        index = len(weights_fd)
        weight = -weights_fd[-1] * (order - index + 1.0) / index
        if abs(weight) < threshold:
            break
        weights_fd.append(weight)
    frac_expected = np.full(len(close), np.nan)
    width = len(weights_fd)
    for index in range(width - 1, len(close)):
        window = close.to_numpy()[index - width + 1:index + 1]
        frac_expected[index] = np.dot(weights_fd, window[::-1])
    check("frac_diff", taflow.FracDiff(close, order, threshold).compute(), frac_expected,
          "NumPy AFML fixed-width fractional differentiation", atol=1e-8)

    # Wilder-smoothed Relative Momentum Index.
    momentum, period = 5, 14
    rmi_expected = np.full(len(close), np.nan)
    gains = losses = 0.0
    count = 0
    close_values = close.to_numpy()
    for index in range(momentum, len(close)):
        movement = close_values[index] - close_values[index - momentum]
        gain, loss = max(movement, 0.0), max(-movement, 0.0)
        count += 1
        if count <= period:
            gains += gain
            losses += loss
            if count < period:
                continue
        else:
            gains = (gains * (period - 1.0) + gain) / period
            losses = (losses * (period - 1.0) + loss) / period
        rmi_expected[index] = 50.0 if gains + losses == 0 else 100.0 * gains / (gains + losses)
    check("rmi", taflow.RelativeMomentumIndex(close, period, momentum).compute(), rmi_expected,
          "NumPy Wilder-smoothed momentum")

    hurst = close.rolling(n).apply(
        lambda x: (np.clip(
            np.log((np.cumsum(x - x.mean()).max()
                    - np.cumsum(x - x.mean()).min()) / x.std(ddof=0)) / np.log(len(x)),
            0.0, 1.0)
            if x.std(ddof=0) > 0.0 else 0.5), raw=True)
    check("hurst", taflow.Hurst(close, n).compute(), hurst,
          "pandas Rolling.apply rescaled-range estimator", atol=1e-8)
    check("fractal_dimension", taflow.FractalDimension(close, n).compute(), 2.0 - hurst,
          "two minus pandas rescaled-range Hurst", atol=1e-8)

    # Independent two-state Kalman filter using NumPy scalar algebra.
    alpha_value, beta_value = 0.0, 1.0
    p_aa, p_ab, p_bb = 1.0, 0.0, 1.0
    delta_k, observation_variance = 1e-4, 1e-3
    kalman = [np.empty(len(close)) for _ in range(4)]
    for index, (x_value, y_value) in enumerate(zip(close, other)):
        predicted_aa, predicted_ab, predicted_bb = p_aa + delta_k, p_ab, p_bb + delta_k
        innovation = y_value - (alpha_value + beta_value * x_value)
        innovation_variance = (predicted_aa + 2.0 * predicted_ab * x_value
                               + predicted_bb * x_value * x_value
                               + observation_variance)
        gain_alpha = (predicted_aa + predicted_ab * x_value) / innovation_variance
        gain_beta = (predicted_ab + predicted_bb * x_value) / innovation_variance
        alpha_value += gain_alpha * innovation
        beta_value += gain_beta * innovation
        p_aa = ((1.0 - gain_alpha) * predicted_aa
                - gain_alpha * x_value * predicted_ab)
        p_ab = ((1.0 - gain_alpha) * predicted_ab
                - gain_alpha * x_value * predicted_bb)
        p_bb = -gain_beta * predicted_ab + (1.0 - gain_beta * x_value) * predicted_bb
        kalman[0][index], kalman[1][index] = beta_value, alpha_value
        kalman[2][index], kalman[3][index] = innovation, np.sqrt(innovation_variance)
    kalman_actual = taflow.KalmanHedgeRatio(
        close, other, delta_k, observation_variance).compute()
    compare(rows, "pandas", "kalman_hedge_ratio", "beta", kalman_actual, kalman[0],
            atol=1e-9, note="NumPy two-state Kalman filter_update")

    # Fixed-bin anchored volume profile.
    bins, value_area = 24, 0.7
    profile = [np.empty(len(close)) for _ in range(3)]
    histogram = np.zeros(bins)
    session_low = None
    session_high = 0.0
    step = 1.0
    for index in range(len(close)):
        if anchor[index] or session_low is None:
            session_low = low.iloc[index]
            session_high = high.iloc[index]
            step = max((session_high - session_low) / bins, 1e-12)
            histogram.fill(0.0)
        session_low = min(session_low, low.iloc[index])
        session_high = max(session_high, high.iloc[index])
        bin_index = int(np.clip(int((close.iloc[index] - session_low) / step), 0, bins - 1))
        histogram[bin_index] += volume.iloc[index]
        poc = int(np.argmax(histogram))
        target = histogram.sum() * value_area
        left = right = poc
        accumulated = histogram[poc]
        while accumulated < target and (left > 0 or right + 1 < bins):
            if left == 0:
                right += 1
            elif right + 1 == bins:
                left -= 1
            elif histogram[left - 1] >= histogram[right + 1]:
                left -= 1
            else:
                right += 1
            accumulated = histogram[left:right + 1].sum()
        profile[0][index] = (poc + 0.5) * step + session_low
        profile[1][index] = (right + 0.5) * step + session_low
        profile[2][index] = (left + 0.5) * step + session_low
    profile_actual = taflow.SessionVolumeLevels(
        high, low, close, volume, anchor, bins, value_area).compute()
    for name, actual, expected in zip(("poc", "vah", "val"), profile_actual,
                                      profile, strict=True):
        compare(rows, "pandas", "session_volume_levels", name, actual, expected,
                atol=1e-8, note="NumPy fixed-bin anchored volume profile")


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
        "math_abs": pl.col("other").sub(100.0).abs(),
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
        "median": taflow.RollingMedian(close, n).compute(),
        "quantile": taflow.RollingQuantile(close, n, q).compute(),
        "variance": taflow.RollingVariance(close, timeperiod=n).compute(),
        "stddev": taflow.RollingStandardDeviation(close, timeperiod=n).compute(),
        "skew": taflow.RollingSkew(standardized, n).compute(),
        "kurtosis": taflow.RollingKurtosis(standardized, n).compute(),
        "covariance": taflow.RollingCov(close, other, n).compute(),
        "correlation": taflow.RollingCorrelation(close, other, timeperiod=n).compute(),
        "ewm_variance": taflow.ExponentiallyWeightedVariance(close, n).compute(),
        "ewm_stddev": taflow.ExponentiallyWeightedStandardDeviation(close, n).compute(),
        "cumulative_sum": taflow.CumulativeSum(close).compute(),
        "cumulative_product": taflow.CumulativeProduct(close / 100.0).compute(),
        "cumulative_minimum": taflow.CumulativeMinimum(close).compute(),
        "cumulative_maximum": taflow.CumulativeMaximum(close).compute(),
        "math_abs": taflow.MathAbs(other - 100.0).compute(),
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

    # These families intentionally pin TAFlow's causal/initialization
    # convention. The pandas-ta result is still executed and quantified so a
    # semantic difference remains visible instead of falling back to a native
    # self-check.
    variant = {
        "expected_difference": True,
        "note": "independently compared; documented initialization/formula convention differs",
    }
    many("arnaud_legoux_moving_average",
         taflow.ArnaudLegouxMovingAverage(close, 10, 0.85, 6.0).compute(),
         pta.alma(c, length=10, distribution_offset=0.85, sigma=6.0),
         ("alma",), **variant)
    many("true_strength_index", taflow.TrueStrengthIndex(close, 13, 25).compute(),
         pta.tsi(c, fast=13, slow=25).iloc[:, 0], ("tsi",), **variant)
    kc = pta.kc(h, l, c, length=20, scalar=2.0, mamode="ema", tr=True)
    many("keltner_channels", taflow.KeltnerChannels(high, low, close, 20, 2.0).compute(),
         kc.iloc[:, [2, 1, 0]], ("upper", "middle", "lower"), **variant)
    many("chaikin_volatility", taflow.ChaikinVolatility(high, low, 10, 10).compute(),
         pta.cvi(h, l, length=10), ("chaikin_volatility",), **variant)
    many("ulcer_index", taflow.UlcerIndex(close, 14).compute(),
         pta.ui(c, length=14), ("ulcer_index",), **variant)
    many("ease_of_movement", taflow.EaseOfMovement(high, low, volume).compute(),
         pta.eom(h, l, c, v, length=1), ("ease_of_movement",), **variant)
    many("volume_price_trend", taflow.VolumePriceTrend(close, volume).compute(),
         pta.pvt(c, v), ("volume_price_trend",), **variant)
    many("parabolic_moving_average_stop",
         taflow.ParabolicMovingAverageStop(high, low, close, 10, 3.0).compute()[0],
         pta.pmax(h, l, c, length=10, multiplier=3.0), ("stop",), **variant)

    many("awesome_oscillator", taflow.AwesomeOscillator(high, low).compute(),
         pta.ao(h, l, fast=5, slow=34), ("ao",))
    many("log_return", taflow.LogReturn(close, 1).compute(),
         pta.log_return(c, length=1), ("log_return",))
    many("force_index", taflow.ForceIndex(close, volume).compute(),
         pta.efi(c, v, length=1, mamode="ema"), ("force_index",),
         note="taflow exposes the unsmoothed one-bar force; pandas-ta EFI length=1")
    many("crossover", taflow.Crossover(close, data["close2"]).compute(),
         pta.cross(c, c2, above=True), ("crossover",))
    many("crossunder", taflow.Crossunder(close, data["close2"]).compute(),
         pta.cross(c, c2, above=False), ("crossunder",))
    many("hull_moving_average", taflow.HullMovingAverage(close, 10).compute(),
         pta.hma(c, length=10), ("hma",))
    many("volume_weighted_moving_average",
         taflow.VolumeWeightedMovingAverage(close, volume, 10).compute(),
         pta.vwma(c, v, length=10), ("vwma",))
    many("zero_lag_exponential_moving_average",
         taflow.ZeroLagExponentialMovingAverage(close, 10).compute(),
         pta.zlma(c, length=10, mamode="ema", talib=False), ("zlema",),
         note="force pandas-ta's native EMA; TA-Lib rejects leading ZLMA NaNs")
    for length in (1, 2, 21):
        many("zero_lag_exponential_moving_average",
             taflow.ZeroLagExponentialMovingAverage(close, length).compute(),
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
    vidya_cases = {
        "constant": np.full(64, 42.0),
        "monotonic": np.linspace(10.0, 90.0, 64),
        "repeated": np.resize(np.array([10.0, 12.0, 12.0, 9.0, 9.0, 12.0]), 64),
        "minimum": np.linspace(5.0, 18.0, 14),
    }
    for case, values in vidya_cases.items():
        many("variable_index_dynamic_average",
             taflow.VariableIndexDynamicAverage(values, 14).compute(),
             pta.vidya(pd.Series(values), length=14),
             (f"vidya[{case}]",), note="required source-shape matrix")
    many("laguerre_relative_strength_index",
         taflow.LaguerreRelativeStrengthIndex(close, 0.5).compute(),
         pta.lrsi(c, length=14, gamma=0.5), ("lrsi",))
    for gamma in (0.1, 0.25, 0.9):
        many("laguerre_relative_strength_index",
             taflow.LaguerreRelativeStrengthIndex(close, gamma).compute(),
             pta.lrsi(c, length=14, gamma=gamma),
             (f"lrsi[gamma={gamma}]",))
    laguerre_cases = {
        "constant": np.full(64, 42.0),
        "monotonic": np.linspace(10.0, 90.0, 64),
        "repeated": np.resize(np.array([10.0, 12.0, 12.0, 9.0, 9.0, 12.0]), 64),
        "minimum": np.array([17.0]),
    }
    for case, values in laguerre_cases.items():
        many("laguerre_relative_strength_index",
             taflow.LaguerreRelativeStrengthIndex(values, 0.5).compute(),
             pta.lrsi(pd.Series(values), length=1, gamma=0.5),
             (f"lrsi[{case}]",), note="required source-shape matrix")
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

    equal_actual = taflow.EqualHighsLows(
        data["high"], data["low"], data["close"], eq_len=3,
        atr_period=200, eq_threshold=0.1).compute()
    equal_note = (
        "SMC liquidity pools are the external equal-high/low analogue; "
        "TAFlow emits causal ATR-thresholded confirmations"
    )
    compare(rows, "smartmoneyconcepts", "equal_highs_lows", "equal_high",
            equal_actual[0], _event_values(reference_liquidity, "Liquidity", "End"),
            note=equal_note, expected_difference=True)
    compare(rows, "smartmoneyconcepts", "equal_highs_lows", "equal_low",
            equal_actual[1], _event_values(reference_liquidity, "Liquidity", "End"),
            note=equal_note, expected_difference=True)
    compare(rows, "smartmoneyconcepts", "equal_highs_lows", "level",
            equal_actual[2], _event_values(reference_liquidity, "Level", "End"),
            note=equal_note, expected_difference=True)

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
        for name in ("taflow", "numpy", "pandas-ta-classic", "polars",
                     "smartmoneyconcepts")
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
    parser.add_argument("--oracle", choices=("all", "numpy", "pandas", "pandas-ta", "polars", "smc"),
                        default="all")
    parser.add_argument("--report", type=Path, default=HERE / "EXTERNAL_ORACLES.md")
    args = parser.parse_args()
    data = make_data(args.bars)
    rows: list[Result] = []
    runners = {
        "numpy": run_numpy,
        "pandas": run_pandas,
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
