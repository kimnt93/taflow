#!/usr/bin/env python3
"""Verify every canonical TAFlow class against TA-Lib or Wickra.

taflow's canonical CamelCase classes are driven through the shared
``registry`` (TA-Lib names mapped via /CHECK.md). Protocol per function:

  1. The registry selects TA-Lib first and Wickra only when TA-Lib has no
     equivalent. Classes unsupported by both are reported explicitly.
  2. taflow cold state (construct + ``extend``) computes the full series
     -> compared to the oracle.
  3. The state is fed the first 9,000 bars with ``extend`` and continued
     bar-by-bar with ``append`` over the last 1,000 bars. The stitched
     output is compared to the taflow batch result (bitwise —
     chunk-invariance contract) and to the oracle.

Usage:
  uv run python scripts/verification/correctness.py
  uv run python scripts/verification/correctness.py EMA ATR
"""

from __future__ import annotations

import argparse
import datetime as _dt
import inspect
import json
import importlib.metadata
import platform
import sys
import traceback
from pathlib import Path

import numpy as np

try:
    from .registry import (
        CORRECTNESS_EVIDENCE_DIR,
        VERIFY_DIR,
        Spec,
        build_registry,
        constructor_value,
        make_data,
        resolve_specs,
    )
except ImportError:  # Support direct execution from this directory.
    from registry import (
        CORRECTNESS_EVIDENCE_DIR,
        VERIFY_DIR,
        Spec,
        build_registry,
        constructor_value,
        make_data,
        resolve_specs,
    )

RTOL = 1e-8
ATOL = 1e-10
CHUNK_SIZES = (1, 10, 1_000)


# ---------------------------------------------------------------------------
# Comparison helpers
# ---------------------------------------------------------------------------


def as_tuple(result) -> tuple:
    return result if isinstance(result, tuple) else (result,)


def compare(
    actual,
    expected,
    actual_indices: tuple[int, ...] | None = None,
    expected_indices: tuple[int, ...] | None = None,
    rtol: float = RTOL,
    atol: float = ATOL,
) -> dict:
    actuals, expecteds = as_tuple(actual), as_tuple(expected)
    if actual_indices is not None:
        actuals = tuple(actuals[index] for index in actual_indices)
    if expected_indices is not None:
        expecteds = tuple(expecteds[index] for index in expected_indices)
    if len(actuals) != len(expecteds):
        return {
            "passed": False,
            "error": f"output arity {len(actuals)} != {len(expecteds)}",
        }
    nan_mismatches, infinity_mismatches, max_err = 0, 0, 0.0
    for a, b in zip(actuals, expecteds):
        a = np.asarray(a, dtype=np.float64)
        b = np.asarray(b, dtype=np.float64)
        if a.shape != b.shape:
            return {"passed": False, "error": f"shape {a.shape}!={b.shape}"}
        nan_a, nan_b = np.isnan(a), np.isnan(b)
        nan_mismatches += int((nan_a != nan_b).sum())
        inf_a, inf_b = np.isinf(a), np.isinf(b)
        infinity_mismatches += int(
            (
                (inf_a != inf_b) | (inf_a & inf_b & (np.signbit(a) != np.signbit(b)))
            ).sum()
        )
        both = np.isfinite(a) & np.isfinite(b)
        if both.any():
            max_err = max(max_err, float(np.max(np.abs(a[both] - b[both]))))
    passed = (
        nan_mismatches == 0
        and infinity_mismatches == 0
        and all(
            np.allclose(
                np.asarray(x, dtype=np.float64),
                np.asarray(y, dtype=np.float64),
                rtol=rtol,
                atol=atol,
                equal_nan=True,
            )
            for x, y in zip(actuals, expecteds)
        )
    )
    return {
        "passed": passed,
        "nan_mismatches": nan_mismatches,
        "infinity_mismatches": infinity_mismatches,
        "max_abs_error": max_err,
    }


def bitwise_equal(actual, expected) -> bool:
    return all(
        np.array_equal(np.asarray(a), np.asarray(b), equal_nan=True)
        for a, b in zip(as_tuple(actual), as_tuple(expected))
    )


def continue_series(spec: Spec, arrays, split: int) -> tuple:
    """extend(first split bars) + append(rest) -> stitched full series."""
    state = spec.new_state()
    head = Spec.extend(state, [a[:split] for a in arrays])
    bars = list(zip(*[a[split:].tolist() for a in arrays]))
    outputs = [Spec.append_value(state, bar) for bar in bars]
    if len(head) == 1 and np.asarray(head[0]).ndim == 2:
        width = np.asarray(head[0]).shape[1]
        tail = np.asarray(
            [
                [float("nan")] * width if output is None else output
                for output in outputs
            ],
            dtype=np.float64,
        ).reshape((-1, width))
        return (np.concatenate([np.asarray(head[0]), tail], axis=0),)
    arity = len(head)
    columns: list[list[float]] = [[] for _ in range(arity)]
    for out in outputs:
        values = (
            out
            if isinstance(out, tuple)
            else ((None,) * arity if out is None else (out,))
        )
        for column, value in zip(columns, values):
            column.append(float("nan") if value is None else float(value))
    return tuple(
        np.concatenate([np.asarray(h, dtype=np.float64), np.asarray(c)])
        for h, c in zip(head, columns)
    )


def chunked_series(spec: Spec, arrays, chunk: int) -> tuple:
    """Feed a complete history through repeated native ``extend`` calls."""
    state = spec.new_state()
    for start in range(0, len(arrays[0]), chunk):
        stop = min(start + chunk, len(arrays[0]))
        result = state.extend(*[array[start:stop] for array in arrays])
        if result is not state:
            # Canonical adapters are fluent and return themselves. Retain a
            # defensive fallback for a value-returning adapter.
            return result if isinstance(result, tuple) else (result,)
    result = state.compute()
    return result if isinstance(result, tuple) else (result,)


# ---------------------------------------------------------------------------
# Oracles
# ---------------------------------------------------------------------------


def talib_oracle(spec: Spec, arrays):
    import talib
    from talib import abstract

    params = dict(abstract.Function(spec.talib_name).info["parameters"])
    return getattr(talib, spec.talib_name)(*arrays, **params)


