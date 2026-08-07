#!/usr/bin/env python3
"""Benchmark canonical taflow states against independent reference libraries.

The benchmark never imports a TA-Lib compatibility module from taflow.  It
maps descriptive taflow classes to external TA-Lib function names through the
shared CHECK.md registry and measures the Python-visible native vector path,
warm-up, continuation, and independent-stream thread scaling.
"""

from __future__ import annotations

import argparse
import gc
import json
import os
import platform
import statistics
import sys
import threading
import time
from concurrent.futures import ThreadPoolExecutor
from datetime import date
from pathlib import Path
from typing import Callable

import numpy as np

from registry import Spec, build_registry, make_data, resolve_specs
from verify import pandas_oracles, verdict, verify_function

SCHEMA_VERSION = 4
DEFAULT_SIZES = (1_000, 10_000, 100_000, 1_000_000)
DEFAULT_THREADS = (1, 2, 4)
DEFAULT_CHUNKS = (1, 10, 1_000)
DEFAULT_SCENARIOS = ("correctness", "vector", "warmup", "continue", "threads")
MIN_SAMPLE_SECONDS = 0.02


def talib_call(spec: Spec, arrays: list[np.ndarray]):
    import talib
    from talib import abstract

    params = dict(abstract.Function(spec.talib_name).info["parameters"])
    return getattr(talib, spec.talib_name)(*arrays, **params)


def timed(fn: Callable[[], object], repeats: int,
          min_seconds: float = MIN_SAMPLE_SECONDS) -> dict:
    """Warm once, autorange, then return seconds-per-call samples."""
    fn()
    iterations = 1
    while True:
        start = time.perf_counter_ns()
        for _ in range(iterations):
            fn()
        elapsed = (time.perf_counter_ns() - start) / 1e9
        if elapsed >= min_seconds or iterations >= 1 << 18:
            break
        multiplier = max(2, int(min_seconds / max(elapsed, 1e-9)))
        iterations = min(iterations * multiplier, 1 << 18)
    samples = []
    for _ in range(repeats):
        gc.disable()
        start = time.perf_counter_ns()
        for _ in range(iterations):
            fn()
        samples.append((time.perf_counter_ns() - start) / 1e9 / iterations)
        gc.enable()
    return sample_stats(samples)


def sample_stats(samples: list[float], units: int | None = None) -> dict:
    ordered = sorted(samples)
    mean = statistics.fmean(samples)
    block = {
        "mean_ms": mean * 1e3,
        "min_ms": ordered[0] * 1e3,
        "p50_ms": statistics.median(ordered) * 1e3,
        "p99_ms": float(np.percentile(ordered, 99)) * 1e3,
        "samples_s": samples,
    }
    if units is not None:
        block["units_per_second"] = units / mean
    return block


def full_vector(spec: Spec, arrays: list[np.ndarray]):
    return Spec.extend(spec.new_state(), arrays)


def vector_rows(spec: Spec, data: dict, sizes: tuple[int, ...],
                repeats: int) -> list[dict]:
    rows = []
    for size in sizes:
        arrays = spec.arrays(data, size)
        row = {
            "bars": size,
            "taflow_native_vector": timed(
                lambda: full_vector(spec, arrays), repeats),
        }
        row["taflow_native_vector"]["bars_per_second"] = (
            size / (row["taflow_native_vector"]["mean_ms"] / 1e3))
        if spec.talib_name:
            row["talib_original"] = timed(
                lambda: talib_call(spec, arrays), repeats)
            row["talib_original"]["bars_per_second"] = (
                size / (row["talib_original"]["mean_ms"] / 1e3))
            row["speedup"] = (row["talib_original"]["mean_ms"]
                              / row["taflow_native_vector"]["mean_ms"])
        rows.append(row)
    return rows


def warmup_row(spec: Spec, data: dict, bars: int, repeats: int) -> dict:
    arrays = spec.arrays(data, bars)
    row = {
        "bars": bars,
        "taflow_native_warmup": timed(
            lambda: full_vector(spec, arrays), repeats),
    }
    if spec.talib_name:
        row["talib_original"] = timed(
            lambda: talib_call(spec, arrays), repeats)
        row["speedup"] = (row["talib_original"]["mean_ms"]
                          / row["taflow_native_warmup"]["mean_ms"])
    return row


