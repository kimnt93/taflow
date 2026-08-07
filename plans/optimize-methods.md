# TAFlow optimization methods

Concrete speed-up opportunities found by reading the current code
(`crates/taflow-core/src/stream/`, `simd.rs`, `crates/taflow-python/src/`).
Ordered roughly by expected impact per unit of effort. Each item says *what*
to optimize, *the method*, and — where the method is non-obvious — *why it
works* and what it costs.

A recurring constraint appears throughout: **TA-Lib parity restricts
floating-point reassociation.** Any optimization that changes summation order
(SIMD reductions, prefix sums, parallel scans) changes the low bits of the
result. Decide the comparison contract first (bit-exact vs `1e-8` relative
tolerance); several methods below are only admissible under a tolerance
contract.

---

## 1. Build configuration (biggest free win)

### 1.1 The SIMD feature is currently running on the SSE2 baseline

`wide::f64x4` only emits real 256-bit AVX instructions when the target has
AVX enabled. The workspace has no `.cargo/config.toml` and no
`target-feature` flags, so everything compiles for the x86-64 baseline
(SSE2): every `f64x4` op is split into two 128-bit ops, and `mul_add` in
`simd.rs` and in the scalar `Ema::append` **compiles to a libm call or a
mul+add sequence, not an FMA instruction**, because FMA is not in the
baseline. This silently costs 2–4× on every "SIMD" path and also on the
scalar `mul_add` recurrences.

Method:

- For local benchmarking: `RUSTFLAGS="-C target-cpu=native"`.
- For distributable wheels: function multiversioning with the
  [`multiversion`](https://crates.io/crates/multiversion) crate on the hot
  bulk kernels (`sum_f64`, the `*_array`/`*_simd` functions, future
  `extend_slice` loops). It compiles each kernel for
  `x86-64-v3` (AVX2+FMA), `x86-64-v2`, and baseline, and dispatches once at
  startup. This is how NumPy and TA-Lib-style libraries ship portable fast
  binaries.
- Alternative with zero dependencies: build wheels for the `x86-64-v3`
  microarchitecture level and a baseline fallback wheel (maturin supports
  per-wheel RUSTFLAGS), but multiversioning is cleaner.

`lto = "fat"` and `codegen-units = 1` are already set — good; nothing to do
there. Consider adding `profile-guided optimization` (PGO) only at the very
end; it typically buys 5–10% on branchy code like the warm-up paths.

### 1.2 Rewrite `simd.rs` kernels to let LLVM auto-vectorize

The manual `f64x4::new([data[offset], data[offset+1], ...])` pattern does
four bounds-checked scalar loads and a lane insert per element group, then
`to_array()` + four scalar stores. With AVX enabled, a plain iterator loop is
usually *faster* than this, because LLVM emits unaligned vector loads/stores
directly.

Method: replace manual lane packing with `chunks_exact(4)` /
`chunks_exact_mut(4)` (which removes bounds checks and gives LLVM
known-size blocks), or drop `wide` entirely for the element-wise kernels
(`add_arrays`, `sqrt_array`, price transforms, `true_range_simd`, …) and
write plain loops — with `target-cpu` set they auto-vectorize to the same or
better code. Keep `wide` only where auto-vectorization provably fails
(horizontal reductions, shuffles).

### 1.3 Break the dependency chain in `sum_f64`

`sum_f64` accumulates into a single `f64x4`. Vector FP add has ~4-cycle
latency, so a single accumulator runs at 1 add per 4 cycles instead of 2 adds
per cycle. Method: use 4 independent accumulators and combine at the end
(~4–8× on large slices). Caveat: this reassociates the sum — fine for seeds
compared under tolerance, not fine for bit-exact parity (see §6.1).

---

## 2. The Python boundary (dominates realtime latency)

### 2.1 Kill the double materialization in `extend`

Current path for every bulk call:
`StreamingIndicator::extend` → `Vec<Option<f64>>` (16 bytes/element, one
allocation) → `values_from` maps to `Vec<f64>` (second allocation) →
`PyArray1::from_vec`. That is 3× the necessary memory traffic and 2
allocations per call.

Method: add to the core trait

```rust
fn extend_into(&mut self, inputs: &[f64], out: &mut Vec<f64>)
```

with a default implementation that pushes `value.unwrap_or(f64::NAN)`
directly. The PyO3 `extend` allocates one `Vec<f64>` with exact capacity
(or writes straight into a freshly allocated `PyArray1` via
`PyArray1::new` + slice fill) and hands it over zero-copy. `Option<f64>`
should never cross a bulk boundary.

### 2.2 Give every windowed indicator a real `extend_slice` bulk path

Only `Ema` has `extend_slice` today; every other indicator's `extend` calls
`append` per element, which pays the warm-up branch, `Option` handling, and
ring-buffer bookkeeping per bar. For the "vectorize as fast as talib"
goal, the bulk-from-empty path is the one talib users will measure.

Method per family (details in §4):

- SMA/SUM: seed sum + tight add/subtract slice loop (no deque at all in the
  bulk loop; the window contents are just `inputs[i-p..i]`).
- WMA/regression: same — the recurrences on `sum`/`weighted_sum` only need
  slice indexing.
- MAX/MIN/MIDPOINT/WILLR/STOCH/AROON: van Herk–Gil–Werman block max
  (§4.2) instead of a deque.
- EMA-chains (DEMA/TEMA/T3/MACD/APO/PPO/TRIX): fuse into one pass (§4.3).
- End state of the bulk run must exactly equal the state `append` would have
  produced, so streaming continues seamlessly (the `Ema::extend_slice`
  pattern already does this — generalize it).

The clean way to generalize: a small internal trait
`BulkSeed { fn extend_slice(&mut self, &[f64], &mut Vec<f64>) }` with the
per-append fallback, so the PyO3 macro can call it uniformly.

### 2.3 Reduce per-tick Python call overhead

A PyO3 method call costs ~80–150 ns before your Rust code runs; the O(1)
`append` itself is single-digit ns. Two methods:

- **Bundle object**: a `#[pyclass] IndicatorGroup` that owns N indicator
  states and updates all of them from one `append(bar)` call — one boundary
  crossing per bar instead of N. This also enables shared-subexpression reuse
  (one TRANGE feeds ATR/NATR/ADX; one extrema pair feeds STOCH/WILLR).
- Return `f64::NAN` instead of `Option<f64>` from the hot `append` (an
  `Option` → `None`/`PyFloat` conversion allocates a Python object either
  way, but NaN lets callers skip `is None` checks in tight Python loops; keep
  the Option-returning API too if ergonomics matter).

Do not bother releasing the GIL for scalar `append` (the call is too short);
do release it (`py.allow_threads`) around bulk `extend`/`compute` of large
arrays so other Python threads can run. Measured (bench S5, 2026-08-07):
with 1→20 Python threads each appending to its own state, aggregate
throughput is flat ~1× — every taflow call currently holds the GIL, so
multi-symbol feeds serialize. (TA-Lib's binding is equally flat, so taflow
keeps its ~1000×+ per-update advantage at every thread count, but GIL
release in `extend` plus a free-threaded/subinterpreter story is the only
route to real parallel scaling.)

---

## 3. Window and deque primitives

### 3.1 Replace `VecDeque` in `Window` with a fixed ring buffer

`VecDeque` pays a capacity check + wrap mask per push/pop and its API forces
`Option` handling. Capacity is fixed at construction, so a
`Box<[f64]>` + `head: usize` + `len: usize` ring is enough:

```rust
// full-window push becomes:
let old = self.buf[self.head];
self.buf[self.head] = value;
self.head += 1;
if self.head == self.cap { self.head = 0; }
```

This is branch-predictable, never reallocates, and gives you
`as_slices() -> (&[f64], &[f64])` for free when a rescan is needed
(AVGDEV) so the rescan runs on contiguous slices instead of a `VecDeque`
iterator. Apply the same to `LaggedValue` (it only ever needs
one slot of history per lag distance — it is literally a delay line, a ring
of `period` f64 with one index; the current `VecDeque<f64>` + `Option`
machinery is heavier than needed).

### 3.2 `RollingExtrema` maintains two deques when most users need one

`RollingExtrema::append` updates both the max-deque and the min-deque on
every bar. But `Willr` uses one instance for highs (only the max is read)
and one for lows (only the min is read); `Aroon` and `Midprice` are the
same. Each of those indicators therefore maintains **4** monotonic deques
where **2** are needed — the other 2 are pure waste (pops, pushes, and
memory).

Method: split into `MonotonicMax` and `MonotonicMin` (or make
`RollingExtrema` generic over a comparator), and keep a combined version
only for `Minmax`/`Midpoint` where both sides are actually consumed.
Straightforward ~2× on the extrema portion of WILLR/AROON/STOCH/MIDPRICE.

### 3.3 `Maxindex`/`Minindex`: delete the O(period) rescan structure

`RollingIndexExtrema` rescans the whole window whenever the current extreme
expires — worst case O(period) per bar (monotonically decreasing input makes
every max-eviction a rescan). Meanwhile `RollingExtrema::append_indexed`
already returns `(index, value)` pairs from a monotonic deque in amortized
O(1) — and its tie-handling (pop when `value <= input`, so the newest of
equal values wins) matches TA-Lib's `>=`-latest semantics.

Method: implement `Maxindex`/`Minindex`/`Minmaxindex` on top of
`append_indexed` and delete `RollingIndexExtrema`. Less code, strictly
better worst case.

---

## 4. Bulk-path algorithms (the "fast as talib in C" part)

### 4.1 SMA/SUM/WMA/VAR/STDDEV/LINEARREG bulk: pure slice recurrences

In a bulk run over `inputs`, the evicted element is just `inputs[i - p]` —
no ring buffer needed. The steady-state loop body for SMA is two adds and a
multiply on contiguous memory; for the regression family it is the
`weighted_sum`/`sum_y` update already used in `RegressionCore::append` but
without deque traffic. These loops are what TA-Lib itself does; matching its
loop structure also matches its rounding exactly (§6.1), which is why this
form is preferable to prefix-sum vectorization for these functions.

### 4.2 Bulk rolling max/min: van Herk–Gil–Werman (vHGW)

For the bulk path of MAX/MIN/MINMAX/MIDPOINT/WILLR/STOCH/AROON, the
monotonic deque is O(n) but branchy and serial. vHGW computes a sliding
window max in ~3 comparisons per element *independent of window size*, using
only slice scans:

1. Cut the input into blocks of length `p` (the window size).
2. For each block compute a suffix-max array `S` (scan right-to-left) and a
   prefix-max array `P` (scan left-to-right).
3. The window max ending at position `i` is
   `max(S[i - p + 1], P[i])` — the window always spans a suffix of one block
   and a prefix of the next.

The two scans and the final `max` are all vectorizable slice passes; this
typically beats the deque by 3–10× for bulk work, and more for large
periods. Comparisons are exact (no FP reassociation), so **vHGW is safe even
under a bit-exact parity contract.** For AROON/MAXINDEX you need the *index*
of the max with latest-wins ties: run vHGW on values, then resolve the index
only per output (or carry `(value, index)` pairs through the scans with a
tie-breaking max).

Keep the deque for streaming `append`; vHGW is a bulk-only method.

### 4.3 EMA chains: fuse, don't stack

DEMA = EMA(EMA), TEMA, T3 (6 EMAs), MACD (3 EMAs) are currently composed
states. Composition is fine for O(1) `append`, but in bulk each layer would
walk the array separately if built naively from `extend_slice` layers.
Method: a fused bulk loop that advances all constituent EMAs in one pass over
the input keeps everything in registers — one load per bar total instead of
one load+store per layer per bar. This is exactly how TA-Lib's C code does
DEMA/TEMA/T3. Straightforward: the fused loop is just 2–6 `mul_add`s per
bar, and the ending scalar states drop directly into the streaming structs.

### 4.4 Vectorizing the EMA recurrence itself — possible, but read this first

`y[i] = y[i-1] + k(x[i] - y[i-1])` is a first-order linear recurrence; the
serial loop is latency-bound: one FMA per element with a ~4-cycle loop-carried
dependency, so ~4 cycles/element no matter how wide the CPU is. It *can* be
parallelized by block decomposition: over a block of `w` bars,

```
y[i+w] = a^w * y[i] + (weights · x[i+1..i+w+1])   where a = 1-k
```

so you can process 4 interleaved sub-sequences with precomputed `a^w` powers
and stitch them (a "scan" formulation). Realistic gain is 2–4× on long
arrays. **However** it changes rounding versus the serial recurrence and, for
small `a^w`, loses the low bits of history in a different pattern than
TA-Lib. Recommendation: don't do it while the parity contract is unproven;
the fused-chain scalar loop from §4.3 with real FMA (§1.1) already runs at
~1–2 ns/bar, which is at talib parity. Revisit only if profiling shows a
single-EMA bulk pass is actually a bottleneck. This is the classic
example of "hard to optimize because the definition is inherently serial."

### 4.5 Warm-up/steady-state loop splitting

Every `append` branches on warm-up state (`Option` matches, `is_full`
checks, `samples` counters). In bulk paths, split the loop: run the warm-up
prologue (first `p` bars) with the checks, then a steady-state loop with no
branches at all. `Ema::extend_slice` already has this shape; it should be
the template for every generated bulk path. In streaming `append` the branch
is perfectly predicted after warm-up, so it costs little there — don't
contort the streaming API for it.

---

## 5. Per-function notes

| Function(s) | Current cost | Method | Ref |
|---|---|---|---|
| SMA, SUM | O(1) append, no bulk path | slice-recurrence bulk loop | §4.1 |
| MAX, MIN, MINMAX, MIDPOINT, MIDPRICE, WILLR, STOCH(F), AROON(OSC) | amortized O(1) append; deque-based; double-deque waste | split Monotonic deques; vHGW bulk | §3.2, §4.2 |
| MAXINDEX, MININDEX, MINMAXINDEX | worst-case O(p) append | reuse `append_indexed` deque | §3.3 |
| MOM, ROC(P/R/R100) | O(1), VecDeque delay line | fixed ring delay line; bulk = `sub_offset_simd`-style slice op | §3.1 |
| EMA, DEMA, TEMA, T3, MACD(EXT/FIX), APO, PPO, TRIX | O(1) append | fused bulk chains + FMA codegen | §4.3, §1.1 |
| KAMA, MAMA, SAR(EXT), HT_* | O(1), inherently serial and branchy | leave as scalar; only ensure no allocation per bar | — |
| VAR, STDDEV, BBANDS | O(1) rolling moments | bulk slice loop; drift guard | §4.1, §6.2 |
| AVGDEV | **O(p) per append, two passes over a `VecDeque` iterator** | see below | §5.1 |
| CORREL, BETA | O(1) | store `(x,y)` in two flat rings for contiguous reseed; minor | — |
| ATR, NATR, ADX(R), DX, RSI, CMO | O(1) Wilder recurrences | bulk: fuse TRANGE+smoothing in one pass | §4.3 |
| TRANGE, BOP, price transforms, math ops | O(1)/pointwise | plain loops + target-cpu; drop manual lane packing | §1.2 |
| CDL* patterns (upcoming) | pointwise on small lookbacks | share one "candle anatomy" precompute (body, shadows, averages) across all 61 patterns in the bundle object instead of 61 recomputes | §2.3 |

### 5.1 AVGDEV — why O(1) is genuinely hard, and what to do

Mean absolute deviation is `Σ|x_i − μ|/p` where μ itself moves every bar.
The absolute value makes the sum non-decomposable: when μ changes, every
term's sign-split can change, so there is no exact add/remove accumulator.
Exact sub-linear options exist — keep the window in an order-statistic tree
split at μ, maintaining `count` and `sum` on each side, giving
O(log p) per bar — but the constant factor and code weight are large,
and TA-Lib's own AVGDEV is O(n·p), so parity pressure is low.

Recommendation: keep O(p) but make the scan cheap: with the ring buffer of
§3.1 the window is two contiguous slices, so the mean pass and the deviation
pass become one fused vectorizable pass (compute `Σx` incrementally like SMA
— that removes the first pass entirely; TA-Lib's newest-to-oldest summation
order only matters if you insist on bit-exactness, which is the one reason
the current code rescans for the mean). For typical `p ≤ 30`, a contiguous
fused scan is ~10–20 ns — good enough.

---

## 6. Numerical correctness interacting with speed

### 6.1 Reassociation vs the parity contract

TA-Lib sums windows in a specific order with plain doubles. SIMD reductions
(`sum_f64` with 4 lanes), multi-accumulator sums, prefix-sum tricks, and the
EMA block scan all change that order and thus the low bits.
Currently `Ema::extend_slice` seeds with `simd::sum_f64` while `Sma::append`
accumulates serially — the same indicator can already produce two slightly
different values for the same data depending on the path taken.
Decide once, globally:

- **Tolerance contract (recommended)**: assert `|a−b| ≤ 1e-8·max(1,|b|)`
  against TA-Lib. Then all reassociation-based methods are admissible.
- **Bit-exact contract**: then every seed/window sum must replicate TA-Lib's
  loop order; SIMD is restricted to comparisons (vHGW is still fine) and
  element-wise ops. Bulk and streaming paths must share the exact same
  seeding code (they mostly do today — keep enforcing chunk-invariance
  tests).

### 6.2 Rolling-accumulator drift on infinite streams

`sum += new − old` (SMA, SUM, WMA, moments, pair moments) accumulates
rounding error without bound on an endless stream — after millions of bars
of large prices (`1e6`-scale inputs like the test data), the SMA can drift
visibly from a fresh recomputation. Batch TA-Lib never sees this because
every call starts fresh; a *persistent* streaming library does.

Cheap, effective method: **periodic reseeding** — every `K` evictions
(e.g. `K = 64·p` bars, amortized cost ~1/64 of one window scan per bar),
recompute the accumulator from the ring buffer contents. Zero cost in the
steady loop except a counter. Alternative: Neumaier-compensated add/remove
(one extra FMA-ish op per update, keeps drift near 1 ulp continuously).
Reseeding is simpler and keeps the hot path untouched — recommended.
`RollingMoments::sum_squares` is the most exposed (squares amplify
magnitude) and should get this first; note it also suffers cancellation when
`variance ≈ 0` (the `max(0.0)` clamp in `Stddev` hints this is already
happening) — the eviction form `(new−old)(new+old)` used there is good, but
reseeding still bounds the long-run drift.

### 6.3 `compute()` full-history cache

Per the plan, `compute()` must not re-run past bars. Method: each state (or
the Python-side wrapper) appends the produced value to a growing `Vec<f64>`
output cache at `append` time (8 bytes/bar, geometric growth, amortized
O(1)); `compute()` is then a single memcpy into a fresh NumPy array.
Don't try to hand out a zero-copy view of the Rust cache — the lifetime
coupling with a growing Vec isn't worth it; one memcpy of contiguous f64 runs
at memory bandwidth (~10 GB/s ⇒ 1M bars ≈ 1 ms).

---

## 7. Measure before and after

- Two numbers per function, always separated: **bulk ns/bar** (1K/10K/100K/1M,
  vs TA-Lib via its Python bindings *and* vs raw C timing if possible) and
  **streaming ns/append** (Rust-level via criterion, and Python-level so the
  boundary cost of §2.3 is visible as its own line).
- `perf stat` for the vectorization sanity check: after §1.1, bulk kernels
  should show ~2 doubles/cycle on element-wise ops; if not, the loop didn't
  vectorize — check with `cargo asm` or `--emit=llvm-ir`.
- Add a long-stream drift test (10M synthetic bars, compare streaming SMA/VAR
  against fresh batch recomputation) to validate §6.2 before and after.
- Benchmark with realistic periods (5–200); deque vs vHGW crossover and
  ring-buffer wins are period-dependent.

## Suggested order of work

1. §1.1 target-feature/multiversioning + §1.2 kernel cleanup — hours of work,
   affects every bulk path.
2. §2.1 `extend_into` + §2.2 generalized `extend_slice` scaffold — this is
   the structural piece the remaining checklist items should be built on, so
   do it before implementing the ~70 remaining functions.
3. §3 primitives (ring buffer, split deques, delete `RollingIndexExtrema`).
4. §4.2 vHGW and §4.3 fused chains for the bulk paths.
5. §6.2 reseeding policy + drift tests.
6. §2.3 bundle object (also the natural home for shared candle anatomy and
   TR/extrema sharing).