def wickra_oracle(spec: Spec, arrays):
    """Execute the explicit Wickra binding registered for ``spec``."""
    import wickra

    binding = spec.wickra
    if binding is None:
        raise LookupError(f"no Wickra binding for {spec.cls.__name__}")
    oracle_class = getattr(wickra, binding.name)
    taflow_parameters = inspect.signature(spec.cls.__init__).parameters
    series_names = {name.replace("_", "").lower() for name in spec.series_args}
    synonyms = {
        "period": ("timeperiod", "period"),
        "window": ("period", "timeperiod"),
        "atr_period": ("timeperiod",),
        "tenkan_period": ("tenkan",),
        "kijun_period": ("kijun",),
        "senkou_b_period": ("senkou",),
        "ema_period": ("timeperiod",),
        "observation_var": ("observation_variance",),
        "max_lag": ("lag",),
        "lp_period": ("low_period",),
        "hp_period": ("high_period",),
        "long": ("slow",),
        "short": ("fast",),
        "schaff_period": ("tclength",),
    }
    if binding.name in {"McGinleyDynamic", "VIDYA", "JMA"}:
        synonyms["period"] = ("length", "timeperiod", "period")
    kwargs = {}
    for name, parameter in inspect.signature(oracle_class).parameters.items():
        candidates = (name, *synonyms.get(name, ()))
        target = next(
            (
                candidate
                for candidate in candidates
                if candidate in taflow_parameters
                and candidate.replace("_", "").lower() not in series_names
            ),
            None,
        )
        if target is not None:
            kwargs[name] = constructor_value(spec, target)
        elif parameter.default is inspect.Parameter.empty:
            raise TypeError(
                f"cannot map required Wickra parameter {name!r} for {spec.cls.__name__}"
            )
    oracle = oracle_class(**kwargs)
    batch_arrays = list(arrays)
    if binding.cross_section:
        batch_arrays = cross_section_oracle_arrays(binding.cross_section, arrays)
    elif binding.input_mode == "high_low_midpoint":
        batch_arrays = [(np.asarray(arrays[0]) + np.asarray(arrays[1])) * 0.5]
    elif binding.input_mode == "swap_pair":
        batch_arrays = [arrays[1], arrays[0]]
    elif binding.input_mode == "trade_pair":
        batch_arrays = [arrays[0], arrays[1], np.ones(len(arrays[0]), dtype=np.bool_)]
    elif binding.input_mode == "triple_close":
        batch_arrays = [arrays[0], arrays[0], arrays[0]]
    for index, name in enumerate(spec.series_args):
        if name == "timestamp":
            # TAFlow exposes Unix nanoseconds; Wickra Candle uses milliseconds.
            batch_arrays[index] = np.asarray(batch_arrays[index]) // 1_000_000
    if binding.prepend_zero_close:
        batch_arrays.insert(0, np.zeros_like(arrays[0]))
    result = oracle.batch(*batch_arrays)
    if binding.name == "TDSequential":
        matrix = np.asarray(result.tolist(), dtype=np.float64)
        setup = np.nan_to_num(matrix[:, 0], nan=0.0)
        return np.maximum(setup, 0.0), np.maximum(-setup, 0.0)
    if hasattr(result, "tolist") and hasattr(result, "shape"):
        matrix = np.asarray(result.tolist(), dtype=np.float64)
        if matrix.ndim == 2:
            if spec.cls.__name__ in {
                "TimeOfDayReturnProfile",
                "DayOfWeekReturnProfile",
                "IntradayVolatilityProfile",
                "VolumeByTimeProfile",
            }:
                return matrix
            return tuple(matrix[:, index] for index in range(matrix.shape[1]))
    return result


def pandas_ta_oracle(spec: Spec, arrays):
    """Execute an explicitly registered pandas-ta-classic batch reference."""
    import pandas as pd
    import pandas_ta_classic as pandas_ta

    binding = spec.pandas_ta
    if binding is None:
        raise LookupError(f"no pandas-ta-classic binding for {spec.cls.__name__}")
    function = getattr(pandas_ta, binding.name)
    constructor = inspect.signature(spec.cls.__init__).parameters
    aliases = {
        "length": ("length", "timeperiod", "period"),
        "signal": ("signal",),
        "tclength": ("tclength",),
        "fast": ("fast",),
        "slow": ("slow",),
        "factor": ("factor",),
        "phase": ("phase",),
        "multiplier": ("multiplier",),
        "roc1": ("roc1",),
        "roc2": ("roc2",),
        "roc3": ("roc3",),
        "roc4": ("roc4",),
        "sma1": ("sma1",),
        "sma2": ("sma2",),
        "sma3": ("sma3",),
        "sma4": ("sma4",),
    }
    kwargs = dict(binding.extra_kwargs)
    for oracle_name, candidates in aliases.items():
        target = next((name for name in candidates if name in constructor), None)
        if target is not None:
            kwargs[oracle_name] = constructor_value(spec, target)
    series_names = set(spec.series_args)
    for name in inspect.signature(function).parameters:
        if name in constructor and name not in series_names and name not in kwargs:
            kwargs[name] = constructor_value(spec, name)
    result = function(*(pd.Series(array) for array in arrays), **kwargs)
    if result is None:
        raise ValueError(f"{binding.name} returned no output")
    matrix = result.to_numpy(dtype=np.float64)
    if matrix.ndim == 2:
        return tuple(matrix[:, index] for index in range(matrix.shape[1]))
    return matrix


def cross_section_oracle_arrays(mode: str, arrays):
    """Expand aggregate TAFlow breadth inputs into Wickra constituent rows."""
    bars = len(arrays[0])
    if mode == "volume":
        advancing, declining = arrays
        change = np.tile(np.array([1.0, -1.0]), (bars, 1))
        volume = np.column_stack((advancing, declining))
        flags = np.zeros((bars, 2), dtype=np.bool_)
        return [change, volume, flags, flags.copy()]

    universe = 8
    member = np.arange(universe)[None, :]
    changes = np.zeros((bars, universe), dtype=np.float64)
    volume = np.ones((bars, universe), dtype=np.float64)
    new_high = np.zeros((bars, universe), dtype=np.bool_)
    new_low = np.zeros((bars, universe), dtype=np.bool_)

    if mode in {"advance_decline", "trin"}:
        advancers, decliners = arrays[:2]
        changes[member < advancers[:, None]] = 1.0
        declining = (member >= advancers[:, None]) & (
            member < (advancers + decliners)[:, None]
        )
        changes[declining] = -1.0
        if mode == "trin":
            advancing_volume, declining_volume = arrays[2:]
            advancing = changes > 0.0
            volume[advancing] = np.repeat(
                advancing_volume / advancers, advancers.astype(np.int64)
            )
            volume[declining] = np.repeat(
                declining_volume / decliners, decliners.astype(np.int64)
            )
        return [changes, volume, new_high, new_low]

    if mode == "extrema":
        highs, lows = arrays
        new_high = member < highs[:, None]
        new_low = (member >= highs[:, None]) & (member < (highs + lows)[:, None])
        return [changes, volume, new_high, new_low]

    selected, totals = arrays
    signal = member < selected[:, None]
    # ``totals`` is fixed at eight in generated evidence, but retain this mask
    # so the conversion remains correct for smaller explicit universes.
    active = member < totals[:, None]
    volume[~active] = 0.0
    if mode == "buy_signal":
        return [changes, volume, new_high, new_low, signal]
    return [changes, volume, new_high, new_low, signal]


