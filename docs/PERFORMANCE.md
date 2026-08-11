# Performance: what changed and how

TAFlow was rebuilt for speed under one hard constraint: **bit-exactness could
not regress.** Streaming and batch must agree bitwise, chunked input must agree
bitwise, and TA-Lib parity must hold. Several attractive optimizations were
implemented, measured, found to change low-order bits, and thrown away.

This page describes the methods, the measured results, and the things that were
deliberately *not* done. Raw numbers live in
[`../verify/BENCHMARK.md`](../verify/BENCHMARK.md); the
per-function work list is
[`../plans/optimize-checklist.md`](../plans/optimize-checklist.md).

## The contract

Three properties are asserted on every verification run, for all 393 indicators:

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

**Preallocated per-period MAVP states** advance causally from their TA-Lib
alignment offsets. This replaces lazy catch-up history, which could still grow
without bound when a permitted period was never selected. The stricter state
uses bounded memory and performs no allocation in `append`; its 1M-vector rate
is 8.85M bars/s (0.79× TA-Lib), while warmed one-bar continuation remains over
21,000× faster than recomputing the full oracle history.

## Build

Kernels are plain auto-vectorizable loops with
[`multiversion`](https://crates.io/crates/multiversion) runtime dispatch
(AVX2+FMA / AVX / SSE4.2), replacing hand-written SIMD intrinsics that were
being compiled for the SSE2 baseline. Still zero `unsafe`, and one wheel runs
well on old and new CPUs.

Dispatch initially covered only the element-wise kernels, which left the
recurrence kernels — the ones that actually use `f64::mul_add` — without FMA on
a stock wheel, where each call fell back to libm's `fma()`. Benchmarking a
portable build caught it: T3 sat at 0.51× vs TA-Lib while the same source built
with `target-cpu=native` cleared 1×. Extending dispatch to ten recurrence
kernels fixed it:

| kernel | before | after |
|---|---:|---:|
| T3 | 68M bars/s | **289M** (4.2×) |
| TRIX | 124M | **491M** (4.0×) |
| MACD family | ~135M | **405–429M** (~3×) |
| PPO / APO | 179M / 223M | **505M / 532M** |
| KAMA | 157M | **345M** |
| BBANDS | 151M | **318M** |

The dispatched portable build is now *faster* than a whole-crate
`target-cpu=native` build for these kernels, so portability costs nothing.
`mul_add` is an explicitly fused operation and libm's `fma()` is correctly
rounded, so hardware FMA returns the identical bit pattern — verified by
fingerprinting 18 kernels × 4 chunkings × 250k bars against the pre-change
build: all 81 fingerprints identical.

Never build a released wheel with `-C target-cpu=native`: it defeats the
runtime dispatch and produces a binary that crashes on older CPUs.
`make build-native` exists for local measurement only.

## Two lessons worth carrying forward

**Better asymptotics are not automatically faster.** The index-returning
extrema (MAXINDEX/MININDEX/MINMAXINDEX) had already been "optimized" by
replacing TA-Lib's O(period) rescan with a monotonic deque — amortized O(1)
instead of O(period). It was **4–6× slower**. The deque pays two unpredictable
pop-loops on every single bar, while the rescan does contiguous,
branch-predictable work only when the tracked candidate ages out, roughly once
per `period` bars. Restoring a faithful replica of the C state machine took
MAXINDEX from 0.68× to 1.93×.

## The bug no Rust test could catch

An audit comparing every core bulk kernel against every PyO3 binding found
**seven kernels that nothing called**. Each had been written, unit-tested, and
proven bitwise-equal to the per-bar path in Rust — while the Python `extend`
still looped `append` one bar at a time. Every Rust test passed the whole time,
because the kernels themselves were correct; they were simply unreachable from
the API users actually use.

| function | before wiring | after |
|---|---|---|
| Donchian | 19.3M bars/s | **214.5M** |
| RollingCorrelation | 0.41× | **1.91×** |
| TripleExponentialRateOfChange | 0.98× | **3.38×** |
| PlusDirectionalIndicator | 0.97× | **1.79×** |
| KnowSureThing | 30.0M bars/s | **66.2M** |
| RollingBeta | 1.00× | **1.52×** |
| VariablePeriodMovingAverage | 0.37× | 0.79× |

Two subtler variants of the same bug turned up later: a pyclass that correctly
called `extend_slice_into` on a type that never overrode it (so it silently
resolved to the per-bar trait default), and a real bulk signature whose body was
`for input in inputs { self.append(input) }`.

If you add a bulk kernel to this codebase, check that a binding calls it *and*
that the call reaches a real implementation. The test suite will not tell you —
every one of these passed its correctness tests the whole time.

## Measured results

Kernel throughput vs TA-Lib at 10,000 bars, before and after:

| Function | Before | After |
|---|---:|---:|
| CandleGapSideSideWhite | 1.43× | **3.97×** |
| TripleExponentialRateOfChange (TRIX) | 0.98× | **3.38×** |
| RollingMax (MAX) | 0.33× | **2.47×** |
| MoneyFlowIndex (MFI) | 0.57× | **2.37×** |
| SimpleMovingAverage (SMA) | 1.14× | **2.15×** |
| PercentagePriceOscillator (PPO) | 0.88× | **2.09×** |
| RollingArgmax (MAXINDEX) | 0.68× | **1.93×** |
| RollingCorrelation (CORREL) | 0.41× | **1.91×** |
| StochasticOscillator (STOCH) | 0.92× | **1.57×** |
| TripleExponentialAverage (T3) | 0.29× | **1.40×** |
| BollingerBands (BBANDS) | 0.46× | **1.19×** |

153 of the 161 TA-Lib-mapped functions now meet or beat the C implementation
at 10k bars (median **1.61×**, mean **2.01×**). Per-function
numbers across 1k/10k/100k/1M bars, plus append latency and thread scaling, are
in [the benchmark report](../verify/BENCHMARK.md).

The independently corrected extension paths retain high throughput through the
canonical Python interface at 10,000 bars:

| Function | Python batch throughput |
|---|---:|
| ZeroLagExponentialMovingAverage | **208.2M bars/s** |
| TomDeMarkSequential | **169.4M bars/s** |
| KlingerVolumeOscillator | **91.5M bars/s** |
| FairValueGap | **88.9M bars/s** |
| VariableIndexDynamicAverage | **88.1M bars/s** |
| EvenBetterSinewave | **52.3M bars/s** |
| FisherTransform | **27.5M bars/s** |
| JurikMovingAverage | **11.7M bars/s** |

VIDYA keeps rolling positive/negative momentum sums in O(1), increasing the
corrected path from about 15.3M to 88.1M bars/s. JMA keeps its 66-sample
volatility average in O(1), increasing the exact adaptive recurrence from about
7.6M to 11.7M bars/s. The JMA recurrence remains more expensive than the
previous simplified formula, but direct local measurement still places the
Rust-backed Python call over 200× ahead of pandas-ta-classic for 10,000 bars.

Reproduce on your own machine:

```bash
make bench                   # everything
make bench ARGS="SMA MAX"    # a subset
```

## Numerical fixes found along the way

Chasing performance surfaced several correctness problems, and fixing them
taught a lesson worth stating plainly: **when a library is verified against
another implementation, "more accurate" and "more correct" are not the same
thing.**

**Four functions now match TA-Lib bitwise.** The obvious fix for a drifting
sliding accumulator is to periodically recompute it from the window. Measured,
that made VAR and STDDEV *worse* at every interval tried — because TA-Lib's own
accumulators drift too, and reseeding moves us toward mathematical truth and
away from the oracle we are checked against. Replicating TA-Lib's exact
statement order instead gives an exact match:

| function | drift before | now |
|---|---|---|
| RollingVariance | 3.14e-09 | **0.0, bitwise** |
| RollingStandardDeviation | 2.61e-08 | **0.0, bitwise** |
| RollingCorrelation | 4.29e-09 | **0.0, bitwise** |
| RollingBeta | 5.48e-11 | **0.0, bitwise** |

CORREL and BETA need *different* accumulation orders (`TA_CORREL` removes the
trailing bar before adding the new one; `TA_BETA` adds, emits, then removes),
so they no longer share a moments struct.

**Periodic reseeding is still right where the oracle does not drift.** TA-Lib's
CCI rescans its buffer for the average each bar rather than sliding it, so
reseeding converges onto it: CCI went from 2.40e-09 to 1.32e-11 with a reseed
every 64 appends, at bar positions that are identical regardless of chunking so
chunk invariance survives.

**CORREL also used the wrong formula.** It computed
`(n·Σxy − Σx·Σy) / √((n·Σxx − Σx²)(n·Σyy − Σy²))` while TA-Lib's C divides by
the period *inside* each term — algebraically identical, numerically not, and
it exceeded tolerance on near-zero correlations.

**Sometimes the oracle is the broken one.** RollingZScore appeared to fail by
1.95e-08. Checked against 50-digit Decimal arithmetic, taflow was within
3.7e-15 of exact and *pandas* was off by 2.3e-08 — its rolling `std` uses an
add/remove accumulator that degrades on low-variance windows. The verification
oracle was replaced with a fresh per-window computation, not the implementation.

**Three candle patterns disagreed with themselves.** `CDL3BLACKCROWS` used a
window offset by one bar, `CDLMATHOLD` averaged 11 bodies while dividing by 10,
and `CDLHAMMER` emitted its first signal one bar early — all in the streaming
path, contradicting the batch path of the same pattern. The existing per-file
tests never fired those patterns; a randomized batch-vs-streaming test across
all 61 caught them.

**A SIMD reduction was silently breaking the contract.** `sum_f64` summed in
four lanes while streaming paths accumulated serially, so the same indicator
could produce different low bits depending on which path ran. It is now serial,
and documented as deliberately so.

### A note on FMA

Bitwise TA-Lib parity depends on plain multiply-and-add, so fused
multiply-add would break it. Tested directly: builds with and without
`-C target-cpu=native` produce identical results at 1M bars, because Rust
guarantees IEEE semantics and will not contract without an explicit `mul_add`
call. The parity is safe under `make build-native`.

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
