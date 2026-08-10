# TAFlow optimization — implementation plan (2026-08-08)

Successor to [optimize-methods.md](optimize-methods.md). That document proposed methods;
this one is grounded in what is **actually in the tree today** and in the measured
benchmark evidence (`verify/evidence/benchmark/*.md`, generated 2026-08-08, 287 functions).
It is written so an implementing agent can execute it top-to-bottom without re-deriving
the analysis. Every claim cites a file:line in the current tree.

**Scope: ALL 287 canonical classes.** Named functions below (MFI, T3, STOCH, …) are
worked examples of their *family* — the fix described for one applies to every member of
that family. §B is the taxonomy that maps every function to a family and every family to
a method, so the sweep is exhaustive, not a whitelist of slow functions.

## Goal

For **every** canonical class, `m = SomeIndicator(inputs...); m.compute()` must run at
TA-Lib-C speed or better on the whole-vector path, while keeping streaming `append` O(1)
and keeping the existing correctness contract: `continue_vs_batch_bitwise: true` and
chunk invariance (see any report JSON → `correctness`). **Any bulk fast path must leave
the streaming state bit-identical to what per-bar `append` would have produced** —
`Ema::extend_slice` (`crates/taflow-core/src/stream/ema.rs:64-81`) is the reference
pattern.

---

## A. The universal class template (apply to all 287 classes)

Every indicator, fast or slow, converges on one architecture. This is the template the
whole plan serves; the phases below build its pieces.

```
Python wrapper (thin)                 PyO3 class                        taflow-core
─────────────────────                 ──────────────────────────        ─────────────────────────
SomeIndicator(series...)   ──────►    __new__ + extend(arrays)  ──────► extend_slice_into(&[f64]…, &mut Vec<f64>)
  as_float64_series × k               • borrow slices                    • warm-up prologue (branches)
  no other logic                      • py.allow_threads {…}             • steady-state loop (no branches,
                                      • push into outputs: Vec<f64>        no deque, slice-indexed, FMA)
m.append(bar)              ──────►    core.append() → Option<f64>       • end-state == per-append state
m.compute()                ──────►    outputs.clone() → numpy           append(): O(1), ring-buffer state
                                        (one memcpy, zero-copy wrap)
```

Rules, uniform across the board:

1. **All history caching lives in Rust** (`outputs: Vec<f64>` per output, appended at
   both append- and extend-time). Python wrappers hold zero state beyond the native
   object. `compute()` = one memcpy (measured 0.27 ms / 1M bars — acceptable; don't
   chase zero-copy views of a growing Vec).
2. **Bulk work goes through a real slice kernel**, never a per-bar `append` loop, and
   never materializes `Vec<Option<f64>>` (warm-up = NaN in-place).
3. **Streaming state uses fixed ring buffers**, never `VecDeque`, never per-bar
   allocation, never a rescan unless the math truly requires it (§B families define
   which do).
4. **The bulk kernel's exit state is bit-identical to per-append** (chunk-invariance
   contract), so streaming continues seamlessly after any bulk call.
5. **GIL released** around every bulk kernel.

`MoneyFlowIndex` (`crates/taflow-python/src/indicators/money_flow_index.rs`) is the
closest existing example of the *binding* shape (Rust cache + `compute()`), and
`Ema::extend_slice` of the *kernel* shape. Neither is complete — MFI's "bulk" path is a
per-append loop and EMA's binding predates the cache — the point is every class gets
**both** halves.

---

## B. Family taxonomy — one method per family, covering all 287 functions

Classify every function once, then apply the family recipe mechanically. Current
measured range is from the 2026-08-08 reports (kernel speedup vs TA-Lib where an oracle
exists, absolute bars/s otherwise).

