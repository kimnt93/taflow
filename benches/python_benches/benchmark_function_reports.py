"""Generate reproducible per-function TA-Lib/TAFlow benchmark matrices.

This runner intentionally separates batch calls, one-call state backfills, and
scalar streaming. A future Rust Pipeline is reported as unavailable until the
shared execution engine exists; state ``extend`` is not relabeled as Pipeline.
"""

from __future__ import annotations

import argparse
import gc
import json
import os
import platform
import resource
import statistics
import time
from dataclasses import dataclass
from datetime import date
from multiprocessing import Pipe, Process
from pathlib import Path

import numpy as np
import talib as original_talib

import taflow
from taflow import talib as taflow_talib


SIZES = (100, 1_000, 10_000, 100_000, 1_000_000)
MODES = ("talib_batch", "taflow_batch", "taflow_state_extend", "taflow_streaming")
WARMUP_BARS = 10_000
DEFAULT_REPEATS = 5
LATENCY_SAMPLE_COUNT = 10_000


@dataclass(frozen=True)
class FunctionSpec:
    name: str
    state_type: type
    state_args: tuple
    batch_args: tuple
    batch_kwargs: dict
    inputs: tuple[str, ...]


FUNCTIONS = {
    "MA": FunctionSpec("MA", taflow.MovingAverage, (20, 1), (), {"timeperiod": 20, "matype": 1}, ("close",)),
    "BBANDS": FunctionSpec(
        "BBANDS",
        taflow.BollingerBands,
        (20, 2.0, 2.0, 0),
        (),
        {"timeperiod": 20, "nbdevup": 2.0, "nbdevdn": 2.0, "matype": 0},
        ("close",),
    ),
    "ACCBANDS": FunctionSpec(
        "ACCBANDS", taflow.AccelerationBands, (20,), (), {"timeperiod": 20}, ("high", "low", "close")
    ),
    "SAR": FunctionSpec(
        "SAR", taflow.ParabolicSar, (0.02, 0.2), (), {"acceleration": 0.02, "maximum": 0.2}, ("high", "low")
    ),
    "SAREXT": FunctionSpec(
        "SAREXT",
        taflow.ParabolicSarExtended,
        (0.0, 0.0, 0.02, 0.02, 0.2, 0.02, 0.02, 0.2),
        (),
        {
            "startvalue": 0.0,
            "offsetonreverse": 0.0,
            "accelerationinitlong": 0.02,
            "accelerationlong": 0.02,
            "accelerationmaxlong": 0.2,
            "accelerationinitshort": 0.02,
            "accelerationshort": 0.02,
            "accelerationmaxshort": 0.2,
        },
        ("high", "low"),
    ),
    "IMI": FunctionSpec(
        "IMI",
        taflow.IntradayMomentumIndex,
        (14,),
        (),
        {"timeperiod": 14},
        ("open", "close"),
    ),
}


def make_data(size: int) -> dict[str, np.ndarray]:
    index = np.arange(size, dtype=np.float64)
    close = 100.0 + index * 0.0007 + np.sin(index * 0.017) * 4.0 + np.cos(index * 0.003) * 1.5
    return {
        "open": close - np.cos(index * 0.019) * 1.3,
        "close": close,
        "high": close + 1.0 + np.abs(np.sin(index * 0.013)),
        "low": close - 1.0 - np.abs(np.cos(index * 0.011)),
    }


def select_inputs(spec: FunctionSpec, data: dict[str, np.ndarray], start: int, stop: int):
    return tuple(data[name][start:stop] for name in spec.inputs)


def current_rss_bytes() -> int:
    with open("/proc/self/statm", encoding="ascii") as statm:
        resident_pages = int(statm.read().split()[1])
    return resident_pages * os.sysconf("SC_PAGE_SIZE")


def peak_rss_bytes() -> int:
    # Linux reports ru_maxrss in KiB.
    return resource.getrusage(resource.RUSAGE_SELF).ru_maxrss * 1024


def percentile(samples: list[float], quantile: float) -> float:
    return float(np.percentile(np.asarray(samples), quantile, method="linear"))