def _continue_taflow(spec: Spec, base_arrays: list[np.ndarray],
                     update_arrays: list[np.ndarray], chunk: int,
                     repeats: int) -> dict:
    samples = []
    calls = (len(update_arrays[0]) + chunk - 1) // chunk
    for _ in range(repeats):
        state = spec.new_state()
        state.extend(*base_arrays)
        gc.disable()
        start = time.perf_counter_ns()
        if chunk == 1:
            for bar in zip(*update_arrays):
                state.append(*bar)
        else:
            for offset in range(0, len(update_arrays[0]), chunk):
                state.extend(*[a[offset:offset + chunk] for a in update_arrays])
        samples.append((time.perf_counter_ns() - start) / 1e9)
        gc.enable()
    result = sample_stats(samples)
    mean_s = result["mean_ms"] / 1e3
    result.update({
        "calls": calls,
        "bars": len(update_arrays[0]),
        "mean_call_us": mean_s / calls * 1e6,
        "mean_bar_us": mean_s / len(update_arrays[0]) * 1e6,
        "bars_per_second": len(update_arrays[0]) / mean_s,
    })
    return result


def continuation_rows(spec: Spec, data: dict, base: int, updates: int,
                      chunks: tuple[int, ...], repeats: int) -> list[dict]:
    arrays = spec.arrays(data, base + updates)
    base_arrays = [a[:base] for a in arrays]
    update_arrays = [a[base:] for a in arrays]
    rows = []
    for chunk in chunks:
        row = {
            "base_bars": base,
            "update_bars": updates,
            "chunk": chunk,
            "taflow_continue": _continue_taflow(
                spec, base_arrays, update_arrays, chunk, repeats),
        }
        if spec.talib_name:
            full = [a[:base + chunk] for a in arrays]
            row["talib_full_recompute"] = timed(
                lambda: talib_call(spec, full), min(repeats, 3))
            tail_bars = min(base + chunk, spec.lookback + chunk + 1)
            tail = [a[base + chunk - tail_bars:base + chunk] for a in arrays]
            row["talib_tail_recompute"] = timed(
                lambda: talib_call(spec, tail), min(repeats, 3))
            tf_us = row["taflow_continue"]["mean_call_us"]
            row["speedup_vs_full"] = (
                row["talib_full_recompute"]["mean_ms"] * 1e3 / tf_us)
            row["speedup_vs_tail"] = (
                row["talib_tail_recompute"]["mean_ms"] * 1e3 / tf_us)
        rows.append(row)
    return rows


def _thread_wall(workers: list[Callable[[], None]]) -> float:
    barrier = threading.Barrier(len(workers) + 1)

    def run(worker):
        barrier.wait()
        worker()

    with ThreadPoolExecutor(max_workers=len(workers)) as pool:
        futures = [pool.submit(run, worker) for worker in workers]
        start = time.perf_counter_ns()
        barrier.wait()
        for future in futures:
            future.result()
    return (time.perf_counter_ns() - start) / 1e9


def thread_rows(spec: Spec, data: dict, bars: int, updates: int,
                thread_counts: tuple[int, ...], repeats: int) -> list[dict]:
    arrays = spec.arrays(data, bars + updates)
    base_arrays = [a[:bars] for a in arrays]
    update_arrays = [a[bars:] for a in arrays]
    rows = []
    for count in thread_counts:
        row = {"threads": count, "bars": bars, "updates": updates}
        vector_samples = []
        talib_samples = []
        continue_samples = []
        for _ in range(min(repeats, 3)):
            vector_samples.append(_thread_wall([
                lambda a=arrays: full_vector(spec, a) for _ in range(count)
            ]))
            states = [spec.new_state() for _ in range(count)]
            for state in states:
                state.extend(*base_arrays)
            continue_samples.append(_thread_wall([
                lambda state=state: [state.append(*bar)
                                     for bar in zip(*update_arrays)]
                for state in states
            ]))
            if spec.talib_name:
                talib_samples.append(_thread_wall([
                    lambda a=arrays: talib_call(spec, a)
                    for _ in range(count)
                ]))
        row["taflow_native_vector"] = sample_stats(
            vector_samples, count * bars)
        row["taflow_continue"] = sample_stats(
            continue_samples, count * updates)
        if talib_samples:
            row["talib_original_vector"] = sample_stats(
                talib_samples, count * bars)
            row["vector_speedup"] = (
                row["talib_original_vector"]["mean_ms"]
                / row["taflow_native_vector"]["mean_ms"])
        rows.append(row)
    one = next((row for row in rows if row["threads"] == 1), None)
    if one:
        for row in rows:
            row["taflow_vector_scaling"] = (
                row["taflow_native_vector"]["units_per_second"]
                / one["taflow_native_vector"]["units_per_second"])
            row["taflow_continue_scaling"] = (
                row["taflow_continue"]["units_per_second"]
                / one["taflow_continue"]["units_per_second"])
            if "talib_original_vector" in row:
                row["talib_vector_scaling"] = (
                    row["talib_original_vector"]["units_per_second"]
                    / one["talib_original_vector"]["units_per_second"])
    return rows


