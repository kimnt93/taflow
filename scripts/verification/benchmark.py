#!/usr/bin/env python3
"""Benchmark correctness-gated TAFlow classes against registered oracles.

The target priority is TA-Lib, Wickra, explicit NumPy overrides, then SMC.
Every per-indicator report includes whole-vector API/kernel measurements and
fresh-state warm-up at 1, 5, and 10 threads. Raw samples remain in JSON.
"""

from __future__ import annotations

import argparse
import gc
import importlib.metadata
import json
import platform
import statistics
import sys
import threading
import time
from concurrent.futures import ThreadPoolExecutor
from datetime import date
from typing import Callable

import numpy as np

from correctness import (
    numpy_oracle,
    smc_oracle,
    talib_oracle,
    verdict,
    verify_function,
    wickra_oracle,
)
from registry import (
    BENCHMARK_EVIDENCE_DIR,
    VERIFY_DIR,
    Spec,
    build_registry,
    make_data,
    resolve_specs,
)

SCHEMA_VERSION = 3
DEFAULT_SIZES = (1_000, 10_000, 100_000, 1_000_000)
DEFAULT_WARMUP_SIZES = (1, 10, 100, 1_000)
DEFAULT_THREADS = (1, 5, 10)


def oracle_call(spec: Spec, arrays: list[np.ndarray]):
    """Invoke the highest-priority external batch implementation."""
    if spec.talib_name:
        return talib_oracle(spec, arrays)
    if spec.wickra:
        return wickra_oracle(spec, arrays)
    if spec.numpy:
        return numpy_oracle(spec, arrays)
    if spec.smc:
        return smc_oracle(spec, arrays)
    raise LookupError(f"no external oracle for {spec.cls.__name__}")


def taflow_api_call(spec: Spec, arrays: list[np.ndarray]):
    """Run construct, public ``extend``, and public ``compute``."""
    state = spec.new_state()
    state.extend(*arrays)
    return state.compute()


def taflow_kernel_call(spec: Spec, arrays: list[np.ndarray]):
    """Run the bound native state's bulk method without history conversion."""
    state = spec.new_state()
    native = getattr(state, "_state", state)
    return native.extend(*arrays)


def _summary(samples: list[float], iterations: int = 1) -> dict:
    """Summarize seconds-per-call samples while preserving their raw values."""
    return {
        "mean_ms": statistics.fmean(samples) * 1_000.0,
        "median_ms": statistics.median(samples) * 1_000.0,
        "min_ms": min(samples) * 1_000.0,
        "iterations_per_sample": iterations,
        "samples_s": samples,
    }


def timed(call: Callable[[], object], repeats: int) -> dict:
    """Warm, autorange short calls, and retain per-call timing samples."""
    call()
    iterations = 1
    while True:
        start = time.perf_counter_ns()
        for _ in range(iterations):
            call()
        elapsed = (time.perf_counter_ns() - start) / 1e9
        if elapsed >= 0.005 or iterations >= 1 << 16:
            break
        iterations *= 2
    samples = []
    for _ in range(repeats):
        gc.disable()
        start = time.perf_counter_ns()
        for _ in range(iterations):
            call()
        samples.append((time.perf_counter_ns() - start) / 1e9 / iterations)
        gc.enable()
    return _summary(samples, iterations)


def vector_results(
    spec: Spec,
    data: dict[str, np.ndarray],
    sizes: tuple[int, ...],
    repeats: int,
) -> list[dict]:
    """Measure public API, native kernel, and reference batch paths."""
    rows = []
    for bars in sizes:
        arrays = spec.arrays(data, bars)
        api = timed(lambda: taflow_api_call(spec, arrays), repeats)
        kernel = timed(lambda: taflow_kernel_call(spec, arrays), repeats)
        reference = timed(lambda: oracle_call(spec, arrays), repeats)
        api["bars_per_second"] = bars / (api["mean_ms"] / 1_000.0)
        kernel["bars_per_second"] = bars / (kernel["mean_ms"] / 1_000.0)
        rows.append({
            "bars": bars,
            "taflow_api": api,
            "taflow_kernel": kernel,
            "reference": reference,
            "api_speedup": reference["mean_ms"] / api["mean_ms"],
            "kernel_speedup": reference["mean_ms"] / kernel["mean_ms"],
        })
    return rows


def _thread_wall(calls: list[Callable[[], object]]) -> float:
    """Measure independent calls released simultaneously from one barrier."""
    barrier = threading.Barrier(len(calls) + 1)

    def run(call: Callable[[], object]) -> None:
        barrier.wait()
        call()

    with ThreadPoolExecutor(max_workers=len(calls)) as pool:
        futures = [pool.submit(run, call) for call in calls]
        start = time.perf_counter_ns()
        barrier.wait()
        for future in futures:
            future.result()
    return (time.perf_counter_ns() - start) / 1e9