def numpy_oracle(spec: Spec, arrays):
    """Execute one explicit NumPy ufunc override on the supplied arrays."""

    def trailing_windows(values: np.ndarray, period: int) -> np.ndarray:
        if len(values) < period:
            return np.empty((0, period), dtype=np.asarray(values).dtype)
        return np.lib.stride_tricks.sliding_window_view(values, period)

    def align_warm(values: np.ndarray, period: int) -> np.ndarray:
        output = np.full(len(arrays[0]), np.nan)
        output[period - 1 :] = values
        return output

    def lag():
        period = int(constructor_value(spec, "timeperiod"))
        output = np.full(len(arrays[0]), np.nan)
        if len(output) > period:
            output[period:] = arrays[0][:-period]
        return output

    def rolling_mode():
        period = int(constructor_value(spec, "timeperiod"))
        source = np.asarray(arrays[0])
        if source.size < period:
            return np.full(source.size, np.nan)
        if np.unique(source).size == source.size:
            return align_warm(source[: source.size - period + 1], period)
        windows = trailing_windows(source, period)
        values = []
        for window in windows:
            finite = window[~np.isnan(window)]
            if not len(finite):
                values.append(np.nan)
                continue
            unique, counts = np.unique(finite, return_counts=True)
            maximum = counts.max()
            winners = set(unique[counts == maximum].tolist())
            values.append(next(value for value in window if value in winners))
        return align_warm(np.asarray(values), period)

    def rolling_rank():
        period = int(constructor_value(spec, "timeperiod"))
        windows = trailing_windows(np.asarray(arrays[0]), period)
        last = windows[:, -1]
        ranks = (
            (windows < last[:, None]).sum(axis=1)
            + (windows == last[:, None]).sum(axis=1)
        ) / period
        return align_warm(ranks, period)

    def rolling_winsorize():
        period = int(constructor_value(spec, "timeperiod"))
        windows = trailing_windows(np.asarray(arrays[0]), period)
        lower = np.quantile(windows, constructor_value(spec, "lower"), axis=1)
        upper = np.quantile(windows, constructor_value(spec, "upper"), axis=1)
        return align_warm(np.clip(windows[:, -1], lower, upper), period)

    def ewm_components():
        period = int(constructor_value(spec, "timeperiod"))
        alpha = 2.0 / (period + 1.0)
        left = np.asarray(arrays[0], dtype=np.float64)
        right = np.asarray(
            arrays[1] if len(arrays) > 1 else arrays[0], dtype=np.float64
        )
        variance_left = np.zeros(len(left))
        variance_right = np.zeros(len(left))
        covariance = np.zeros(len(left))
        mean_left, mean_right = left[0], right[0]
        for index in range(1, len(left)):
            delta_left = left[index] - mean_left
            delta_right = right[index] - mean_right
            mean_left += alpha * delta_left
            mean_right += alpha * delta_right
            variance_left[index] = (1.0 - alpha) * (
                variance_left[index - 1] + alpha * delta_left * delta_left
            )
            variance_right[index] = (1.0 - alpha) * (
                variance_right[index - 1] + alpha * delta_right * delta_right
            )
            covariance[index] = (1.0 - alpha) * (
                covariance[index - 1] + alpha * delta_left * delta_right
            )
        return variance_left, variance_right, covariance

    def ewm_correlation():
        left, right, covariance = ewm_components()
        denominator = np.sqrt(left * right)
        return np.divide(
            covariance,
            denominator,
            out=np.zeros_like(covariance),
            where=denominator > 0.0,
        )

    def drawdown():
        values = np.asarray(arrays[0])
        peaks = np.maximum.accumulate(values)
        return (
            np.divide(values, peaks, out=np.zeros_like(values), where=peaks != 0.0)
            - 1.0
        )

    def rolling_r_squared():
        period = int(constructor_value(spec, "period"))
        left = trailing_windows(np.asarray(arrays[0]), period)
        right = trailing_windows(np.asarray(arrays[1]), period)
        left_centered = left - left.mean(axis=1, keepdims=True)
        right_centered = right - right.mean(axis=1, keepdims=True)
        numerator = np.sum(left_centered * right_centered, axis=1)
        denominator = np.sqrt(
            np.sum(left_centered**2, axis=1) * np.sum(right_centered**2, axis=1)
        )
        correlation = np.divide(
            numerator,
            denominator,
            out=np.zeros_like(numerator),
            where=denominator != 0.0,
        )
        return align_warm(correlation**2, period)

    def rolling_projection_mean():
        period = int(constructor_value(spec, "period"))
        return align_warm(
            trailing_windows(np.asarray(arrays[0]), period).mean(axis=1), period
        )

    def crossing(over: bool):
        left, right = map(np.asarray, arrays)
        output = np.zeros(len(left))
        if over:
            output[1:] = ((left[:-1] <= right[:-1]) & (left[1:] > right[1:])).astype(
                float
            )
        else:
            output[1:] = ((left[:-1] >= right[:-1]) & (left[1:] < right[1:])).astype(
                float
            )
        return output

    def direction(rising: bool):
        period = int(constructor_value(spec, "timeperiod"))
        values = np.asarray(arrays[0])
        output = np.full(len(values), np.nan)
        relation = values[period:] > values[:-period]
        if not rising:
            relation = values[period:] < values[:-period]
        output[period:] = relation.astype(float)
        return output

    def higher_high():
        high = np.asarray(arrays[0])
        output = np.full(len(high), np.nan)
        output[1:] = (high[1:] > high[:-1]).astype(float)
        return output

    def bar_relation(kind: str):
        high, low = map(np.asarray, arrays)
        output = np.full(len(high), np.nan)
        relations = {
            "lower": low[1:] < low[:-1],
            "inside": (high[1:] < high[:-1]) & (low[1:] > low[:-1]),
            "outside": (high[1:] > high[:-1]) & (low[1:] < low[:-1]),
            "gap_up": low[1:] > high[:-1],
            "gap_down": high[1:] < low[:-1],
        }
        output[1:] = relations[kind].astype(float)
        return output

    def bars_since():
        condition = np.asarray(arrays[0], dtype=bool)
        output = np.zeros(len(condition))
        count = None
        for index, active in enumerate(condition):
            count = 0 if active else (0 if count is None else count + 1)
            output[index] = count
        return output

    def event_memory(kind: str):
        condition = np.asarray(arrays[0], dtype=bool)
        values = np.asarray(arrays[1], dtype=np.float64)
        output = np.full(len(values), np.nan)
        current = None
        for index, (active, value) in enumerate(zip(condition, values)):
            if kind == "value":
                if active:
                    current = value
            elif active or current is None:
                current = value
            elif kind == "highest":
                current = max(current, value)
            else:
                current = min(current, value)
            if current is not None:
                output[index] = current
        return output

    def rolling_mean_series(values: np.ndarray, period: int, offset: int = 0):
        output = np.full(len(arrays[0]), np.nan)
        means = trailing_windows(values, period).mean(axis=1)
        output[period - 1 + offset : period - 1 + offset + len(means)] = means
        return output

    def annualized_garman_klass_yang_zhang():
        open_, high, low, close = map(np.asarray, arrays)
        term = (
            0.5 * np.log(high[1:] / low[1:]) ** 2
            - (2.0 * np.log(2.0) - 1.0) * np.log(close[1:] / open_[1:]) ** 2
            + np.log(open_[1:] / close[:-1]) ** 2
        )
        period = int(constructor_value(spec, "timeperiod"))
        output = np.full(len(close), np.nan)
        output[period:] = (
            np.sqrt(trailing_windows(term, period).mean(axis=1))
            * np.sqrt(252.0)
            * 100.0
        )
        return output

    def annualized_close_to_close():
        close = np.asarray(arrays[0])
        returns = np.log(close[1:] / close[:-1])
        period = int(constructor_value(spec, "timeperiod"))
        windows = trailing_windows(returns, period)
        output = np.full(len(close), np.nan)
        output[period:] = (
            np.sqrt(
                np.maximum((windows**2).mean(axis=1) - windows.mean(axis=1) ** 2, 0.0)
            )
            * np.sqrt(252.0)
            * 100.0
        )
        return output

    def linear_decay():
        period = int(constructor_value(spec, "timeperiod"))
        weights = np.arange(1.0, period + 1.0)
        values = (
            trailing_windows(np.asarray(arrays[0]), period) @ weights / weights.sum()
        )
        return align_warm(values, period)

    def exponentially_weighted_sum():
        period = int(constructor_value(spec, "timeperiod"))
        decay = 1.0 - 2.0 / (period + 1.0)
        output = np.empty(len(arrays[0]))
        previous = 0.0
        for index, value in enumerate(arrays[0]):
            previous = value + decay * previous
            output[index] = previous
        return output

    def average_daily_dollar_value():
        period = int(constructor_value(spec, "timeperiod"))
        products = np.asarray(arrays[0]) * np.asarray(arrays[1])
        return align_warm(trailing_windows(products, period).mean(axis=1), period)

    def rolling_hedge_ratio():
        period = int(constructor_value(spec, "timeperiod"))
        x = trailing_windows(np.asarray(arrays[0]), period)
        y = trailing_windows(np.asarray(arrays[1]), period)
        xc = x - x.mean(axis=1, keepdims=True)
        yc = y - y.mean(axis=1, keepdims=True)
        variance = np.sum(xc * xc, axis=1)
        beta = np.divide(
            np.sum(xc * yc, axis=1),
            variance,
            out=np.zeros(len(x)),
            where=variance > 0.0,
        )
        return align_warm(beta, period)

    def rolling_entropy():
        period = int(constructor_value(spec, "timeperiod"))
        source = np.asarray(arrays[0])
        if source.size < period:
            return np.full(source.size, np.nan)
        if np.unique(source).size == source.size:
            return align_warm(np.full(source.size - period + 1, np.log(period)), period)
        values = []
        for window in trailing_windows(source, period):
            _, counts = np.unique(window[~np.isnan(window)], return_counts=True)
            probabilities = counts.astype(float) / period
            values.append(-np.sum(probabilities * np.log(probabilities)))
        return align_warm(np.asarray(values), period)

    def fractal_dimension():
        period = int(constructor_value(spec, "timeperiod"))
        windows = trailing_windows(np.asarray(arrays[0]), period)
        if not len(windows):
            return np.full(len(arrays[0]), np.nan)

        def ranges(values):
            deviations = values - values.mean(axis=-1, keepdims=True)
            cumulative = np.cumsum(deviations, axis=-1)
            spread = cumulative.max(axis=-1) - cumulative.min(axis=-1)
            deviation = np.sqrt(np.mean(deviations**2, axis=-1))
            return np.divide(
                spread,
                deviation,
                out=np.full_like(spread, np.nan),
                where=(spread > 0.0) & (deviation > 0.0),
            )

        whole = ranges(windows)
        half = period // 2
        halves = np.nanmean(
            np.column_stack(
                (ranges(windows[:, :half]), ranges(windows[:, half : 2 * half]))
            ),
            axis=1,
        )
        hurst = np.clip(
            (np.log(halves) - np.log(whole)) / (np.log(half) - np.log(period)),
            0.0,
            1.0,
        )
        hurst[~np.isfinite(hurst)] = 0.5
        return align_warm(2.0 - hurst, period)

    def rolling_ou_half_life():
        period = int(constructor_value(spec, "timeperiod"))
        price = np.asarray(arrays[0])
        if len(price) <= period:
            return np.full(len(price), np.nan)
        previous = price[:-1]
        change = np.diff(price)
        output = np.full(len(price), np.nan)
        x = trailing_windows(change, period)
        y = trailing_windows(previous, period)
        xc, yc = x - x.mean(axis=1, keepdims=True), y - y.mean(axis=1, keepdims=True)
        variance = np.mean(yc * yc, axis=1)
        covariance = np.mean(xc * yc, axis=1)
        rate = np.divide(
            -covariance, variance, out=np.zeros_like(covariance), where=variance > 0.0
        )
        output[period:] = np.divide(
            np.log(2.0), rate, out=np.full_like(rate, np.nan), where=rate > 0.0
        )
        return output

    def rolling_spread_zscore():
        period = int(constructor_value(spec, "timeperiod"))
        x = trailing_windows(np.asarray(arrays[0]), period)
        y = trailing_windows(np.asarray(arrays[1]), period)
        xc, yc = x - x.mean(axis=1, keepdims=True), y - y.mean(axis=1, keepdims=True)
        variance = np.sum(xc * xc, axis=1)
        beta = np.divide(
            np.sum(xc * yc, axis=1),
            variance,
            out=np.zeros(len(x)),
            where=variance > 0.0,
        )
        spread = y - beta[:, None] * x
        deviation = spread.std(axis=1)
        score = np.divide(
            spread[:, -1] - spread.mean(axis=1),
            deviation,
            out=np.zeros(len(x)),
            where=deviation > 0.0,
        )
        return align_warm(score, period)

    def cusum():
        threshold = float(constructor_value(spec, "threshold"))
        output = np.zeros(len(arrays[0]))
        positive = negative = 0.0
        for index, change in enumerate(arrays[0]):
            positive = max(positive + change, 0.0)
            negative = max(negative - change, 0.0)
            if positive > threshold:
                positive = 0.0
                output[index] = 1.0
            elif negative > threshold:
                negative = 0.0
                output[index] = -1.0
        return output

    def fractional_difference():
        d = float(constructor_value(spec, "d"))
        threshold = float(constructor_value(spec, "threshold"))
        weights = [1.0]
        while True:
            k = len(weights)
            weight = -weights[-1] * (d - k + 1.0) / k
            if abs(weight) < threshold:
                break
            weights.append(weight)
        width = len(weights)
        output = np.full(len(arrays[0]), np.nan)
        windows = trailing_windows(np.asarray(arrays[0]), width)
        output[width - 1 :] = windows[:, ::-1] @ np.asarray(weights)
        return output

    def roll_spread():
        period = int(constructor_value(spec, "timeperiod"))
        delta = np.diff(arrays[0], prepend=arrays[0][0])
        if len(delta) <= period:
            return np.full(len(delta), np.nan)
        left, right = (
            trailing_windows(delta[1:], period),
            trailing_windows(delta[:-1], period),
        )
        output = np.full(len(delta), np.nan)
        covariance = np.sum(
            (left - left.mean(axis=1, keepdims=True))
            * (right - right.mean(axis=1, keepdims=True)),
            axis=1,
        ) / (period - 1)
        output[period:] = 2.0 * np.sqrt(np.maximum(-covariance, 0.0))
        return output

    def rolling_percentile():
        period = int(constructor_value(spec, "timeperiod"))
        q = float(constructor_value(spec, "percentile")) / 100.0
        return align_warm(
            np.quantile(trailing_windows(np.asarray(arrays[0]), period), q, axis=1),
            period,
        )

    def premium_discount():
        close = np.asarray(arrays[0])
        period = int(constructor_value(spec, "window"))
        zone = np.empty(len(close))
        equilibrium = np.empty(len(close))
        for index, value in enumerate(close):
            window = close[max(0, index + 1 - period) : index + 1]
            equilibrium[index] = (window.max() + window.min()) * 0.5
            zone[index] = np.sign(value - equilibrium[index])
        return zone, equilibrium

    def fibonacci_levels():
        close = np.asarray(arrays[0])
        period = int(constructor_value(spec, "window"))
        outputs = [np.empty(len(close)) for _ in range(7)]
        ratios = (0.0, 0.236, 0.382, 0.5, 0.618, 0.786, 1.0)
        for index in range(len(close)):
            window = close[max(0, index + 1 - period) : index + 1]
            high, low = np.nanmax(window), np.nanmin(window)
            span = high - low
            for output, ratio in zip(outputs, ratios):
                output[index] = high - span * ratio
        return tuple(outputs)

    def anchored_vwap():
        high, low, close, volume, anchor = arrays
        multiplier = float(constructor_value(spec, "standard_deviation_multiplier"))
        outputs = [np.empty(len(close)) for _ in range(3)]
        wp = ws = total = 0.0
        for i, values in enumerate(zip(high, low, close, volume, anchor)):
            h, l, c, v, reset = values
            if reset:
                wp = ws = total = 0.0
            typical = (h + l + c) / 3.0
            wp += typical * v
            ws += typical * typical * v
            total += v
            mean = wp / total if total else np.nan
            deviation = (
                multiplier * np.sqrt(max(ws / total - mean * mean, 0.0))
                if total
                else np.nan
            )
            outputs[0][i], outputs[1][i], outputs[2][i] = (
                mean,
                mean + deviation,
                mean - deviation,
            )
        return tuple(outputs)

    def pivot_points():
        high, low, close, anchor = arrays
        outputs = [np.full(len(close), np.nan) for _ in range(5)]
        running_high = running_low = running_close = None
        levels = [np.nan] * 5
        for i, (h, l, c, reset) in enumerate(zip(high, low, close, anchor)):
            if reset:
                if running_high is not None:
                    pivot = (running_high + running_low + running_close) / 3.0
                    span = running_high - running_low
                    levels = [
                        pivot,
                        2 * pivot - running_low,
                        2 * pivot - running_high,
                        pivot - span,
                        pivot + span,
                    ]
                running_high, running_low, running_close = h, l, c
            else:
                running_high = h if running_high is None else max(running_high, h)
                running_low = l if running_low is None else min(running_low, l)
                running_close = c
            for output, value in zip(outputs, levels):
                output[i] = value
        return tuple(outputs)

    def opening_range():
        high, low, close, anchor = arrays
        bars = int(constructor_value(spec, "bars"))
        outputs = [np.empty(len(close)) for _ in range(3)]
        count = 0
        range_high = -np.inf
        range_low = np.inf
        for i, (h, l, c, reset) in enumerate(zip(high, low, close, anchor)):
            if reset:
                count = 0
                range_high = -np.inf
                range_low = np.inf
            if count < bars:
                range_high = max(range_high, h)
                range_low = min(range_low, l)
                count += 1
            outputs[0][i], outputs[1][i] = range_high, range_low
            outputs[2][i] = 1 if c > range_high else -1 if c < range_low else 0
        return tuple(outputs)

    def session_volume_levels():
        high, low, close, volume, anchor = arrays
        bins = int(constructor_value(spec, "bins"))
        area = float(constructor_value(spec, "value_area"))
        outputs = [np.empty(len(close)) for _ in range(3)]
        histogram = np.zeros(bins)
        session_low = None
        step = 1.0
        for i, (h, l, c, v, reset) in enumerate(zip(high, low, close, volume, anchor)):
            if reset or session_low is None:
                session_low = l
                step = max((h - l) / bins, 1e-12)
                histogram.fill(0.0)
            session_low = min(session_low, l)
            index = int(np.clip(int((c - session_low) / step), 0, bins - 1))
            histogram[index] += v
            poc = int(np.argmax(histogram))
            left = right = poc
            accumulated = histogram[poc]
            target = histogram.sum() * area
            while accumulated < target and (left > 0 or right + 1 < bins):
                if left == 0:
                    right += 1
                elif right + 1 == bins or histogram[left - 1] >= histogram[right + 1]:
                    left -= 1
                else:
                    right += 1
                accumulated = histogram[left : right + 1].sum()
            outputs[0][i] = (poc + 0.5) * step + session_low
            outputs[1][i] = (right + 0.5) * step + session_low
            outputs[2][i] = (left + 0.5) * step + session_low
        return tuple(outputs)

    def confirmed_swings(high, low, length):
        high, low = map(np.asarray, (high, low))
        width = 2 * length + 1
        outputs = [np.full(len(high), np.nan) for _ in range(3)]
        since = None
        for index in range(width - 1, len(high)):
            center = index - length
            high_window = high[index - width + 1 : index + 1]
            low_window = low[index - width + 1 : index + 1]
            is_high, is_low = (
                high[center] >= high_window.max(),
                low[center] <= low_window.min(),
            )
            if is_high and not is_low:
                signal, level = 1.0, high[center]
            elif is_low and not is_high:
                signal, level = -1.0, low[center]
            else:
                signal = level = np.nan
            since = (
                (since + 1 if since is not None else None) if np.isnan(signal) else 0
            )
            outputs[0][index], outputs[1][index], outputs[2][index] = (
                signal,
                level,
                np.nan if since is None else since,
            )
        return tuple(outputs)

    def swing_values():
        return confirmed_swings(
            arrays[0], arrays[1], int(constructor_value(spec, "swing_length"))
        )

    def smoothed_trend_channel():
        high, low, close = map(np.asarray, arrays)
        period = int(constructor_value(spec, "length"))
        outputs = [np.full(len(close), np.nan) for _ in range(2)]
        side = 1
        high_mean = trailing_windows(high, period).mean(axis=1)
        low_mean = trailing_windows(low, period).mean(axis=1)
        for index in range(period - 1, len(close)):
            ah, al = high_mean[index - period + 1], low_mean[index - period + 1]
            if close[index] > ah:
                side = 1
            elif close[index] < al:
                side = -1
            outputs[0][index], outputs[1][index] = (al, ah) if side > 0 else (ah, al)
        return tuple(outputs)

    def position_hold():
        output = np.empty(len(arrays[0]))
        position = 0.0
        for index, value in enumerate(arrays[0]):
            if value != 0.0:
                position = value
            output[index] = position
        return output

    def entry_exit():
        entry, exit_ = (np.asarray(value, dtype=bool) for value in arrays)
        output = np.empty(len(entry))
        position = 0.0
        for index, (enter, leave) in enumerate(zip(entry, exit_)):
            if enter and not leave:
                position = 1.0
            elif leave and not enter:
                position = -1.0
            output[index] = position
        return output

    def session_extrema():
        reset, high, low = arrays
        outputs = [np.empty(len(high)) for _ in range(2)]
        current_high = current_low = None
        for index, (new, h, l) in enumerate(zip(reset, high, low)):
            if new or current_high is None:
                current_high, current_low = h, l
            else:
                current_high, current_low = max(current_high, h), min(current_low, l)
            outputs[0][index], outputs[1][index] = current_high, current_low
        return tuple(outputs)

    def previous_high_low():
        reset, high, low = arrays
        outputs = [np.full(len(high), np.nan) for _ in range(4)]
        running_high = running_low = previous_high = previous_low = None
        for index, (new, h, l) in enumerate(zip(reset, high, low)):
            if new:
                if running_high is not None:
                    previous_high, previous_low = running_high, running_low
                running_high, running_low = h, l
            else:
                running_high = h if running_high is None else max(running_high, h)
                running_low = l if running_low is None else min(running_low, l)
            if previous_high is not None:
                outputs[0][index], outputs[1][index] = previous_high, previous_low
                if h > previous_high:
                    outputs[2][index] = 1.0
                if l < previous_low:
                    outputs[3][index] = 1.0
        return tuple(outputs)

    def retracements():
        swing_spec = spec
        signals, levels, _ = swing_values()
        close = np.asarray(arrays[2])
        outputs = [np.full(len(close), np.nan) for _ in range(3)]
        last_high = last_low = leg_high = leg_low = direction_value = None
        deepest = 0.0
        for index, (signal, level, value) in enumerate(zip(signals, levels, close)):
            if signal > 0:
                last_high = level
                if last_low is not None:
                    leg_high, leg_low, direction_value, deepest = (
                        level,
                        last_low,
                        1.0,
                        0.0,
                    )
            elif signal < 0:
                last_low = level
                if last_high is not None:
                    leg_high, leg_low, direction_value, deepest = (
                        last_high,
                        level,
                        -1.0,
                        0.0,
                    )
            if direction_value is not None:
                outputs[0][index] = direction_value
            if direction_value is not None and leg_high > leg_low:
                current = max(
                    (leg_high - value if direction_value > 0 else value - leg_low)
                    / (leg_high - leg_low)
                    * 100.0,
                    0.0,
                )
                deepest = max(deepest, current)
                outputs[1][index], outputs[2][index] = current, deepest
        return tuple(outputs)

    def average_true_range(high, low, close, period):
        result = np.full(len(close), np.nan)
        true_range = np.full(len(close), np.nan)
        true_range[1:] = np.maximum.reduce(
            (
                high[1:] - low[1:],
                np.abs(high[1:] - close[:-1]),
                np.abs(low[1:] - close[:-1]),
            )
        )
        if len(close) > period:
            result[period] = np.mean(true_range[1 : period + 1])
            for index in range(period + 1, len(close)):
                result[index] = (
                    result[index - 1] * (period - 1) + true_range[index]
                ) / period
        return result

    def bos_choch():
        high, low, close = map(np.asarray, arrays)
        length = int(constructor_value(spec, "swing_length"))
        signals, levels, _ = confirmed_swings(high, low, length)
        outputs = [np.full(len(close), np.nan) for _ in range(4)]
        recent = []
        pending = None
        trend = None
        for index, (signal, swing_level, value) in enumerate(
            zip(signals, levels, close)
        ):
            if pending is not None and (
                (pending[0] > 0 and value > pending[1])
                or (pending[0] < 0 and value < pending[1])
            ):
                outputs[3][index], outputs[2][index] = pending
                trend = pending[0]
                pending = None
            if index >= 2 * length:
                recent.append((signal, swing_level))
                recent = recent[-4:]
                if len(recent) == 4:
                    bullish = (
                        recent[0][0] < 0 < recent[1][0]
                        and recent[2][0] < 0 < recent[3][0]
                        and recent[0][1] < recent[2][1]
                        and recent[1][1] < recent[3][1]
                    )
                    bearish = (
                        recent[0][0] > 0 > recent[1][0]
                        and recent[2][0] > 0 > recent[3][0]
                        and recent[0][1] > recent[2][1]
                        and recent[1][1] > recent[3][1]
                    )
                    direction_value = 1.0 if bullish else -1.0 if bearish else None
                    if direction_value is not None:
                        outputs[0][index] = direction_value
                        if trend is not None and trend != direction_value:
                            outputs[1][index] = direction_value
                        outputs[2][index] = recent[1][1]
                        pending = (direction_value, recent[1][1])
        return tuple(outputs)

    def equal_highs_lows():
        high, low, close = map(np.asarray, arrays)
        length = int(constructor_value(spec, "eq_len"))
        atr_period = int(constructor_value(spec, "atr_period"))
        threshold = float(constructor_value(spec, "eq_threshold"))
        signals, levels, _ = confirmed_swings(high, low, length)
        atr = average_true_range(high, low, close, atr_period)
        outputs = [np.full(len(close), np.nan) for _ in range(3)]
        previous_high = previous_low = None
        for index, (signal, level) in enumerate(zip(signals, levels)):
            if signal > 0:
                if (
                    previous_high is not None
                    and np.isfinite(atr[index])
                    and abs(level - previous_high) < atr[index] * threshold
                ):
                    outputs[0][index], outputs[2][index] = 1.0, level
                previous_high = level
            elif signal < 0:
                if (
                    previous_low is not None
                    and np.isfinite(atr[index])
                    and abs(level - previous_low) < atr[index] * threshold
                ):
                    outputs[1][index], outputs[2][index] = 1.0, level
                previous_low = level
        return tuple(outputs)

    def liquidity_pools():
        high, low = map(np.asarray, arrays)
        length = int(constructor_value(spec, "swing_length"))
        tolerance = float(constructor_value(spec, "range_percent"))
        signals, levels, _ = confirmed_swings(high, low, length)
        outputs = [np.full(len(high), np.nan) for _ in range(3)]
        pools = []
        sequence = 0
        for index, (signal, level, h, l) in enumerate(zip(signals, levels, high, low)):
            if signal != 0 and np.isfinite(signal):
                side = 1 if signal > 0 else -1
                choices = [
                    (i, abs(pool["level"] - level), pool["seq"])
                    for i, pool in enumerate(pools)
                    if pool["side"] == side
                    and abs(pool["level"] - level) <= tolerance * pool["level"]
                ]
                if choices:
                    slot = min(choices, key=lambda item: (item[1], item[2]))[0]
                    pool = pools[slot]
                    pool["level"] = (
                        max(pool["level"], level)
                        if side > 0
                        else min(pool["level"], level)
                    )
                    pool["count"] += 1
                    if pool["count"] >= 2:
                        outputs[0][index], outputs[1][index] = side, pool["level"]
                else:
                    pools.append(
                        {"side": side, "level": level, "count": 1, "seq": sequence}
                    )
                    sequence += 1
            retained = []
            for pool in pools:
                swept = pool["count"] >= 2 and (
                    (pool["side"] > 0 and h >= pool["level"])
                    or (pool["side"] < 0 and l <= pool["level"])
                )
                if swept:
                    outputs[2][index], outputs[1][index] = pool["side"], pool["level"]
                else:
                    retained.append(pool)
            pools = retained
        return tuple(outputs)

    def order_blocks():
        high, low, close, volume = map(np.asarray, arrays)
        structure_length = int(constructor_value(spec, "swing_length"))
        internal_length = int(constructor_value(spec, "internal_length"))
        atr_period = int(constructor_value(spec, "atr_period"))
        threshold = float(constructor_value(spec, "threshold"))
        structure_signal, structure_level, _ = confirmed_swings(
            high, low, structure_length
        )
        internal_signal, internal_level, _ = confirmed_swings(
            high, low, internal_length
        )
        atr = average_true_range(high, low, close, atr_period)
        outputs = [np.full(len(close), np.nan) for _ in range(5)]
        internal_high = internal_low = structure_high = structure_low = None
        zones = []
        for index in range(len(close)):
            volatile = (
                np.isfinite(atr[index])
                and high[index] - low[index] >= threshold * atr[index]
            )
            signal, level = internal_signal[index], internal_level[index]
            if signal > 0:
                internal_high = (level, volume[index], volatile)
                if (
                    structure_high is not None
                    and internal_low is not None
                    and not internal_low[2]
                    and level > structure_high
                ):
                    (
                        outputs[0][index],
                        outputs[1][index],
                        outputs[2][index],
                        outputs[3][index],
                    ) = 1.0, level, internal_low[0], internal_low[1]
                    zones.append((1.0, level, internal_low[0]))
                    structure_high = level
            elif signal < 0:
                internal_low = (level, volume[index], volatile)
                if (
                    structure_low is not None
                    and internal_high is not None
                    and not internal_high[2]
                    and level < structure_low
                ):
                    (
                        outputs[0][index],
                        outputs[1][index],
                        outputs[2][index],
                        outputs[3][index],
                    ) = -1.0, internal_high[0], level, internal_high[1]
                    zones.append((-1.0, internal_high[0], level))
                    structure_low = level
            if structure_signal[index] > 0:
                structure_high = structure_level[index]
            elif structure_signal[index] < 0:
                structure_low = structure_level[index]
            retained = []
            for direction_value, top, bottom in zones:
                filled = (direction_value > 0 and low[index] <= bottom) or (
                    direction_value < 0 and high[index] >= top
                )
                if filled:
                    outputs[4][index] = direction_value
                else:
                    retained.append((direction_value, top, bottom))
            zones = retained
        return tuple(outputs)

    def rolling_calmar():
        period = int(constructor_value(spec, "timeperiod"))
        values = np.asarray(arrays[0], dtype=np.float64)
        output = np.full(values.shape, np.nan)
        if len(values) < period:
            return output
        windows = np.lib.stride_tricks.sliding_window_view(values, period)
        peaks = np.maximum.accumulate(windows, axis=1)
        drawdowns = np.min(
            np.divide(
                windows,
                peaks,
                out=np.zeros_like(windows),
                where=peaks != 0.0,
            )
            - 1.0,
            axis=1,
        )
        averages = np.mean(windows, axis=1)
        output[period - 1 :] = np.divide(
            averages,
            -drawdowns,
            out=np.zeros_like(averages),
            where=drawdowns < 0.0,
        )
        return output

    def rolling_recovery_factor():
        period = int(constructor_value(spec, "timeperiod"))
        values = np.asarray(arrays[0], dtype=np.float64)
        output = np.full(values.shape, np.nan)
        if len(values) < period:
            return output
        windows = np.lib.stride_tricks.sliding_window_view(values, period)
        peaks = np.maximum.accumulate(windows, axis=1)
        drawdowns = np.max(
            np.divide(
                peaks - windows,
                peaks,
                out=np.zeros_like(windows),
                where=peaks != 0.0,
            ),
            axis=1,
        )
        changes = windows[:, -1] - windows[:, 0]
        output[period - 1 :] = np.divide(
            changes,
            drawdowns,
            out=np.zeros_like(changes),
            where=drawdowns > 0.0,
        )
        return output

    def causal_ichimoku():
        high, low, close = (np.asarray(array, dtype=np.float64) for array in arrays)

        def midpoint(period_name: str) -> np.ndarray:
            period = int(constructor_value(spec, period_name))
            output = np.full(high.shape, np.nan)
            if len(high) >= period:
                highs = np.lib.stride_tricks.sliding_window_view(high, period)
                lows = np.lib.stride_tricks.sliding_window_view(low, period)
                output[period - 1 :] = (highs.max(axis=1) + lows.min(axis=1)) * 0.5
            return output

        tenkan = midpoint("tenkan")
        kijun = midpoint("kijun")
        span_a = (tenkan + kijun) * 0.5
        span_b = midpoint("senkou")
        return tenkan, kijun, span_a, span_b, close.copy()

    functions = {
        "causal lag": lag,
        "numpy.cumsum": lambda: np.cumsum(arrays[0]),
        "numpy.cumprod": lambda: np.cumprod(arrays[0]),
        "rolling mode": rolling_mode,
        "rolling percentile rank": rolling_rank,
        "rolling winsorize": rolling_winsorize,
        "ewm variance": lambda: ewm_components()[0],
        "ewm standard deviation": lambda: np.sqrt(ewm_components()[0]),
        "ewm covariance": lambda: ewm_components()[2],
        "ewm correlation": ewm_correlation,
        "drawdown from cumulative maximum": drawdown,
        "numpy.maximum.accumulate": lambda: np.maximum.accumulate(arrays[0]),
        "numpy.minimum.accumulate": lambda: np.minimum.accumulate(arrays[0]),
        "rolling squared correlation": rolling_r_squared,
        "rolling projection mean": rolling_projection_mean,
        "causal crossover": lambda: crossing(True),
        "causal crossunder": lambda: crossing(False),
        "period-over-period rising": lambda: direction(True),
        "period-over-period falling": lambda: direction(False),
        "higher high relation": higher_high,
        "lower low relation": lambda: bar_relation("lower"),
        "inside bar relation": lambda: bar_relation("inside"),
        "outside bar relation": lambda: bar_relation("outside"),
        "gap up relation": lambda: bar_relation("gap_up"),
        "gap down relation": lambda: bar_relation("gap_down"),
        "bars since condition": bars_since,
        "last value when condition": lambda: event_memory("value"),
        "highest since condition": lambda: event_memory("highest"),
        "lowest since condition": lambda: event_memory("lowest"),
        "signal delay": lag,
        "annualized Garman-Klass-Yang-Zhang volatility": annualized_garman_klass_yang_zhang,
        "annualized close-to-close volatility": annualized_close_to_close,
        "linear decay weighted mean": linear_decay,
        "one-based cumulative count": lambda: np.arange(1.0, len(arrays[0]) + 1.0),
        "exponentially weighted sum": exponentially_weighted_sum,
        "rolling average dollar volume": average_daily_dollar_value,
        "rolling OLS hedge ratio": rolling_hedge_ratio,
        "rolling Shannon entropy": rolling_entropy,
        "two-chunk rescaled-range dimension": fractal_dimension,
        "rolling OU half life": rolling_ou_half_life,
        "rolling hedged-spread z-score": rolling_spread_zscore,
        "CUSUM event filter": cusum,
        "fixed-width fractional differencing": fractional_difference,
        "rolling Roll spread estimator": roll_spread,
        "rolling percentile": rolling_percentile,
        "causal cross event": lambda: np.maximum(crossing(True), crossing(False)),
        "rolling premium-discount zone": premium_discount,
        "rolling Fibonacci levels": fibonacci_levels,
        "anchored VWAP deviation bands": anchored_vwap,
        "anchored classic pivot points": pivot_points,
        "anchored opening range": opening_range,
        "anchored volume levels": session_volume_levels,
        "causal confirmed swing pivots": swing_values,
        "smoothed trend channel": smoothed_trend_channel,
        "nonzero position hold": position_hold,
        "entry-exit position state": entry_exit,
        "explicit-session extrema": session_extrema,
        "previous-session high-low": previous_high_low,
        "causal swing retracements": retracements,
        "causal BOS and CHOCH events": bos_choch,
        "causal dual-scale order blocks": order_blocks,
        "causal liquidity pools": liquidity_pools,
        "causal equal pivot levels": equal_highs_lows,
        "rolling calmar on equity": rolling_calmar,
        "rolling recovery factor on equity": rolling_recovery_factor,
        "causal ichimoku components": causal_ichimoku,
        "numpy.abs": lambda: np.abs(arrays[0]),
        "numpy.arccosh": lambda: np.arccosh(arrays[0]),
        "numpy.arcsinh": lambda: np.arcsinh(arrays[0]),
        "numpy.arctanh": lambda: np.arctanh(arrays[0]),
        "numpy.cbrt": lambda: np.cbrt(arrays[0]),
        "numpy.tan reciprocal": lambda: 1.0 / np.tan(arrays[0]),
        "numpy.degrees": lambda: np.degrees(arrays[0]),
        "numpy.log1p": lambda: np.log1p(arrays[0]),
        "numpy.radians": lambda: np.radians(arrays[0]),
        "numpy.sign/abs/power": lambda: (
            np.sign(arrays[0])
            * np.abs(arrays[0]) ** constructor_value(spec, "exponent")
        ),
    }
    return functions[spec.numpy.name]()