def environment() -> dict:
    import taflow
    try:
        import talib
        talib_version = talib.__version__
    except ImportError:
        talib_version = None
    cpu = ""
    try:
        cpu = next(line.split(":", 1)[1].strip()
                   for line in Path("/proc/cpuinfo").read_text().splitlines()
                   if line.startswith("model name"))
    except (OSError, StopIteration):
        pass
    return {
        "date": date.today().isoformat(),
        "platform": platform.platform(),
        "python": platform.python_version(),
        "numpy": np.__version__,
        "taflow": getattr(taflow, "__version__", "unknown"),
        "talib": talib_version,
        "cpu": cpu,
        "rustflags": os.environ.get("RUSTFLAGS", ""),
        "native_vector_note": (
            "taflow class.extend over contiguous NumPy arrays; this exercises "
            "the compiled Rust bulk/SIMD-capable path. SIMD availability and "
            "target features depend on the installed wheel/build flags."
        ),
    }


def fmt_rate(value: float | None) -> str:
    if value is None:
        return "—"
    for suffix, scale in (("G", 1e9), ("M", 1e6), ("K", 1e3)):
        if value >= scale:
            return f"{value / scale:.2f}{suffix}"
    return f"{value:.1f}"


def render(report: dict) -> str:
    title = report["canonical_class"]
    alias = report.get("talib_name")
    lines = [f"# {title} benchmark" + (f" (`{alias}` oracle)" if alias else ""), ""]
    correctness = report.get("correctness")
    if correctness:
        lines += [f"Correctness: **{verdict(correctness)}**.", ""]
    lines += [report["environment"]["native_vector_note"], ""]
    if report.get("vector"):
        lines += ["## Whole-vector performance", "",
                  "| Bars | TAFlow native ms | TAFlow bars/s | TA-Lib ms | Speedup |",
                  "|---:|---:|---:|---:|---:|"]
        for row in report["vector"]:
            tf = row["taflow_native_vector"]
            ta = row.get("talib_original", {})
            lines.append(f"| {row['bars']:,} | {tf['mean_ms']:.3f} | "
                         f"{fmt_rate(tf['bars_per_second'])} | "
                         f"{ta.get('mean_ms', float('nan')):.3f} | "
                         + (f"{row['speedup']:.2f}×" if "speedup" in row else "—") + " |")
        lines.append("")
    if report.get("warmup"):
        row = report["warmup"]
        lines += ["## Warm-up", "",
                  f"Construct + native extend over {row['bars']:,} bars: "
                  f"**{row['taflow_native_warmup']['mean_ms']:.3f} ms**" +
                  (f"; TA-Lib {row['talib_original']['mean_ms']:.3f} ms."
                   if "talib_original" in row else "."), ""]
    if report.get("continuation"):
        lines += ["## Warmed continuation", "",
                  "| Base | Chunk | TAFlow µs/call | TAFlow bars/s | TA-Lib full µs | vs full | vs tail |",
                  "|---:|---:|---:|---:|---:|---:|---:|"]
        for row in report["continuation"]:
            tf = row["taflow_continue"]
            full = row.get("talib_full_recompute", {})
            lines.append(f"| {row['base_bars']:,} | {row['chunk']:,} | "
                         f"{tf['mean_call_us']:.3f} | {fmt_rate(tf['bars_per_second'])} | "
                         f"{full.get('mean_ms', float('nan')) * 1e3:.3f} | "
                         + (f"{row['speedup_vs_full']:.2f}× | {row['speedup_vs_tail']:.2f}× |"
                            if "speedup_vs_full" in row else "— | — |"))
        lines.append("")
    if report.get("threads"):
        lines += ["## Independent-stream threads", "",
                  "| Threads | TAFlow vector bars/s | Vector scaling | TAFlow continue updates/s | Continue scaling | TA-Lib vector bars/s |",
                  "|---:|---:|---:|---:|---:|---:|"]
        for row in report["threads"]:
            talib = row.get("talib_original_vector", {})
            lines.append(f"| {row['threads']} | "
                         f"{fmt_rate(row['taflow_native_vector']['units_per_second'])} | "
                         f"{row['taflow_vector_scaling']:.2f}× | "
                         f"{fmt_rate(row['taflow_continue']['units_per_second'])} | "
                         f"{row['taflow_continue_scaling']:.2f}× | "
                         f"{fmt_rate(talib.get('units_per_second'))} |")
        lines.append("")
    lines += ["---", "Times include Python conversion/binding overhead. Raw samples are retained in JSON.", ""]
    return "\n".join(lines)