def warmup_results(
    spec: Spec,
    data: dict[str, np.ndarray],
    sizes: tuple[int, ...],
    thread_counts: tuple[int, ...],
    repeats: int,
) -> list[dict]:
    """Measure fresh native states and references at each bars/thread point."""
    rows = []
    for bars in sizes:
        arrays = spec.arrays(data, bars)
        for threads in thread_counts:
            taflow_samples = [
                _thread_wall([
                    lambda: taflow_kernel_call(spec, arrays)
                    for _ in range(threads)
                ])
                for _ in range(min(repeats, 3))
            ]
            reference_samples = [
                _thread_wall([
                    lambda: oracle_call(spec, arrays)
                    for _ in range(threads)
                ])
                for _ in range(min(repeats, 3))
            ]
            taflow = _summary(taflow_samples)
            reference = _summary(reference_samples)
            rows.append({
                "bars": bars,
                "threads": threads,
                "taflow": taflow,
                "reference": reference,
                "speedup": reference["mean_ms"] / taflow["mean_ms"],
            })
    return rows


def environment() -> dict:
    """Record dependency and host versions needed to interpret timings."""
    import taflow
    import talib
    import wickra

    return {
        "date": date.today().isoformat(),
        "platform": platform.platform(),
        "python": platform.python_version(),
        "numpy": np.__version__,
        "taflow": getattr(taflow, "__version__", "unknown"),
        "talib": talib.__version__,
        "wickra": wickra.__version__,
        "smartmoneyconcepts": importlib.metadata.version("smartmoneyconcepts"),
    }


def _rate(value: float) -> str:
    """Format a bars-per-second value with a compact SI suffix."""
    for suffix, scale in (("G", 1e9), ("M", 1e6), ("K", 1e3)):
        if value >= scale:
            return f"{value / scale:.2f}{suffix}"
    return f"{value:.1f}"


def render_evidence(report: dict) -> str:
    """Render one complete per-indicator benchmark report."""
    alias = report["oracle_name"]
    lines = [
        f"# {report['canonical_class']} benchmark (`{alias}` oracle)",
        "",
        "Correctness: **MATCH**.",
        "",
        "taflow class.extend over contiguous NumPy arrays; this exercises the "
        "compiled Rust bulk/SIMD-capable path. SIMD availability and target "
        "features depend on the installed wheel/build flags.",
        "",
        "## Whole-vector performance",
        "",
        "| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | "
        "Reference ms | API speedup | Kernel speedup |",
        "|---:|---:|---:|---:|---:|---:|---:|---:|",
    ]
    for row in report["vector"]:
        lines.append(
            f"| {row['bars']:,} | {row['taflow_api']['mean_ms']:.3f} | "
            f"{_rate(row['taflow_api']['bars_per_second'])} | "
            f"{row['taflow_kernel']['mean_ms']:.3f} | "
            f"{_rate(row['taflow_kernel']['bars_per_second'])} | "
            f"{row['reference']['mean_ms']:.3f} | "
            f"{row['api_speedup']:.2f}× | {row['kernel_speedup']:.2f}× |"
        )
    lines += [
        "",
        "## Fresh-state warm-up",
        "",
        "| Bars | Threads | TAFlow ms | Reference ms | Speedup |",
        "|---:|---:|---:|---:|---:|",
    ]
    for row in report["warmup"]:
        lines.append(
            f"| {row['bars']:,} | {row['threads']} | "
            f"{row['taflow']['mean_ms']:.3f} | "
            f"{row['reference']['mean_ms']:.3f} | {row['speedup']:.2f}× |"
        )
    lines += [
        "", "---",
        "Times include Python conversion/binding overhead. Raw samples are retained in JSON.",
        "",
    ]
    return "\n".join(lines)


def render_aggregate(reports: list[dict], env: dict) -> str:
    """Render the authoritative cross-indicator benchmark summary."""
    lines = [
        "# TAFlow benchmark", "",
        f"Generated {env['date']} with Python {env['python']}, NumPy "
        f"{env['numpy']}, TA-Lib {env['talib']}, Wickra {env['wickra']}, "
        f"SMC {env['smartmoneyconcepts']}, and TAFlow {env['taflow']}.", "",
        "Only `MATCH` indicators are timed. Speedup is reference time divided "
        "by TAFlow time; values above 1× favor TAFlow. Each cell is API/kernel.",
        "",
        "| Class | Target | 1k | 10k | 100k | 1m |",
        "|---|---|---:|---:|---:|---:|",
    ]
    for report in sorted(reports, key=lambda item: item["canonical_class"]):
        by_size = {row["bars"]: row for row in report["vector"]}
        cells = []
        for size in DEFAULT_SIZES:
            row = by_size.get(size)
            cells.append(
                f"{row['api_speedup']:.2f}×/{row['kernel_speedup']:.2f}×"
                if row else "—"
            )
        lines.append(
            f"| {report['canonical_class']} | {report['oracle']} "
            f"`{report['oracle_name']}` | " + " | ".join(cells) + " |"
        )
    lines += [
        "", "Complete vector and warm-up/thread tables plus raw samples are "
        "stored under `verify/evidence/benchmark/`.",
    ]
    return "\n".join(lines) + "\n"