| # | Family | Members (representative — classify the rest by structure) | Measured today | Method |
|---|---|---|---|---|
| F1 | Pointwise / price transforms | ADD SUB MULT DIV SQRT trig/exp/log family, AVGPRICE MEDPRICE TYPPRICE WCLPRICE, TRANGE BOP, gap_up/down, heikin_ashi, crossover/under, rising/falling, signed_power | 1–7.7× (already fast) | Plain loops + `chunks_exact`, drop manual `f64x4` lane packing, multiversion dispatch (Phase 2). No structural work. |
| F2 | Cumulative scans | OBV AD cumulative_* drawdown value_when bars_since highest/lowest_since | 1.8–7× | Serial by definition but trivial; bulk slice loop, nothing else. |
| F3 | Delay / offset | MOM ROC ROCP ROCR ROCR100 LAG signal_delay log_return | 1.1–1.2× | Ring delay line streaming (`LaggedValue` → fixed ring); bulk = one offset slice op (`out[i] = f(x[i], x[i-p])`), fully vectorizable. |
| F4 | Windowed sums / moments / regressions | SMA SUM WMA TRIMA CMO CCI MFI ADOSC ULTOSC VWMA DecayLinear, VAR STDDEV BBANDS AVGDEV zscore, CORREL BETA rolling_cov, LINEARREG(+SLOPE/ANGLE/INTERCEPT) TSF, kurtosis skew sharpe sortino calmar information_ratio autocorr alpha, rolling_vwap, spread_zscore, hedge_ratio | 0.4–1.3× | O(1) add/evict recurrences; **bulk loop indexes the input slice directly** (evicted element is `inputs[i-p]` — no ring in the bulk loop at all); warm-up/steady split; periodic reseed for drift (§Guardrails). Multi-sum indicators (ULTOSC's 3 windows, BBANDS' mean+moments) share ONE ring/window, never parallel deques. Worked examples: Phase 4.4 (MFI), 4.6 (ULTOSC), 4.7 (BBANDS), 4.8 (CORREL/regressions). |
| F5 | Rolling extrema | MAX MIN MINMAX MAXINDEX MININDEX MINMAXINDEX MIDPOINT MIDPRICE WILLR STOCH STOCHF STOCHRSI AROON AROONOSC Donchian, ichimoku, ulcer_index, premium_discount, retracements/fib (window hi-lo), previous_high_low, opening_range, equal_highs_lows, swing/structure detectors that need window extremes | **0.23–0.39×, worst family** | Bulk: van Herk–Gil–Werman (3 comparisons/element, period-independent, **bit-exact safe**). Streaming: split monotonic max/min deques (today every one-sided consumer pays for both sides). Phase 3. |
| F6 | EMA / Wilder recurrence chains | EMA DEMA TEMA T3 TRIX MACD(EXT/FIX) APO PPO ZLEMA, RSI ATR NATR ADX ADXR DX ±DI ±DM, TSI, mass_index, KST, chaikin_volatility, RMI, laguerre_rsi, ewm_* family, TrueStrengthIndex, McGinley, schaff_trend_cycle (MACD+stoch), awesome/accelerator oscillators (SMA diffs) | 0.29–1.6× | Serial recurrence — don't fight it; **fuse the chain into one pass** (all constituent EMA states advance in registers, one load/bar, 2–6 FMAs) instead of stacking N `extend` layers. FMA codegen from Phase 2 matters most here. Worked example: Phase 4.1 (T3). Do NOT do block-scan EMA vectorization (parity risk, marginal gain — optimize-methods.md §4.4). |
| F7 | Order statistics over a window | rolling_median rolling_quantile rolling_rank time_series_rank rolling_mode rolling_entropy rolling_winsorize IMI | 1.8M–71M bars/s (entropy 1.83M is a bottom-5 outlier) | **Sorted ring**: keep window values in a sorted `Box<[f64]>` alongside the FIFO ring; insert/evict by binary search + `copy_within` memmove — O(log p) search + O(p) move, but the move is a SIMD memcpy, unbeatable for p ≤ ~512 vs tree structures. Median/quantile/winsorize/rank all read straight off it. Mode & entropy: incremental count map — maintain `Σ n·ln n` by delta on the two touched bins → O(1)/bar (entropy today likely recomputes the histogram per bar). |
| F8 | Inherently serial adaptive | KAMA MAMA SAR SAREXT HT_* (6) VIDYA JMA supertrend pmax fisher_transform kalman_hedge_ratio negative/positive_volume_index vortex td_sequential | 0.5–1.1× (HT_DCPHASE/SINE/TRENDMODE 2.0–2.4M bars/s) | Accept serial; the wins are mechanical: fixed rings instead of VecDeques, zero per-bar allocation, `mul_add` everywhere, and for the HT family hoist the shared Hilbert plumbing (the 10× spread between HT_DCPERIOD 22M and HT_DCPHASE 2.3M with the same core transform proves avoidable overhead — profile, share the WMA/detrender pipeline). Worked example: Phase 4.5 (KAMA). |
| F9 | Candle patterns (61) | CDL* | 0.20–1.5× | Batch is already incremental — fix the *streaming* states (per-bar 10-element rescans ×4–8) and wire batch kernels into `extend`. Phase 4.2. Later: shared candle-anatomy precompute across all 61 (Phase 6 bundle). |
| F10 | Per-bar model fits & long-memory | Hurst (8.2M) FractalDimension (8.2M) OrnsteinUhlenbeckHalfLife (10.4M) FracDiff (**1.24M, slowest in repo**) roll_spread rolling_information/beta-style fits | 1.2M–18M bars/s | FracDiff = convolution with precomputed weights: contiguous ring + SIMD dot product per bar (O(w) but vectorized; weights computed once) — likely 5–10× from removing VecDeque iteration alone. Hurst/OU/fractal-dim: replace per-bar refits with incremental sums where the estimator allows (OU is an AR(1) regression → all five sums slide in O(1), F4 method); where it doesn't (Hurst R/S per scale), keep O(p) but make it contiguous-slice scans. |
| F11 | Structure / session / event detectors | order_block (3.95M) BOS/CHoCH (14M) liquidity (4.39M) fair_value_gap swing_highs_lows sessions session_volume_levels anchored_vwap inside/outside_bar higher_high/lower_low pivot_points | 4M–330M | Heterogeneous; audit each for the two classic sins: per-bar allocation (Vec/String building in append) and O(k) rescans of retained event lists. Rule: event lists get capacity-bounded rings + incremental invariants. The three < 5M bars/s (order_block, liquidity, fib_retracement 5.1M) get individual profiling first — likely allocation-bound. |
| F12 | Dispatch / meta | MA MAVP | MA 0.93×, MAVP 0.42× | MA inherits its target family's kernel via the dispatcher. MAVP: Phase 4.3 (bounded history ring, stop updating every materialized state per bar). |