def _smc_frame(open_, high, low, close):
    """Build the lowercase OHLC frame required by smartmoneyconcepts."""
    import pandas as pd

    return pd.DataFrame(
        {
            "open": open_,
            "high": high,
            "low": low,
            "close": close,
            "volume": np.ones_like(close),
        }
    )


def _smc_event_flags(frame, signal: str, event_index: str) -> np.ndarray:
    """Project SMC's future event indices onto their causal event bars."""
    result = np.full(len(frame), np.nan)
    for direction, index in zip(frame[signal], frame[event_index]):
        if not np.isnan(direction) and not np.isnan(index) and int(index) > 0:
            result[int(index)] = direction
    return result


def smc_oracle(spec: Spec, arrays):
    """Execute explicitly aligned smartmoneyconcepts batch references."""
    import pandas as pd
    from smartmoneyconcepts import smc

    if spec.cls.__name__ == "FairValueGap":
        frame = _smc_frame(*arrays)
        result = smc.fvg(frame, join_consecutive=False)
        return (
            result["FVG"].shift(1).to_numpy(),
            result["Top"].shift(1).to_numpy(),
            result["Bottom"].shift(1).to_numpy(),
            _smc_event_flags(result, "FVG", "MitigatedIndex"),
        )
    if spec.cls.__name__ == "Sessions":
        new_session, high, low = arrays
        groups = np.cumsum(new_session.astype(np.int64)) - 1
        within = np.arange(len(groups))
        starts = np.maximum.accumulate(np.where(new_session, within, 0))
        index = (
            pd.Timestamp("2024-01-01")
            + pd.to_timedelta(groups, unit="D")
            + pd.to_timedelta(within - starts, unit="ns")
        )
        close = (high + low) * 0.5
        frame = _smc_frame(close, high, low, close)
        frame.index = index
        result = smc.sessions(frame, "Custom", "00:00", "23:59", "UTC")
        return tuple(result[name].to_numpy() for name in ("Active", "High", "Low"))
    raise LookupError(f"no SMC adapter for {spec.cls.__name__}")