def summarize(samples_ms: list[float]) -> dict[str, float | list[float]]:
    return {
        "unit": "ms",
        "samples": samples_ms,
        "min": min(samples_ms),
        "mean": statistics.fmean(samples_ms),
        "p50": percentile(samples_ms, 50),
        "p95": percentile(samples_ms, 95),
        "p99": percentile(samples_ms, 99),
        "max": max(samples_ms),
    }


def prepare_call(mode: str, spec: FunctionSpec, data: dict[str, np.ndarray], size: int):
    batch_inputs = select_inputs(spec, data, 0, size)
    if mode == "talib_batch":
        function = getattr(original_talib, spec.name)
        return lambda: function(*batch_inputs, *spec.batch_args, **spec.batch_kwargs)
    if mode == "taflow_batch":
        function = getattr(taflow_talib, spec.name)
        return lambda: function(*batch_inputs, *spec.batch_args, **spec.batch_kwargs)
    if mode == "taflow_state_extend":
        return lambda: spec.state_type(*spec.state_args).extend(*batch_inputs)
    if mode == "taflow_streaming":
        state = spec.state_type(*spec.state_args)
        history = select_inputs(spec, data, 0, WARMUP_BARS)
        state.extend(*history)
        updates = select_inputs(spec, data, WARMUP_BARS, WARMUP_BARS + size)

        def append_all():
            value = None
            for row in zip(*updates):
                value = state.append(*row)
            return value

        return append_all
    raise ValueError(f"unsupported mode: {mode}")


def benchmark_child(connection, name: str, mode: str, size: int, repeats: int):
    try:
        spec = FUNCTIONS[name]
        total_size = size + WARMUP_BARS if mode == "taflow_streaming" else size
        data = make_data(total_size)
        warmup_call = prepare_call(mode, spec, data, size)
        warmup_result = warmup_call()
        del warmup_result, warmup_call
        gc.collect()
        rss_before = current_rss_bytes()
        wall_samples = []
        cpu_samples = []
        for _ in range(repeats):
            call = prepare_call(mode, spec, data, size)
            cpu_start = time.process_time_ns()
            wall_start = time.perf_counter_ns()
            result = call()
            wall_samples.append((time.perf_counter_ns() - wall_start) / 1_000_000.0)
            cpu_samples.append((time.process_time_ns() - cpu_start) / 1_000_000.0)
            del result, call
            gc.collect()
        wall = summarize(wall_samples)
        cpu = summarize(cpu_samples)
        throughput_samples = [size / (sample / 1000.0) for sample in wall_samples]
        connection.send(
            {
                "available": True,
                "bars": size,
                "repeats": repeats,
                "wall_latency": wall,
                "cpu_time": cpu,
                "throughput_bars_per_second": {
                    "unit": "bars/s",
                    "mean": statistics.fmean(throughput_samples),
                    "p50": percentile(throughput_samples, 50),
                    "min": min(throughput_samples),
                    "max": max(throughput_samples),
                },
                "peak_rss_delta_bytes": {
                    "unit": "bytes",
                    "value": max(0, peak_rss_bytes() - rss_before),
                    "inputs_excluded": True,
                },
            }
        )
    except BaseException as error:
        connection.send({"available": False, "error": f"{type(error).__name__}: {error}"})
    finally:
        connection.close()


def run_isolated(name: str, mode: str, size: int, repeats: int) -> dict:
    parent, child = Pipe(duplex=False)
    process = Process(target=benchmark_child, args=(child, name, mode, size, repeats))
    process.start()
    child.close()
    result = parent.recv()
    process.join()
    if process.exitcode != 0 and result.get("available"):
        raise RuntimeError(f"benchmark child exited with {process.exitcode}")
    return result


def normalize_outputs(result) -> tuple[np.ndarray, ...]:
    if isinstance(result, tuple):
        return tuple(np.asarray(item, dtype=np.float64) for item in result)
    return (np.asarray(result, dtype=np.float64),)


