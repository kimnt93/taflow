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

SCHEMA_VERSION = 5
DEFAULT_SIZES = (1_000, 10_000, 100_000, 1_000_000)
DEFAULT_THREADS = (1, 5, 10)
DEFAULT_WARMUP_SIZES = (1, 10, 100, 1_000)
DEFAULT_CHUNKS = (1, 10, 1_000)
DEFAULT_SCENARIOS = ("correctness", "vector", "warmup", "continue", "threads")
MIN_SAMPLE_SECONDS = 0.02


def talib_call(spec: Spec, arrays: list[np.ndarray]):
    import talib
    from talib import abstract

    params = dict(abstract.Function(spec.talib_name).info["parameters"])
    return getattr(talib, spec.talib_name)(*arrays, **params)


def selected_reference(spec: Spec) -> dict:
    path = Path(__file__).parent / "SOURCE_COMPARISON.json"
    if not path.exists() or not spec.cls:
        return {}
    rows = json.loads(path.read_text())
    return next((row for row in rows if row["class"] == spec.cls.__name__), {})


def external_reference_call(spec: Spec, arrays: list[np.ndarray], reference: dict):
    """Invoke timed external references currently supported by the harness."""
    source = reference.get("source")
    if source == "TA-Lib":
        return talib_call(spec, arrays)
    if source == "NumPy":
        functions = {
            "math_abs": np.abs,
            "math_acosh": np.arccosh,
            "math_asinh": np.arcsinh,
            "math_atanh": np.arctanh,
            "math_cbrt": np.cbrt,
            "math_cot": lambda value: 1.0 / np.tan(value),
            "math_degrees": np.degrees,
            "math_log1p": np.log1p,
            "math_radians": np.radians,
            "signed_power": lambda value: np.sign(value) * np.abs(value) ** 2.0,
        }
        return functions[spec.snake](arrays[0])
    if source == "Polars":
        import polars as pl
        series = pl.Series(arrays[0])
        functions = {
            "cumulative_maximum": lambda: series.cum_max(),
            "cumulative_minimum": lambda: series.cum_min(),
            "cumulative_product": lambda: series.cum_prod(),
            "cumulative_sum": lambda: series.cum_sum(),
            "ewm_var": lambda: series.ewm_var(
                span=spec.ctor_kwargs.get("timeperiod", 14), adjust=False, bias=True),
        }
        return functions[spec.snake]()
    raise KeyError(source)


def has_timed_reference(reference: dict) -> bool:
    return reference.get("source") in {"TA-Lib", "NumPy", "Polars"}


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


def native_vector(spec: Spec, arrays: list[np.ndarray]):
    """Run the compiled state directly, excluding Python history bookkeeping."""
    state = spec.new_state()
    native = getattr(state, "_state", state)
    return native.extend(*arrays)


def vector_rows(spec: Spec, data: dict, sizes: tuple[int, ...],
                repeats: int) -> list[dict]:
    reference = selected_reference(spec)
    rows = []
    for size in sizes:
        arrays = spec.arrays(data, size)
        row = {
            "bars": size,
            "taflow_canonical_vector": timed(
                lambda: full_vector(spec, arrays), repeats),
            "taflow_native_kernel": timed(
                lambda: native_vector(spec, arrays), repeats),
        }
        for key in ("taflow_canonical_vector", "taflow_native_kernel"):
            row[key]["bars_per_second"] = size / (row[key]["mean_ms"] / 1e3)
        if has_timed_reference(reference):
            row["external_reference"] = timed(
                lambda: external_reference_call(spec, arrays, reference), repeats)
            row["external_reference"]["bars_per_second"] = (
                size / (row["external_reference"]["mean_ms"] / 1e3))
            row["speedup_canonical"] = (row["external_reference"]["mean_ms"]
                                         / row["taflow_canonical_vector"]["mean_ms"])
            row["speedup_kernel"] = (row["external_reference"]["mean_ms"]
                                      / row["taflow_native_kernel"]["mean_ms"])
        rows.append(row)
    return rows


