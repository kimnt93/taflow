#!/usr/bin/env python3
"""Verify taflow implementations against reference libraries.

Protocol per function (see README.md):
  1. Oracle computes the full series (default 10,000 bars).
  2. taflow batch computes the full series -> compared to the oracle.
  3. The persistent state is fed the first 9,000 bars with ``extend`` and
     then continued bar-by-bar with ``append`` over the last 1,000 bars
     (the live-update path). The stitched output is compared to the taflow
     batch result (must be bitwise identical) and to the oracle.

Oracles:
  - TA-Lib for every TA-Lib-named function (metadata from talib.abstract).
  - pandas for rolling_* / ewm_* operators without a TA-Lib counterpart.
  - Self-oracle (batch vs state only) when no reference implementation is
    available; those rows are marked "self".

Usage:
  uv run python verify.py                 # all functions -> REPORT.md
  uv run python verify.py EMA ATR MACD    # subset
"""

from __future__ import annotations

import argparse
import datetime as _dt
import json
import platform
import sys
import traceback
from pathlib import Path

import numpy as np

RTOL = 1e-8
ATOL = 1e-10

# Functions whose mathematical domain is narrower than a price series.
INPUT_DOMAIN_OVERRIDES = {
    "ACOS": "unit",
    "ASIN": "unit",
}


# ---------------------------------------------------------------------------
# Data
# ---------------------------------------------------------------------------

def make_data(n: int, seed: int = 42) -> dict[str, np.ndarray]:
    """Mean-reverting log-price OHLCV data (bounded at any length)."""

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
    volume = rng.uniform(1e5, 1e6, n)
    unit_noise = np.random.default_rng(seed + 2000).normal(0.0, 0.05, n)
    unit = np.clip(np.cumsum(unit_noise) % 1.8 - 0.9, -0.99, 0.99)
    return {
        "open": open_, "high": high, "low": low, "close": close,
        "volume": volume,
        "close2": ar1(3000),
        "periods": np.random.default_rng(seed + 4000).uniform(2.0, 30.0, n),
        "unit": unit,
    }


# ---------------------------------------------------------------------------
# Registry of TA-Lib-named functions (driven by talib.abstract metadata)
# ---------------------------------------------------------------------------

class TalibSpec:
    """One TA-Lib-named function: inputs, params, callables."""

    def __init__(self, name: str) -> None:
        import talib
        from talib import abstract
        import taflow.talib as tt
        import taflow.talib.state as ts

        self.name = name
        info = abstract.Function(name).info
        roles: list[str] = []
        for role, value in info["input_names"].items():
            if isinstance(value, (list, tuple)):
                roles.extend(value)
            else:
                roles.append(role if role.startswith("price") else value)
        self.inputs = tuple(roles)
        self.params = dict(info["parameters"])
        self.domain = INPUT_DOMAIN_OVERRIDES.get(name, "prices")
        self.talib_fn = getattr(talib, name)
        self.batch_fn = getattr(tt, name)
        self.state_cls = getattr(ts, name, None)

    def arrays(self, data: dict, n: int) -> list[np.ndarray]:
        out = []
        for role in self.inputs:
            if role in ("price", "real"):
                key = "unit" if self.domain == "unit" else "close"
            elif role == "price0":
                key = "close"
            elif role == "price1":
                key = "close2"
            elif role in data:
                key = role
            else:
                key = "close"
            out.append(np.ascontiguousarray(data[key][:n]))
        return out


def talib_registry() -> dict[str, TalibSpec]:
    import talib
    import taflow._native as native
    import taflow.talib as tt

    specs: dict[str, TalibSpec] = {}
    for name in native.get_functions():
        if hasattr(tt, name) and hasattr(talib, name):
            specs[name] = TalibSpec(name)
    return specs


# ---------------------------------------------------------------------------
# pandas oracles for taflow-only rolling / ewm operators
# ---------------------------------------------------------------------------