Sweep procedure for the implementing agent:

1. Generate the work list from `verify/BENCHMARK.md`.
2. Tag each row with a family (most are obvious from the core file's structure; the
   table above seeds ~200 of them).
3. Apply the family recipe: streaming state → ring-buffer form; bulk kernel →
   family-shaped `extend_slice_into`; binding → template §A.
4. Gate each function (see Acceptance gates, §D) before checking it off.

Functions already ≥ 1× kernel speedup (F1/F2/F3 and parts of F4/F6) still get the §A
binding template (Phase 1) and the Phase 2 codegen lift — that work is uniform and
cheap — but skip bespoke kernel work for them.

---

## C. Phases (ordered by measured impact per unit of effort)

The benchmark separates `TAFlow API ms` (public Python class: construct → `extend` →
`compute`) from `TAFlow kernel ms` (raw PyO3 object `extend`, no wrapper bookkeeping, no
`compute`) — see `scripts/verification/benchmark.py`. Two
independent problems fall out:

1. **The Python wrapper layer** is 3–12× slower than its own Rust kernel for ~55 classes.
   At 10k bars: SMA API 0.477 ms vs kernel 0.045 ms (10.6×); BBANDS 2.246 vs 0.227;
   STOCH 2.131 vs 0.544; MIDPRICE 3.107 vs 0.457; AROON 1.263 vs 0.458.
   On 1M bars: SMA kernel 4.9 ms → API 58.2 ms; Aroon kernel 43.3 ms → API 144.9 ms.
2. **Whole kernel families are slow** (F5 extrema 0.23–0.39×, F6 chains like T3 0.29×,
   F9 candles 0.20–0.5×, F4 stragglers, F7/F10 absolute-throughput outliers).

### Phase 1 — Unify the Python layer on the Rust-cache template (§A) — biggest measured win

Current state: two binding families.

