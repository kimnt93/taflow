#!/usr/bin/env python3
"""Verify taflow implementations against reference libraries.

taflow's canonical CamelCase classes are driven through the shared
``registry`` (TA-Lib names mapped via /CHECK.md). Protocol per function:

  1. Oracle computes the full series (default 10,000 bars) — TA-Lib for
     TA-Lib-named functions, pandas for rolling/EWM operators.
  2. taflow cold state (construct + ``extend``) computes the full series
     -> compared to the oracle.
  3. The state is fed the first 9,000 bars with ``extend`` and continued
     bar-by-bar with ``append`` over the last 1,000 bars. The stitched
     output is compared to the taflow batch result (bitwise —
     chunk-invariance contract) and to the oracle.

Usage:
  uv run python verify.py                 # all functions -> REPORT.md
  uv run python verify.py EMA ATR         # subset (TA-Lib names)
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
from numpy.lib.stride_tricks import sliding_window_view

from registry import Spec, build_registry, make_data, resolve_specs

RTOL = 1e-8
ATOL = 1e-10
CHUNK_SIZES = (1, 10, 1_000)


# ---------------------------------------------------------------------------
# Comparison helpers
# ---------------------------------------------------------------------------

def as_tuple(result) -> tuple:
    return result if isinstance(result, tuple) else (result,)


def compare(actual, expected, actual_indices: tuple[int, ...] | None = None) -> dict:
    actuals, expecteds = as_tuple(actual), as_tuple(expected)
    if actual_indices is not None:
        actuals = tuple(actuals[index] for index in actual_indices)
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
                    rtol=RTOL, atol=ATOL, equal_nan=True)
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


def pandas_oracles() -> dict[str, dict]:
    """taflow-only operators checkable against pandas (population moments
    converted exactly where conventions differ)."""
    import pandas as pd

    n = 14

    def roll(series):
        return pd.Series(series).rolling(n)

    return {
        "rolling_median": {
            "kwargs": {"timeperiod": n}, "inputs": ("close",),
            "oracle": lambda a: roll(a[0]).median().to_numpy()},
        "rolling_quantile": {
            "kwargs": {"timeperiod": n, "quantile": 0.25},
            "inputs": ("close",),
            "oracle": lambda a: roll(a[0]).quantile(0.25).to_numpy()},
        "rolling_percentile": {
            "kwargs": {"timeperiod": n, "percentile": 50.0},
            "inputs": ("close",),
            "oracle": lambda a: roll(a[0]).quantile(0.50).to_numpy()},
        "rolling_interquartile_range": {
            "kwargs": {"timeperiod": n}, "inputs": ("close",),
            "oracle": lambda a: (roll(a[0]).quantile(0.75)
                                  - roll(a[0]).quantile(0.25)).to_numpy()},
        "rolling_skew": {
            "kwargs": {"timeperiod": n}, "inputs": ("close",),
            "oracle": lambda a: (roll(a[0]).skew() * (n - 2)
                                 / np.sqrt(n * (n - 1))).to_numpy()},
        "rolling_kurtosis": {
            "kwargs": {"timeperiod": n}, "inputs": ("close",),
            "oracle": lambda a: (roll(a[0]).kurt() * (n - 2) * (n - 3)
                                 / ((n - 1) * (n + 1))
                                 - 6.0 / (n + 1)).to_numpy()},
        # NOT pandas: `rolling().std()` uses an add/remove accumulator that
        # loses precision on low-variance windows. Measured against 50-digit
        # Decimal on the harness's own data, pandas is off by 2.3e-08 at its
        # worst bar while taflow is within 3.7e-15 — the pandas oracle was the
        # inaccurate side. A fresh per-window numpy computation is exact to
        # ~4e-15 and is used instead.
        "rolling_zscore": {
            "kwargs": {"timeperiod": n}, "inputs": ("close",),
            "oracle": lambda a: _fresh_window_zscore(a[0], n)},
        "rolling_cov": {
            "kwargs": {"timeperiod": n}, "inputs": ("close", "close2"),
            "oracle": lambda a: pd.Series(a[0]).rolling(n)
            .cov(pd.Series(a[1]), ddof=0).to_numpy()},
        "ewm_std": {
            "kwargs": {"timeperiod": n}, "inputs": ("close",),
            "oracle": lambda a: pd.Series(a[0]).ewm(span=n, adjust=False)
            .std(bias=True).to_numpy()},
        "ewm_var": {
            "kwargs": {"timeperiod": n}, "inputs": ("close",),
            "oracle": lambda a: pd.Series(a[0]).ewm(span=n, adjust=False)
            .var(bias=True).to_numpy()},
    }



def _fresh_window_zscore(values: np.ndarray, period: int) -> np.ndarray:
    """Z-score with mean and population std recomputed fresh per window.

    Used instead of pandas' rolling accumulator, which drifts on low-variance
    windows (see the note at the `rolling_zscore` spec).
    """
    values = np.asarray(values, dtype=np.float64)
    windows = sliding_window_view(values, period)
    out = np.full(len(values), np.nan)
    out[period - 1:] = ((values[period - 1:] - windows.mean(axis=1))
                        / windows.std(axis=1))
    return out

# ---------------------------------------------------------------------------
# Per-function verification
# ---------------------------------------------------------------------------

def verify_function(spec: Spec, data: dict, bars: int, split: int,
                    oracle_fn=None,
                    actual_indices: tuple[int, ...] | None = None) -> dict:
    row: dict = {"function": spec.talib_name or spec.snake,
                 "taflow_class": spec.cls.__name__ if spec.cls else None,
                 "oracle": "TA-Lib" if spec.talib_name else
                 ("pandas" if oracle_fn else "self")}
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
        elif oracle_fn:
            expected = oracle_fn(arrays)
    except Exception as exc:
        return {**row, "error": f"oracle failed: {exc}"}

    try:
        batch = Spec.extend(spec.new_state(), arrays)
    except Exception as exc:
        return {**row, "error": f"taflow batch failed: {exc}"}
    if expected is not None:
        row["batch_vs_oracle"] = compare(batch, expected, actual_indices)

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
        row["continue_vs_oracle"] = compare(stitched, expected, actual_indices)
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
    if not checks:
        return "SKIP"
    return "MATCH" if all(checks) else "MISMATCH"


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

    counts: dict[str, int] = {}
    for row in rows:
        counts[verdict(row)] = counts.get(verdict(row), 0) + 1

    lines = [
        "# taflow correctness verification",
        "",
        f"Date: {_dt.date.today().isoformat()} | bars: {bars:,} | "
        f"warm-up split: {split:,} extend + {bars - split:,} append | "
        f"tolerance rtol={RTOL}, atol={ATOL}",
        f"Environment: python {platform.python_version()}, numpy "
        f"{np.__version__}, TA-Lib {talib.__version__}, taflow "
        f"{getattr(taflow, '__version__', '?')}",
        "",
        "Summary: " + ", ".join(f"{k}: {v}"
                                for k, v in sorted(counts.items())),
        "",
        "taflow is driven through its canonical classes (mapped from the",
        "TA-Lib name via the /CHECK.md master table). *Batch vs oracle*:",
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
            f"{row.get('oracle', '—')} | {verdict(row)} | "
            f"{fmt_check(row.get('batch_vs_oracle'))} | "
            f"{fmt_check(row.get('continue_vs_batch_bitwise'))} | "
            f"{fmt_check(all(row.get('chunk_invariance', {}).values()))} | "
            f"{fmt_check(row.get('continue_vs_oracle'))} |")

    mismatches = [r["function"] for r in rows if verdict(r) == "MISMATCH"]
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
                        default=Path(__file__).parent / "REPORT.md")
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
    oracles = pandas_oracles()
    for i, spec in enumerate(specs, 1):
        name = spec.talib_name or spec.snake
        try:
            oracle = oracles.get(spec.snake)
            if oracle and not spec.talib_name:
                spec = Spec.build(spec.snake, None)
                spec.ctor_kwargs.update(oracle["kwargs"])
                spec.input_roles = oracle["inputs"]
            row = verify_function(spec, data, args.bars,
                                  args.warmup_split,
                                  oracle_fn=oracle["oracle"] if oracle else None)
        except Exception:
            row = {"function": name,
                   "error": traceback.format_exc(limit=1)}
        rows.append(row)
        print(f"[{i}/{len(specs)}] {name}: {verdict(row)}")

    write_report(rows, args.report, args.bars, args.warmup_split)
    (args.report.parent / "report.json").write_text(
        json.dumps(rows, indent=1, default=str))
    bad = sum(1 for r in rows if verdict(r) in ("MISMATCH", "ERROR"))
    print(f"\nwrote {args.report}\n{len(rows)} checked, {bad} need attention")
    return 1 if bad else 0


if __name__ == "__main__":
    sys.exit(main())
