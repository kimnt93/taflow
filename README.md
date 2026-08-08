<p align="center">
  <h1 align="center">TAFlow</h1>
  <p align="center">
    Rust technical analysis with O(1) streaming updates — TA-Lib parity, no C dependencies
  </p>
  <p align="center">
    <a href="docs/INDICATORS.md">Indicators</a> ·
    <a href="docs/STREAMING.md">Streaming</a> ·
    <a href="docs/PIPELINES.md">Pipelines</a> ·
    <a href="docs/DATA.md">Data&nbsp;in&nbsp;/&nbsp;out</a> ·
    <a href="docs/PERFORMANCE.md">Performance</a>
  </p>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/functions-287-blue" alt="287 functions" />
  <img src="https://img.shields.io/badge/TA--Lib_parity-161-blue" alt="161 TA-Lib functions" />
  <img src="https://img.shields.io/badge/correctness-287%2F287_MATCH-brightgreen" alt="287/287 match" />
  <img src="https://img.shields.io/badge/unsafe-zero-brightgreen" alt="zero unsafe" />
  <img src="https://img.shields.io/badge/C_deps-zero-orange" alt="zero C deps" />
  <img src="https://img.shields.io/badge/license-MIT-lightgrey" alt="MIT" />
</p>

## What it is

**287 indicators** in one library: the complete 161-function TA-Lib surface
(overlap, momentum, volume, volatility, statistics, cycles, and all 61 candle
patterns) plus 126 extended operators — rolling statistics, EWM operators,
Smart Money Concepts, volatility estimators, and signal helpers. Pure Rust
kernels, PyO3 bindings, no C compiler and no TA-Lib installation.

Three things make it different from a batch TA library:

- **Streaming is the primary path.** Every indicator keeps bounded Rust state.
  `append(bar)` is **O(1)** no matter how long the history is, so a live feed
  costs the same on bar 10 and bar 10,000,000.
- **Chunk invariance is a tested contract.** Feeding 10,000 bars at once, in
  chunks of 7, or one at a time produces **bitwise identical** output and
  internal state — asserted for all 287 functions on every run.
- **The full history is free.** `compute()` returns the entire aligned series
  from a Rust-side cache — no recomputation, one memcpy.

## Install