# ---------------------------------------------------------------------------
# Per-function verification
# ---------------------------------------------------------------------------


def verify_function(
    spec: Spec,
    data: dict,
    bars: int,
    split: int,
    actual_indices: tuple[int, ...] | None = None,
) -> dict:
    row: dict = {
        "function": spec.talib_name or spec.snake,
        "taflow_class": spec.cls.__name__ if spec.cls else None,
        "oracle": spec.oracle_source,
        "oracle_name": spec.oracle_name,
    }
    if spec.oracle_variant:
        row["variant"] = spec.oracle_variant
    if spec.warnings:
        row["warnings"] = list(spec.warnings)
    if spec.error:
        row["error"] = spec.error
        return row

    arrays = spec.arrays(data, bars)
    expected = None
    try:
        if spec.talib_name:
            expected = talib_oracle(spec, arrays)
        elif spec.wickra:
            expected = wickra_oracle(spec, arrays)
        elif spec.pandas_ta:
            expected = pandas_ta_oracle(spec, arrays)
        elif spec.numpy:
            expected = numpy_oracle(spec, arrays)
        elif spec.smc:
            expected = smc_oracle(spec, arrays)
    except Exception as exc:
        return {**row, "error": f"oracle failed: {exc}"}

    try:
        batch = Spec.extend(spec.new_state(), arrays)
    except Exception as exc:
        return {**row, "error": f"taflow batch failed: {exc}"}
    if expected is not None:
        if spec.wickra:
            tolerance = {"rtol": spec.wickra.rtol, "atol": spec.wickra.atol}
        elif spec.pandas_ta:
            tolerance = {
                "rtol": spec.pandas_ta.rtol,
                "atol": spec.pandas_ta.atol,
            }
        elif spec.smc and spec.cls.__name__ == "Sessions":
            tolerance = {"atol": 2e-5}
        else:
            tolerance = {}
        selected_actual = actual_indices or (
            spec.wickra.actual_indices
            if spec.wickra
            else spec.pandas_ta.actual_indices
            if spec.pandas_ta
            else None
        )
        selected_expected = (
            spec.wickra.oracle_indices
            if spec.wickra
            else spec.pandas_ta.oracle_indices
            if spec.pandas_ta
            else None
        )
        row["batch_vs_oracle"] = compare(
            batch,
            expected,
            selected_actual,
            selected_expected,
            **tolerance,
        )

    try:
        stitched = continue_series(spec, arrays, split)
    except Exception as exc:
        row["error"] = f"continue path failed: {exc}"
        return row
    row["continue_vs_batch_bitwise"] = bitwise_equal(stitched, batch)
    row["chunk_invariance"] = {
        str(chunk): bitwise_equal(chunked_series(spec, arrays, chunk), batch)
        for chunk in CHUNK_SIZES
    }
    if expected is not None:
        row["continue_vs_oracle"] = compare(
            stitched,
            expected,
            selected_actual,
            selected_expected,
            **tolerance,
        )
    return row