def correctness_result(name: str, size: int = 100_000) -> dict:
    spec = FUNCTIONS[name]
    data = make_data(size)
    inputs = select_inputs(spec, data, 0, size)
    oracle = normalize_outputs(getattr(original_talib, name)(*inputs, *spec.batch_args, **spec.batch_kwargs))
    results = {
        "taflow_batch": normalize_outputs(getattr(taflow_talib, name)(*inputs, *spec.batch_args, **spec.batch_kwargs)),
        "taflow_state_extend": normalize_outputs(spec.state_type(*spec.state_args).extend(*inputs)),
    }
    comparisons = {}
    for mode, actual_outputs in results.items():
        max_error = 0.0
        nan_mismatches = 0
        for actual, expected in zip(actual_outputs, oracle):
            nan_mismatches += int(np.count_nonzero(np.isnan(actual) != np.isnan(expected)))
            valid = ~(np.isnan(actual) | np.isnan(expected))
            if np.any(valid):
                max_error = max(max_error, float(np.max(np.abs(actual[valid] - expected[valid]))))
        comparisons[mode] = {
            "outputs": len(actual_outputs),
            "nan_mismatches": nan_mismatches,
            "max_absolute_error": max_error,
            "allclose_rtol": 1e-8,
            "allclose_atol": 1e-10,
            "passed": nan_mismatches == 0
            and all(np.allclose(a, e, rtol=1e-8, atol=1e-10, equal_nan=True) for a, e in zip(actual_outputs, oracle)),
        }
    return {"oracle": "TA-Lib Python 0.7.1", "bars": size, "comparisons": comparisons}


def streaming_latency(name: str) -> dict:
    spec = FUNCTIONS[name]
    data = make_data(WARMUP_BARS + LATENCY_SAMPLE_COUNT)
    state = spec.state_type(*spec.state_args)
    state.extend(*select_inputs(spec, data, 0, WARMUP_BARS))
    updates = select_inputs(spec, data, WARMUP_BARS, WARMUP_BARS + LATENCY_SAMPLE_COUNT)
    samples_ns = []
    for row in zip(*updates):
        started = time.perf_counter_ns()
        state.append(*row)
        samples_ns.append(float(time.perf_counter_ns() - started))
    return {
        "unit": "ns/append",
        "sample_count": len(samples_ns),
        "timer_overhead_included": True,
        "mean": statistics.fmean(samples_ns),
        "p50": percentile(samples_ns, 50),
        "p95": percentile(samples_ns, 95),
        "p99": percentile(samples_ns, 99),
        "max": max(samples_ns),
    }


def environment_metadata() -> dict:
    return {
        "platform": platform.platform(),
        "machine": platform.machine(),
        "python": platform.python_version(),
        "numpy": np.__version__,
        "taflow": taflow.__version__,
        "talib_python": getattr(original_talib, "__version__", "unknown"),
    }


def generate(name: str, repeats: int) -> dict:
    matrix = {}
    for size in SIZES:
        matrix[str(size)] = {mode: run_isolated(name, mode, size, repeats) for mode in MODES}
        matrix[str(size)]["taflow_pipeline"] = {
            "available": False,
            "reason": "The shared multi-indicator Rust Pipeline is scheduled but not implemented.",
        }
    return {
        "schema_version": 2,
        "function": name,
        "date": date.today().isoformat(),
        "environment": environment_metadata(),
        "correctness_benchmark": correctness_result(name),
        "benchmark_protocol": {
            "sizes": list(SIZES),
            "repeats": repeats,
            "process_warmup_runs": 1,
            "stream_history_warmup_bars": WARMUP_BARS,
            "input_dtype": "float64",
            "input_generation": "deterministic trend plus sinusoidal components",
            "peak_memory_metric": "isolated-process peak RSS delta after input allocation",
            "latency_percentiles": "linear interpolation over repeated end-to-end calls",
        },
        "benchmark_matrix": matrix,
        "streaming_append_latency": streaming_latency(name),
    }


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("functions", nargs="*", choices=sorted(FUNCTIONS))
    parser.add_argument("--repeats", type=int, default=DEFAULT_REPEATS)
    parser.add_argument("--output-dir", type=Path, default=Path("reports"))
    args = parser.parse_args()
    names = args.functions or list(FUNCTIONS)
    for name in names:
        path = args.output_dir / f"{name}.json"
        previous = json.loads(path.read_text()) if path.exists() else {}
        generated = generate(name, args.repeats)
        if "stream_benchmark" in previous:
            generated["rust_stream_benchmark"] = previous["stream_benchmark"]
        generated["correctness"] = previous.get("correctness", {})
        generated["series_sizes"] = previous.get("series_sizes", {})
        path.write_text(json.dumps(generated, indent=2) + "\n")
        print(f"wrote {path}")


if __name__ == "__main__":
    main()