Requires Python ≥ 3.9. Building from source (the first two routes) also needs a
Rust toolchain — [rustup](https://rustup.rs) is enough.

**From the package index** — the package is named `taflow`
*(not yet published; use a route below until it is)*:

```bash
pip install taflow
uv add taflow
```

**From the git repository** — no clone needed, builds the extension for you:

```bash
pip install "git+https://github.com/kimnt93/taflow"
uv pip install "git+https://github.com/kimnt93/taflow"
pip install "git+https://github.com/kimnt93/taflow@main"   # branch, tag or commit
```

**From a source checkout** — the development route:

```bash
git clone https://github.com/kimnt93/taflow
cd taflow

make install    # build the release wheel and install it into the active env
make dev        # editable debug build + dev dependencies, for hacking on taflow
make build      # just compile the native extension (.so) in place
```

NumPy is the only required dependency; Polars and Arrow support are extras:

```bash
pip install "taflow[adapters]"     # pyarrow + polars
```

## How to use

### 1. Start from the data you already have

Pass a NumPy array, a Python list, a pandas Series, a Polars Series or an
Arrow array — all five work identically, converted once at the boundary:

```python
import numpy as np, pandas as pd, polars as pl, pyarrow as pa
from taflow import SimpleMovingAverage

close = np.cumsum(np.random.default_rng(0).normal(0, 1, 500)) + 100.0

SimpleMovingAverage(close, timeperiod=10).compute()                     # numpy
SimpleMovingAverage(close.tolist(), timeperiod=10).compute()            # list
SimpleMovingAverage(pd.Series(close), timeperiod=10).compute()          # pandas
SimpleMovingAverage(pl.Series("close", close), timeperiod=10).compute() # polars
SimpleMovingAverage(pa.array(close), timeperiod=10).compute()           # arrow
```

Working from a dataframe, hand over the columns:

```python
frame = pd.DataFrame({"open": o, "high": h, "low": l, "close": c, "volume": v})

frame["sma_10"] = SimpleMovingAverage(frame["close"], timeperiod=10).compute()
```

The same line works unchanged on a Polars DataFrame. Full detail — multi-column
frames, output converters, custom containers — in
**[docs/DATA.md](docs/DATA.md)**.

### 2. Call the indicator

Every class has the same shape: construct with the data, read the result.
Outputs are `float64` arrays the same length as the input, `NaN` through
warm-up, so indices always line up with your bars.

```python
from taflow import RelativeStrengthIndex, MoneyFlowIndex, AverageTrueRange, Aroon

rsi = RelativeStrengthIndex(close, timeperiod=14).compute()
mfi = MoneyFlowIndex(high, low, close, volume, timeperiod=14).compute()
atr = AverageTrueRange(high, low, close, timeperiod=14).compute()

down, up = Aroon(high, low, timeperiod=14).compute()     # multi-output → tuple
```

**One wrinkle.** Most classes take their series first and configuration after.
A minority (64 of 299 — Bollinger Bands, the stochastics, MACD, the `Rolling*`
statistics) take configuration first, so pass the data by keyword:

```python
from taflow import BollingerBands, StochasticOscillator

upper, middle, lower = BollingerBands(values=close, period=20).compute()
slowk, slowd = StochasticOscillator(high=high, low=low, close=close).compute()
```

Keywords always work. **[docs/INDICATORS.md](docs/INDICATORS.md)** lists every
class with its TA-Lib name, parameters and exact constructor order.

### 3. Go live

Backfill once, then update per tick. No window is recomputed, so the cost per
bar is flat:

```python
from taflow import ExponentialMovingAverage

ema = ExponentialMovingAverage(timeperiod=20)
ema.extend(history)          # backfill in one call

for tick in feed:
    ema.append(tick.close)   # O(1)
    if ema.value is not None:
        ...

ema.compute()                # full aligned series, from cache
```

Resuming with `append` after `extend` gives bitwise the same numbers as if
every bar had arrived one at a time. Warm-up handling, `reset()`, threading and
cost details are in **[docs/STREAMING.md](docs/STREAMING.md)**.

### 4. Compute many indicators in one pass

A pipeline is a causal graph: each bar is dispatched once, and shared
sub-expressions are evaluated once no matter how many outputs use them.

```python
from taflow.op import TAPipeline

pipe = TAPipeline()
high_s, low_s, close_s = pipe.source("high"), pipe.source("low"), pipe.source("close")

fast = pipe.indicator("fast", ExponentialMovingAverage(timeperiod=12), close_s)
slow = pipe.indicator("slow", ExponentialMovingAverage(timeperiod=26), close_s)
atr_n = pipe.indicator("atr", AverageTrueRange(timeperiod=14), high_s, low_s, close_s)

pipe.output("macd", pipe.expression("macd", fast - slow))
pipe.output("normalized", pipe.expression("normalized", (fast - slow) / atr_n))

result = pipe.extend({"high": high, "low": low, "close": close})   # columns
tick = pipe.append({"high": 101.2, "low": 99.8, "close": 100.5})   # one bar
```

`fast` and `slow` each feed two outputs but step once per bar — verified with
step-counting tests. The full guide, including two sharp edges (unreachable
nodes are never stepped; chaining propagates warm-up `NaN`), is in
**[docs/PIPELINES.md](docs/PIPELINES.md)**.

## Documentation

| Document | What it covers |
|---|---|
| [Indicators](docs/INDICATORS.md) | All 299 classes by category — TA-Lib name, parameters, constructor order, the shared class contract |
| [Streaming](docs/STREAMING.md) | Live updates, warm-up, backfill-then-stream, chunk invariance, `reset`, threading, per-tick cost |
| [Pipelines](docs/PIPELINES.md) | Building causal graphs, expressions, evaluate-once semantics, custom nodes, when not to use one |
| [Data in / out](docs/DATA.md) | Every accepted input container, output converters, dataframes, the adapter gateway, `RollingApply`, `SessionFlags` |
| [Performance](docs/PERFORMANCE.md) | What was optimized and how, measured results, the bit-exactness contract, and which optimizations were rejected |
| [Correctness report](verify/REPORT.md) | Current per-function oracle status |
| [Benchmarks](verify/benchmark_reports/BENCHMARK.md) | Throughput vs TA-Lib at 1k/10k/100k/1M bars |

## Correctness and performance

Correctness is verified before performance is measured, on every run.

- **Oracle verification** — every function is checked against TA-Lib (or
  pandas, for rolling and EWM operators) on batch output, on a 9k-warm-up +
  1k-live-append continuation, and for bitwise chunk invariance at chunk sizes
  1, 10 and 1000. Current status: **287/287 MATCH**.
- **Benchmarks** — bulk throughput against TA-Lib at 1k/10k/100k/1M bars, plus
  per-append latency and thread scaling, with raw timing samples retained.

```bash
make check                   # unit tests + oracle parity for all 287 functions
make verify ARGS="EMA ATR"   # oracle parity for a subset
make bench  ARGS="SMA MAX"   # benchmark a subset
```

131 of the 161 TA-Lib functions meet or beat the C implementation at 10k bars,
and every extended operator clears 20M bars/s. On a live feed the advantage is
structural rather than a constant factor: per-tick cost is flat while a
recompute-the-window approach grows with the period. See
[docs/PERFORMANCE.md](docs/PERFORMANCE.md) for methods and measurements.

## Development

```bash
make dev            # editable build
make build          # compile the native extension (.so) in place, optimized
make check          # unit tests + oracle parity — the gate before any commit
make test           # cargo test --workspace + pytest, without the oracle pass
make lint           # clippy, warnings denied
make fmt            # rustfmt
make build-native   # -C target-cpu=native, local measurement only — never ship
```

`make help` lists every target.

Contributor workflow: the roadmap and optimization checklist live in
[`plans/`](plans/); every function must pass the review gates in
[`CHECK.md`](CHECK.md) (naming and module rules, typing, docs, one function per
file). Any change touching a kernel must keep chunk invariance bitwise green —
that is the contract the whole design rests on.

## Layout

```
crates/taflow-core/     # Rust kernels: batch + streaming state, zero unsafe
crates/taflow-python/   # PyO3 bindings, zero-copy NumPy boundary
python/taflow/          # indicator classes, pipelines, adapters
docs/                   # indicators, streaming, pipelines, data, performance
tests/                  # Python tests (pipelines, adapters)
verify/                 # standalone uv project: oracle verification + benchmarks
plans/ + CHECK.md       # roadmap, optimization checklist, review contract
```

## License

MIT — see [LICENSE](LICENSE).