def verdict(row: dict) -> str:
    if "error" in row:
        return "ERROR"
    checks = [
        row[k].get("passed", False)
        for k in ("batch_vs_oracle", "continue_vs_oracle")
        if k in row
    ]
    if "continue_vs_batch_bitwise" in row:
        checks.append(row["continue_vs_batch_bitwise"])
    if "chunk_invariance" in row:
        checks.extend(row["chunk_invariance"].values())
    lifecycle = row.get("continue_vs_batch_bitwise", False) and all(
        row.get("chunk_invariance", {}).values()
    )
    if row.get("oracle") is None:
        return "NO_EXTERNAL_ORACLE" if lifecycle else "FAIL"
    if row.get("variant") and lifecycle:
        return "VARIANT"
    return "MATCH" if checks and all(checks) else "FAIL"


# ---------------------------------------------------------------------------
# Report
# ---------------------------------------------------------------------------


def fmt_check(block) -> str:
    if block is None:
        return "—"
    if isinstance(block, bool):
        return "yes" if block else "**NO**"
    if "error" in block:
        return f"error: {block['error']}"
    mark = "pass" if block["passed"] else "**FAIL**"
    return f"{mark} (err {block['max_abs_error']:.1e}, nan {block['nan_mismatches']})"


def write_report(rows: list[dict], path: Path, bars: int, split: int) -> None:
    import talib
    import taflow
    import wickra

    counts: dict[str, int] = {}
    for row in rows:
        counts[verdict(row)] = counts.get(verdict(row), 0) + 1

    lines = [
        "# TAFlow correctness verification",
        "",
        f"Date: {_dt.date.today().isoformat()} | bars: {bars:,} | "
        f"warm-up split: {split:,} extend + {bars - split:,} append | "
        f"tolerance rtol={RTOL}, atol={ATOL}",
        f"Environment: python {platform.python_version()}, numpy "
        f"{np.__version__}, TA-Lib {talib.__version__}, Wickra "
        f"{wickra.__version__}, pandas-ta-classic "
        f"{importlib.metadata.version('pandas-ta-classic')}, SMC "
        f"{importlib.metadata.version('smartmoneyconcepts')}, TAFlow "
        f"{getattr(taflow, '__version__', '?')}",
        "",
        "Summary: " + ", ".join(f"{k}: {v}" for k, v in sorted(counts.items())),
        "",
        "TAFlow is driven only through canonical Python classes. The registry",
        "selects TA-Lib, Wickra, pandas-ta-classic, explicit NumPy formula "
        "oracles, then SMC.",
        "*Batch vs oracle*:",
        "cold `extend` over the full series against the reference;",
        "*continue vs batch*: 9k `extend` + 1k `append` stitched output",
        "bitwise-identical to one-shot batch (chunk invariance); *continue",
        "vs oracle*: the stitched output against the reference. Repeated",
        f"native `extend` chunks {list(CHUNK_SIZES)} are also checked bitwise.",
        "",
        "| **Class** | **Target** | **Verdict** | **Batch vs oracle** | "
        "**Continue vs oracle** |",
        "|---|---|---|---|---|",
    ]
    for row in sorted(rows, key=lambda r: (verdict(r) == "MATCH", r["function"])):
        lines.append(
            f"| {row.get('taflow_class') or row['function']} | "
            f"{row.get('oracle') or '—'} | {verdict(row)} | "
            f"{fmt_check(row.get('batch_vs_oracle'))} | "
            f"{fmt_check(row.get('continue_vs_oracle'))} |"
        )

    mismatches = [r["function"] for r in rows if verdict(r) == "FAIL"]
    errors = [r["function"] for r in rows if verdict(r) == "ERROR"]
    warned = [r["function"] for r in rows if r.get("warnings")]
    lines += [
        "",
        "## Follow-ups",
        "",
        "- Mismatches: " + (", ".join(mismatches) or "none"),
        "- Errors (class/mapping/runtime): " + (", ".join(errors) or "none"),
        "- Compared at TA-Lib defaults only (unmapped params): "
        + (", ".join(warned) or "none"),
    ]
    path.write_text("\n".join(lines) + "\n")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("functions", nargs="*")
    parser.add_argument("--bars", type=int, default=10_000)
    parser.add_argument("--warmup-split", type=int, default=9_000)
    parser.add_argument("--report", type=Path, default=VERIFY_DIR / "CORRECTNESS.md")
    args = parser.parse_args()

    data = make_data(args.bars)
    registry = build_registry()
    if args.functions:
        specs, unknown = resolve_specs(args.functions, registry)
    else:
        specs, unknown = list(registry.values()), []
    if unknown:
        print(f"unknown: {', '.join(unknown)}", file=sys.stderr)
        return 1

    rows: list[dict] = []
    CORRECTNESS_EVIDENCE_DIR.mkdir(parents=True, exist_ok=True)
    for i, spec in enumerate(specs, 1):
        name = spec.talib_name or spec.snake
        try:
            row = verify_function(spec, data, args.bars, args.warmup_split)
        except Exception:
            row = {"function": name, "error": traceback.format_exc(limit=1)}
        rows.append(row)
        evidence_path = CORRECTNESS_EVIDENCE_DIR / f"{spec.snake}.json"
        evidence_path.write_text(json.dumps(row, indent=2, default=str) + "\n")
        print(f"[{i}/{len(specs)}] {name}: {verdict(row)}")

    write_report(rows, args.report, args.bars, args.warmup_split)
    bad = sum(1 for r in rows if verdict(r) in ("FAIL", "ERROR"))
    print(f"\nwrote {args.report}\n{len(rows)} checked, {bad} need attention")
    return 1 if bad else 0


if __name__ == "__main__":
    sys.exit(main())
