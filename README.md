<p align="center">
  <h1 align="center">TAFlow</h1>
  <p align="center">
    Rust technical analysis with O(1) streaming updates — TA-Lib parity, no C dependencies
  </p>
  <p align="center">
    Rust-backed, stateful technical analysis for Python
  </p>
</p>

## Contents

1. [Indicators](docs/INDICATORS.md)
2. [Streaming](docs/STREAMING.md)
3. [Pipelines](docs/PIPELINES.md)
4. [Data IO](docs/DATA.md)
5. [Correctness](#correctness)
6. [Performance](#performance)

<p align="center">
  <img src="https://img.shields.io/badge/indicators-393-blue" alt="393 indicators" />
  <img src="https://img.shields.io/badge/TA--Lib_parity-161-blue" alt="161 TA-Lib functions" />
  <img src="https://img.shields.io/badge/correctness-393%2F393_MATCH-brightgreen" alt="393/393 externally matched" />
  <img src="https://img.shields.io/badge/unsafe-zero-brightgreen" alt="zero unsafe" />
  <img src="https://img.shields.io/badge/C_deps-zero-orange" alt="zero C deps" />
  <img src="https://img.shields.io/badge/license-MIT-lightgrey" alt="MIT" />
</p>

## What it is

**393 indicator classes** in one library: the complete 161-function TA-Lib surface
(overlap, momentum, volume, volatility, statistics, cycles, and all 61 candle
patterns) plus 232 extended operators — rolling statistics, EWM operators,
Smart Money Concepts, volatility estimators, and signal helpers. Pure Rust
kernels, PyO3 bindings, no C compiler and no TA-Lib installation.

Three things make it different from a batch TA library:

- **Streaming is the primary path.** Every indicator keeps bounded Rust state.
  `append(bar)` is **O(1)** no matter how long the history is, so a live feed
  costs the same on bar 10 and bar 10,000,000.
- **Chunk invariance is a tested contract.** Feeding 10,000 bars at once, in
  chunks of 7, or one at a time produces **bitwise identical** output and
  internal state — asserted for all 393 indicators by the verification registry.
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

Every class takes its required input series first and configuration after it.
Configuration has documented defaults, so positional or keyword style works:

```python
from taflow import BollingerBands, StochasticOscillator

upper, middle, lower = BollingerBands(close, period=20).compute()
slowk, slowd = StochasticOscillator(high, low, close).compute()
```

Keywords always work. **[docs/INDICATORS.md](docs/INDICATORS.md)** lists every
class with its TA-Lib name, parameters and exact constructor order.

### 3. Go live

Backfill once, then update per tick. No window is recomputed, so the cost per
bar is flat:

```python
from taflow import ExponentialMovingAverage

ema = ExponentialMovingAverage(np.array([], dtype=np.float64), timeperiod=20)
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
import numpy as np
from taflow import AverageTrueRange, ExponentialMovingAverage
from taflow.op import TAPipeline

pipe = TAPipeline()
high_s, low_s, close_s = pipe.source("high"), pipe.source("low"), pipe.source("close")

empty = np.array([], dtype=np.float64)
fast = pipe.indicator("fast", ExponentialMovingAverage(empty, timeperiod=12), close_s)
slow = pipe.indicator("slow", ExponentialMovingAverage(empty, timeperiod=26), close_s)
atr_n = pipe.indicator(
    "atr", AverageTrueRange(empty, empty, empty, timeperiod=14), high_s, low_s, close_s
)

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
| [Indicators](docs/INDICATORS.md) | Indicator classes by category — TA-Lib name, parameters, constructor order, the shared class contract |
| [Streaming](docs/STREAMING.md) | Live updates, warm-up, backfill-then-stream, chunk invariance, `reset`, threading, per-tick cost |
| [Pipelines](docs/PIPELINES.md) | Building causal graphs, expressions, evaluate-once semantics, custom nodes, when not to use one |
| [Data in / out](docs/DATA.md) | Every accepted input container, output converters, dataframes, the adapter gateway, `RollingApply`, `SessionFlags` |
| [Correctness](verify/CORRECTNESS.md) | External oracle and lifecycle result for all 393 registered indicators |
| [Benchmarks](verify/BENCHMARK.md) | Correctness-gated vector timings for all 393 registered indicators |
| [Optimization notes](docs/PERFORMANCE.md) | What was optimized, the bit-exactness contract, and which optimizations were rejected |

## Performance

The authoritative benchmark was generated on 2026-08-11 after every registered
indicator passed its selected external oracle. It covers all 393 indicators at
1k, 10k, 100k, and 1m bars, plus fresh-state runs at 1, 5, and 10 concurrent
threads. Each row reports Python API and native-kernel speedup against the same
reference used for correctness.

See [verify/BENCHMARK.md](verify/BENCHMARK.md) for the aggregate vector table.
Complete warm-up/thread matrices and raw repeated samples are retained under
[`verify/evidence/benchmark/`](verify/evidence/benchmark/).

### Reproduce it

```bash
make bench                   # all 393 indicators, 1k/10k/100k/1M bars
make bench ARGS="SMA MAX"    # a subset
```

`make build-native` builds with `-C target-cpu=native` for local measurement.
It must never be shipped, since the resulting binary may use instructions that
are unavailable on older CPUs.

## Correctness

Correctness is verified before performance is measured, on every run.

- **Oracle verification** — all 393 indicators match their selected independent
  TA-Lib, Wickra, pandas-ta-classic, NumPy, or smartmoneyconcepts reference.
  The same run checks cold batch, warmed continuation, reset/replay, and repeated
  native `extend` chunks of 1, 10, and 1,000 bars.
- Four functions — VAR, STDDEV, CORREL and BETA — reproduce TA-Lib
  **bitwise**, byte for byte at 1M bars, by replicating its exact accumulation
  order.
- Every public class is also checked for constructor backfill, empty-state
  startup, scalar append, chunked continuation, fluent identity, length, and
  reset behavior. Current status: **400/400 public interfaces pass**.

```bash
make check                   # unit tests + oracle parity for all 393 indicators
make verify ARGS="EMA ATR"   # oracle parity for a subset
```

See [verify/CORRECTNESS.md](verify/CORRECTNESS.md) for every registered class,
[verify/BENCHMARK.md](verify/BENCHMARK.md) for timings, and
[docs/PERFORMANCE.md](docs/PERFORMANCE.md) for optimization methods.

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
