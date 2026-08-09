#!/usr/bin/env python3
"""Focused TAFlow, independent-pandas, and Wickra benchmark comparisons."""

from __future__ import annotations

import argparse
import importlib.metadata
import json
import platform
import statistics
import time
from datetime import date
from pathlib import Path

import numpy as np
import pandas as pd
import talib
import taflow
import wickra


SIZES = (1_000, 10_000, 100_000, 1_000_000)
HERE = Path(__file__).parent


def pandas_reference(equity: np.ndarray, period: int) -> np.ndarray:
    """Independent pandas rolling-apply maximum-drawdown oracle."""

    return pd.Series(equity).rolling(period).apply(
        lambda window: np.max(np.divide(
            np.maximum.accumulate(window) - window,
            np.maximum.accumulate(window),
            out=np.zeros_like(window),
            where=np.maximum.accumulate(window) > 0.0,
        )),
        raw=True,
    ).to_numpy()


def elapsed_ms(call, repeats: int) -> float:
    call()
    samples = []
    for _ in range(repeats):
        start = time.perf_counter_ns()
        call()
        samples.append((time.perf_counter_ns() - start) / 1e6)
    return statistics.median(samples)


def render(report: dict) -> str:
    lines = [
        "# RollingMaximumDrawdown focused comparison",
        "",
        "Correctness: **MATCH** against independent pandas and Wickra 0.9.9. "
        "TA-Lib: **N/A** (no `MAXDRAWDOWN` function).",
        "",
        "| Bars | TAFlow ms | pandas ms | Wickra ms | vs pandas | vs Wickra |",
        "|---:|---:|---:|---:|---:|---:|",
    ]
    for row in report["rows"]:
        lines.append(
            f"| {row['bars']:,} | {row['taflow_ms']:.3f} | "
            f"{row['pandas_ms']:.3f} | {row['wickra_ms']:.3f} | "
            f"{row['speedup_vs_pandas']:.2f}× | {row['speedup_vs_wickra']:.2f}× |"
        )
    lines += [
        "",
        "Times are median fresh-state class/batch calls after one timing warm-up over "
        "deterministic positive equity data; "
        "speedup is reference time divided by TAFlow time.",
        "",
    ]
    return "\n".join(lines)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repeats", type=int, default=3)
    parser.add_argument("--period", type=int, default=14)
    parser.add_argument(
        "--output",
        type=Path,
        default=HERE / "benchmark_reports" / "rolling_maximum_drawdown_wickra.json",
    )
    args = parser.parse_args()

    rng = np.random.default_rng(20260810)
    equity = 100.0 * np.exp(np.cumsum(rng.normal(0.0, 0.002, max(SIZES))))
    rows = []
    for size in SIZES:
        values = np.ascontiguousarray(equity[:size])
        actual = taflow.RollingMaximumDrawdown(values, args.period).compute()
        pandas_expected = pandas_reference(values, args.period)
        wickra_expected = np.asarray(
            wickra.MaxDrawdown(args.period).batch(values), dtype=np.float64
        )
        np.testing.assert_allclose(
            actual, pandas_expected, rtol=1e-12, atol=1e-12, equal_nan=True
        )
        np.testing.assert_allclose(
            actual, wickra_expected, rtol=1e-12, atol=1e-12, equal_nan=True
        )
        taflow_ms = elapsed_ms(
            lambda: taflow.RollingMaximumDrawdown(values, args.period).compute(),
            args.repeats,
        )
        pandas_ms = elapsed_ms(
            lambda: pandas_reference(values, args.period), args.repeats
        )
        wickra_ms = elapsed_ms(
            lambda: wickra.MaxDrawdown(args.period).batch(values), args.repeats
        )
        rows.append({
            "bars": size,
            "taflow_ms": taflow_ms,
            "pandas_ms": pandas_ms,
            "wickra_ms": wickra_ms,
            "speedup_vs_pandas": pandas_ms / taflow_ms,
            "speedup_vs_wickra": wickra_ms / taflow_ms,
        })

    report = {
        "date": date.today().isoformat(),
        "class": "RollingMaximumDrawdown",
        "wickra_name": "MaxDrawdown",
        "period": args.period,
        "repeats": args.repeats,
        "correctness": {"pandas": "MATCH", "wickra": "MATCH", "talib": "N/A"},
        "versions": {
            package: importlib.metadata.version(package)
            for package in ("taflow", "pandas", "wickra", "TA-Lib")
        },
        "environment": {
            "platform": platform.platform(),
            "python": platform.python_version(),
        },
        "talib_has_maximum_drawdown": "MAXDRAWDOWN" in talib.get_functions(),
        "rows": rows,
    }
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(report, indent=2) + "\n")
    args.output.with_suffix(".md").write_text(render(report))
    print(render(report))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