- **Family A** (`crates/taflow-python/src/indicators/*.rs`, 167 pyclasses): already has
  the Rust `outputs: Vec<f64>` cache + `compute()` (`money_flow_index.rs:10-15, 57-59`);
  thin Python wrapper. Correct architecture (MFI API≈kernel: 0.153 vs 0.145 ms @10k).
- **Family B** (`crates/taflow-python/src/state_api.rs`, 64 `Stateful*` pyclasses): no
  Rust cache; the pure-Python wrapper keeps history as a Python list of boxed floats —
  `python/taflow/_unary_state.py:50-77` does `self._values.extend(np.asarray(v).tolist())`
  per extend and `np.asarray(self._values)` per compute. Multi-output is worse:
  `python/taflow/bollinger_bands.py:89-91,105-107` stores N Python 3-tuples and
  transposes with `zip(*)` on every `compute()`; `python/taflow/aroon.py:65-67,78` keeps
  parallel Python lists. This round-trip IS the 10.6× SMA gap.

Tasks:

1.1. Add per-output `Vec<f64>` caches + `compute()` + cache-clearing `reset()` to every
`Stateful*` class. The macros make this a macro-body change, not 64 edits:
`scalar_state_class!` (`state_api.rs:41-81`), `oscillator_state_class!` (:1230),
`deviation_state_class!` (:1573), `bivariate_statistic_class!` (:1618),
`unary_state_class!` (:2128), `binary_state_class!` (:2200), `price3_state_class!` (:2256).

1.2. Gut the Python wrappers to pure delegation (like
`python/taflow/money_flow_index.py:112-121`): delete `_values` bookkeeping from
`_unary_state.py`, `_price_state.py`, `_ohlc_state.py`, `_volume_state.py`, the ~26
modules with the same code inlined (e.g. `relative_strength_index.py:72-84`), and all
multi-output wrappers (`bollinger_bands.py`, `aroon.py`, `macd*.py`, `stoch*.py`, …).

1.3. Multi-output `compute()` returns a tuple of arrays from per-output Rust caches —
never per-bar Python tuples, never `zip(*)`.

1.4. Keep `as_float64_series` (`python/taflow/_series.py:11-43`) — once per array per
extend is fine.

### Phase 2 — Build configuration: stop shipping SSE2-baseline wheels (free win, hours)

Confirmed: **no** `.cargo/config.toml`, no CI, no RUSTFLAGS anywhere; benchmark
environment records `"rustflags": ""` on an AVX2+FMA-capable i7-10750H. `wide::f64x4`
splits into 2× SSE2 ops; no `mul_add` compiles to FMA.