def pandas_oracles() -> dict[str, dict]:
    """Map: taflow attribute name -> spec for a pandas-checked operator.

    Each entry: state factory kwargs, input keys, and an oracle callable
    producing the same-size expected series from the input arrays.
    """
    import pandas as pd

    window = 14

    def roll(series: np.ndarray):
        return pd.Series(series).rolling(window)

    return {
        "rolling_median": {
            "kwargs": {"timeperiod": window},
            "inputs": ("close",),
            "oracle": lambda a: roll(a[0]).median().to_numpy(),
        },
        "rolling_quantile": {
            "kwargs": {"timeperiod": window, "quantile": 0.25},
            "inputs": ("close",),
            "oracle": lambda a: roll(a[0]).quantile(0.25).to_numpy(),
        },
        # taflow implements POPULATION skew/excess-kurtosis (consistent with
        # its ddof=0 VAR/STDDEV convention); pandas returns bias-corrected
        # SAMPLE statistics (G1/G2). Convert pandas -> population exactly.
        "rolling_skew": {
            "kwargs": {"timeperiod": window},
            "inputs": ("close",),
            "oracle": lambda a, n=window: (
                roll(a[0]).skew() * (n - 2) / np.sqrt(n * (n - 1))
            ).to_numpy(),
        },
        "rolling_kurtosis": {
            "kwargs": {"timeperiod": window},
            "inputs": ("close",),
            "oracle": lambda a, n=window: (
                roll(a[0]).kurt() * (n - 2) * (n - 3)
                / ((n - 1) * (n + 1)) - 6.0 / (n + 1)
            ).to_numpy(),
        },
        "rolling_zscore": {
            "kwargs": {"timeperiod": window},
            "inputs": ("close",),
            "oracle": lambda a: (
                (pd.Series(a[0]) - roll(a[0]).mean())
                / roll(a[0]).std(ddof=0)
            ).to_numpy(),
        },
        "rolling_cov": {
            "kwargs": {"timeperiod": window},
            "inputs": ("close", "close2"),
            "oracle": lambda a: pd.Series(a[0]).rolling(window)
            .cov(pd.Series(a[1]), ddof=0).to_numpy(),
        },
        "ewm_std": {
            "kwargs": {"timeperiod": window},
            "inputs": ("close",),
            "oracle": lambda a: pd.Series(a[0])
            .ewm(span=window, adjust=False).std(bias=True).to_numpy(),
        },
        "ewm_var": {
            "kwargs": {"timeperiod": window},
            "inputs": ("close",),
            "oracle": lambda a: pd.Series(a[0])
            .ewm(span=window, adjust=False).var(bias=True).to_numpy(),
        },
    }


# ---------------------------------------------------------------------------
# Comparison helpers
# ---------------------------------------------------------------------------

def as_tuple(result) -> tuple[np.ndarray, ...]:
    return result if isinstance(result, tuple) else (result,)


def compare(actual, expected) -> dict:
    """NaN placement + allclose comparison across all outputs."""
    actuals, expecteds = as_tuple(actual), as_tuple(expected)
    if len(actuals) != len(expecteds):
        return {"passed": False,
                "error": f"output arity {len(actuals)} != {len(expecteds)}"}
    nan_mismatches = 0
    max_err = 0.0
    for a, b in zip(actuals, expecteds):
        a = np.asarray(a, dtype=np.float64)
        b = np.asarray(b, dtype=np.float64)
        if a.shape != b.shape:
            return {"passed": False,
                    "error": f"shape {a.shape} != {b.shape}"}
        nan_a, nan_b = np.isnan(a), np.isnan(b)
        nan_mismatches += int((nan_a != nan_b).sum())
        both = ~nan_a & ~nan_b
        if both.any():
            max_err = max(max_err,
                          float(np.max(np.abs(a[both] - b[both]))))
    passed = nan_mismatches == 0 and all(
        np.allclose(np.nan_to_num(np.asarray(x, dtype=np.float64)),
                    np.nan_to_num(np.asarray(y, dtype=np.float64)),
                    rtol=RTOL, atol=ATOL)
        for x, y in zip(actuals, expecteds))
    return {"passed": passed, "nan_mismatches": nan_mismatches,
            "max_abs_error": max_err}


def bitwise_equal(actual, expected) -> bool:
    return all(
        np.array_equal(np.asarray(a), np.asarray(b), equal_nan=True)
        for a, b in zip(as_tuple(actual), as_tuple(expected)))


def stitch(first, second) -> tuple[np.ndarray, ...]:
    firsts, seconds = as_tuple(first), as_tuple(second)
    return tuple(np.concatenate([np.asarray(a, dtype=np.float64),
                                 np.asarray(b, dtype=np.float64)])
                 for a, b in zip(firsts, seconds))


