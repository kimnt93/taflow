#!/usr/bin/env python3
"""TAFlow vs TA-Lib Python-interface benchmark runner.

One script implements the whole plan in ``plans/benchmark-plan.md``:

  S1  bulk compute            sizes 100 .. 10M, talib/taflow batch + state-cold
  S2  live continuation       base history x append chunk {1,10,100,1000}
  S3  correctness vs TA-Lib   NaN placement, max abs error, chunk invariance
  S4  functions not in TA-Lib same grids, self-oracle only

Zero-edit registry: functions are discovered from ``taflow.talib`` /
``taflow.talib.state``; inputs, default parameters, and lookback come from
TA-Lib's ``abstract`` metadata. Adding a new indicator to taflow requires no
change here. Functions whose TA-Lib alias does not exist in the installed
TA-Lib automatically run as S4.

Usage:
  python benches/bench.py                # every discovered function, full grid
  python benches/bench.py EMA ATR MACD   # a subset
  python benches/bench.py --quick        # sizes <= 100k, 3 repeats
  python benches/bench.py --aggregate-only
Outputs: reports/<FN>.json, reports/<FN>.md, reports/BENCHMARK.md
"""

from __future__ import annotations

import argparse
import gc
import json
import multiprocessing as mp
import platform
import sys
import time
from dataclasses import dataclass
from datetime import date
from pathlib import Path

import numpy as np

SCHEMA_VERSION = 3
S1_SIZES = (100, 1_000, 10_000, 100_000, 1_000_000, 10_000_000)
S2_CHUNKS = (1, 10, 100, 1_000)
S5_THREADS = (1, 2, 5, 10, 20)
S5_BASE = 100_000            # per-thread warmed history for the parallel test
S5_UPDATES_TAFLOW = 2_000    # appends per thread
S5_UPDATES_TALIB = 10        # full recomputes per thread (each is O(base))
CORRECTNESS_BARS = 100_000
CHUNK_INVARIANCE_CHUNKS = (1, 10, 1_000)
DEFAULT_REPEATS = 20
MIN_TIMED_SECONDS = 0.02  # autorange target per repeat
S2_EXTRA_BARS = 60_000    # update stream appended after the base history
ALLCLOSE_RTOL = 1e-8
ALLCLOSE_ATOL = 1e-10

# The only per-function knowledge the registry cannot derive: input domains
# for functions whose mathematical domain is narrower than a price series.
INPUT_DOMAIN_OVERRIDES = {
    "ACOS": "unit",
    "ASIN": "unit",
}


# ---------------------------------------------------------------------------
# Data generation
# ---------------------------------------------------------------------------

def make_data(n: int, seed: int = 42) -> dict[str, np.ndarray]:
    """Mean-reverting log-price OHLCV series.

    An AR(1) in log space keeps prices bounded (~[25, 400]) at any length, so
    EXP/COSH/LN stay finite even at 10M bars, unlike a drifting random walk.
    """
    def ar1_fast(offset: int) -> np.ndarray:
        """Exact AR(1) scan `x[i] = decay*x[i-1] + noise[i]`, vectorized.

        Within a block: x[i] = decay^i * cumsum(noise[j] * decay^-j)[i]; the
        block size caps decay^-j at e^~4 so nothing overflows.
        """
        r = np.random.default_rng(seed + offset)
        noise = r.normal(0.0, 0.02, n)
        decay = 1.0 - 0.001
        block = 4096
        pows = decay ** np.arange(block)
        inv_pows = decay ** -np.arange(block)
        x = np.empty(n)
        carry = 0.0
        for start in range(0, n, block):
            b = noise[start:start + block]
            m = len(b)
            conv = pows[:m] * np.cumsum(b * inv_pows[:m])
            x[start:start + m] = conv + carry * decay * pows[:m]
            carry = x[start + m - 1]
        return 100.0 * np.exp(x)

    close = ar1_fast(0)
    r2 = np.random.default_rng(seed + 1000)
    spread = close * 0.01
    high = close + r2.uniform(0.0, 1.0, n) * spread
    low = close - r2.uniform(0.0, 1.0, n) * spread
    open_ = low + r2.uniform(0.0, 1.0, n) * (high - low)
    volume = r2.uniform(1e5, 1e6, n)

    unit_noise = np.random.default_rng(seed + 2000).normal(0.0, 0.05, n)
    unit = np.clip(np.cumsum(unit_noise) % 1.8 - 0.9, -0.99, 0.99)

    return {
        "open": open_, "high": high, "low": low, "close": close,
        "volume": volume,
        "close2": ar1_fast(3000),
        "periods": np.random.default_rng(seed + 4000).uniform(2.0, 30.0, n),
        "unit": unit,
    }


