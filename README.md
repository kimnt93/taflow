<p align="center">
  <h1 align="center">TAFlow</h1>
  <p align="center">
    Rust technical analysis with O(1) streaming updates — TA-Lib parity, no C dependencies
  </p>
  <p align="center">
    <a href="docs/INDICATORS.md">Indicator reference</a> ·
    <a href="verify/REPORT.md">Correctness report</a> ·
    <a href="verify/benchmark_reports/BENCHMARK.md">Benchmarks</a>
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
  costs the same on bar 10 and bar 10,000,000. Batch libraries recompute the
  whole window every tick.
- **Chunk invariance is a tested contract.** Feeding 10,000 bars at once, in
  chunks of 7, or one at a time produces **bitwise identical** output and
  bitwise identical internal state. This is asserted for all 287 functions on
  every run, not assumed.
- **The full history is free.** `compute()` returns the entire aligned series
  from a Rust-side cache — no recomputation, one memcpy.

## Install

Requires Python ≥ 3.9. Installing from source (the first two routes) also
needs a Rust toolchain — [rustup](https://rustup.rs) is enough, no C compiler
and no TA-Lib installation.

**From the package index** — the package is named `taflow`
*(not yet published; use one of the routes below until it is)*:

```bash
pip install taflow
uv add taflow
```

**From the git repository** — no clone needed, builds the extension for you:

```bash
pip install "git+https://github.com/kimnt93/taflow"
uv pip install "git+https://github.com/kimnt93/taflow"

# a specific branch, tag, or commit
pip install "git+https://github.com/kimnt93/taflow@main"
```

**From a source checkout** — the development route:

```bash
git clone https://github.com/kimnt93/taflow
cd taflow

make install    # build the release wheel and install it into the active env
make dev        # editable debug build + dev dependencies, for hacking on taflow
make build      # just compile the native extension (.so) in place
```

`make help` lists every target. Verify the result and see how fast it is on
your machine:

```bash
make check      # unit tests + oracle parity for all 287 functions
make bench      # throughput against TA-Lib
```

Wheels dispatch at runtime between AVX2+FMA, AVX, and SSE4.2 kernels, so a
single wheel runs fast on old and new CPUs alike.

NumPy is the only required dependency. The Arrow and Polars converters are
optional extras:

```bash
pip install "taflow[arrow]"      # pyarrow
pip install "taflow[polars]"     # polars
pip install "taflow[adapters]"   # both
```

## Using indicators

Every one of the 287 classes has the same shape. Pass a series to the
constructor and read the result:

```python
import numpy as np
from taflow import SimpleMovingAverage, RelativeStrengthIndex, BollingerBands

sma = SimpleMovingAverage(close, timeperiod=30).compute()
rsi = RelativeStrengthIndex(close, timeperiod=14).compute()

upper, middle, lower = BollingerBands(close, timeperiod=20).compute()
```

Outputs are `float64` NumPy arrays the same length as the input, with `NaN`
during warm-up so indices always line up with your bars. Multi-output
indicators return a tuple of arrays.

Multi-series indicators take their inputs positionally, in OHLCV order:

```python
from taflow import MoneyFlowIndex, AverageTrueRange, Aroon

mfi = MoneyFlowIndex(high, low, close, volume, timeperiod=14).compute()
atr = AverageTrueRange(high, low, close, timeperiod=14).compute()
down, up = Aroon(high, low, timeperiod=14).compute()
```

[`docs/INDICATORS.md`](docs/INDICATORS.md) lists every class with its TA-Lib
name, parameters, and required inputs.

### Streaming

The reason TAFlow exists. Backfill once, then update per tick — no window
recomputation, no growing cost:

```python
from taflow import ExponentialMovingAverage

ema = ExponentialMovingAverage(timeperiod=20)
ema.extend(history)          # backfill; NaN through the warm-up period
ema.append(next_close)       # O(1) live update

ema.value                    # latest value, or None while warming up
ema.compute()                # the whole aligned series, cached
len(ema)                     # bars consumed so far
ema.reset()                  # clear state and history in place
```

`append` returns the indicator, so updates chain. A state resumed with
`extend` after `append` calls continues exactly as if every bar had arrived
one at a time — that is the chunk-invariance guarantee, and it is what makes
backfill-then-stream safe.

## Pipelines

A pipeline is a causal dependency graph: each bar is dispatched **once**, and
shared sub-expressions are evaluated once no matter how many outputs use
them. Indicator nodes keep their state between bars, so the graph streams.

```python
from taflow.op import TAPipeline
from taflow import ExponentialMovingAverage, AverageTrueRange

pipe = TAPipeline()

high, low, close = pipe.source("high"), pipe.source("low"), pipe.source("close")

fast = pipe.indicator("fast", ExponentialMovingAverage(timeperiod=12), close)
slow = pipe.indicator("slow", ExponentialMovingAverage(timeperiod=26), close)
atr  = pipe.indicator("atr",  AverageTrueRange(timeperiod=14), high, low, close)

# Expression nodes support arithmetic on other nodes and on scalars.
pipe.output("macd", pipe.expression("macd", fast - slow))
pipe.output("normalized", pipe.expression("normalized", (fast - slow) / atr))
pipe.output("atr", atr)
```

Run it over columns, or one bar at a time:

```python
result = pipe.extend({"high": high, "low": low, "close": close})
result["macd"]        # np.ndarray, same length as the input

tick = pipe.append({"high": 101.2, "low": 99.8, "close": 100.5})
tick["normalized"]    # float for this bar

pipe.reset()          # reset every stateful node in the graph
```

`pipe.outputs` lists the registered output names. Because `fast` and `slow`
feed two different outputs but are evaluated once per bar, a pipeline is
strictly cheaper than driving the same indicators separately.

## Converters

TAFlow computes on contiguous `float64`. The adapters convert at the edges so
you can stay in whatever dataframe library you use:

```python
from taflow.op import AdaptInput, ToNumpy, ToPandas, ToPolars, ToArrow, ToList

values = AdaptInput(df, column="close")     # DataFrame/Series/Arrow/list → float64 array
series = ToPandas(sma, name="sma_30")       # results back out
frame  = ToPolars(sma, name="sma_30")
arr    = ToArrow(sma)
```

Input conversion is also implicit: passing a pandas Series, a Polars series,
an Arrow array, or a plain Python list to any indicator works — it is
converted once per call, then all computation happens in Rust.

Register your own container type with the gateway:

```python
from taflow.op import TAAdapterGateway
TAAdapterGateway.register("mylib", MyAdapter)    # needs .input() and .output()
```

Two more helpers live in the same namespace: `RollingApply(series, timeperiod, fn)`
for incremental-compatible custom reducers, and `SessionFlags(session_ids)` to
turn session identifiers into native session-boundary flags.

## What's new in this implementation

TAFlow was rebuilt for speed with one hard constraint: **bit-exactness could
not regress.** Streaming and batch must agree bitwise, chunked input must
agree bitwise, and TA-Lib parity must hold. Several attractive optimizations
were measured, found to change low-order bits, and deliberately rejected —
the rejections are documented in
[`plans/optimize-checklist.md`](plans/optimize-checklist.md).

**Boundary and memory**

- **Rust-side output caches.** History used to round-trip through Python
  lists of boxed floats; `compute()` was rebuilding an array from them. Now
  every class caches `Vec<f64>` in Rust and `compute()` is a single memcpy.
  This alone removed a 3–12× overhead on ~55 classes.
- **Bulk slice kernels.** `extend` no longer loops per bar through
  `Option<f64>`; each family has a real slice kernel with a warm-up prologue
  and a branch-free steady loop, writing `NaN` in place.
- **Cache-tiled scratch.** Bulk kernels process input in L2-resident tiles, so
  throughput no longer collapses on million-bar arrays.
- **The GIL is released** around every bulk kernel, so multiple symbols can
  compute in parallel threads.

**Algorithms**

- **van Herk–Gil–Werman sliding extrema** for the MAX/MIN/WILLR/STOCH/AROON/
  MIDPRICE family — ~3 comparisons per element *independent of window size*,
  replacing rescan-on-eviction. Comparison-only, so it is bit-exact by
  construction.
- **Fused recurrence chains.** T3's six EMAs, the MACD family, and the Wilder
  chains (ADX/ADXR/DI) advance in registers in one pass instead of stacking
  layers that each walk the array.
- **Monotonic deques, split.** One-sided consumers (WILLR, MIDPRICE, AROON)
  were maintaining both a max and a min deque and discarding half the work.
- **Running sums in candle patterns.** The 61 pattern states recomputed 10-bar
  body/shadow averages up to 8 times per bar; they now slide incrementally.
- **Sorted-ring order statistics** for median/quantile/rank/winsorize, and
  incremental count maps for entropy and mode.
- **Fixed ring buffers** replace `VecDeque` throughout; no allocation happens
  in any `append`, and `reset()` never reallocates.
- **Precomputed trigonometric `make help` lists every target. Or build and install directly:

```bash
maturin develop --release -m crates/taflow-python/Cargo.toml     # editable
maturin build   --release -m crates/taflow-python/Cargo.toml     # wheel → dist/
```tables** for the Hilbert DC-phase Fourier loop,
  proven bitwise identical to the runtime expression by test.
- **Lazy per-period MAVP states** with in-order catch-up replay: retained
  history dropped from unbounded (200,000 samples in benchmark conditions) to
  176.

**Build**

- Kernels are plain auto-vectorizable loops with
  [`multiversion`](https://crates.io/crates/multiversion) runtime dispatch
  (AVX2+FMA / AVX / SSE4.2) instead of hand-written SIMD intrinsics compiled
  for the SSE2 baseline. Still zero `unsafe`.

**Numerical fixes found along the way**

- `CORREL` used an algebraically-equivalent but numerically different form
  from TA-Lib's C; it now replicates `TA_CORREL` exactly, including its
  variance-product guard.
- Long-running sliding accumulators now reseed periodically, bounding drift
  from ~1.6e-11 to ~6.7e-13 over 200k bars.
- Three candle patterns (`CDL3BLACKCROWS`, `CDLMATHOLD`, `CDLHAMMER`) had
  streaming paths that disagreed with their own batch paths — an off-by-one
  window, an average over 11 bodies instead of 10, and a signal emitted one
  bar early. All three are fixed and covered by tests.

## Correctness and performance

Correctness is verified before performance is measured, on every run.

- **Oracle verification** — every function is checked against TA-Lib (or
  pandas, for rolling and EWM operators) on batch output, on a 9k-warm-up +
  1k-live-append continuation, and for bitwise chunk invariance at chunk sizes
  1, 10, and 1000. Current status: **287/287 MATCH**
  ([verify/REPORT.md](verify/REPORT.md)).
- **Benchmarks** — bulk throughput against TA-Lib at 1k/10k/100k/1M bars, plus
  per-append latency and thread scaling. Per-function reports with raw timing
  samples are in
  [verify/benchmark_reports/](verify/benchmark_reports/BENCHMARK.md).

```bash
make check                   # unit tests + oracle parity for all 287 functions
make verify ARGS="EMA ATR"   # oracle parity for a subset
make bench  ARGS="SMA MAX"   # benchmark a subset
```

The streaming advantage is structural rather than a constant factor: TAFlow's
per-bar update is O(1) while a batch library recomputes its window, so on a
live feed TAFlow is ~2 orders of magnitude cheaper per tick and the gap widens
with history length.

## Development

```bash
make dev            # editable build
make build          # compile the native extension (.so) in place, optimized
make check          # unit tests + oracle parity — the gate before any commit
make test           # cargo test --workspace + pytest, without the oracle pass
make lint           # clippy, warnings denied
make fmt            # rustfmt
make build-native   # -C target-cpu=native, for local measurement only
```

Never ship a `target-cpu=native` build — released wheels rely on runtime
dispatch for portability.

Contributor workflow: the roadmap and optimization checklist live in
[`plans/`](plans/); every function must pass the review gates in
[`CHECK.md`](CHECK.md) (naming and module rules, typing, docs, one function
per file). Any change touching a kernel must keep chunk invariance bitwise
green — that is the contract the whole design rests on.

## Layout

```
crates/taflow-core/     # Rust kernels: batch + streaming state, zero unsafe
crates/taflow-python/   # PyO3 bindings, zero-copy NumPy boundary
python/taflow/          # indicator classes, pipelines, adapters
docs/INDICATORS.md      # reference for all 287 functions
verify/                 # standalone uv project: oracle verification + benchmarks
plans/ + CHECK.md       # roadmap, optimization checklist, review contract
```

## License

MIT — see [LICENSE](LICENSE).