2.1. Local/dev benching: `RUSTFLAGS="-C target-cpu=native"` via a `justfile`/script
recipe (don't bake into wheel builds).

2.2. Wheels: `multiversion` crate on the hot bulk kernels (batch fns, new
`extend_slice_into` loops, simd.rs) — `x86-64-v3` / `x86-64-v2` / baseline, dispatch
once. Zero `unsafe`, preserves the `BENCHMARK.md:5` claim (update its wording anyway:
report both baseline and v3 numbers).

2.3. `simd.rs` cleanup: replace manual `f64x4::new([...])` packing + `to_array()`
scatter (every kernel, e.g. `sum_f64` at `simd.rs:15-40`) with `chunks_exact(4)` /
plain loops — with real target features LLVM auto-vectorizes better than manual packing.
Multi-accumulator `sum_f64` **only** where reassociation is admissible (§Guardrails).

### Phase 3 — F5 extrema family (worst kernels: 0.23–0.39×)

Current state:

- Batch max/min/argmax/argmin/aroon/midprice/midpoint use track-and-rescan
  (`rolling_max.rs:42-47`, `rolling_argmax.rs:40-46`, `aroon.rs:75-80,118-123`,
  `rolling_midprice.rs:72-91`, `rolling_midpoint.rs:65-84`) — O(n·p) worst case.
- Batch STOCH is an **unconditional O(n·fastk_period) double loop**
  (`stoch.rs:196-209`) + two extra MA passes + 3 intermediate Vecs.
- Streaming `RollingMax`/`RollingMin` each wrap a full `RollingExtrema`
  (`rolling_extrema.rs:118-119` via macro :76-116) maintaining **both** deques per
  append (:30-62). WILLR keeps two of them (`volume_states.rs:281-311`) = 4 deques,
  2 consumed; same for `RollingMidprice` (`rolling_price.rs:59-86`) and STOCH
  (`stoch.rs:28-91`).
- `RollingIndexExtrema` (`rolling_extrema.rs:198-265`) does an O(p) `reduce` rescan on
  every eviction (:236-249).
- `src/sliding_window.rs` has correct monotonic-deque sliding max/min with **zero
  callers** — use or delete.

Tasks:

3.1. Shared **vHGW** bulk kernels (values, and `(value, index)` with TA-Lib latest-wins
ties): block suffix-max + prefix-max scans, window max = `max(S[i-p+1], P[i])`.
Comparisons only → bit-exact safe. Wire into every F5 batch path.

3.2. Rewrite batch STOCH/STOCHF/STOCHRSI: vHGW extremes + fused %K→MA→MA without the
three intermediate allocations.

3.3. Split `RollingExtrema` → `MonotonicMax` / `MonotonicMin`; migrate one-sided
consumers (WILLR, `RollingMidprice`, STOCH, AROON, `RollingMax/Min`); keep the combined
struct for MINMAX/MIDPOINT (`rolling_price.rs:13-51` already right). ~2× on the extrema
share of those appends.

3.4. Delete `RollingIndexExtrema`; rebuild Argmax/Argmin/MinmaxIndex on `append_indexed`
(`rolling_extrema.rs:30-62`). Preserve warm-up quirks: `RollingMinmaxIndexValue{0,0}`
(:320-367) and batch `0.0`-not-NaN fill (`rolling_argmax.rs:22`).

3.5. Wire the fast batch kernels into pyclass `extend` (Phase 5.2 scaffold) — today most
pyclass `extend`s drive streaming state per bar, so batch-side wins are invisible.

3.6. Delete or use `sliding_window.rs` (`lib.rs:5` export, no callers).

### Phase 4 — Worked examples for the remaining families (each generalizes to its family)

4.1. **T3 → all of F6**: `t3.rs:40-45,84-105` stacks six `ExponentialMovingAverage`
structs, six Option-matched dependent appends/bar; batch (:21-31) is a per-append map.
Fused bulk loop: all six EMA states in registers, warm-up prologue split, exit state
bit-identical. Then DEMA/TEMA/TRIX/MACD family, and Wilder chains (fuse TRANGE +
smoothing for ATR/ADX; RSI's gain/loss split + smoothing in one pass).

4.2. **Candles (F9)**: batch already keeps incremental sliding sums
(`candle_mathold.rs:142-216`, `candle_risefall3methods.rs:165-271`; 56/61 files) — but
the streaming states rescan 10-element averages per bar (`candle_mathold.rs:52-105`: 4
scans/bar; `candle_risefall3methods.rs:52-129`: 5; `candle_3whitesoldiers.rs`: 8;
42/61 affected) and the pyclass drives *these* — hence CDLMATHOLD at 0.22×. Fix: running
sums in the streaming states (add on push, subtract evicted candle's contribution);
`reset()` clears in place instead of `Self::new()` realloc (`candle_mathold.rs:116`);
batch switches generic `cr`/`ca` runtime-match (`pattern.rs:235-258`) to the
monomorphized variants (:169-233).

4.3. **MAVP (F12)**: `mavp.rs:50-113` — unbounded `history` Vec (:83), updates every
materialized period's dispatcher per bar (:87-92), full-history replay per new period
(:95-104). Redesign: ring of last `maxperiod` values; SMA-type = compute requested
period's mean from the ring (O(p) ≤ maxperiod, TA-Lib's own cost); EMA-type = lazy
states, bounded update set. Cap memory; document semantics.

4.4. **MFI (F4 multi-window)**: kernel-bound ~13 ns/bar. `mfi.rs:81-112` pushes two
`Window` VecDeques/bar though flows are mutually exclusive (:88-94); `extend_slice`
(:119-139) is validation + per-append. Fix: one **signed** flow ring (sign test on evict
maintains both sums), real bulk path (typical-price/flow slice pass + O(1) slide loop).
Generalizes to every F4 member with parallel windows.

4.5. **KAMA (F8)**: already O(1) (`kama.rs:65-105`) but two VecDeques where a fixed ring
+ lag index suffice; bulk loop indexes the input slice directly (no deques in bulk).
Generalizes: every F8 member gets the ring/no-alloc/FMA sweep, nothing more.

4.6. **ULTOSC (F4)**: streaming = 3 `FlowWindow`s = **6 VecDeques/bar**
(`ultosc.rs:98-100`); bulk (:131-222) already O(n) sliding sums but 2 scratch Vecs.
One shared bp ring + one tr ring (capacity = largest period), three sum pairs over them;
bulk folds bp/tr into the slide loop.

4.7. **BBANDS (F4)**: `bbands.rs:94-105` pushes input into the MA's window AND the
moments' window. Share one window (RollingMoments already holds the sum → mean is free
for the SMA default; keep dispatcher for exotic matypes). Bulk: single pass → 3 bands
(batch at :23-46 currently drives streaming with 3 pushes/bar).

4.8. **CORREL/BETA/LINEARREG/TSF (F4 regressions)**: `rolling_corr.rs:21-71` already
O(1) sliding-moments; the 0.43–0.67× gap is codegen + Option plumbing — Phases 2 + 5.2
close most of it. Add periodic reseed (non-centered `n·sxy − sx·sy` drifts).

4.9. **Order statistics (F7)**: implement the shared **sorted-ring** primitive
(sorted `Box<[f64]>` + FIFO ring; binary-search insert/evict with `copy_within`), then
rebuild rolling_median/quantile/rank/winsorize/time_series_rank on it; count-map with
incremental `Σ n·ln n` for rolling_entropy (1.83M bars/s today) and rolling_mode.

4.10. **F10/F11 outliers** (FracDiff 1.24M, order_block 3.95M, liquidity 4.39M,
fib_retracement 5.13M, Hurst/OU/fractal-dim ~8–10M, HT_DCPHASE/SINE/TRENDMODE ~2M):
FracDiff → precomputed-weight dot product over a contiguous ring (SIMD); OU → incremental
AR(1) sums (F4 method); HT family → profile and hoist the shared Hilbert pipeline
(HT_DCPERIOD at 22M proves the transform itself isn't the cost); order_block/liquidity/
fib → profile for per-bar allocation and event-list rescans first, then bounded rings.

### Phase 5 — Core plumbing the kernels land on (do 5.1–5.2 before Phases 3–4 if sequencing tightly)

5.1. **Fixed ring buffer** replaces `VecDeque` in `Window` (`window.rs:15-45`):
`Box<[f64]>` + head + len, `as_slices() -> (&[f64], &[f64])`; same for `LaggedValue`
(`lagged_common.rs:10-39`). Then sweep the **85 files** in `src/stream/` using raw
`VecDeque` (worklist: `grep -l VecDeque src/stream/`) — mechanical, good fan-out task.

5.2. **Bulk scaffold**: add to `StreamingIndicator` (`stream/indicator.rs:8-44`)
`fn extend_slice_into(&mut self, inputs: &[f64], out: &mut Vec<f64>)` (multi-input
indicators get inherent-impl equivalents), default = per-append fallback. Requirements:
NaN warm-up (no `Option` across the bulk boundary — `extend_into` at
`indicator.rs:33-43` already does this), warm-up/steady split, end-state identical to
per-append. Wire every pyclass `extend` to it: today **143/156** Family A files loop
`append` per element (`adv.rs:29-42` pattern); MFI/CCI's `extend_slice` are fake
(`mfi.rs:119-139`, `cci.rs:98-116`); the `Vec<Option<f64>>` double materialization in
`money_flow_index.rs:36-55` dies here.

5.3. **GIL release**: zero `allow_threads` in the tree. Wrap core bulk calls in
`py.allow_threads` (materialize slice borrows first). Skip scalar `append`.

5.4. `append` keeps `Option<f64>` (API compat); the cache-push + Option double-write
(`money_flow_index.rs:30-34`) is fine. Optional later: `update() -> f64` NaN variant.

5.5. **Dead code**: `crates/taflow-python/src/func_api.rs` (1987 lines) and
`metadata.rs` (384) are not declared in `lib.rs:5-7` — not compiled; delete or wire in
deliberately. `import taflow` eagerly imports 260 modules (112 ms) — consider PEP 562
lazy `__getattr__`, low priority.

### Phase 6 — Later

- **Bundle object** (`IndicatorGroup`): one boundary crossing/bar updates N indicators;
  shared TRANGE/extrema/candle-anatomy across members (natural home for the 61-pattern
  anatomy share). optimize-methods.md §2.3.
- **Drift guards**: periodic reseed for all `sum += new − old` accumulators; add the
  10M-bar drift test first. §6.2 of the old plan.
- **PGO**; free-threaded Python / subinterpreters after GIL release lands.

---

## D. Acceptance gates (apply per function, every phase)

1. **Correctness**: `continue_vs_batch_bitwise: true`, chunk invariance (1/10/1000),
   TA-Lib `max_abs_error` within existing tolerance — all 287 green after every phase.
2. **API ≈ kernel**: `TAFlow API ms` within ~10% of `TAFlow kernel ms` at ≥10k bars
   (update `scripts/verification/registry.py` if `extend` stops returning arrays for Family B).
3. **Kernel vs TA-Lib**: every TA-Lib-mapped function ≥ 1× at 10k bars for F1–F6/F9/F12;
   ≥ 0.8× where TA-Lib's C is doing identical serial work (F8, ULTOSC-style multi-window).
   Custom functions (no oracle): no function below 20M bars/s without a written
   justification in its report.
4. **No hidden costs**: zero per-bar allocation in any `append` (verify with a
   heap-profiling spot check on the F11 set); `reset()` never reallocates.
5. **Benchmark discipline**: current reports only cover 1k/10k (`protocol.sizes`).
   Run 100k and 1M too (`scripts/verification/benchmark.py` supports it) — wrapper overhead vanishes
   and kernel quality dominates at 1M; both regimes matter. Keep recording `rustflags`.

## Guardrails (read before writing code)

1. **Parity contract is bit-exact between streaming and batch** (chunk invariance).
   Every bulk path must produce bit-identical outputs *and* end-state vs per-append.
   vHGW (comparisons) is safe. Reassociating sums (multi-accumulator `sum_f64`, SIMD
   reductions feeding outputs) is **not**, unless the seed path is shared by both
   streaming and bulk (as `Ema::extend_slice` shares `simd::sum_f64`; note optimize-
   methods.md §6.1 — SMA and EMA already seed differently; don't widen that hole).
   TA-Lib-side comparison is tolerance-based, so external parity is less brittle.
2. **FMA changes rounding.** Phase 2 may flip low bits vs current wheels. Internal chunk
   invariance survives (same instructions both paths); verify all 287 vs TA-Lib before
   shipping; if a function breaks tolerance, pin its contraction explicitly.
3. **Warm-up semantics to preserve**: MAXINDEX/MININDEX emit `0.0` not NaN
   (`rolling_argmax.rs:22`); `RollingMinmaxIndexValue{0,0}`; candle int outputs.

## Suggested execution order

| # | Work | Effort | Expected effect |
|---|---|---|---|
| 1 | Phase 1 (Rust caches everywhere, gut Python wrappers) | 1–2 days | SMA API 0.11× → ~1×; kills the 3–12× wrapper tax on ~55 classes |
| 2 | Phase 2 (multiversion + native bench flags + simd.rs cleanup) | hours–1 day | Broad kernel lift; FMA on every recurrence (F4/F6/F8) |
| 3 | Phase 5.1–5.2 (ring Window + bulk scaffold) | 1–2 days | Structural base; every later kernel lands on it |
| 4 | Phase 3 (vHGW + monotonic split + STOCH rewrite) | 1–2 days | F5: 0.23–0.39× → ≥1× bulk |
| 5 | Phase 4 (family worked examples, then sweep each family via §B) | 3–5 days | F4/F6/F7/F9/F10/F11 to gates |
| 6 | Phase 5.3–5.5, Phase 6 | as scheduled | GIL scaling, drift safety, cleanup |

After each phase: `maturin develop --release`, run
`uv run python scripts/verification/benchmark.py`, diff evidence under
`verify/evidence/benchmark/`, and confirm every JSON correctness block is
green before moving on.