def input_arrays(spec: "FunctionSpec", data: dict, n: int | None = None,
                 offset: int = 0) -> list[np.ndarray]:
    out = []
    for name in spec.inputs:
        if name in ("price", "real"):
            key = "unit" if spec.domain == "unit" else "close"
        elif name == "price0":
            key = "close"
        elif name == "price1":
            key = "close2"
        elif name in data:
            key = name
        else:
            key = "close"
        arr = data[key]
        if n is not None:
            arr = arr[offset:offset + n]
        out.append(np.ascontiguousarray(arr))
    return out


# ---------------------------------------------------------------------------
# Registry
# ---------------------------------------------------------------------------

@dataclass
class FunctionSpec:
    name: str
    inputs: tuple[str, ...]
    params: dict
    in_talib: bool
    lookback: int
    has_state: bool
    state_error: str | None = None
    domain: str = "prices"


def build_registry() -> dict[str, FunctionSpec]:
    import talib
    from talib import abstract
    import taflow._native as native
    import taflow.talib as tt
    import taflow.talib.state as ts

    specs: dict[str, FunctionSpec] = {}
    for name in native.get_functions():
        if not hasattr(tt, name):
            continue  # declared but not yet implemented
        in_talib = hasattr(talib, name)
        inputs: tuple[str, ...]
        params: dict
        lookback = 0
        if in_talib:
            info = abstract.Function(name).info
            flat: list[str] = []
            for value in info["input_names"].values():
                if isinstance(value, (list, tuple)):
                    flat.extend(value)
                else:
                    flat.append(value)
            # abstract reports the *default* series (e.g. 'close') for single
            # price inputs; keep the role names so BETA maps price0/price1.
            roles = []
            for role, value in info["input_names"].items():
                if isinstance(value, (list, tuple)):
                    roles.extend(value)
                else:
                    roles.append(role if role.startswith("price") else value)
            inputs = tuple(roles)
            params = dict(info["parameters"])
            lookback = int(abstract.Function(name).lookback)
        else:
            # Future non-TA-Lib functions: fall back to a close-only spec;
            # richer taflow-native metadata can extend this without edits here.
            inputs, params = ("price",), {}

        has_state, state_error = False, None
        if hasattr(ts, name):
            try:
                getattr(ts, name)(**params)
                has_state = True
            except Exception as exc:  # constructor mismatch: run batch-only
                state_error = f"{type(exc).__name__}: {exc}"
        specs[name] = FunctionSpec(
            name=name, inputs=inputs, params=params, in_talib=in_talib,
            lookback=lookback, has_state=has_state, state_error=state_error,
            domain=INPUT_DOMAIN_OVERRIDES.get(name, "prices"),
        )
    return specs


# ---------------------------------------------------------------------------
# Callables under test
# ---------------------------------------------------------------------------

def talib_call(spec: FunctionSpec, arrays: list[np.ndarray]):
    import talib
    return getattr(talib, spec.name)(*arrays, **spec.params)


def taflow_batch_call(spec: FunctionSpec, arrays: list[np.ndarray]):
    import taflow.talib as tt
    return getattr(tt, spec.name)(*arrays, **spec.params)


def new_state(spec: FunctionSpec):
    import taflow.talib.state as ts
    return getattr(ts, spec.name)(**spec.params)


def state_extend(state, arrays: list[np.ndarray]):
    return state.extend(*arrays)


# ---------------------------------------------------------------------------
# Timing helpers (run inside child processes)
# ---------------------------------------------------------------------------

def timed_mean_call(fn, repeats: int) -> list[float]:
    """Autoranged mean seconds-per-call, one entry per repeat."""
    fn()  # warm-up (allocations, code paths)
    iters = 1
    while True:
        t0 = time.perf_counter_ns()
        for _ in range(iters):
            fn()
        elapsed = (time.perf_counter_ns() - t0) / 1e9
        if elapsed >= MIN_TIMED_SECONDS or iters >= 1 << 20:
            break
        iters = min(iters * max(2, int(MIN_TIMED_SECONDS / max(elapsed, 1e-9))),
                    1 << 20)
    samples = []
    for _ in range(repeats):
        gc.disable()
        t0 = time.perf_counter_ns()
        for _ in range(iters):
            fn()
        samples.append((time.perf_counter_ns() - t0) / 1e9 / iters)
        gc.enable()
    return samples


def stats_block(samples_s: list[float], bars: int | None = None) -> dict:
    arr = np.asarray(samples_s)
    out = {
        "mean_ms": float(arr.mean() * 1e3),
        "min_ms": float(arr.min() * 1e3),
        "p50_ms": float(np.median(arr) * 1e3),
        "repeats": len(samples_s),
    }
    if bars:
        out["ops_per_sec"] = float(bars / arr.mean())
    return out