def parse_ints(value: str) -> tuple[int, ...]:
    """Parse a comma-separated integer command-line option."""
    return tuple(int(item.replace("_", "")) for item in value.split(",") if item)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("functions", nargs="*")
    parser.add_argument("--sizes", type=parse_ints, default=DEFAULT_SIZES)
    parser.add_argument("--warmup-sizes", type=parse_ints,
                        default=DEFAULT_WARMUP_SIZES)
    parser.add_argument("--threads", type=parse_ints, default=DEFAULT_THREADS)
    parser.add_argument("--repeats", type=int, default=3)
    parser.add_argument("--correctness-bars", type=int, default=10_000)
    parser.add_argument("--quick", action="store_true")
    parser.add_argument("--resume", action="store_true")
    args = parser.parse_args()
    if args.quick:
        args.sizes = (1_000, 10_000)
        args.repeats = min(args.repeats, 2)

    registry = build_registry()
    specs, unknown = (resolve_specs(args.functions, registry)
                      if args.functions else (list(registry.values()), []))
    if unknown:
        print("unknown functions: " + ", ".join(unknown), file=sys.stderr)
        return 2
    specs = [spec for spec in specs
             if spec.oracle_source and not spec.oracle_variant]
    required = max(*args.sizes, *args.warmup_sizes, args.correctness_bars)
    data = make_data(required)
    env = environment()
    BENCHMARK_EVIDENCE_DIR.mkdir(parents=True, exist_ok=True)
    failures = 0
    for index, spec in enumerate(specs, 1):
        evidence_json = BENCHMARK_EVIDENCE_DIR / f"{spec.snake}.json"
        if args.resume and evidence_json.exists():
            prior = json.loads(evidence_json.read_text())
            vector_sizes = {row["bars"] for row in prior.get("vector", [])}
            warmup_points = {(row["bars"], row["threads"])
                             for row in prior.get("warmup", [])}
            expected_points = {(bars, threads) for bars in args.warmup_sizes
                               for threads in args.threads}
            if (prior.get("schema_version") == SCHEMA_VERSION
                    and set(args.sizes).issubset(vector_sizes)
                    and expected_points.issubset(warmup_points)):
                print(f"[{index}/{len(specs)}] {spec.cls.__name__}: reused")
                continue
        check = verify_function(
            spec, data, args.correctness_bars,
            args.correctness_bars * 9 // 10,
        )
        status = verdict(check)
        if status != "MATCH":
            failures += 1
            print(f"[{index}/{len(specs)}] {spec.cls.__name__}: {status}; not timed")
            continue
        report = {
            "schema_version": SCHEMA_VERSION,
            "canonical_class": spec.cls.__name__,
            "snake_name": spec.snake,
            "oracle": spec.oracle_source,
            "oracle_name": spec.oracle_name,
            "environment": env,
            "protocol": {
                "sizes": args.sizes,
                "warmup_sizes": args.warmup_sizes,
                "threads": args.threads,
                "repeats": args.repeats,
            },
            "correctness": {"verdict": status, "evidence": check},
            "vector": vector_results(spec, data, args.sizes, args.repeats),
            "warmup": warmup_results(
                spec, data, args.warmup_sizes, args.threads, args.repeats
            ),
        }
        evidence_json.write_text(json.dumps(report, indent=2, default=float) + "\n")
        (BENCHMARK_EVIDENCE_DIR / f"{spec.snake}.md").write_text(
            render_evidence(report)
        )
        print(f"[{index}/{len(specs)}] {spec.cls.__name__}: benchmarked")

    reports = []
    for path in sorted(BENCHMARK_EVIDENCE_DIR.glob("*.json")):
        candidate = json.loads(path.read_text())
        if candidate.get("schema_version") == SCHEMA_VERSION:
            reports.append(candidate)
    (VERIFY_DIR / "BENCHMARK.md").write_text(render_aggregate(reports, env))
    print(f"wrote {VERIFY_DIR / 'BENCHMARK.md'} ({len(reports)} indicators)")
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main())