def run_spec(spec: Spec, args, data: dict, env: dict) -> dict:
    report = {
        "schema_version": SCHEMA_VERSION,
        "canonical_class": spec.cls.__name__ if spec.cls else spec.snake,
        "snake_name": spec.snake,
        "talib_name": spec.talib_name,
        "constructor_kwargs": spec.ctor_kwargs,
        "inputs": spec.input_roles,
        "environment": env,
        "protocol": {
            "repeats": args.repeats,
            "sizes": args.sizes,
            "warmup_bars": args.warmup_bars,
            "continue_bars": args.continue_bars,
            "chunks": args.chunks,
            "threads": args.threads,
        },
    }
    if spec.error:
        report["error"] = spec.error
        return report
    scenarios = set(args.scenarios)
    if "correctness" in scenarios:
        oracle = pandas_oracles().get(spec.snake)
        check_spec = spec
        if oracle and not spec.talib_name:
            check_spec = Spec.build(spec.snake, None)
            check_spec.ctor_kwargs.update(oracle["kwargs"])
            check_spec.input_roles = oracle["inputs"]
        report["correctness"] = verify_function(
            check_spec, data, args.correctness_bars,
            min(args.warmup_bars, args.correctness_bars - 1),
            oracle_fn=oracle["oracle"] if oracle else None)
    if "vector" in scenarios:
        report["vector"] = vector_rows(spec, data, args.sizes, args.repeats)
    if "warmup" in scenarios:
        report["warmup"] = warmup_row(
            spec, data, args.warmup_bars, args.repeats)
    if "continue" in scenarios:
        report["continuation"] = continuation_rows(
            spec, data, args.warmup_bars, args.continue_bars,
            args.chunks, args.repeats)
    if "threads" in scenarios:
        report["threads"] = thread_rows(
            spec, data, min(args.thread_bars, args.warmup_bars),
            min(args.continue_bars, 1_000), args.threads, args.repeats)
    return report


def aggregate(reports: list[dict]) -> str:
    lines = ["# TAFlow verification benchmark", "",
             f"Generated {date.today().isoformat()} from {len(reports)} functions.", "",
             "| Canonical class | TA-Lib | Correctness | Largest vector bars/s | Speedup | Append µs | 4T vector scaling |",
             "|---|---|---|---:|---:|---:|---:|"]
    for report in reports:
        vector = (report.get("vector") or [None])[-1]
        continuation = next((r for r in report.get("continuation", [])
                             if r["chunk"] == 1), None)
        threaded = (report.get("threads") or [None])[-1]
        check = report.get("correctness")
        speedup = (f"{vector['speedup']:.2f}×"
                   if vector and "speedup" in vector else "—")
        append_us = (f"{continuation['taflow_continue']['mean_call_us']:.3f}"
                     if continuation else "—")
        scaling = (f"{threaded['taflow_vector_scaling']:.2f}×"
                   if threaded else "—")
        lines.append(
            f"| {report['canonical_class']} | {report.get('talib_name') or '—'} | "
            f"{verdict(check) if check else ('ERROR' if report.get('error') else '—')} | "
            f"{fmt_rate(vector['taflow_native_vector']['bars_per_second']) if vector else '—'} | "
            f"{speedup} | {append_us} | {scaling} |")
    return "\n".join(lines) + "\n"


