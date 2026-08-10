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


def compare(actual, expected, actual_indices: tuple[int, ...] | None = None,
            expected_indices: tuple[int, ...] | None = None,
            rtol: float = RTOL, atol: float = ATOL) -> dict:
    actuals, expecteds = as_tuple(actual), as_tuple(expected)
    if actual_indices is not None:
        actuals = tuple(actuals[index] for index in actual_indices)
    if expected_indices is not None:
        expecteds = tuple(expecteds[index] for index in expected_indices)
    if len(actuals) != len(expecteds):
        return {"passed": False,
                "error": f"output arity {len(actuals)} != {len(expecteds)}"}
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
            ((inf_a != inf_b) | (inf_a & inf_b & (np.signbit(a) != np.signbit(b)))).sum()
        )
        both = np.isfinite(a) & np.isfinite(b)
        if both.any():
            max_err = max(max_err, float(np.max(np.abs(a[both] - b[both]))))
    passed = nan_mismatches == 0 and infinity_mismatches == 0 and all(
        np.allclose(np.asarray(x, dtype=np.float64),
                    np.asarray(y, dtype=np.float64),
                    rtol=rtol, atol=atol, equal_nan=True)
        for x, y in zip(actuals, expecteds))
    return {"passed": passed, "nan_mismatches": nan_mismatches,
            "infinity_mismatches": infinity_mismatches,
            "max_abs_error": max_err}


def bitwise_equal(actual, expected) -> bool:
    return all(np.array_equal(np.asarray(a), np.asarray(b), equal_nan=True)
               for a, b in zip(as_tuple(actual), as_tuple(expected)))


