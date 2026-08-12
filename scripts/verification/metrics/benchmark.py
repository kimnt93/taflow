"""Correctness-gated benchmark of public TAFlow metrics and their oracles.

Benchmarking is deliberately disabled unless ``--run`` is supplied.  The
same registered oracle used for correctness is always the published peer.
"""

from __future__ import annotations

import argparse
import gc
import json
import os
import statistics
import tempfile
import time
from datetime import UTC, datetime
from pathlib import Path
from typing import Callable

import numpy as np

try:
    from .correctness import oracle_result, require_oracle, verify_metric
    from .registry import (
        BENCHMARK_EVIDENCE_DIR,
        VERIFY_DIR,
        MetricSpec,
        resolve_specs,
    )
except ImportError:
    from correctness import (  # type: ignore[no-redef]
        oracle_result,
        require_oracle,
        verify_metric,
    )
    from registry import (
        BENCHMARK_EVIDENCE_DIR,
        VERIFY_DIR,
        MetricSpec,
        resolve_specs,
    )  # type: ignore[no-redef]

DEFAULT_SIZES = (1_000, 10_000, 100_000)


def timed(call: Callable[[], object], repeats: int) -> dict[str, object]:
    call()
    iterations = 1
    while iterations < 65_536:
        start = time.perf_counter_ns()
        for _ in range(iterations):
            call()
        if time.perf_counter_ns() - start >= 5_000_000:
            break
        iterations *= 2
    samples: list[float] = []
    for _ in range(repeats):
        gc.disable()
        try:
            start = time.perf_counter_ns()
            for _ in range(iterations):
                call()
            samples.append((time.perf_counter_ns() - start) / 1e9 / iterations)
        finally:
            gc.enable()
    return {
        "median_seconds": statistics.median(samples),
        "median_absolute_deviation_seconds": statistics.median(
            abs(value - statistics.median(samples)) for value in samples
        ),
        "iterations_per_sample": iterations,
        "samples_seconds": samples,
    }


def timed_setup_action(
    setup: Callable[[], object], action: Callable[[object], object], repeats: int
) -> dict[str, object]:
    """Time only an action while rebuilding equivalent state before each sample."""
    samples: list[float] = []
    for _ in range(repeats):
        state = setup()
        gc.disable()
        try:
            start = time.perf_counter_ns()
            action(state)
            samples.append((time.perf_counter_ns() - start) / 1e9)
        finally:
            gc.enable()
    median = statistics.median(samples)
    return {
        "median_seconds": median,
        "median_absolute_deviation_seconds": statistics.median(
            abs(value - median) for value in samples
        ),
        "iterations_per_sample": 1,
        "samples_seconds": samples,
    }


def benchmark_sharpe_execution_profiles(
    cls: type, values: np.ndarray, kwargs: dict[str, object], repeats: int
) -> dict[str, object]:
    """Measure representative boundary, continuation, chunk, and conversion costs."""
    import pandas as pd
    import polars as pl
    import pyarrow as pa

    empty = np.array([], dtype=np.float64)
    half = values.size // 2
    profiles: dict[str, object] = {}
    profiles["native_bulk"] = timed_setup_action(
        lambda: cls.from_returns(empty, **kwargs)._state,
        lambda state: (state.extend(values), state.compute()),
        repeats,
    )
    for chunk_size in (32, 1_024):
        profiles[f"chunks_{chunk_size}"] = timed_setup_action(
            lambda: cls.from_returns(empty, **kwargs),
            lambda state, size=chunk_size: [
                state.extend(values[offset : offset + size])
                for offset in range(0, values.size, size)
            ],
            repeats,
        )
    profiles["scalar_append"] = timed_setup_action(
        lambda: cls.from_returns(empty, **kwargs),
        lambda state: [state.append(float(value)) for value in values],
        repeats,
    )
    profiles["warmed_continuation"] = timed_setup_action(
        lambda: cls.from_returns(values[:half], **kwargs),
        lambda state: state.extend(values[half:]),
        repeats,
    )
    cached = cls.from_returns(values, **kwargs)
    profiles["cached_compute"] = timed(cached.compute, repeats)
    containers = {
        "numpy": values,
        "list": values.tolist(),
        "pandas": pd.Series(values),
        "polars": pl.Series("returns", values),
        "arrow": pa.array(values),
    }
    profiles["container_end_to_end"] = {
        name: timed(
            lambda container=container: cls.from_returns(container, **kwargs).compute(),
            repeats,
        )
        for name, container in containers.items()
    }
    return profiles


