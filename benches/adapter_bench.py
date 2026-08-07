#!/usr/bin/env python3
"""Benchmark execution-graph and container-adapter overhead.

This intentionally complements (rather than changes) ``benches/bench.py``:
that runner measures indicator parity and TA-Lib speed, while this runner
measures conversion and one-pass graph execution for NumPy, Python lists, and
optional Arrow/Polars containers.
"""
from __future__ import annotations

import argparse
import json
import time
from pathlib import Path

import numpy as np

import taflow
from taflow.talib.state import EMA


def _timed(fn, repeats=5):
    fn()
    samples = []
    for _ in range(repeats):
        start = time.perf_counter_ns()
        fn()
        samples.append((time.perf_counter_ns() - start) / 1e9)
    return {"mean_ms": float(np.mean(samples) * 1e3),
            "p50_ms": float(np.median(samples) * 1e3),
            "ops_per_sec": float(1 / np.mean(samples))}


def _pipeline(values):
    pipeline = taflow.Pipeline()
    ema = pipeline.indicator("ema", EMA(20), pipeline.source("close"))
    pipeline.output("ema", ema)
    return pipeline.extend({"close": values})


def _streaming(values):
    pipeline = taflow.Pipeline()
    ema = pipeline.indicator("ema", EMA(20), pipeline.source("close"))
    pipeline.output("ema", ema)
    pipeline.extend({"close": values})
    return pipeline


def run(sizes, repeats):
    rows = []
    for size in sizes:
        values = np.linspace(90.0, 110.0, size, dtype=np.float64)
        row = {"size": size, "conversion": {}, "pipeline": {}}
        for name, value in (("numpy", values), ("list", values.tolist())):
            row["conversion"][name] = _timed(lambda v=value, n=name: taflow.AdapterGateway.input(v, adapter=n), repeats)
            row["pipeline"][name] = _timed(lambda v=value, n=name: _pipeline(taflow.AdapterGateway.input(v, adapter=n)), repeats)
        stream = _streaming(values)
        row["streaming_append"] = _timed(lambda: stream.append({"close": 105.0}), repeats)
        for name, value in (("arrow", values), ("polars", values)):
            try:
                converted = taflow.AdapterGateway.output(value, adapter=name)
            except ImportError:
                continue
            row["conversion"][name] = _timed(lambda v=converted, n=name: taflow.AdapterGateway.input(v, adapter=n), repeats)
        rows.append(row)
    return rows


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--quick", action="store_true", help="run through 100k bars")
    parser.add_argument("--repeats", type=int, default=5)
    parser.add_argument("--output", type=Path)
    args = parser.parse_args()
    sizes = [1_000, 10_000, 100_000] if args.quick else [1_000, 10_000, 100_000, 1_000_000]
    result = {"benchmark": "execution-adapters", "sizes": sizes, "rows": run(sizes, args.repeats)}
    text = json.dumps(result, indent=2)
    if args.output:
        args.output.write_text(text + "\n")
    else:
        print(text)


if __name__ == "__main__":
    main()