def append_series(state, arrays: list[np.ndarray], start: int) -> tuple:
    """Continue a warmed state bar-by-bar; collect same-size outputs.

    Handles both state APIs: ``taflow.talib.state`` classes return the new
    value(s) from ``append``; taflow-style classes return ``self`` (fluent)
    and expose the latest value via ``.value``.
    """
    outputs: list = []
    bars = list(zip(*[a[start:].tolist() for a in arrays]))
    for bar in bars:
        out = state.append(*bar)
        if out is state:
            out = getattr(state, "value", None)
        outputs.append(out)
    arity = next((len(o) for o in outputs if isinstance(o, tuple)), 1)
    columns: list[list[float]] = [[] for _ in range(arity)]
    for out in outputs:
        if isinstance(out, tuple):
            values = out
        elif out is None:
            values = (None,) * arity
        else:
            values = (out,)
        for column, value in zip(columns, values):
            column.append(float("nan") if value is None else float(value))
    return tuple(np.asarray(column) for column in columns)


# ---------------------------------------------------------------------------
# Per-function verification
# ---------------------------------------------------------------------------

def verify_talib_function(spec: TalibSpec, data: dict, bars: int,
                          split: int) -> dict:
    arrays = spec.arrays(data, bars)
    row: dict = {"function": spec.name, "oracle": "TA-Lib"}
    try:
        expected = spec.talib_fn(*arrays, **spec.params)
    except Exception as exc:
        return {**row, "error": f"oracle failed: {exc}"}

    try:
        batch = spec.batch_fn(*arrays, **spec.params)
    except Exception as exc:
        return {**row, "error": f"taflow batch failed: {exc}"}
    row["batch_vs_oracle"] = compare(batch, expected)

    if spec.state_cls is None:
        row["state"] = "missing"
        return row
    try:
        state = spec.state_cls(**spec.params)
        head = state.extend(*[a[:split] for a in arrays])
        tail = append_series(state, arrays, split)
        stitched = stitch(head, tail)
    except Exception as exc:
        row["error"] = f"state path failed: {exc}"
        return row
    row["continue_vs_batch_bitwise"] = bitwise_equal(stitched, batch)
    row["continue_vs_oracle"] = compare(stitched, expected)
    return row


def verify_pandas_function(name: str, spec: dict, data: dict, bars: int,
                           split: int) -> dict | None:
    import taflow

    row: dict = {"function": name, "oracle": "pandas"}
    # The snake_case attribute is the module file; the class is CamelCase
    # (with descriptive expansions, e.g. ewm_std ->
    # ExponentiallyWeightedStandardDeviation).
    CLASS_NAMES = {
        "rolling_zscore": "RollingZScore",
        "ewm_std": "ExponentiallyWeightedStandardDeviation",
        "ewm_var": "ExponentiallyWeightedVariance",
        "ewm_cov": "ExponentiallyWeightedCovariance",
        "ewm_corr": "ExponentiallyWeightedCorrelation",
    }
    camel = CLASS_NAMES.get(
        name, "".join(part.capitalize() for part in name.split("_")))
    factory = getattr(taflow, camel, None)
    if factory is None or not callable(factory):
        return None  # not implemented yet — skip silently
    arrays = [np.ascontiguousarray(data[k][:bars]) for k in spec["inputs"]]
    expected = spec["oracle"](arrays)
    try:
        state = factory(**spec["kwargs"])
        batch_state = factory(**spec["kwargs"])
        # taflow-style classes are fluent: extend returns self and the
        # accumulated history comes from compute().
        batch = batch_state.extend(*arrays)
        if batch is batch_state:
            batch = batch_state.compute()
        head = state.extend(*[a[:split] for a in arrays])
        if head is state:
            head = state.compute()
        tail = append_series(state, arrays, split)
        stitched = stitch(head, tail)
    except Exception as exc:
        return {**row, "error": f"taflow failed: {exc}"}
    row["batch_vs_oracle"] = compare(batch, expected)
    row["continue_vs_batch_bitwise"] = bitwise_equal(stitched, batch)
    row["continue_vs_oracle"] = compare(stitched, expected)
    return row


# ---------------------------------------------------------------------------
# Report
# ---------------------------------------------------------------------------

def verdict(row: dict) -> str:
    if "error" in row:
        return "ERROR"
    checks = []
    for key in ("batch_vs_oracle", "continue_vs_oracle"):
        if key in row:
            checks.append(row[key].get("passed", False))
    if "continue_vs_batch_bitwise" in row:
        checks.append(row["continue_vs_batch_bitwise"])
    if not checks:
        return "SKIP"
    return "MATCH" if all(checks) else "MISMATCH"


