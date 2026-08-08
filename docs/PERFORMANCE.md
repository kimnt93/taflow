# Performance: what changed and how

TAFlow was rebuilt for speed under one hard constraint: **bit-exactness could
not regress.** Streaming and batch must agree bitwise, chunked input must agree
bitwise, and TA-Lib parity must hold. Several attractive optimizations were
implemented, measured, found to change low-order bits, and thrown away.

This page describes the methods, the measured results, and the things that were
deliberately *not* done. Raw numbers live in
[`../verify/benchmark_reports/`](../verify/benchmark_reports/BENCHMARK.md); the
per-function work list is
[`../plans/optimize-checklist.md`](../plans/optimize-checklist.md).

## The contract

Three properties are asserted on every verification run, for all 287 functions:

1. **Batch matches the oracle** — TA-Lib for the 161 TA-Lib functions, pandas
   for the rolling and EWM operators.
2. **Streaming matches batch, bitwise** — a 9,000-bar backfill followed by
   1,000 live `append` calls equals the one-shot result exactly.
3. **Chunk invariance, bitwise** — feeding the same data in chunks of 1, 10, or
   1000 produces identical output *and* identical internal state.

Property 3 is the demanding one. It means a bulk fast path may not merely
produce the right answer; it must leave the state exactly where a per-bar run
would have left it. Every kernel below was written to that standard.

## Boundary and memory

**Rust-side output caches.** History used to round-trip through Python lists of
boxed floats — `extend` produced an array, Python called `.tolist()` on it,
and `compute()` rebuilt an array from the list. Multi-output indicators were
worse: they stored one Python tuple per bar and transposed with `zip(*)` on
every `compute()`. Now every class keeps a `Vec<f64>` per output in Rust and
`compute()` is a single memcpy. This removed a 3–12× overhead on ~55 classes;
SMA's end-to-end API time went from 10.6× its own kernel time to about 1.1×.

**Bulk slice kernels.** `extend` no longer loops per bar through
`Option<f64>`. Each family has a real slice kernel: a warm-up prologue with
the branches, then a branch-free steady loop that indexes the input slice
directly (the evicted element is just `inputs[i - period]`, so no ring buffer
is touched inside the loop) and writes `NaN` in place for warm-up.

**Single-pass writes.** `extend` fills the output cache directly instead of
building a temporary vector and copying it in — three passes over the data
became one.

**Cache-tiled scratch.** Bulk kernels that need scratch space process the input
in L2-resident tiles and reuse one allocation across tiles. Before this, the
sliding-extrema kernels allocated 8–32 MB of scratch on a million-bar array and
throughput collapsed; scratch is now a constant ~131 KB regardless of input
length.

**The GIL is released** around every bulk kernel, so independent indicators on
separate threads compute in parallel.

## Algorithms

**van Herk–Gil–Werman sliding extrema** for the MAX/MIN/WILLR/STOCH/AROON/
MIDPRICE family. Roughly 3 comparisons per element *independent of window
size*, replacing rescan-on-eviction (which is O(n·period) in the worst case).
Because it only ever compares and copies values that appear in the input, it is
bit-exact by construction — no floating-point reassociation is involved.

**Fused recurrence chains.** T3's six EMAs, the MACD family, and the Wilder
chains (ADX/ADXR/DI) now advance all their constituent states in registers in
one pass, instead of stacking layers that each walk the array separately.

**Monotonic deques, split.** One-sided consumers (WILLR, MIDPRICE, AROON) were
maintaining both a max-deque and a min-deque per bar and discarding half the
work.

**Running sums in candle patterns.** The 61 pattern states recomputed their
10-bar body and shadow averages up to eight times per bar; they now slide
incrementally, matching the batch path's summation order exactly so the two
stay bitwise equal.

**Sorted-ring order statistics** for median, quantile, rank and winsorize — a
FIFO ring paired with a sorted array maintained by binary-search insert and
`copy_within`. Entropy and mode use incremental count maps instead of
rebuilding a histogram per bar.

**Fixed ring buffers** replace `VecDeque` throughout. No indicator allocates in
`append`, and `reset()` clears in place rather than reallocating.

**Precomputed trigonometric tables** for the Hilbert DC-phase Fourier loop,
which was making up to 100 `sin`/`cos` libm calls per bar on arguments that
depend only on `(i, count)` with `count ≤ 50`. A test asserts every table entry
is bitwise equal to the runtime expression, so the substitution is provably an
identity rather than an approximation.

