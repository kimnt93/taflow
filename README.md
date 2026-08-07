<p align="center">
  <h1 align="center">TAFlow</h1>
  <p align="center">
    Rust technical-analysis library with O(1) streaming updates — TA-Lib compatible, no C dependencies
  </p>
  <p align="center">
    <a href="README.zh-CN.md">中文</a> · <a href="verify/REPORT.md">Correctness report</a> · <a href="reports/BENCHMARK.md">Benchmarks</a>
  </p>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/TA--Lib_functions-161-blue" alt="161 TA-Lib functions" />
  <img src="https://img.shields.io/badge/extended_operators-100%2B-blue" alt="extended operators" />
  <img src="https://img.shields.io/badge/unsafe-zero-brightgreen" alt="zero unsafe" />
  <img src="https://img.shields.io/badge/C_deps-zero-orange" alt="zero C deps" />
  <img src="https://img.shields.io/badge/license-BSD--3--Clause-lightgrey" alt="BSD-3-Clause" />
</p>

## What it is

- **Complete TA-Lib surface** — all 161 functions (overlap, momentum, volume,
  volatility, statistics, cycles, 61 candle patterns) as a drop-in
  `taflow.talib` module. `pip install taflow`, no C compiler.
- **Persistent streaming states** — every indicator keeps bounded Rust state:
  `append(bar)` is **O(1)** regardless of history length, and chunked
  `extend` replay is identical to one-shot batch (chunk-invariant).
- **Beyond TA-Lib** — rolling operators (`rolling_median`, `rolling_zscore`,
  `rolling_skew`, …), EWM operators, Smart Money Concepts
  (`fair_value_gap`, swing structure, order blocks), extended indicators
  (Supertrend-class trend tools, Keltner/Donchian, VWAP variants), and
  signal helpers — same contract: aligned series in, same-length causal
  series out.

## Quick start

```python
import numpy as np
from taflow import talib          # TA-Lib compatible batch API

rsi  = talib.RSI(close, timeperiod=14)
macd, signal, hist = talib.MACD(close, fastperiod=12, slowperiod=26, signalperiod=9)
upper, mid, lower = talib.BBANDS(close, timeperiod=20)
```

Streaming (the reason taflow exists) — feed history once, then update per
tick without recomputing:

```python
from taflow import ExponentialMovingAverage

ema = ExponentialMovingAverage(timeperiod=20)
ema.extend(history)               # backfill, NaN during warm-up
ema.append(next_close)            # O(1) live update
latest = ema.value
full_series = ema.compute()       # cached history, no recomputation
```

TA-Lib-style stateful aliases are also available:
`from taflow.talib.state import RSI, MACD`.

## Correctness and performance

Verified continuously against reference implementations:

- **Oracle verification** — every function is checked against original
  TA-Lib (and pandas for rolling/EWM operators) on batch output, on a
  9k-warmup + 1k-live-append continuation, and for bitwise chunk
  invariance. Current status and known deviations:
  [verify/REPORT.md](verify/REPORT.md). Run it yourself:
  `cd verify && uv sync && uv run python verify.py`.
- **Benchmarks** — one registry-driven runner
  (`python benches/bench.py`) measures bulk throughput vs TA-Lib, live
  continuation latency (flat ~0.2 µs/append vs TA-Lib's O(history)
  recompute), and multi-thread scaling; per-function reports land in
  `reports/`.

## Development

```bash
maturin develop --release -m crates/taflow-python/Cargo.toml   # build
cargo test --workspace                                          # Rust tests
python benches/bench.py EMA --quick                             # bench one function
cd verify && uv sync && uv run python verify.py                 # oracle check
```

Contributor/agent workflow: implementation inventory and priorities live in
[`plans/`](plans/); every function must pass the review gates in
[`CHECK.md`](CHECK.md) (naming and module rules, typing, docs, one function
per file) before its checklist box is ticked.

## Layout

```
crates/taflow-core/     # pure Rust kernels (batch + streaming state, zero unsafe)
crates/taflow-python/   # PyO3 bindings, zero-copy NumPy boundary
python/taflow/          # descriptive streaming API + taflow.talib compat surface
benches/bench.py        # benchmark runner → reports/
verify/                 # standalone uv project: oracle verification → verify/REPORT.md
plans/ + CHECK.md       # roadmap checklists + AI review contract
```

## License

BSD-3-Clause