def warmup_rows(spec: Spec, data: dict, sizes: tuple[int, ...],
                thread_counts: tuple[int, ...], repeats: int) -> list[dict]:
    """Construct fresh independent states at each bar/thread matrix point."""
    rows = []
    reference = selected_reference(spec)
    for bars in sizes:
        arrays = spec.arrays(data, bars)
        for count in thread_counts:
            native_samples = []
            reference_samples = []
            for _ in range(min(repeats, 3)):
                native_samples.append(_thread_wall([
                    lambda a=arrays: native_vector(spec, a) for _ in range(count)
                ]))
                if has_timed_reference(reference):
                    reference_samples.append(_thread_wall([
                        lambda a=arrays: external_reference_call(spec, a, reference)
                        for _ in range(count)
                    ]))
            row = {
                "bars": bars,
                "threads": count,
                "taflow_native_warmup": sample_stats(native_samples, count * bars),
            }
            if reference_samples:
                row["reference_warmup"] = sample_stats(reference_samples, count * bars)
                row["speedup"] = (row["reference_warmup"]["mean_ms"]
                                  / row["taflow_native_warmup"]["mean_ms"])
            rows.append(row)
    return rows


def _continue_taflow(spec: Spec, base_arrays: list[np.ndarray],
                     update_arrays: list[np.ndarray], chunk: int,
                     repeats: int, kernel: bool = False) -> dict:
    samples = []
    calls = (len(update_arrays[0]) + chunk - 1) // chunk
    for _ in range(repeats):
        state = spec.new_state()
        target = getattr(state, "_state", state) if kernel else state
        target.extend(*base_arrays)
        gc.disable()
        start = time.perf_counter_ns()
        if chunk == 1:
            for bar in zip(*update_arrays):
                target.append(*bar)
        else:
            for offset in range(0, len(update_arrays[0]), chunk):
                target.extend(*[a[offset:offset + chunk]
                                for a in update_arrays])
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
            "taflow_canonical_continue": _continue_taflow(
                spec, base_arrays, update_arrays, chunk, repeats),
            "taflow_native_continue": _continue_taflow(
                spec, base_arrays, update_arrays, chunk, repeats, kernel=True),
        }
        if spec.talib_name:
            full = [a[:base + chunk] for a in arrays]
            row["talib_full_recompute"] = timed(
                lambda: talib_call(spec, full), min(repeats, 3))
            tail_bars = min(base + chunk, spec.lookback + chunk + 1)
            tail = [a[base + chunk - tail_bars:base + chunk] for a in arrays]
            row["talib_tail_recompute"] = timed(
                lambda: talib_call(spec, tail), min(repeats, 3))
            tf_us = row["taflow_native_continue"]["mean_call_us"]
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
        native_vector_samples = []
        talib_samples = []
        continue_samples = []
        native_continue_samples = []
        for _ in range(min(repeats, 3)):
            vector_samples.append(_thread_wall([
                lambda a=arrays: full_vector(spec, a) for _ in range(count)
            ]))
            native_vector_samples.append(_thread_wall([
                lambda a=arrays: native_vector(spec, a) for _ in range(count)
            ]))
            states = [spec.new_state() for _ in range(count)]
            for state in states:
                state.extend(*base_arrays)
            continue_samples.append(_thread_wall([
                lambda state=state: [state.append(*bar)
                                     for bar in zip(*update_arrays)]
                for state in states
            ]))
            native_states = [spec.new_state() for _ in range(count)]
            for state in native_states:
                getattr(state, "_state", state).extend(*base_arrays)
            native_continue_samples.append(_thread_wall([
                lambda state=state: [getattr(state, "_state", state).append(*bar)
                                     for bar in zip(*update_arrays)]
                for state in native_states
            ]))
            if spec.talib_name:
                talib_samples.append(_thread_wall([
                    lambda a=arrays: talib_call(spec, a)
                    for _ in range(count)
                ]))
        row["taflow_canonical_vector"] = sample_stats(
            vector_samples, count * bars)
        row["taflow_native_vector"] = sample_stats(
            native_vector_samples, count * bars)
        row["taflow_canonical_continue"] = sample_stats(
            continue_samples, count * updates)
        row["taflow_native_continue"] = sample_stats(
            native_continue_samples, count * updates)
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
                row["taflow_native_continue"]["units_per_second"]
                / one["taflow_native_continue"]["units_per_second"])
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
                  "| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |",
                  "|---:|---:|---:|---:|---:|---:|---:|---:|"]
        for row in report["vector"]:
            api = row["taflow_canonical_vector"]
            kernel = row["taflow_native_kernel"]
            ta = row.get("external_reference", {})
            lines.append(f"| {row['bars']:,} | {api['mean_ms']:.3f} | "
                         f"{fmt_rate(api['bars_per_second'])} | "
                         f"{kernel['mean_ms']:.3f} | "
                         f"{fmt_rate(kernel['bars_per_second'])} | "
                         f"{ta.get('mean_ms', float('nan')):.3f} | "
                         + (f"{row['speedup_canonical']:.2f}×" if "speedup_canonical" in row else "—")
                         + " | " + (f"{row['speedup_kernel']:.2f}×" if "speedup_kernel" in row else "—") + " |")
        lines.append("")
    if report.get("warmup"):
        lines += ["## Fresh-state warm-up", "",
                  "| Bars | Threads | TAFlow ms | Reference ms | Speedup |",
                  "|---:|---:|---:|---:|---:|"]
        for row in report["warmup"]:
            reference = row.get("reference_warmup", {})
            lines.append(
                f"| {row['bars']:,} | {row['threads']} | "
                f"{row['taflow_native_warmup']['mean_ms']:.3f} | "
                f"{reference.get('mean_ms', float('nan')):.3f} | "
                + (f"{row['speedup']:.2f}× |" if "speedup" in row else "— |"))
        lines.append("")
    if report.get("continuation"):
        lines += ["## Warmed continuation", "",
                  "| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |",
                  "|---:|---:|---:|---:|---:|---:|---:|---:|"]
        for row in report["continuation"]:
            api = row["taflow_canonical_continue"]
            tf = row["taflow_native_continue"]
            full = row.get("talib_full_recompute", {})
            lines.append(f"| {row['base_bars']:,} | {row['chunk']:,} | "
                         f"{api['mean_call_us']:.3f} | {tf['mean_call_us']:.3f} | "
                         f"{fmt_rate(tf['bars_per_second'])} | "
                         f"{full.get('mean_ms', float('nan')) * 1e3:.3f} | "
                         + (f"{row['speedup_vs_full']:.2f}× | {row['speedup_vs_tail']:.2f}× |"
                            if "speedup_vs_full" in row else "— | — |"))
        lines.append("")
    if report.get("threads"):
        lines += ["## Independent-stream threads", "",
                  "| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |",
                  "|---:|---:|---:|---:|---:|---:|---:|---:|"]
        for row in report["threads"]:
            talib = row.get("talib_original_vector", {})
            lines.append(f"| {row['threads']} | "
                         f"{fmt_rate(row['taflow_canonical_vector']['units_per_second'])} | "
                         f"{fmt_rate(row['taflow_native_vector']['units_per_second'])} | "
                         f"{row['taflow_vector_scaling']:.2f}× | "
                         f"{fmt_rate(row['taflow_canonical_continue']['units_per_second'])} | "
                         f"{fmt_rate(row['taflow_native_continue']['units_per_second'])} | "
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
            "warmup_sizes": args.warmup_sizes,
            "continue_base": args.continue_base,
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
        reference = selected_reference(spec)
        check_spec = spec
        if oracle and not spec.talib_name:
            check_spec = Spec.build(spec.snake, None)
            check_spec.ctor_kwargs.update(oracle["kwargs"])
            check_spec.input_roles = oracle["inputs"]
        oracle_fn = oracle["oracle"] if oracle else None
        if not spec.talib_name and not oracle_fn and has_timed_reference(reference):
            oracle_fn = lambda arrays: external_reference_call(
                check_spec, arrays, reference
            )
        report["correctness"] = verify_function(
            check_spec, data, args.correctness_bars,
            min(args.continue_base, args.correctness_bars - 1),
            oracle_fn=oracle_fn)
        if oracle_fn and reference:
            report["correctness"]["oracle"] = reference["source"]
    if "vector" in scenarios:
        report["vector"] = vector_rows(spec, data, args.sizes, args.repeats)
    if "warmup" in scenarios:
        report["warmup"] = warmup_rows(
            spec, data, args.warmup_sizes, args.threads, args.repeats)
    if "continue" in scenarios:
        report["continuation"] = continuation_rows(
            spec, data, args.continue_base, args.continue_bars,
            args.chunks, args.repeats)
    if "threads" in scenarios:
        report["threads"] = thread_rows(
            spec, data, min(args.thread_bars, args.continue_base),
            min(args.continue_bars, 1_000), args.threads, args.repeats)
    return report


def aggregate(reports: list[dict]) -> str:
    selected_path = Path(__file__).parent / "SOURCE_COMPARISON.json"
    selected_rows = json.loads(selected_path.read_text()) if selected_path.exists() else []
    selected_by_class: dict[str, list[dict]] = {}
    for row in selected_rows:
        selected_by_class.setdefault(row["class"], []).append(row)
    lines = ["# Correctness and performance", "",
             f"Generated {date.today().isoformat()} from {len(reports)} indicator implementations.", "",
             "Every correctness row uses an external implementation. `VARIANT` means the "
             "external calculation was executed and the documented causal or initialization "
             "difference was observed; it is not a failed comparison. Speedups are "
             "`reference time / TAFlow native-kernel time`, so values above 1× favor TAFlow. "
             "A dash means that reference is correctness-only in the timing harness.", "",
             "## 1. Correctness", "",
             "| Class | Reference | Correctness | Max error |",
             "|---|---|---:|---:|"]
    report_by_class = {report["canonical_class"]: report for report in reports}
    for class_name in sorted(selected_by_class):
        evidence = selected_by_class[class_name]
        first = evidence[0]
        reference = (f"[{first['source']}: `{first['oracle_api']}`]"
                     f"({first['url']})")
        correctness = ("FAIL" if any(row["verdict"] == "FAIL" for row in evidence) else
                       "VARIANT" if any(row["verdict"] == "VARIANT" for row in evidence) else
                       "MATCH")
        max_error = max(row.get("error", 0.0) for row in evidence)
        lines.append(f"| {class_name} | {reference} | {correctness} | `{max_error:.3e}` |")

    lines += ["", "## 2. Performance on vector", "",
              "| Class | Reference | 1k bars | 10k bars | 100k bars | 1m bars |",
              "|---|---|---:|---:|---:|---:|"]
    for report in reports:
        evidence = selected_by_class.get(report["canonical_class"], [{}])
        selected = evidence[0]
        reference = (f"[{selected['source']}: `{selected['oracle_api']}`]({selected['url']})"
                     if selected else "—")
        vector_by_size = {row["bars"]: row for row in report.get("vector", [])}
        cells = []
        for size in DEFAULT_SIZES:
            row = vector_by_size.get(size)
            cells.append(f"{row['speedup_kernel']:.2f}×"
                         if row and "speedup_kernel" in row else "—")
        lines.append(f"| {report['canonical_class']} | {reference} | "
                     + " | ".join(cells) + " |")

    lines += ["", "## 3. Warm up", "",
              "Fresh independent states are constructed and fed the stated number of bars. "
              "Thread columns measure that many states concurrently.", ""]
    for bars in DEFAULT_WARMUP_SIZES:
        lines += [f"### {bars:,} bar" + ("s" if bars != 1 else ""), "",
                  "| Class | Reference | 1 thread | 5 threads | 10 threads |",
                  "|---|---|---:|---:|---:|"]
        for class_name in sorted(report_by_class):
            report = report_by_class[class_name]
            selected = selected_by_class.get(class_name, [{}])[0]
            reference = (f"[{selected['source']}: `{selected['oracle_api']}`]({selected['url']})"
                         if selected else "—")
            by_threads = {row["threads"]: row for row in report.get("warmup", [])
                          if row["bars"] == bars}
            cells = [f"{by_threads[count]['speedup']:.2f}×"
                     if count in by_threads and "speedup" in by_threads[count] else "—"
                     for count in DEFAULT_THREADS]
            lines.append(f"| {class_name} | {reference} | " + " | ".join(cells) + " |")
        lines.append("")
    return "\n".join(lines).rstrip() + "\n"


def write_aggregate(reports: list[dict], reports_dir: Path) -> None:
    """Write the single canonical report to docs and the report artifact dir."""
    content = aggregate(reports)
    (reports_dir / "BENCHMARK.md").write_text(content)
    docs_report = Path(__file__).resolve().parent.parent / "docs" / "CORRECTNESS.md"
    docs_report.write_text(content)


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
    parser.add_argument("--warmup-sizes", type=ints, default=DEFAULT_WARMUP_SIZES)
    parser.add_argument("--continue-base", type=int, default=100_000)
    parser.add_argument("--continue-bars", type=int, default=1_000)
    parser.add_argument("--thread-bars", type=int, default=100_000)
    parser.add_argument("--chunks", type=ints, default=DEFAULT_CHUNKS)
    parser.add_argument("--threads", type=ints, default=DEFAULT_THREADS)
    parser.add_argument("--scenarios", type=lambda value: tuple(value.split(",")),
                        default=DEFAULT_SCENARIOS)
    parser.add_argument("--reports-dir", type=Path,
                        default=Path(__file__).parent / "benchmark_reports")
    parser.add_argument("--aggregate-only", action="store_true",
                        help="regenerate BENCHMARK.md from existing JSON reports")
    args = parser.parse_args()
    if args.quick:
        args.repeats = min(args.repeats, 3)
        args.sizes = tuple(size for size in (1_000, 10_000) if size <= max(args.sizes))
        # Keep the oracle pass at the same 10k scale as verify.py; only the
        # performance matrix is reduced by --quick.
        args.correctness_bars = min(args.correctness_bars, 10_000)
        args.warmup_sizes = tuple(size for size in DEFAULT_WARMUP_SIZES if size <= 1_000)
        args.continue_base = min(args.continue_base, 1_500)
        args.continue_bars = min(args.continue_bars, 100)
        args.thread_bars = min(args.thread_bars, 1_000)
        args.chunks = tuple(sorted({min(chunk, args.continue_bars)
                                   for chunk in args.chunks}))

    registry = build_registry()
    if args.aggregate_only:
        reports = []
        for path in sorted(args.reports_dir.glob("*.json")):
            try:
                candidate = json.loads(path.read_text())
            except json.JSONDecodeError:
                continue
            if candidate.get("schema_version") == SCHEMA_VERSION:
                reports.append(candidate)
        args.reports_dir.mkdir(parents=True, exist_ok=True)
        write_aggregate(reports, args.reports_dir)
        print(f"wrote {args.reports_dir / 'BENCHMARK.md'} from {len(reports)} reports")
        return 0
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
                    max(args.warmup_sizes), args.continue_base + args.continue_bars,
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
    # Aggregate the complete on-disk inventory, so a focused rerun (for
    # example `EMA ATR`) refreshes those rows without dropping other reports.
    all_reports = []
    for path in sorted(args.reports_dir.glob("*.json")):
        try:
            candidate = json.loads(path.read_text())
        except json.JSONDecodeError:
            continue
        if candidate.get("schema_version") == SCHEMA_VERSION:
            all_reports.append(candidate)
    write_aggregate(all_reports, args.reports_dir)
    failures = sum(report.get("error") is not None or
                   (report.get("correctness") is not None
                    and verdict(report["correctness"]) != "MATCH")
                   for report in reports)
    print(f"wrote {args.reports_dir}; {failures} functions need attention")
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main())