def continue_series(spec: Spec, arrays, split: int) -> tuple:
    """extend(first split bars) + append(rest) -> stitched full series."""
    state = spec.new_state()
    head = Spec.extend(state, [a[:split] for a in arrays])
    bars = list(zip(*[a[split:].tolist() for a in arrays]))
    outputs = [Spec.append_value(state, bar) for bar in bars]
    if len(head) == 1 and np.asarray(head[0]).ndim == 2:
        width = np.asarray(head[0]).shape[1]
        tail = np.asarray([
            [float("nan")] * width if output is None else output
            for output in outputs
        ], dtype=np.float64).reshape((-1, width))
        return (np.concatenate([np.asarray(head[0]), tail], axis=0),)
    arity = len(head)
    columns: list[list[float]] = [[] for _ in range(arity)]
    for out in outputs:
        values = out if isinstance(out, tuple) else (
            (None,) * arity if out is None else (out,))
        for column, value in zip(columns, values):
            column.append(float("nan") if value is None else float(value))
    return tuple(
        np.concatenate([np.asarray(h, dtype=np.float64), np.asarray(c)])
        for h, c in zip(head, columns))


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
    series_names = {name.replace("_", "").lower()
                    for name in spec.series_args}
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
    }
    if binding.name in {"McGinleyDynamic", "VIDYA", "JMA"}:
        synonyms["period"] = ("length", "timeperiod", "period")
    kwargs = {}
    for name, parameter in inspect.signature(oracle_class).parameters.items():
        candidates = (name, *synonyms.get(name, ()))
        target = next((candidate for candidate in candidates
                       if candidate in taflow_parameters
                       and candidate.replace("_", "").lower() not in series_names), None)
        if target is not None:
            kwargs[name] = constructor_value(spec, target)
        elif parameter.default is inspect.Parameter.empty:
            raise TypeError(
                f"cannot map required Wickra parameter {name!r} for "
                f"{spec.cls.__name__}"
            )
    oracle = oracle_class(**kwargs)
    batch_arrays = list(arrays)
    if binding.cross_section:
        batch_arrays = cross_section_oracle_arrays(binding.cross_section, arrays)
    elif binding.input_mode == "high_low_midpoint":
        batch_arrays = [(np.asarray(arrays[0]) + np.asarray(arrays[1])) * 0.5]
    elif binding.input_mode == "swap_pair":
        batch_arrays = [arrays[1], arrays[0]]
    for index, name in enumerate(spec.series_args):
        if name == "timestamp":
            # TAFlow exposes Unix nanoseconds; Wickra Candle uses milliseconds.
            batch_arrays[index] = np.asarray(batch_arrays[index]) // 1_000_000
    if binding.prepend_zero_close:
        batch_arrays.insert(0, np.zeros_like(arrays[0]))
    result = oracle.batch(*batch_arrays)
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
        new_low = (member >= highs[:, None]) & (
            member < (highs + lows)[:, None]
        )
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
    functions = {
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

    return pd.DataFrame({
        "open": open_, "high": high, "low": low, "close": close,
        "volume": np.ones_like(close),
    })


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
        index = (pd.Timestamp("2024-01-01")
                 + pd.to_timedelta(groups, unit="D")
                 + pd.to_timedelta(within - starts, unit="ns"))
        close = (high + low) * 0.5
        frame = _smc_frame(close, high, low, close)
        frame.index = index
        result = smc.sessions(frame, "Custom", "00:00", "23:59", "UTC")
        return tuple(result[name].to_numpy()
                     for name in ("Active", "High", "Low"))
    raise LookupError(f"no SMC adapter for {spec.cls.__name__}")

# ---------------------------------------------------------------------------
# Per-function verification
# ---------------------------------------------------------------------------

def verify_function(spec: Spec, data: dict, bars: int, split: int,
                    actual_indices: tuple[int, ...] | None = None) -> dict:
    row: dict = {"function": spec.talib_name or spec.snake,
                 "taflow_class": spec.cls.__name__ if spec.cls else None,
                 "oracle": spec.oracle_source,
                 "oracle_name": spec.oracle_name}
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
        elif spec.smc and spec.cls.__name__ == "Sessions":
            tolerance = {"atol": 2e-5}
        else:
            tolerance = {}
        selected_actual = actual_indices or (
            spec.wickra.actual_indices if spec.wickra else None
        )
        selected_expected = (
            spec.wickra.oracle_indices if spec.wickra else None
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
    checks = [row[k].get("passed", False)
              for k in ("batch_vs_oracle", "continue_vs_oracle") if k in row]
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
    return (f"{mark} (err {block['max_abs_error']:.1e}, "
            f"nan {block['nan_mismatches']})")


def write_report(rows: list[dict], path: Path, bars: int,
                 split: int) -> None:
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
        f"{wickra.__version__}, SMC "
        f"{importlib.metadata.version('smartmoneyconcepts')}, TAFlow "
        f"{getattr(taflow, '__version__', '?')}",
        "",
        "Summary: " + ", ".join(f"{k}: {v}"
                                for k, v in sorted(counts.items())),
        "",
        "TAFlow is driven only through canonical Python classes. The registry",
        "selects TA-Lib, Wickra, explicit NumPy ufunc overrides, then SMC.",
        "*Batch vs oracle*:",
        "cold `extend` over the full series against the reference;",
        "*continue vs batch*: 9k `extend` + 1k `append` stitched output",
        "bitwise-identical to one-shot batch (chunk invariance); *continue",
        "vs oracle*: the stitched output against the reference. Repeated",
        f"native `extend` chunks {list(CHUNK_SIZES)} are also checked bitwise.",
        "",
        "| Function | taflow class | Oracle | Verdict | Batch vs oracle | "
        "Continue vs batch | Extend chunks | Continue vs oracle |",
        "|---|---|---|---|---|---|---|---|",
    ]
    for row in sorted(rows, key=lambda r: (verdict(r) == "MATCH",
                                           r["function"])):
        lines.append(
            f"| {row['function']} | {row.get('taflow_class') or '—'} | "
            f"{row.get('oracle') or '—'} | {verdict(row)} | "
            f"{fmt_check(row.get('batch_vs_oracle'))} | "
            f"{fmt_check(row.get('continue_vs_batch_bitwise'))} | "
            f"{fmt_check(all(row.get('chunk_invariance', {}).values()))} | "
            f"{fmt_check(row.get('continue_vs_oracle'))} |")

    mismatches = [r["function"] for r in rows if verdict(r) == "FAIL"]
    errors = [r["function"] for r in rows if verdict(r) == "ERROR"]
    warned = [r["function"] for r in rows if r.get("warnings")]
    lines += ["", "## Follow-ups", "",
              "- Mismatches: " + (", ".join(mismatches) or "none"),
              "- Errors (class/mapping/runtime): "
              + (", ".join(errors) or "none"),
              "- Compared at TA-Lib defaults only (unmapped params): "
              + (", ".join(warned) or "none")]
    path.write_text("\n".join(lines) + "\n")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("functions", nargs="*")
    parser.add_argument("--bars", type=int, default=10_000)
    parser.add_argument("--warmup-split", type=int, default=9_000)
    parser.add_argument("--report", type=Path,
                        default=VERIFY_DIR / "CORRECTNESS.md")
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
            row = verify_function(spec, data, args.bars,
                                  args.warmup_split)
        except Exception:
            row = {"function": name,
                   "error": traceback.format_exc(limit=1)}
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