# --- child-process cell runners --------------------------------------------

def _cell_bulk(conn, spec: FunctionSpec, size: int, repeats: int,
               data: dict) -> None:
    arrays = input_arrays(spec, data, n=size)
    result: dict = {"size": size}
    if spec.in_talib:
        result["talib_batch"] = stats_block(
            timed_mean_call(lambda: talib_call(spec, arrays), repeats), size)
    result["taflow_batch"] = stats_block(
        timed_mean_call(lambda: taflow_batch_call(spec, arrays), repeats), size)
    if spec.has_state:
        def cold():
            state_extend(new_state(spec), arrays)
        result["taflow_state_cold"] = stats_block(
            timed_mean_call(cold, repeats), size)
    conn.send(result)


def _cell_continuation(conn, spec: FunctionSpec, base: int, chunk: int,
                       repeats: int, data: dict) -> None:
    """taflow: extend(base) untimed, then timed appends/extends of `chunk`.

    talib baselines: full recompute over base+chunk, and tail recompute over
    lookback+chunk (the expert-user window).
    """
    updates = max(20, min(500, 20_000 // chunk))
    need = base + chunk * updates
    arrays_full = input_arrays(spec, data, n=need)
    base_arrays = [a[:base] for a in arrays_full]

    result: dict = {"base": base, "chunk": chunk, "timed_updates": updates}

    if spec.has_state:
        state = new_state(spec)
        state_extend(state, base_arrays)
        lat_ns = np.empty(updates)
        if chunk == 1:
            bars = [a[base:base + updates] for a in arrays_full]
            gc.disable()
            for i in range(updates):
                bar = [b[i] for b in bars]
                t0 = time.perf_counter_ns()
                state.append(*bar)
                lat_ns[i] = time.perf_counter_ns() - t0
            gc.enable()
        else:
            gc.disable()
            for i in range(updates):
                sl = [a[base + i * chunk: base + (i + 1) * chunk]
                      for a in arrays_full]
                t0 = time.perf_counter_ns()
                state.extend(*sl)
                lat_ns[i] = time.perf_counter_ns() - t0
            gc.enable()
        result["taflow"] = {
            "mean_latency_us": float(lat_ns.mean() / 1e3),
            "p50_us": float(np.median(lat_ns) / 1e3),
            "p99_us": float(np.percentile(lat_ns, 99) / 1e3),
            "updates_per_sec": float(1e9 / lat_ns.mean()),
            "bars_per_sec": float(chunk * 1e9 / lat_ns.mean()),
        }

    if spec.in_talib:
        window = [a[:base + chunk] for a in arrays_full]
        samples = timed_mean_call(lambda: talib_call(spec, window),
                                  min(repeats, 3))
        result["talib_full_recompute"] = {
            "mean_latency_us": float(np.mean(samples) * 1e6)}
        tail_n = min(base + chunk, spec.lookback + chunk + 1)
        tail = [a[base + chunk - tail_n: base + chunk] for a in arrays_full]
        samples = timed_mean_call(lambda: talib_call(spec, tail),
                                  min(repeats, 3))
        result["talib_tail_window"] = {
            "mean_latency_us": float(np.mean(samples) * 1e6),
            "window_bars": tail_n,
        }
        if spec.has_state:
            t = result["taflow"]["mean_latency_us"]
            result["speedup_vs_full"] = (
                result["talib_full_recompute"]["mean_latency_us"] / t if t else None)
            result["speedup_vs_tail"] = (
                result["talib_tail_window"]["mean_latency_us"] / t if t else None)
    conn.send(result)


def _run_threads(n_threads: int, worker) -> float:
    """Run `worker(i)` on n_threads Python threads; return wall seconds from
    simultaneous start (barrier) to last join."""
    import threading
    barrier = threading.Barrier(n_threads + 1)

    def runner(i):
        barrier.wait()
        worker(i)

    threads = [threading.Thread(target=runner, args=(i,))
               for i in range(n_threads)]
    for t in threads:
        t.start()
    barrier.wait()
    t0 = time.perf_counter_ns()
    for t in threads:
        t.join()
    return (time.perf_counter_ns() - t0) / 1e9


def _cell_parallel(conn, spec: FunctionSpec, threads: int, repeats: int,
                   data: dict) -> None:
    """S5: N independent streams (one per thread), e.g. N symbols on a live
    feed. taflow: each thread appends to its own warmed state. talib: each
    thread recomputes the full base+1 window per update. Aggregate
    updates/sec across threads is the figure of merit; scaling vs the
    1-thread row shows who releases the GIL during compute."""
    base = S5_BASE
    result: dict = {"threads": threads, "base": base, "chunk": 1}

    if spec.has_state:
        updates = S5_UPDATES_TAFLOW
        arrays = input_arrays(spec, data, n=base + updates)
        base_arrays = [a[:base] for a in arrays]
        # one warmed state per thread; identical per-thread workload
        states = []
        for _ in range(threads):
            st = new_state(spec)
            state_extend(st, base_arrays)
            states.append(st)
        bars = list(zip(*[a[base:base + updates].tolist() for a in arrays]))

        def taflow_worker(i):
            st = states[i]
            for bar in bars:
                st.append(*bar)

        walls = []
        for _ in range(repeats):
            gc.disable()
            walls.append(_run_threads(threads, taflow_worker))
            gc.enable()
        wall = float(np.mean(walls))
        result["taflow"] = {
            "wall_s": wall,
            "updates_per_thread": updates,
            "agg_updates_per_sec": threads * updates / wall,
            "per_update_us": wall / updates * 1e6,  # latency seen by a thread
        }

    if spec.in_talib:
        updates = S5_UPDATES_TALIB
        arrays = input_arrays(spec, data, n=base + 1)

        def talib_worker(i):
            for _ in range(updates):
                talib_call(spec, arrays)

        walls = []
        for _ in range(repeats):
            gc.disable()
            walls.append(_run_threads(threads, talib_worker))
            gc.enable()
        wall = float(np.mean(walls))
        result["talib_full_recompute"] = {
            "wall_s": wall,
            "updates_per_thread": updates,
            "agg_updates_per_sec": threads * updates / wall,
        }

    if "taflow" in result and "talib_full_recompute" in result:
        result["speedup"] = (result["taflow"]["agg_updates_per_sec"]
                             / result["talib_full_recompute"]
                             ["agg_updates_per_sec"])
    conn.send(result)


def _as_tuple(result) -> tuple[np.ndarray, ...]:
    return result if isinstance(result, tuple) else (result,)


def _compare(a: np.ndarray, b: np.ndarray) -> dict:
    a = np.asarray(a, dtype=np.float64)
    b = np.asarray(b, dtype=np.float64)
    nan_a, nan_b = np.isnan(a), np.isnan(b)
    nan_mismatches = int((nan_a != nan_b).sum())
    both = ~nan_a & ~nan_b
    max_err = float(np.max(np.abs(a[both] - b[both]))) if both.any() else 0.0
    passed = nan_mismatches == 0 and bool(
        np.allclose(a[both], b[both], rtol=ALLCLOSE_RTOL, atol=ALLCLOSE_ATOL))
    return {"nan_mismatches": nan_mismatches,
            "max_abs_error": max_err, "passed": passed}


def _merge_compare(outputs_a, outputs_b) -> dict:
    per = [_compare(x, y) for x, y in zip(_as_tuple(outputs_a),
                                          _as_tuple(outputs_b))]
    return {
        "outputs": len(per),
        "nan_mismatches": sum(p["nan_mismatches"] for p in per),
        "max_abs_error": max(p["max_abs_error"] for p in per),
        "passed": all(p["passed"] for p in per),
    }


def _cell_correctness(conn, spec: FunctionSpec, data: dict) -> None:
    n = CORRECTNESS_BARS
    arrays = input_arrays(spec, data, n=n)
    result: dict = {"bars": n}
    taflow_out = taflow_batch_call(spec, arrays)

    if spec.in_talib:
        talib_out = talib_call(spec, arrays)
        result["batch_vs_talib"] = _merge_compare(taflow_out, talib_out)

    if spec.has_state:
        state_out = state_extend(new_state(spec), arrays)
        oracle = talib_out if spec.in_talib else taflow_out
        key = "state_vs_talib" if spec.in_talib else "state_vs_batch"
        result[key] = _merge_compare(state_out, oracle)

        reference = _as_tuple(state_out)
        chunk_ok = True
        for chunk in CHUNK_INVARIANCE_CHUNKS:
            state = new_state(spec)
            pieces = []
            for start in range(0, n, chunk):
                sl = [a[start:start + chunk] for a in arrays]
                pieces.append(_as_tuple(state_extend(state, sl)))
            joined = tuple(np.concatenate([p[i] for p in pieces])
                           for i in range(len(pieces[0])))
            for got, want in zip(joined, reference):
                if not np.array_equal(np.asarray(got), np.asarray(want),
                                      equal_nan=True):
                    chunk_ok = False
        result["chunk_invariance"] = {
            "chunks": list(CHUNK_INVARIANCE_CHUNKS),
            "bitwise_identical": chunk_ok,
        }

    checks = [v.get("passed") for v in result.values()
              if isinstance(v, dict) and "passed" in v]
    checks.append(result.get("chunk_invariance", {}).get("bitwise_identical",
                                                         True))
    result["passed"] = all(c is not False for c in checks)
    conn.send(result)


def run_in_child(target, *args, timeout: int = 1800):
    ctx = mp.get_context("fork")
    parent, child = ctx.Pipe(duplex=False)
    proc = ctx.Process(target=target, args=(child, *args))
    proc.start()
    child.close()
    try:
        result = parent.recv() if parent.poll(timeout) else {
            "error": f"timeout after {timeout}s"}
    except EOFError:
        result = {"error": f"child crashed (exit {proc.exitcode})"}
    proc.join(30)
    if proc.is_alive():
        proc.kill()
    return result


# ---------------------------------------------------------------------------
# Reports
# ---------------------------------------------------------------------------

def environment_block() -> dict:
    import numpy
    import taflow
    cpu = ""
    try:
        for line in Path("/proc/cpuinfo").read_text().splitlines():
            if line.startswith("model name"):
                cpu = line.split(":", 1)[1].strip()
                break
    except OSError:
        pass
    env = {
        "platform": platform.platform(),
        "python": platform.python_version(),
        "numpy": numpy.__version__,
        "taflow": getattr(taflow, "__version__", "unknown"),
        "cpu": cpu,
    }
    try:
        import talib
        env["talib_python"] = talib.__version__
    except ImportError:
        env["talib_python"] = None
    return env


def fmt_ms(v) -> str:
    return f"{v:.4f}" if v is not None else "—"


def fmt_ops(v) -> str:
    if v is None:
        return "—"
    for unit, div in (("G", 1e9), ("M", 1e6), ("K", 1e3)):
        if v >= div:
            return f"{v / div:.1f}{unit}"
    return f"{v:.0f}"


def fmt_x(v) -> str:
    if v is None:
        return "—"
    return f"{v:,.0f}×" if v >= 100 else f"{v:.2f}×"


def render_md(report: dict) -> str:
    spec_in_talib = report["in_talib"]
    lines = [f"# {report['function']} — benchmark"
             + (" vs TA-Lib " + report["environment"]["talib_python"]
                if spec_in_talib else " (not in TA-Lib — self-oracle)"), ""]

    c = report.get("correctness") or {}
    if c:
        status = "**PASS**" if c.get("passed") else "**FAIL** ⚠️"
        details = []
        for key in ("batch_vs_talib", "state_vs_talib", "state_vs_batch"):
            if key in c:
                details.append(f"{key.replace('_', ' ')}: max abs err "
                               f"{c[key]['max_abs_error']:.2e}, "
                               f"{c[key]['nan_mismatches']} NaN mismatches")
        if "chunk_invariance" in c:
            ok = c["chunk_invariance"]["bitwise_identical"]
            details.append("chunk replay "
                           + ("bitwise-identical" if ok else "DIVERGES"))
        lines += [f"Correctness: {status} @{c.get('bars', 0):,} bars — "
                  + "; ".join(details), ""]
    if report.get("state_error"):
        lines += [f"State class unavailable: `{report['state_error']}` — "
                  "batch modes only.", ""]

    bulk = report.get("bulk") or []
    if bulk:
        lines += ["## Bulk compute (mean seconds per call, "
                  f"{report['protocol']['repeats']} repeats)", ""]
        if spec_in_talib:
            lines += ["| Bars | TA-Lib ms | TAFlow ms | Speedup | TAFlow ops/s "
                      "| State-cold ms | Speedup |",
                      "|---:|---:|---:|---:|---:|---:|---:|"]
        else:
            lines += ["| Bars | TAFlow ms | TAFlow ops/s | State-cold ms |",
                      "|---:|---:|---:|---:|"]
        for row in bulk:
            if "error" in row:
                lines.append(f"| {row['size']:,} | ERROR: {row['error']} |")
                continue
            tb = row.get("talib_batch", {})
            fb = row.get("taflow_batch", {})
            sc = row.get("taflow_state_cold", {})
            if spec_in_talib:
                s1 = (tb.get("mean_ms") / fb["mean_ms"]
                      if tb and fb.get("mean_ms") else None)
                s2 = (tb.get("mean_ms") / sc["mean_ms"]
                      if tb and sc.get("mean_ms") else None)
                lines.append(
                    f"| {row['size']:,} | {fmt_ms(tb.get('mean_ms'))} | "
                    f"{fmt_ms(fb.get('mean_ms'))} | {fmt_x(s1)} | "
                    f"{fmt_ops(fb.get('ops_per_sec'))} | "
                    f"{fmt_ms(sc.get('mean_ms'))} | {fmt_x(s2)} |")
            else:
                lines.append(
                    f"| {row['size']:,} | {fmt_ms(fb.get('mean_ms'))} | "
                    f"{fmt_ops(fb.get('ops_per_sec'))} | "
                    f"{fmt_ms(sc.get('mean_ms'))} |")
        lines.append("")

    cont = report.get("continuation") or []
    if cont:
        lines += ["## Live continuation (latency per update; TA-Lib = full "
                  "recompute of base+chunk)", ""]
        if spec_in_talib:
            lines += ["| Base | Chunk | TAFlow µs | TA-Lib µs | Speedup | "
                      "Tail-window µs | vs tail | TAFlow bars/s |",
                      "|---:|---:|---:|---:|---:|---:|---:|---:|"]
        else:
            lines += ["| Base | Chunk | TAFlow µs | TAFlow bars/s |",
                      "|---:|---:|---:|---:|"]
        for row in cont:
            if "error" in row:
                lines.append(f"| {row['base']:,} | {row['chunk']:,} | "
                             f"ERROR: {row['error']} |")
                continue
            tf = row.get("taflow", {})
            if spec_in_talib:
                full = row.get("talib_full_recompute", {}).get(
                    "mean_latency_us")
                tail = row.get("talib_tail_window", {}).get("mean_latency_us")
                lines.append(
                    f"| {row['base']:,} | {row['chunk']:,} | "
                    f"{tf.get('mean_latency_us', float('nan')):.2f} | "
                    f"{full:.1f} | {fmt_x(row.get('speedup_vs_full'))} | "
                    f"{tail:.2f} | {fmt_x(row.get('speedup_vs_tail'))} | "
                    f"{fmt_ops(tf.get('bars_per_sec'))} |"
                    if tf and full is not None else
                    f"| {row['base']:,} | {row['chunk']:,} | — | "
                    f"{full if full is not None else float('nan'):.1f} | — | "
                    f"— | — | — |")
            elif tf:
                lines.append(
                    f"| {row['base']:,} | {row['chunk']:,} | "
                    f"{tf['mean_latency_us']:.2f} | "
                    f"{fmt_ops(tf.get('bars_per_sec'))} |")
        k1 = [r for r in cont if r["chunk"] == 1 and r.get("taflow")]
        if k1:
            r = k1[-1]
            lines += ["", f"Append latency (base {r['base']:,}, chunk 1): "
                      f"p50 {r['taflow']['p50_us']:.2f} µs, "
                      f"p99 {r['taflow']['p99_us']:.2f} µs."]
        lines.append("")

    par = report.get("parallel") or []
    if par:
        lines += [f"## Parallel continuation ({S5_BASE:,}-bar warmed history "
                  "per thread, one independent stream per thread)", ""]
        lines += ["| Threads | TAFlow agg updates/s | Scaling | "
                  "TA-Lib agg updates/s | Scaling | Speedup |",
                  "|---:|---:|---:|---:|---:|---:|"]
        tf1 = next((r["taflow"]["agg_updates_per_sec"] for r in par
                    if r["threads"] == 1 and r.get("taflow")), None)
        tl1 = next((r["talib_full_recompute"]["agg_updates_per_sec"]
                    for r in par
                    if r["threads"] == 1 and r.get("talib_full_recompute")),
                   None)
        for row in par:
            if "error" in row:
                lines.append(f"| {row['threads']} | ERROR: {row['error']} |")
                continue
            tf = row.get("taflow", {}).get("agg_updates_per_sec")
            tl = row.get("talib_full_recompute", {}).get(
                "agg_updates_per_sec")
            lines.append(
                f"| {row['threads']} | {fmt_ops(tf)} | "
                + (f"{tf / tf1:.2f}×" if tf and tf1 else "—") + " | "
                + f"{fmt_ops(tl)} | "
                + (f"{tl / tl1:.2f}×" if tl and tl1 else "—") + " | "
                + f"{fmt_x(row.get('speedup'))} |")
        lines += ["", "Each thread owns its own state/stream (N-symbol live "
                  "feed model). Scaling >1× with threads requires the "
                  "underlying call to release the GIL.", ""]

    lines += ["---", "Python-interface measurement: numbers include "
              "conversion/boundary overhead by design. Rust-core-only "
              "numbers live in criterion benches and are not comparable.", ""]
    return "\n".join(lines)


def aggregate(reports_dir: Path) -> str:
    rows = []
    for path in sorted(reports_dir.glob("*.json")):
        try:
            rep = json.loads(path.read_text())
        except json.JSONDecodeError:
            continue
        if rep.get("schema_version") != SCHEMA_VERSION:
            continue
        bulk = rep.get("bulk") or []
        best = max(bulk, key=lambda r: r["size"], default=None)
        speedup = ops = None
        if best:
            fb = best.get("taflow_batch", {})
            tb = best.get("talib_batch", {})
            ops = fb.get("ops_per_sec")
            if tb.get("mean_ms") and fb.get("mean_ms"):
                speedup = tb["mean_ms"] / fb["mean_ms"]
        cont = [r for r in (rep.get("continuation") or [])
                if r["chunk"] == 1 and r.get("taflow")]
        cbest = max(cont, key=lambda r: r["base"], default=None)
        par = [r for r in (rep.get("parallel") or []) if r.get("taflow")]
        par_scale = None
        if par:
            p1 = next((r for r in par if r["threads"] == 1), None)
            pmax = max(par, key=lambda r: r["threads"])
            if p1 and pmax["threads"] > 1:
                par_scale = (pmax["taflow"]["agg_updates_per_sec"]
                             / p1["taflow"]["agg_updates_per_sec"])
        c = rep.get("correctness") or {}
        if rep["in_talib"]:
            correct = "PASS" if c.get("passed") else (
                "FAIL" if c else "—")
        else:
            correct = "self-PASS" if c.get("passed") else (
                "self-FAIL" if c else "—")
        rows.append({
            "fn": rep["function"], "in_talib": rep["in_talib"],
            "correct": correct, "speedup": speedup, "ops": ops,
            "bulk_size": best["size"] if best else None,
            "p50": cbest["taflow"]["p50_us"] if cbest else None,
            "cont_speedup": cbest.get("speedup_vs_full") if cbest else None,
            "cont_base": cbest["base"] if cbest else None,
            "par_scale": par_scale,
            "par_threads": max((r["threads"] for r in par), default=None),
        })
    if not rows:
        return "# BENCHMARK\n\nNo schema-v3 reports found.\n"

    biggest = max((r["bulk_size"] or 0) for r in rows)
    cont_base = max((r["cont_base"] or 0) for r in rows)
    lines = [
        "# Aggregate benchmark: TAFlow vs TA-Lib (Python interface)", "",
        f"Generated {date.today().isoformat()} from "
        f"{len(rows)} schema-v{SCHEMA_VERSION} reports.", "",
        f"| Function | In TA-Lib | Correct | Bulk speedup @{biggest:,} "
        f"| Bulk ops/s | Append p50 | Cont. speedup @{cont_base:,} "
        f"| Thread scaling |",
        "|---|---|---|---:|---:|---:|---:|---:|",
    ]
    for r in sorted(rows, key=lambda r: r["fn"]):
        par = ("—" if r["par_scale"] is None
               else f"{r['par_scale']:.2f}× @{r['par_threads']}T")
        lines.append(
            f"| {r['fn']} | {'yes' if r['in_talib'] else 'no'} | "
            f"{r['correct']} | {fmt_x(r['speedup'])} | {fmt_ops(r['ops'])} | "
            + (f"{r['p50']:.2f} µs" if r["p50"] is not None else "—")
            + f" | {fmt_x(r['cont_speedup'])} | {par} |")

    speedups = [r["speedup"] for r in rows if r["speedup"]]
    fails = [r["fn"] for r in rows if "FAIL" in r["correct"]]
    slower = [r["fn"] for r in rows if r["speedup"] and r["speedup"] < 1.0]
    lines += ["", "## Summary", ""]
    if speedups:
        lines.append(f"- Median bulk speedup: {np.median(speedups):.2f}× "
                     f"across {len(speedups)} TA-Lib-comparable functions.")
    lines.append(f"- Correctness failures: "
                 f"{', '.join(fails) if fails else 'none'}.")
    lines.append(f"- Slower than TA-Lib in bulk: "
                 f"{', '.join(slower) if slower else 'none'}.")
    return "\n".join(lines) + "\n"


# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------

def run_function(spec: FunctionSpec, sizes: list[int], bases: list[int],
                 repeats: int, scenarios: set[str], data_cache: dict,
                 reports_dir: Path) -> None:
    def data_for(n: int) -> dict:
        # Canonicalize to a few shared sizes so the cache survives across
        # functions (children fork and share it copy-on-write). Sizes above
        # LARGE_KEEP are evicted before generating another large set.
        LARGE_KEEP = 2_000_000
        candidates = [CORRECTNESS_BARS, 200_000,
                      1_000_000 + S2_EXTRA_BARS, 10_000_000 + S2_EXTRA_BARS]
        key = next((c for c in candidates if c >= n), n)
        if key not in data_cache:
            if key > LARGE_KEEP:
                for other in [k for k in data_cache if k > LARGE_KEEP]:
                    del data_cache[other]
            data_cache[key] = make_data(key)
        return data_cache[key]

    report: dict = {
        "schema_version": SCHEMA_VERSION,
        "function": spec.name,
        "in_talib": spec.in_talib,
        "date": date.today().isoformat(),
        "environment": environment_block(),
        "protocol": {
            "repeats": repeats,
            "seed_base": 42,
            "isolated_processes": True,
            "autorange_min_seconds": MIN_TIMED_SECONDS,
            "params": spec.params,
            "inputs": list(spec.inputs),
            "allclose": {"rtol": ALLCLOSE_RTOL, "atol": ALLCLOSE_ATOL},
        },
        "state_error": spec.state_error,
    }

    if "s3" in scenarios:
        report["correctness"] = run_in_child(
            _cell_correctness, spec, data_for(CORRECTNESS_BARS))

    if "s1" in scenarios:
        rows = []
        for size in sizes:
            cell = run_in_child(_cell_bulk, spec, size, repeats,
                                data_for(max(size, CORRECTNESS_BARS)))
            cell.setdefault("size", size)
            rows.append(cell)
        report["bulk"] = rows

    if "s2" in scenarios and spec.has_state:
        rows = []
        for base in bases:
            need = base + S2_EXTRA_BARS
            for chunk in S2_CHUNKS:
                cell = run_in_child(_cell_continuation, spec, base, chunk,
                                    repeats, data_for(need))
                cell.setdefault("base", base)
                cell.setdefault("chunk", chunk)
                rows.append(cell)
        report["continuation"] = rows

    if "s5" in scenarios and (spec.has_state or spec.in_talib):
        rows = []
        for threads in S5_THREADS:
            cell = run_in_child(_cell_parallel, spec, threads,
                                min(repeats, 3),
                                data_for(S5_BASE + S5_UPDATES_TAFLOW))
            cell.setdefault("threads", threads)
            rows.append(cell)
        report["parallel"] = rows

    # Partial-scenario runs must not clobber sections measured earlier.
    reports_dir.mkdir(parents=True, exist_ok=True)
    out_json = reports_dir / f"{spec.name}.json"
    if out_json.exists():
        try:
            old = json.loads(out_json.read_text())
        except json.JSONDecodeError:
            old = {}
        if old.get("schema_version") == SCHEMA_VERSION:
            for key in ("correctness", "bulk", "continuation", "parallel"):
                if key not in report and key in old:
                    report[key] = old[key]
    out_json.write_text(json.dumps(report, indent=1, default=float))
    (reports_dir / f"{spec.name}.md").write_text(render_md(report))


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("functions", nargs="*",
                        help="function names (default: all discovered)")
    parser.add_argument("--repeats", type=int, default=DEFAULT_REPEATS)
    parser.add_argument("--max-size", type=int, default=S1_SIZES[-1])
    parser.add_argument("--quick", action="store_true",
                        help="sizes <= 100k, 3 repeats")
    parser.add_argument("--scenarios", default="s1,s2,s3,s5",
                        help="comma list of s1,s2,s3,s5 "
                             "(s5 = parallel-thread continuation)")
    parser.add_argument("--reports-dir", type=Path, default=Path("reports"))
    parser.add_argument("--aggregate-only", action="store_true")
    parser.add_argument("--list", action="store_true",
                        help="list discovered functions and exit")
    args = parser.parse_args()

    if args.aggregate_only:
        out = args.reports_dir / "BENCHMARK.md"
        out.write_text(aggregate(args.reports_dir))
        print(f"wrote {out}")
        return 0

    registry = build_registry()
    if args.list:
        for spec in registry.values():
            state = "state" if spec.has_state else (
                f"batch-only ({spec.state_error})" if spec.state_error
                else "batch-only")
            print(f"{spec.name:24s} {'talib' if spec.in_talib else 's4  '}  "
                  f"{state}  inputs={','.join(spec.inputs)}")
        return 0

    max_size = 100_000 if args.quick else args.max_size
    repeats = 3 if args.quick else args.repeats
    sizes = [s for s in S1_SIZES if s <= max_size]
    bases = list(sizes)
    scenarios = set(args.scenarios.split(","))

    names = args.functions or sorted(registry)
    unknown = [n for n in names if n not in registry]
    if unknown:
        print(f"unknown functions: {', '.join(unknown)}", file=sys.stderr)
        return 1

    data_cache: dict = {}
    t_start = time.time()
    for i, name in enumerate(names, 1):
        spec = registry[name]
        t0 = time.time()
        run_function(spec, sizes, bases, repeats, scenarios, data_cache,
                     args.reports_dir)
        print(f"[{i}/{len(names)}] {name}: {time.time() - t0:.1f}s")
    out = args.reports_dir / "BENCHMARK.md"
    out.write_text(aggregate(args.reports_dir))
    print(f"done in {time.time() - t_start:.0f}s — wrote {out}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