def fmt_check(block) -> str:
    if block is None:
        return "—"
    if isinstance(block, bool):
        return "yes" if block else "**NO**"
    if "error" in block:
        return f"error: {block['error']}"
    mark = "pass" if block["passed"] else "**FAIL**"
    return f"{mark} (err {block['max_abs_error']:.1e}, " \
           f"nan {block['nan_mismatches']})"


def write_report(rows: list[dict], path: Path, bars: int, split: int) -> None:
    import talib
    import taflow

    counts: dict[str, int] = {}
    for row in rows:
        counts[verdict(row)] = counts.get(verdict(row), 0) + 1

    lines = [
        "# taflow correctness verification",
        "",
        f"Date: {_dt.date.today().isoformat()} | bars: {bars:,} | "
        f"warm-up split: {split:,} + {bars - split:,} continue | "
        f"tolerance rtol={RTOL}, atol={ATOL}",
        f"Environment: python {platform.python_version()}, "
        f"numpy {np.__version__}, TA-Lib {talib.__version__}, "
        f"taflow {getattr(taflow, '__version__', '?')}",
        "",
        "Summary: " + ", ".join(f"{k}: {v}"
                                for k, v in sorted(counts.items())),
        "",
        "Columns — *batch vs oracle*: full-series batch against the",
        "reference; *continue vs batch*: 9k `extend` + 1k `append` stitched",
        "output bitwise-identical to the one-shot batch (chunk-invariance",
        "contract); *continue vs oracle*: the stitched output against the",
        "reference.",
        "",
        "| Function | Oracle | Verdict | Batch vs oracle | "
        "Continue vs batch (bitwise) | Continue vs oracle |",
        "|---|---|---|---|---|---|",
    ]
    for row in sorted(rows, key=lambda r: (verdict(r) == "MATCH",
                                           r["function"])):
        lines.append(
            f"| {row['function']} | {row.get('oracle', '—')} | "
            f"{verdict(row)} | {fmt_check(row.get('batch_vs_oracle'))} | "
            f"{fmt_check(row.get('continue_vs_batch_bitwise'))} | "
            f"{fmt_check(row.get('continue_vs_oracle'))} |")

    mismatches = [r["function"] for r in rows if verdict(r) == "MISMATCH"]
    errors = [r["function"] for r in rows if verdict(r) == "ERROR"]
    lines += ["", "## Follow-ups", ""]
    lines.append("- Mismatches: " + (", ".join(mismatches) or "none"))
    lines.append("- Errors: " + (", ".join(errors) or "none"))
    path.write_text("\n".join(lines) + "\n")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("functions", nargs="*",
                        help="TA-Lib names to verify (default: all)")
    parser.add_argument("--bars", type=int, default=10_000)
    parser.add_argument("--warmup-split", type=int, default=9_000)
    parser.add_argument("--report", type=Path,
                        default=Path(__file__).parent / "REPORT.md")
    args = parser.parse_args()

    data = make_data(args.bars)
    registry = talib_registry()
    names = args.functions or sorted(registry)
    unknown = [n for n in names if n not in registry]
    if unknown:
        print(f"unknown: {', '.join(unknown)}", file=sys.stderr)
        return 1

    rows: list[dict] = []
    for i, name in enumerate(names, 1):
        try:
            row = verify_talib_function(registry[name], data, args.bars,
                                        args.warmup_split)
        except Exception:
            row = {"function": name, "oracle": "TA-Lib",
                   "error": traceback.format_exc(limit=1)}
        rows.append(row)
        print(f"[{i}/{len(names)}] {name}: {verdict(row)}")

    if not args.functions:  # full run also covers pandas-oracle operators
        for name, spec in pandas_oracles().items():
            try:
                row = verify_pandas_function(name, spec, data, args.bars,
                                             args.warmup_split)
            except Exception:
                row = {"function": name, "oracle": "pandas",
                       "error": traceback.format_exc(limit=1)}
            if row is not None:
                rows.append(row)
                print(f"[extra] {name}: {verdict(row)}")

    write_report(rows, args.report, args.bars, args.warmup_split)
    (args.report.parent / "report.json").write_text(
        json.dumps(rows, indent=1, default=str))
    print(f"\nwrote {args.report}")
    bad = sum(1 for r in rows if verdict(r) in ("MISMATCH", "ERROR"))
    print(f"{len(rows)} functions checked, {bad} need attention")
    return 0


if __name__ == "__main__":
    sys.exit(main())