**Lazy per-period MAVP states** with in-order catch-up replay. Retained history
dropped from unbounded (200,000 samples under benchmark conditions) to 176.

## Build

Kernels are plain auto-vectorizable loops with
[`multiversion`](https://crates.io/crates/multiversion) runtime dispatch
(AVX2+FMA / AVX / SSE4.2), replacing hand-written SIMD intrinsics that were
being compiled for the SSE2 baseline — the previous build shipped no AVX and no
FMA at all, so every `mul_add` became a multiply plus an add. Still zero
`unsafe`, and one wheel runs well on old and new CPUs.

Never build a released wheel with `-C target-cpu=native`: it defeats the
runtime dispatch and produces a binary that crashes on older CPUs.
`make build-native` exists for local measurement only.

## Measured results

Kernel throughput vs TA-Lib at 10,000 bars, before and after:

| Function | Before | After |
|---|---:|---:|
| RollingMax (MAX) | 0.33× | **2.47×** |
| CandleGapSideSideWhite | 1.43× | **3.97×** |
| SimpleMovingAverage (SMA) | 1.14× | **2.15×** |
| TripleExponentialAverage (T3) | 0.29× | **1.40×** |
| UltimateOscillator (ULTOSC) | 0.55× | **1.39×** |
| BollingerBands (BBANDS) | 0.46× | **1.19×** |

131 of the 161 TA-Lib-mapped functions now meet or beat the C implementation
at 10k bars, and **every** extended operator clears 20M bars/s. Per-function
numbers across 1k/10k/100k/1M bars, plus append latency and thread scaling, are
in [the benchmark reports](../verify/benchmark_reports/BENCHMARK.md).

Reproduce on your own machine:

```bash
make bench                   # everything
make bench ARGS="SMA MAX"    # a subset
```

## Numerical fixes found along the way

Chasing performance surfaced four correctness problems:

- **CORREL used the wrong form.** Our code computed
  `(n·Σxy − Σx·Σy) / √((n·Σxx − Σx²)(n·Σyy − Σy²))` while TA-Lib's C divides by
  the period *inside* each term. Algebraically identical, numerically not — it
  exceeded tolerance on near-zero correlations. It now replicates `TA_CORREL`
  exactly, including its variance-product guard.
- **Sliding accumulators drift.** `sum += new − old` accumulates rounding error
  without bound on an endless stream. The pair-moments states now reseed from
  the retained window every 64 appends, at bar positions that are identical
  regardless of chunking so chunk invariance survives. Drift over 200k bars
  fell from ~1.6e-11 to ~6.7e-13.
- **Three candle patterns disagreed with themselves.** `CDL3BLACKCROWS` used a
  window offset by one bar, `CDLMATHOLD` averaged 11 bodies while dividing by
  10, and `CDLHAMMER` emitted its first signal one bar early — all in the
  streaming path, contradicting the batch path of the same pattern. The
  existing per-file tests never fired those patterns; a randomized
  batch-vs-streaming test across all 61 patterns caught them.
- **A SIMD reduction was silently breaking the contract.** `sum_f64` summed in
  four lanes while the streaming paths accumulated serially, so the same
  indicator could produce different low bits depending on which path ran. It is
  now serial, and documented as deliberately so.

## What was deliberately not done

Some optimizations are simply incompatible with the bit-exactness contract:

- **Sliding sums where the current code recomputes fresh.** AwesomeOscillator,
  VWMA, RollingVWAP, UlcerIndex, RollingCalmar, RollingInformationRatio,
  VIDYA's CMO scan and the LINEARREG family all rescan their window in a fixed
  order. Converting them to add/evict accumulators changes low-order bits.
  They got contiguous-ring layouts, allocation removal and pass fusion instead.
- **Multi-accumulator reductions.** Breaking a summation's dependency chain
  into four partial sums is 4× faster and produces a different number.
- **Block-scan EMA vectorization.** The EMA recurrence can be parallelized by
  block decomposition, but it changes rounding relative to the serial form for
  a realistic 2–4× on long arrays. The fused scalar chain with real FMA already
  runs at parity.

Four functions (Hurst, RollingAutocorr, SpreadZScore, OrderBlock) sit at the
serial-dependency floor: their cost is a chain of dependent floating-point adds
whose order is load-bearing, and beating the throughput gate would require
exactly the reassociation above. Those are documented as accepted misses rather
than quietly optimized into wrongness.

## Related

- [Indicator reference](INDICATORS.md)
- [Streaming](STREAMING.md) — why per-tick cost is flat
- [Pipelines](PIPELINES.md) — sharing work across indicators