def benchmark_metric(
    spec: MetricSpec, sizes: tuple[int, ...], repeats: int
) -> dict[str, object]:
    correctness = verify_metric(spec)
    if not correctness["passed"]:
        raise AssertionError(f"correctness gate failed for {spec.class_name}")
    cls = spec.load_class()
    oracle = require_oracle(spec)
    row = next(
        (candidate for candidate in spec.parameter_rows if candidate.name == "default"),
        spec.parameter_rows[0],
    )
    public_kwargs = row.as_kwargs()
    rng = np.random.default_rng(20_260_811)
    rows = []
    execution_profiles = None
    for size in sizes:
        values = np.ascontiguousarray(rng.normal(0.0004, 0.012, size), dtype=np.float64)
        if spec.class_name == "EffectiveNumberOfBets":
            values = np.abs(values)
        if spec.paired:
            benchmark_values = np.ascontiguousarray(
                values * 0.35 + rng.normal(0.0, 0.008, size), dtype=np.float64
            )
            taflow = timed(
                lambda input_values=values, benchmark_input=benchmark_values: cls.from_returns(
                    input_values, benchmark_input, **public_kwargs
                ).compute(),
                repeats,
            )
            reference = timed(
                lambda input_values=values, benchmark_input=benchmark_values: oracle_result(
                    spec, oracle, row, input_values, benchmark_input
                ),
                repeats,
            )
        else:
            factory = getattr(cls, spec.factories[0])
            taflow = timed(
                lambda input_values=values, public_factory=factory: public_factory(
                    input_values, **public_kwargs
                ).compute(),
                repeats,
            )
            reference = timed(
                lambda input_values=values: oracle_result(
                    spec, oracle, row, input_values
                ),
                repeats,
            )
        taflow_seconds = float(taflow["median_seconds"])
        reference_seconds = float(reference["median_seconds"])
        rows.append(
            {
                "observations": size,
                "taflow_public": taflow,
                "oracle": reference,
                "speedup": reference_seconds / taflow_seconds,
                "taflow_observations_per_second": size / taflow_seconds,
            }
        )
        if spec.class_name == "SharpeRatio" and size == min(100_000, max(sizes)):
            execution_profiles = benchmark_sharpe_execution_profiles(
                cls, values, public_kwargs, repeats
            )
    source_distribution, source_version = spec.oracle.source_package
    return {
        "class": spec.class_name,
        "oracle_distribution": source_distribution,
        "oracle_version": source_version,
        "oracle_function": spec.oracle.function,
        "oracle_source_function": spec.oracle.source_function_name,
        "oracle_source": spec.oracle.source_url,
        "correctness_gate": correctness["verdict"],
        "rows": rows,
        "execution_profiles": execution_profiles,
    }


def _atomic_write(path: Path, content: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    descriptor, temporary_name = tempfile.mkstemp(
        dir=path.parent, prefix=f".{path.name}."
    )
    try:
        with os.fdopen(descriptor, "w", encoding="utf-8") as temporary:
            temporary.write(content)
        os.replace(temporary_name, path)
    except BaseException:
        Path(temporary_name).unlink(missing_ok=True)
        raise


def write_results(results: list[dict[str, object]]) -> None:
    BENCHMARK_EVIDENCE_DIR.mkdir(parents=True, exist_ok=True)
    for result in results:
        _atomic_write(
            BENCHMARK_EVIDENCE_DIR / f"{result['class']}.json",
            json.dumps(result, indent=2, sort_keys=True) + "\n",
        )
    lines = [
        "# Metrics benchmark",
        "",
        f"Generated: {datetime.now(UTC).date().isoformat()}",
        "",
        f"Public end-to-end semantic-factory `compute()` timings for {len(results)} benchmark-eligible metrics; every row passed the external correctness gate first.",
        "",
        "Speedup is reference time divided by TAFlow time; values above 1× favor TAFlow.",
        "",
        "Reference libraries: [empyrical-reloaded](https://github.com/stefan-jansen/empyrical-reloaded), "
        "[QuantStats](https://github.com/ranaroussi/quantstats), "
        "[NumPy](https://numpy.org/), [SciPy](https://scipy.org/), "
        "[PerformanceAnalytics](https://cran.r-project.org/package=PerformanceAnalytics), "
        "[vectorbt](https://vectorbt.dev/), and "
        "[Riskfolio-Lib](https://riskfolio-lib.readthedocs.io/).",
        "",
        "| **Class** | **Target** | **1k** | **10k** | **100k** |",
        "|---|---|---:|---:|---:|",
    ]
    for result in results:
        by_size = {row["observations"]: row for row in result["rows"]}  # type: ignore[index]
        cells = [
            f"{by_size[size]['speedup']:.2f}x" if size in by_size else "—"
            for size in DEFAULT_SIZES
        ]
        lines.append(
            f"| {result['class']} | {result['oracle_distribution']} "
            f"{result['oracle_version']} | " + " | ".join(cells) + " |"
        )
    _atomic_write(VERIFY_DIR / "BENCHMARK.md", "\n".join(lines) + "\n")


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("metrics", nargs="*", help="canonical metric class names")
    parser.add_argument(
        "--metric",
        action="append",
        default=[],
        help="canonical metric class name (repeatable)",
    )
    parser.add_argument(
        "--sizes",
        default=",".join(map(str, DEFAULT_SIZES)),
        help="comma-separated observation counts",
    )
    parser.add_argument("--repeats", type=int, default=9)
    parser.add_argument(
        "--run",
        action="store_true",
        help="required explicit authorization to execute benchmarks",
    )
    parser.add_argument(
        "--list", action="store_true", help="list benchmark-eligible registry entries"
    )
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    requested = args.metrics + args.metric
    specs = [
        spec
        for spec in resolve_specs(requested or None, available_only=not requested)
        if spec.benchmark_eligible
    ]
    if args.list:
        for spec in specs:
            print(
                f"{spec.class_name}\t{spec.oracle.distribution}:{spec.oracle.function}"
            )
        return 0
    if not args.run:
        raise SystemExit(
            "benchmark disabled: pass --run only after explicit authorization"
        )
    if not specs:
        raise RuntimeError("no implemented benchmark-eligible metrics found")
    sizes = tuple(int(value) for value in args.sizes.split(","))
    if any(size <= 0 for size in sizes) or args.repeats < 1:
        raise ValueError("sizes and repeats must be positive")
    results = [benchmark_metric(spec, sizes, args.repeats) for spec in specs]
    write_results(results)
    print(f"benchmarked {len(results)} correctness-gated metrics")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