def ints(value: str) -> tuple[int, ...]:
    return tuple(int(item.replace("_", "")) for item in value.split(",") if item)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("functions", nargs="*",
                        help="TA-Lib, snake, or canonical class names; default all")
    parser.add_argument("--quick", action="store_true")
    parser.add_argument("--list", action="store_true")
    parser.add_argument("--repeats", type=int, default=7)
    parser.add_argument("--sizes", type=ints, default=DEFAULT_SIZES)
    parser.add_argument("--correctness-bars", type=int, default=100_000)
    parser.add_argument("--warmup-bars", type=int, default=100_000)
    parser.add_argument("--continue-bars", type=int, default=1_000)
    parser.add_argument("--thread-bars", type=int, default=100_000)
    parser.add_argument("--chunks", type=ints, default=DEFAULT_CHUNKS)
    parser.add_argument("--threads", type=ints, default=DEFAULT_THREADS)
    parser.add_argument("--scenarios", type=lambda value: tuple(value.split(",")),
                        default=DEFAULT_SCENARIOS)
    parser.add_argument("--reports-dir", type=Path,
                        default=Path(__file__).parent / "benchmark_reports")
    args = parser.parse_args()
    if args.quick:
        args.repeats = min(args.repeats, 3)
        args.sizes = tuple(size for size in (1_000, 10_000) if size <= max(args.sizes))
        args.correctness_bars = min(args.correctness_bars, 2_000)
        args.warmup_bars = min(args.warmup_bars, 1_500)
        args.continue_bars = min(args.continue_bars, 100)
        args.thread_bars = min(args.thread_bars, 1_000)
        args.threads = tuple(value for value in args.threads if value <= 2) or (1,)
        args.chunks = tuple(sorted({min(chunk, args.continue_bars)
                                   for chunk in args.chunks}))

    registry = build_registry()
    if args.list:
        for spec in registry.values():
            print(f"{spec.cls.__name__ if spec.cls else spec.snake:48s} "
                  f"{spec.talib_name or '-':20s} {spec.error or 'ready'}")
        return 0
    specs, unknown = (resolve_specs(args.functions, registry)
                      if args.functions else (list(registry.values()), []))
    if unknown:
        print("unknown functions: " + ", ".join(unknown), file=sys.stderr)
        return 2
    required = max((*args.sizes, args.correctness_bars,
                    args.warmup_bars + args.continue_bars,
                    args.thread_bars + min(args.continue_bars, 1_000)))
    data = make_data(required)
    env = environment()
    args.reports_dir.mkdir(parents=True, exist_ok=True)
    reports = []
    for index, spec in enumerate(specs, 1):
        label = spec.cls.__name__ if spec.cls else spec.snake
        start = time.perf_counter()
        try:
            report = run_spec(spec, args, data, env)
        except Exception as exc:
            report = {
                "schema_version": SCHEMA_VERSION,
                "canonical_class": label,
                "snake_name": spec.snake,
                "talib_name": spec.talib_name,
                "environment": env,
                "error": f"{type(exc).__name__}: {exc}",
            }
        reports.append(report)
        stem = spec.talib_name or spec.snake
        (args.reports_dir / f"{stem}.json").write_text(
            json.dumps(report, indent=2, default=float) + "\n")
        (args.reports_dir / f"{stem}.md").write_text(render(report))
        status = (verdict(report["correctness"])
                  if report.get("correctness") else
                  ("ERROR" if report.get("error") else "DONE"))
        print(f"[{index}/{len(specs)}] {label}: {status} "
              f"({time.perf_counter() - start:.2f}s)")
    (args.reports_dir / "BENCHMARK.md").write_text(aggregate(reports))
    failures = sum(report.get("error") is not None or
                   (report.get("correctness") is not None
                    and verdict(report["correctness"]) != "MATCH")
                   for report in reports)
    print(f"wrote {args.reports_dir}; {failures} functions need attention")
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main())
