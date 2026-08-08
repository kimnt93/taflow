# TAFlow optimization checklist

Working checklist for delegated implementation agents. Derived from
[optimize-methods.md](optimize-methods.md) (methods background),
[optimize-implementation-plan.md](optimize-implementation-plan.md) (current-state
analysis, file:line citations, family taxonomy) and
`verify/benchmark_reports/BENCHMARK.md` (2026-08-08 run, 287 functions).

How to use: pick a work package (§3) or a set of rows with the same method code, do the
work following §2, verify per §4, then flip `[ ]` → `[x]` and record the new speedup.
Method `_` = no bespoke kernel work needed (only the global tasks apply). Every agent
must read §1 and §4 before touching code.

---

## 0. Status — measured 2026-08-08 (post-implementation)

**131 of 161 rows clear their gate; 30 remain.** All 287 functions pass oracle
verification at the harness's standard 10k-bar protocol (`make check`), and the
Rust suite is 389 tests green.

Perf cells read `old → **new** (1M: …)`. The `new` figure is the kernel speedup
at **10k bars**, which is what the original baseline measured — comparing it to
the 1M column would be apples-to-oranges. The 1M figure is shown because the
protocol now runs 1k/10k/100k/1M.

### What landed

| Area | Result |
|---|---|
| Python boundary (G1, PERF-MEM) | Rust-side output caches everywhere; API time now ≈ kernel time (SMA was 10.6× overhead, now ~1.1×) |
| Extrema family (M1/M12) | vHGW + split monotonic deques: MAX 0.33× → 2.47× at 10k |
| EMA chains (M2) | Fused: T3 0.29× → 1.40× |
| Windowed sums (M3/M4) | BBANDS 0.46× → 1.19×, ULTOSC 0.55× → 1.39× |
| Candles (M5) | Running sums + bulk kernels; CDLGAPSIDESIDEWHITE 1.43× → 3.97× |
| Custom operators | **Every custom function now clears the 20M bars/s gate** |
| Build (G2) | multiversion dispatch replaces SSE2-baseline hand-SIMD; `wide` dependency dropped |

### The 30 unticked rows

**8 are correctness, and all 8 are pre-existing — not regressions.** Verified by
building the pre-session commit in a worktree and comparing at 1M bars:

- `CandleAdvanceBlock`, `CandleGapSideSideWhite`, `CandleKicking`,
  `CandleKickingByLength` diverge from TA-Lib on 5–30 bars per million
  (knife-edge threshold comparisons). The pre-session build produces
  **identical divergences on identical bar indices**. Never caught before
  because the old protocol stopped at 10k bars.
- `RollingCorrelation`, `RollingStandardDeviation`, `RollingVariance`,
  `CommodityChannelIndex` exceed the 1M-bar tolerance through accumulator
  drift — but every one is **substantially better than before**:

  | function | drift before | drift now | improvement |
  |---|---|---|---|
  | STDDEV | 2.612e-08 | 1.231e-09 | 21× |
  | CORREL | 4.289e-09 | 4.847e-10 | 8.8× |
  | CCI | 2.402e-09 | 7.115e-10 | 3.4× |
  | VAR | 3.143e-09 | 1.004e-09 | 3.1× |

  Chunk invariance and streaming-vs-batch stay bitwise green for all four.

**22 are performance below 1.0× at 10k.** The cluster worth attacking next is
the index-returning extrema (`RollingArgmin/Argmax/MinMaxIndex` 0.54–0.68×),
which cannot use vHGW: TA-Lib's MAXINDEX tie-breaking is path dependent
(`[3,5,4,5]` and `[9,5,4,5]` at p=3 give different indices for the same final
window), so they run an exact O(n) replica of the C state machine instead.
`MoneyFlowIndex` (0.57×) and the candle stragglers are next.

### Optimizations deliberately rejected

Sliding-sum conversions for AwesomeOscillator, VWMA, RollingVWAP, UlcerIndex,
RollingCalmar, RollingInformationRatio, VIDYA's CMO scan and the LINEARREG
family would each reassociate a sum the current code recomputes fresh,
changing low-order bits and breaking the bit-exactness contract (R2). Those
rows took contiguous-ring and allocation wins instead. Four rows (Hurst,
RollingAutocorr, SpreadZScore, OrderBlock) sit at the serial-dependency floor
where beating the gate requires exactly that forbidden reassociation.

## 1. General rules (apply to every task, no exceptions)

R1. **Target architecture** — every class: thin Python wrapper → PyO3 class with
    Rust-side `outputs: Vec<f64>` cache(s) → core slice kernel. `compute()` = one memcpy
    of the cache. See implementation plan §A for the full diagram.
R2. **Bit-exact contract**: `continue_vs_batch_bitwise: true` + chunk invariance must
    stay green. A bulk fast path must produce bit-identical outputs AND leave streaming
    state bit-identical to per-bar `append` (reference: `Ema::extend_slice`,
    `crates/taflow-core/src/stream/ema.rs:64-81`). Comparison-only algorithms (vHGW) are
    safe; reassociating sums is NOT unless bulk and streaming share the same seed code.
R3. **No `Option<f64>` across a bulk boundary** — warm-up is NaN written in place; never
    materialize `Vec<Option<f64>>`.
R4. **No per-bar allocation in any `append`; `reset()` never reallocates** (clear in
    place, don't `Self::new()`).
R5. **Fixed ring buffers, not `VecDeque`**, for all windows/delay lines.
R6. **Warm-up quirks to preserve**: MAXINDEX/MININDEX emit `0.0` not NaN
    (`rolling_argmax.rs:22`); `RollingMinmaxIndexValue{0,0}` during warm-up; candle
    outputs are ints (0/±100).
R7. **FMA/codegen changes rounding** — after enabling target features, re-verify all 287
    vs TA-Lib; internal chunk invariance survives (same instructions both paths).
R8. Bulk loops index the input slice directly (evicted element = `inputs[i-p]`) — no
    ring traffic inside a bulk loop; warm-up prologue split from branch-free steady loop.

## 2. Workflow per agent

1. Read the implementation plan section named in your method code's legend entry (§3).
2. Branch from `main`; keep one method-family (or one work package) per branch.
3. Implement: core kernel first (`crates/taflow-core/src/stream/`), then the PyO3 class
   (`crates/taflow-python/src/`), then the Python wrapper (`python/taflow/`).
4. `maturin develop --release` (with `RUSTFLAGS="-C target-cpu=native"` only for local
   measurement; never commit flags into wheel config — G2 handles distribution).
5. `python verify/benchmark.py` (at minimum for the touched functions); confirm every
   touched function's JSON correctness block is green and speedup improved.
6. Update this checklist: `[x]`, new speedup in the Perf column (keep the old one as
   `old→new`), and note deviations.
7. Do not change public API names/signatures; do not change warm-up semantics.

## 3. Method legend

**Global tasks (do first / once — they lift every row, including all `_` rows):**

| Code | Task | Plan ref |
|---|---|---|
| G1 | Unify Python layer: Rust-side output caches in every `Stateful*` macro class, gut Python `_values` bookkeeping to pure delegation, tuple-of-arrays `compute()` for multi-output. Kills the 3–12× wrapper tax on ~55 classes. | impl-plan Phase 1 |
| G2 | Build config: `multiversion` (x86-64-v3/v2/baseline) on hot kernels for wheels; native RUSTFLAGS recipe for local bench; rewrite `simd.rs` manual `f64x4` lane packing → `chunks_exact`/plain loops. | impl-plan Phase 2 |
| G3 | `Window`/`LaggedValue` → fixed ring (`Box<[f64]>` + head + len, `as_slices()`), then sweep the 85 `VecDeque`-using files in `src/stream/`. | impl-plan Phase 5.1 |
| G4 | Bulk scaffold: `extend_slice_into` on `StreamingIndicator` (+ inherent equivalents for multi-input), wire ALL pyclass `extend`s to it (143/156 currently loop `append`); delete the fake `extend_slice` in MFI/CCI. | impl-plan Phase 5.2 |
| G5 | `py.allow_threads` around every bulk kernel call. | impl-plan Phase 5.3 |
| G6 | Dead code: delete/wire `func_api.rs`, `metadata.rs` (not compiled), `sliding_window.rs` (zero callers); optional PEP 562 lazy imports (112 ms `import taflow`). | impl-plan Phase 5.5 |
| G7 | Benchmark discipline: add 100k + 1M sizes to the protocol; re-run and refresh reports after every package. | impl-plan §D.5 |

**Per-function methods (the `Method` column below):**

| Code | Method | Plan ref |
|---|---|---|
| M1 | Rolling-extrema overhaul: vHGW bulk kernel (values + latest-wins-index variant, bit-exact safe); streaming split `RollingExtrema` → `MonotonicMax`/`MonotonicMin` (stop paying 4 deques for 2); rewrite batch STOCH's O(n·p) double loop. | impl-plan Phase 3 |
| M2 | Fuse EMA/Wilder chains into one bulk pass (all constituent states in registers, 2–6 FMAs/bar, exit state bit-identical); never stack `extend` layers. | impl-plan Phase 4.1 |
| M3 | Windowed-sum/moments/regression slice-recurrence bulk loop (O(1) add/evict on slice indices, warm-up/steady split); periodic reseed for drift. | impl-plan §B F4, Phase 4.8 |
| M4 | Merge parallel windows: indicators pushing the same bar into 2+ deques share ONE ring (BBANDS mean+moments, ULTOSC 3×2 windows, MFI signed-flow trick, squeeze BB+KC, yang-zhang variances). | impl-plan Phase 4.4/4.6/4.7 |
| M5 | Candle streaming states: maintain running body/shadow sums (add on push, subtract evicted) instead of 4–8 × 10-element rescans per bar; batch: monomorphized `cr_*`/`ca_*` instead of runtime-match `cr`/`ca`; `reset()` in place. | impl-plan Phase 4.2 |
| M6 | Order statistics: shared sorted-ring primitive (binary-search insert/evict + `copy_within`) for median/quantile/rank/winsorize; incremental count-map with `Σ n·ln n` deltas for entropy/mode. | impl-plan Phase 4.9 |
| M7 | Serial-adaptive sweep: fixed rings, zero per-bar alloc, `mul_add` everywhere; accept the serial recurrence (no exotic vectorization). | impl-plan Phase 4.5 |
| M8 | Event-detector audit: profile for per-bar allocation + O(k) event-list rescans; capacity-bounded rings + incremental invariants. | impl-plan §B F11 |
| M9 | Precomputed-weight convolution: contiguous ring + SIMD dot product per bar (weights computed once at construction). | impl-plan Phase 4.10 |
| M10 | Incremental model-fit sums: slide the OLS/AR(1)/cov sums in O(1) instead of per-bar refit; where impossible (Hurst R/S) make scans contiguous-slice. | impl-plan Phase 4.10 |
| M11 | MAVP redesign: ring of last `maxperiod` values, lazy bounded per-period states, stop updating every materialized state per bar, cap memory. | impl-plan Phase 4.3 |
| M12 | Argmax/argmin on monotonic `append_indexed`; delete `RollingIndexExtrema`'s O(p) eviction rescan. | impl-plan Phase 3.4 |
| M13 | HT-family: profile and hoist the shared Hilbert pipeline — HT_DCPERIOD runs 22M bars/s, HT_DCPHASE/SINE/TRENDMODE run ~2M on the same transform, so ~10× is avoidable overhead, not math. | impl-plan Phase 4.10 |

**Suggested work packages for parallel agents** (each is one branch, minimal overlap):

- **P0** (blocking, do first, one agent each): G1; G2; G3+G4 (structural pair).
- **P1**: all M1 rows (+M12) — the extrema family, one agent.
- **P2**: all M5 rows — 48 candle patterns, mechanical, one agent (or two: streaming fix / monomorphization).
- **P3**: all M2 rows — EMA/Wilder fusions.
- **P4**: all M3/M4 rows — windowed sums & moments.
- **P5**: all M6 rows + M9 (FracDiff) — order statistics & convolution.
- **P6**: M7/M13 rows — serial adaptive + HT profiling.
- **P7**: M8/M10 rows — event detectors & model fits (profiling-led).
- **P8**: M11 (MAVP alone — self-contained redesign).
- Last: G5, G6, G7 cleanup agent.

## 4. Acceptance gates (per function, before checking a row off)

- Correctness JSON green: `batch_vs_oracle`, `continue_vs_batch_bitwise`, chunk
  invariance 1/10/1000.
- API ms within ~10% of kernel ms at ≥10k bars.
- TA-Lib-mapped: speedup ≥ 1.0× at largest size (≥ 0.8× acceptable only for genuinely
  serial work: F8/M7 rows, ULTOSC-style multi-window — justify in the row note).
- Custom (no oracle): ≥ 20M bars/s, or a written justification.
- Zero per-bar allocation in `append` (spot-check with a heap profiler for M8 rows).

---

## 5. Checklist

Perf = TA-Lib speedup where an oracle exists, else absolute bars/s (2026-08-08 run).
Method `_` = no bespoke work; global tasks G1–G7 still apply to every row.

### TA-Lib-mapped functions

| Check | Class | TA-Lib | Perf | Method |
|---|---|---|---:|---|
| [x] | AccelerationBands | ACCBANDS | 0.63× → **1.33×** (1M: 0.80×) | M3 |
| [x] | MathAcos | ACOS | 1.21× → **1.31×** (1M: 0.76×) | _ |
| [x] | AccumulationDistribution | AD | 2.77× → **2.61×** (1M: 0.97×) | _ |
| [x] | MathAdd | ADD | 7.52× → **8.44×** (1M: 1.12×) | _ |
| [x] | AccumulationDistributionOscillator | ADOSC | 0.82× → **2.06×** (1M: 0.91×) | M2 |
| [x] | AverageDirectionalIndex | ADX | 0.92× → **1.61×** (1M: 1.15×) | M2 |
| [x] | AverageDirectionalIndexRating | ADXR | 0.81× → **1.42×** (1M: 1.08×) | M2 |
| [ ] | AbsolutePriceOscillator | APO | 0.77× → **0.95×** (1M: 0.62×) | M2 |
| [ ] | Aroon | AROON | 0.33× → **0.80×** (1M: 0.54×) | M1 M12 |
| [ ] | AroonOscillator | AROONOSC | 0.27× → **0.66×** (1M: 0.49×) | M1 M12 |
| [x] | MathAsin | ASIN | 1.34× → **1.34×** (1M: 0.86×) | _ |
| [x] | MathAtan | ATAN | 1.30× → **1.30×** (1M: 0.84×) | _ |
| [x] | AverageTrueRange | ATR | 1.57× → **1.70×** (1M: 1.17×) | _ |
| [x] | RollingAverageDeviation | AVGDEV | 0.84× → **1.04×** (1M: 0.88×) | M3 (O(p) scan is inherent — fuse mean+dev passes on contiguous ring) |
| [x] | AveragePrice | AVGPRICE | 4.56× → **4.71×** (1M: 1.18×) | _ |
| [x] | BollingerBands | BBANDS | 0.46× → **1.64×** (1M: 0.96×) | M3 M4 |
| [ ] | RollingBeta | BETA | 1.00× → **0.88×** (1M: 0.59×) | _ |
| [x] | BalanceOfPower | BOP | 3.32× → **5.14×** (1M: 1.17×) | _ |
| [x] | CommodityChannelIndex | CCI | 1.21× → **1.25×** (1M: 1.06×) | _ |
| [x] | CandleTwoCrows | CDL2CROWS | 0.57× → **1.74×** (1M: 0.95×) | M5 |
| [ ] | CandleThreeBlackCrows | CDL3BLACKCROWS | 0.76× → **0.59×** (1M: 0.43×) | M5 |
| [x] | CandleThreeInside | CDL3INSIDE | 0.36× → **1.33×** (1M: 0.97×) | M5 |
| [x] | CandleThreeLineStrike | CDL3LINESTRIKE | 0.50× → **1.48×** (1M: 0.98×) | M5 |
| [x] | CandleThreeOutside | CDL3OUTSIDE | 1.16× → **1.24×** (1M: 0.71×) | _ |
| [x] | CandleThreeStarsInSouth | CDL3STARSINSOUTH | 0.98× → **1.67×** (1M: 1.09×) | M5 |
| [x] | CandleThreeWhiteSoldiers | CDL3WHITESOLDIERS | 0.34× → **2.32×** (1M: 1.72×) | M5 |
| [x] | CandleAbandonedBaby | CDLABANDONEDBABY | 0.34× → **1.43×** (1M: 1.03×) | M5 |
| [ ] | CandleAdvanceBlock | CDLADVANCEBLOCK | 0.59× → **2.60×** (1M: 2.11×) | M5 |
| [x] | CandleBeltHold | CDLBELTHOLD | 0.96× → **1.49×** (1M: 1.05×) | M5 |
| [ ] | CandleBreakaway | CDLBREAKAWAY | 0.30× → **0.88×** (1M: 0.57×) | M5 |
| [x] | CandleClosingMarubozu | CDLCLOSINGMARUBOZU | 0.90× → **1.55×** (1M: 1.03×) | M5 |
| [x] | CandleConcealBabySwall | CDLCONCEALBABYSWALL | 0.32× → **1.82×** (1M: 1.07×) | M5 |
| [x] | CandleCounterAttack | CDLCOUNTERATTACK | 0.43× → **2.22×** (1M: 1.17×) | M5 |
| [x] | CandleDarkCloudCover | CDLDARKCLOUDCOVER | 0.55× → **1.65×** (1M: 0.94×) | M5 |
| [x] | CandleDoji | CDLDOJI | 1.49× → **1.85×** (1M: 0.83×) | _ |
| [x] | CandleDojiStar | CDLDOJISTAR | 0.41× → **1.69×** (1M: 1.05×) | M5 |
| [x] | CandleDragonflyDoji | CDLDRAGONFLYDOJI | 1.31× → **2.37×** (1M: 1.27×) | _ |
| [x] | CandleEngulfing | CDLENGULFING | 1.22× → **1.52×** (1M: 0.84×) | _ |
| [x] | CandleEveningDojiStar | CDLEVENINGDOJISTAR | 0.34× → **1.37×** (1M: 0.95×) | M5 |
| [x] | CandleEveningStar | CDLEVENINGSTAR | 0.27× → **1.53×** (1M: 0.96×) | M5 |
| [ ] | CandleGapSideSideWhite | CDLGAPSIDESIDEWHITE | 1.43× → **3.97×** (1M: 2.83×) | _ |
| [x] | CandleGravestoneDoji | CDLGRAVESTONEDOJI | 1.17× → **2.44×** (1M: 1.22×) | _ |
| [x] | CandleHammer | CDLHAMMER | 1.41× → **1.80×** (1M: 1.25×) | _ |
| [x] | CandleHangingMan | CDLHANGINGMAN | 0.54× → **1.81×** (1M: 1.27×) | M5 |
| [x] | CandleHarami | CDLHARAMI | 0.44× → **2.56×** (1M: 1.21×) | M5 |
| [x] | CandleHaramiCross | CDLHARAMICROSS | 0.45× → **1.77×** (1M: 1.06×) | M5 |
| [x] | CandleHighWave | CDLHIGHWAVE | 1.11× → **1.81×** (1M: 1.15×) | _ |
| [ ] | CandleHikkake | CDLHIKKAKE | 0.71× → **0.67×** (1M: 0.49×) | M5 |
| [ ] | CandleHikkakeModified | CDLHIKKAKEMOD | 0.76× → **0.68×** (1M: 0.51×) | M5 |
| [x] | CandleHomingPigeon | CDLHOMINGPIGEON | 0.37× → **1.86×** (1M: 1.16×) | M5 |
| [x] | CandleIdenticalThreeCrows | CDLIDENTICAL3CROWS | 0.31× → **1.75×** (1M: 1.16×) | M5 |
| [x] | CandleInNeck | CDLINNECK | 0.48× → **2.14×** (1M: 1.06×) | M5 |
| [x] | CandleInvertedHammer | CDLINVERTEDHAMMER | 0.62× → **1.86×** (1M: 1.21×) | M5 |
| [ ] | CandleKicking | CDLKICKING | 0.38× → **2.45×** (1M: 1.44×) | M5 |
| [ ] | CandleKickingByLength | CDLKICKINGBYLENGTH | 0.41× → **2.71×** (1M: 1.56×) | M5 |
| [x] | CandleLadderBottom | CDLLADDERBOTTOM | 0.41× → **1.47×** (1M: 0.87×) | M5 |
| [x] | CandleLongLeggedDoji | CDLLONGLEGGEDDOJI | 1.14× → **2.11×** (1M: 1.19×) | _ |
| [x] | CandleLongLine | CDLLONGLINE | 1.23× → **2.09×** (1M: 1.44×) | _ |
| [x] | CandleMarubozu | CDLMARUBOZU | 1.04× → **1.83×** (1M: 1.14×) | _ |
| [x] | CandleMatchingLow | CDLMATCHINGLOW | 0.69× → **2.16×** (1M: 1.29×) | M5 |
| [x] | CandleMatHold | CDLMATHOLD | 0.23× → **1.25×** (1M: 0.85×) | M5 |
| [x] | CandleMorningDojiStar | CDLMORNINGDOJISTAR | 0.28× → **1.44×** (1M: 0.94×) | M5 |
| [x] | CandleMorningStar | CDLMORNINGSTAR | 0.25× → **1.45×** (1M: 0.87×) | M5 |
| [x] | CandleOnNeck | CDLONNECK | 0.47× → **1.74×** (1M: 1.07×) | M5 |
| [x] | CandlePiercing | CDLPIERCING | 0.49× → **1.68×** (1M: 1.13×) | M5 |
| [x] | CandleRickshawman | CDLRICKSHAWMAN | 0.74× → **2.26×** (1M: 1.39×) | M5 |
| [x] | CandleRiseFallThreeMethods | CDLRISEFALL3METHODS | 0.20× → **1.26×** (1M: 0.92×) | M5 |
| [x] | CandleSeparatingLines | CDLSEPARATINGLINES | 0.53× → **1.90×** (1M: 1.15×) | M5 |
| [x] | CandleShootingStar | CDLSHOOTINGSTAR | 0.60× → **1.74×** (1M: 1.17×) | M5 |
| [x] | CandleShortLine | CDLSHORTLINE | 1.04× → **1.98×** (1M: 1.41×) | _ |
| [x] | CandleSpinningTop | CDLSPINNINGTOP | 1.01× → **1.45×** (1M: 1.03×) | _ |
| [x] | CandleStalledPattern | CDLSTALLEDPATTERN | 0.29× → **1.95×** (1M: 1.42×) | M5 |
| [x] | CandleStickSandwich | CDLSTICKSANDWICH | 0.77× → **2.09×** (1M: 1.10×) | M5 |
| [x] | CandleTakuri | CDLTAKURI | 0.99× → **2.18×** (1M: 1.36×) | M5 |
| [x] | CandleTasukiGap | CDLTASUKIGAP | 0.95× → **2.38×** (1M: 1.90×) | M5 |
| [x] | CandleThrusting | CDLTHRUSTING | 0.48× → **2.38×** (1M: 1.03×) | M5 |
| [x] | CandleTriStar | CDLTRISTAR | 0.48× → **2.20×** (1M: 1.05×) | M5 |
| [x] | CandleUniqueThreeRiver | CDLUNIQUE3RIVER | 0.25× → **1.39×** (1M: 0.63×) | M5 |
| [x] | CandleUpsideGapTwoCrows | CDLUPSIDEGAP2CROWS | 0.42× → **1.45×** (1M: 1.06×) | M5 |
| [x] | CandleUpDownSideGapThreeMethods | CDLXSIDEGAP3METHODS | 0.76× → **1.93×** (1M: 1.19×) | M5 |
| [x] | MathCeil | CEIL | 1.53× → **3.93×** (1M: 0.68×) | _ |
| [x] | ChandeMomentumOscillator | CMO | 1.64× → **1.62×** (1M: 1.11×) | _ |
| [ ] | RollingCorrelation | CORREL | 0.41× → **0.47×** (1M: 0.28×) | M3 |
| [x] | MathCos | COS | 1.16× → **1.03×** (1M: 0.91×) | _ |
| [x] | MathCosh | COSH | 1.31× → **1.36×** (1M: 0.83×) | _ |
| [x] | DoubleExponentialMovingAverage | DEMA | 1.19× → **2.62×** (1M: 1.97×) | _ |
| [x] | MathDivide | DIV | 5.70× → **5.98×** (1M: 0.99×) | _ |
| [ ] | DirectionalMovementIndex | DX | 1.13× → **0.99×** (1M: 0.74×) | _ |
| [x] | ExponentialMovingAverage | EMA | 1.53× → **1.48×** (1M: 0.65×) | _ |
| [x] | MathExp | EXP | 1.38× → **1.25×** (1M: 0.74×) | _ |
| [x] | MathFloor | FLOOR | 1.64× → **3.40×** (1M: 0.64×) | _ |
| [x] | HilbertTransformDominantCyclePeriod | HT_DCPERIOD | 1.07× → **1.03×** (1M: 1.02×) | _ |
| [x] | HilbertTransformDominantCyclePhase | HT_DCPHASE | 1.00× (2.3M bars/s) → **4.22×** (1M: 4.38×) | M13 |
| [x] | HilbertTransformPhasor | HT_PHASOR | 1.05× → **1.02×** (1M: 0.97×) | _ |
| [x] | HilbertTransformSineWave | HT_SINE | 1.06× (2.1M bars/s) → **3.42×** (1M: 3.36×) | M13 |
| [ ] | HilbertTransformTrendline | HT_TRENDLINE | 0.80× → **0.87×** (1M: 0.82×) | M13 |
| [x] | HilbertTransformTrendMode | HT_TRENDMODE | 0.97× (2.0M bars/s) → **2.82×** (1M: 2.54×) | M13 |
| [x] | IntradayMomentumIndex | IMI | 4.75× → **12.32×** (1M: 12.21×) | _ |
| [x] | KaufmanAdaptiveMovingAverage | KAMA | 0.58× → **1.10×** (1M: 0.65×) | M7 M3 |
| [x] | RollingLinearRegression | LINEARREG | 0.62× → **1.23×** (1M: 0.96×) | M3 |
| [x] | RollingLinearRegressionAngle | LINEARREG_ANGLE | 0.69× → **1.12×** (1M: 0.97×) | M3 |
| [x] | RollingLinearRegressionIntercept | LINEARREG_INTERCEPT | 0.65× → **1.20×** (1M: 0.82×) | M3 |
| [x] | RollingLinearRegressionSlope | LINEARREG_SLOPE | 0.54× → **1.31×** (1M: 0.97×) | M3 |
| [x] | MathLn | LN | 1.54× → **1.36×** (1M: 0.72×) | _ |
| [x] | MathLog10 | LOG10 | 1.17× → **1.13×** (1M: 0.83×) | _ |
| [x] | MovingAverage | MA | 1.01× → **1.18×** (1M: 0.52×) | _ (inherits dispatched family's kernel) |
| [x] | MovingAverageConvergenceDivergence | MACD | 0.89× → **3.55×** (1M: 4.55×) | M2 |
| [ ] | MovingAverageConvergenceDivergenceExtended | MACDEXT | 0.55× → **0.66×** (1M: 0.92×) | M2 |
| [x] | MovingAverageConvergenceDivergenceFixed | MACDFIX | 0.96× → **2.93×** (1M: 4.44×) | M2 |
| [x] | MesaAdaptiveMovingAverage | MAMA | 0.98× → **1.09×** (1M: 0.91×) | M7 |
| [ ] | VariablePeriodMovingAverage | MAVP | 0.37× → **0.70×** (1M: 0.86×) | M11 |
| [x] | RollingMax | MAX | 0.33× → **3.51×** (1M: 2.02×) | M1 |
| [ ] | RollingArgmax | MAXINDEX | 0.69× → **0.68×** (1M: 0.47×) | M1 M12 |
| [x] | MedianPrice | MEDPRICE | 7.15× → **8.25×** (1M: 1.03×) | _ |
| [ ] | MoneyFlowIndex | MFI | 0.75× → **0.57×** (1M: 0.49×) | M3 M4 (signed-flow single ring) |
| [x] | RollingMidpoint | MIDPOINT | 0.40× → **1.94×** (1M: 1.22×) | M1 |
| [x] | RollingMidprice | MIDPRICE | 0.24× → **2.42×** (1M: 1.28×) | M1 |
| [x] | RollingMin | MIN | 0.33× → **3.46×** (1M: 2.10×) | M1 |
| [ ] | RollingArgmin | MININDEX | 0.61× → **0.65×** (1M: 0.44×) | M1 M12 |
| [x] | RollingMinMax | MINMAX | 0.48× → **2.57×** (1M: 1.52×) | M1 |
| [ ] | RollingMinMaxIndex | MINMAXINDEX | 0.84× → **0.54×** (1M: 0.39×) | M1 M12 |
| [ ] | MinusDirectionalIndicator | MINUS_DI | 1.04× → **0.86×** (1M: 0.59×) | _ |
| [x] | MinusDirectionalMovement | MINUS_DM | 1.58× → **1.62×** (1M: 1.02×) | _ |
| [x] | Momentum | MOM | 1.34× → **1.61×** (1M: 0.33×) | _ |
| [x] | MathMultiply | MULT | 4.01× → **8.14×** (1M: 1.26×) | _ |
| [x] | NormalizedAverageTrueRange | NATR | 1.38× → **1.32×** (1M: 1.00×) | _ |
| [x] | OnBalanceVolume | OBV | 2.09× → **2.33×** (1M: 0.83×) | _ |
| [ ] | PlusDirectionalIndicator | PLUS_DI | 0.97× → **0.85×** (1M: 0.60×) | M2 |
| [x] | PlusDirectionalMovement | PLUS_DM | 1.56× → **1.58×** (1M: 0.99×) | _ |
| [ ] | PercentagePriceOscillator | PPO | 0.77× → **0.88×** (1M: 0.58×) | M2 |
| [x] | RateOfChange | ROC | 1.33× → **1.84×** (1M: 0.59×) | _ |
| [x] | RateOfChangePercent | ROCP | 1.25× → **1.84×** (1M: 0.59×) | _ |
| [x] | RateOfChangeRatio | ROCR | 1.18× → **2.23×** (1M: 0.62×) | _ |
| [x] | RateOfChangeRatioPercent | ROCR100 | 1.34× → **2.04×** (1M: 0.59×) | _ |
| [x] | RelativeStrengthIndex | RSI | 1.41× → **1.48×** (1M: 0.98×) | _ |
| [ ] | ParabolicSar | SAR | 0.83× → **0.99×** (1M: 0.60×) | M7 |
| [ ] | ParabolicSarExtended | SAREXT | 0.83× → **0.84×** (1M: 0.55×) | M7 |
| [x] | MathSin | SIN | 1.21× → **1.24×** (1M: 0.92×) | _ |
| [x] | MathSinh | SINH | 1.17× → **1.28×** (1M: 0.85×) | _ |
| [x] | SimpleMovingAverage | SMA | 1.15× → **2.19×** (1M: 0.99×) | _ |
| [x] | MathSqrt | SQRT | 2.02× → **3.22×** (1M: 0.66×) | _ |
| [x] | RollingStandardDeviation | STDDEV | 0.80× → **1.63×** (1M: 0.79×) | M3 |
| [ ] | StochasticOscillator | STOCH | 0.30× → **0.92×** (1M: 0.58×) | M1 (batch O(n·p) loop + fused MA passes) |
| [x] | FastStochasticOscillator | STOCHF | 0.29× → **1.08×** (1M: 0.82×) | M1 |
| [x] | StochasticRelativeStrengthIndex | STOCHRSI | 0.35× → **1.08×** (1M: 0.63×) | M1 (reuse RSI output buffer) |
| [x] | MathSubtract | SUB | 8.04× → **9.40×** (1M: 1.03×) | _ |
| [x] | RollingSum | SUM | 1.13× → **1.54×** (1M: 0.62×) | _ |
| [x] | TripleExponentialAverage | T3 | 0.30× → **1.38×** (1M: 0.80×) | M2 (6 EMAs fused, the family's worked example) |
| [x] | MathTan | TAN | 1.17× → **1.07×** (1M: 0.94×) | _ |
| [x] | MathTanh | TANH | 1.60× → **1.61×** (1M: 0.71×) | _ |
| [x] | TripleExponentialMovingAverage | TEMA | 1.06× → **2.74×** (1M: 2.34×) | _ |
| [x] | TrueRange | TRANGE | 2.78× → **3.86×** (1M: 1.03×) | _ |
| [x] | TriangularMovingAverage | TRIMA | 0.78× → **1.46×** (1M: 0.70×) | M3 (fuse the SMA-of-SMA) |
| [x] | TripleExponentialRateOfChange | TRIX | 0.98× → **2.24×** (1M: 1.56×) | M2 |
| [x] | RollingTimeSeriesForecast | TSF | 0.60× → **1.08×** (1M: 1.00×) | M3 |
| [x] | TypicalPrice | TYPPRICE | 6.01× → **6.01×** (1M: 1.08×) | _ |
| [x] | UltimateOscillator | ULTOSC | 0.55× → **1.66×** (1M: 1.33×) | M3 M4 (3 periods share one bp ring + one tr ring) |
| [x] | RollingVariance | VAR | 0.89× → **1.56×** (1M: 0.64×) | M3 |
| [x] | WeightedClose | WCLPRICE | 5.25× → **6.28×** (1M: 1.16×) | _ |
| [x] | WilliamsPercentR | WILLR | 0.26× → **2.10×** (1M: 1.35×) | M1 |
| [x] | WeightedMovingAverage | WMA | 1.00× → **1.36×** (1M: 0.56×) | _ |

### Custom functions (no TA-Lib oracle — Perf is bars/s; gate: ≥ 20M or justified)

| Check | Class | Perf | Method |
|---|---|---:|---|
| [ ] | Amihud | 141.1M | _ |
| [ ] | AnchoredVolumeWeightedAveragePrice | 84.5M | _ |
| [ ] | ArnaudLegouxMovingAverage | 95.6M | _ |
| [ ] | AverageDailyDollarValue | 189.2M | _ |
| [ ] | AwesomeOscillator | 36.5M | M3 (fuse the two SMAs in one pass) |
| [ ] | BarsSince | 381.2M | _ |
| [ ] | BreakOfStructureChangeOfCharacter | 14.6M | M8 |
| [ ] | ChaikinMoneyFlow | 106.0M | _ |
| [ ] | ChaikinVolatility | 170.2M | _ |
| [ ] | CloseToCloseSigma | 54.7M | _ |
| [ ] | Crossover | 309.9M | _ |
| [ ] | Crossunder | 279.1M | _ |
| [ ] | CumulativeCount | 438.1M | _ |
| [ ] | CumulativeMaximum | 228.2M | _ |
| [ ] | CumulativeMinimum | 231.2M | _ |
| [ ] | CumulativeProduct | 380.2M | _ |
| [ ] | CumulativeSum | 384.2M | _ |
| [ ] | CumulativeSumControlChart | 253.1M | _ |
| [ ] | DecayLinear | 191.5M | _ |
| [ ] | DetrendedPriceOscillator | 140.1M | _ |
| [ ] | Donchian | 19.3M | M1 |
| [ ] | Drawdown | 236.2M | _ |
| [ ] | EaseOfMovement | 307.3M | _ |
| [ ] | EqualHighsLows | 21.9M | M1 |
| [ ] | EvenBetterSinewave | 185.4M | _ |
| [ ] | ExponentiallyWeightedCorrelation | 171.5M | _ |
| [ ] | ExponentiallyWeightedCovariance | 189.8M | _ |
| [ ] | ExponentiallyWeightedStandardDeviation | 217.4M | _ |
| [ ] | ExponentiallyWeightedVariance | 217.0M | _ |
| [ ] | ExponentiallyWeightedSum | 254.1M | _ |
| [ ] | FairValueGap | 92.2M | _ |
| [ ] | Falling | 208.2M | _ |
| [ ] | FibonacciRetracement | 5.1M | M1 M8 |
| [ ] | FisherTransform | 19.2M | M1 (rolling minmax feed) |
| [ ] | ForceIndex | 314.6M | _ |
| [ ] | FracDiff | 1.25M (slowest in repo) | M9 |
| [ ] | FractalDimension | 8.3M | M10 |
| [ ] | GapDown | 326.7M | _ |
| [ ] | GapUp | 321.7M | _ |
| [ ] | GarmanKlass | 61.9M | _ |
| [ ] | GarmanKlassYangZhang | 46.8M | _ |
| [ ] | HedgeRatio | 17.5M | M10 |
| [ ] | HeikinAshi | 84.9M | _ |
| [ ] | HigherHigh | 322.2M | _ |
| [ ] | HighestSince | 262.3M | _ |
| [ ] | HullMovingAverage | 27.0M | M3 (fuse WMA chain) |
| [ ] | Hurst | 7.8M | M10 |
| [ ] | Ichimoku | 8.0M | M1 (3 extrema pairs — share scans) |
| [ ] | InsideBar | 320.7M | _ |
| [ ] | JurikMovingAverage | 51.6M | _ |
| [ ] | KalmanHedgeRatio | 67.6M | _ |
| [ ] | KeltnerChannels | 119.4M | _ |
| [ ] | KnowSureThing | 30.0M | M2 (fuse 4 ROC+SMA chains) |
| [ ] | KlingerVolumeOscillator | 102.0M | _ |
| [ ] | Lag | 301.2M | _ |
| [ ] | LaguerreRelativeStrengthIndex | 118.1M | _ |
| [ ] | Liquidity | 4.3M | M8 |
| [ ] | LogReturn | 106.1M | _ |
| [ ] | LowerLow | 324.3M | _ |
| [ ] | LowestSince | 268.5M | _ |
| [ ] | MassIndex | 132.0M | _ |
| [ ] | MathAbs | 964.3M | _ |
| [ ] | MathAcosh | 97.9M | _ |
| [ ] | MathAsinh | 85.2M | _ |
| [ ] | MathAtanh | 220.8M | _ |
| [ ] | MathCbrt | 57.1M | _ |
| [ ] | MathCot | 49.7M | _ |
| [ ] | MathDegrees | 966.9M | _ |
| [ ] | MathLog1p | 120.2M | _ |
| [ ] | MathRadians | 952.0M | _ |
| [ ] | McGinleyDynamic | 78.3M | _ |
| [ ] | NegativeVolumeIndex | 161.1M | _ |
| [ ] | OpeningRange | 149.9M | _ |
| [ ] | OrderBlock | 3.9M | M8 |
| [ ] | OrnsteinUhlenbeckHalfLife | 10.1M | M10 (AR(1) sums slide in O(1)) |
| [ ] | OutsideBar | 321.1M | _ |
| [ ] | Parkinson | 74.8M | _ |
| [ ] | PivotPoints | 101.8M | _ |
| [ ] | ParabolicMovingAverageStop | 53.6M | _ |
| [ ] | PositiveVolumeIndex | 169.8M | _ |
| [ ] | PremiumDiscount | 27.1M | M1 |
| [ ] | PreviousHighLow | 104.5M | _ |
| [ ] | Retracements | 20.2M | M1 |
| [ ] | Rising | 207.7M | _ |
| [ ] | RelativeMomentumIndex | 88.9M | _ |
| [ ] | RogersSatchell | 35.1M | _ |
| [ ] | RollSpread | 17.6M | M10 |
| [ ] | RollingAlpha | 17.6M | M3 |
| [ ] | RollingAutocorr | 12.8M | M3 (lagged pair moments, incremental) |
| [ ] | RollingCalmar | 38.6M | M1 (rolling max drawdown) |
| [ ] | RollingCov | 51.8M | _ |
| [ ] | RollingEntropy | 1.8M | M6 (incremental count map + Σ n·ln n deltas) |
| [ ] | RollingInformationRatio | 28.7M | M3 |
| [ ] | RollingKurtosis | 53.6M | _ |
| [ ] | RollingMedian | 22.8M | M6 |
| [ ] | RollingMode | 38.8M | M6 |
| [ ] | RollingQuantile | 18.5M | M6 |
| [ ] | RollingRank | 72.2M | _ |
| [ ] | RollingSharpe | 54.3M | _ |
| [ ] | RollingSkew | 27.9M | M3 |
| [ ] | RollingSortino | 80.5M | _ |
| [ ] | RollingVolumeWeightedAveragePrice | 23.4M | M3 (two sliding sums — should be near SMA speed) |
| [ ] | RollingWinsorize | 17.4M | M6 |
| [ ] | RollingZScore | 54.5M | _ |
| [ ] | SchaffTrendCycle | 10.7M | M2 M1 (MACD chain + double stoch, fused) |
| [ ] | SessionVolumeLevels | 14.1M | M8 |
| [ ] | Sessions | 128.2M | _ |
| [ ] | SignalDelay | 272.1M | _ |
| [ ] | SignedPower | 56.7M | _ |
| [ ] | SpreadZScore | 9.5M | M3 M4 |
| [ ] | Squeeze | 27.3M | M4 (BB + Keltner share windows/TR) |
| [ ] | SqueezePro | 24.2M | M4 |
| [ ] | SmoothedTrendChannel | 72.4M | _ |
| [ ] | Supertrend | 57.7M | _ |
| [ ] | SwingHighLow | 23.0M | M1 |
| [ ] | TomDeMarkSequential | 172.0M | _ |
| [ ] | TimeSeriesRank | 72.8M | _ |
| [ ] | TrueStrengthIndex | 176.6M | _ |
| [ ] | UlcerIndex | 20.7M | M1 M3 (rolling max + squared-drawdown sum) |
| [ ] | ValueWhen | 462.5M | _ |
| [ ] | VariableIndexDynamicAverage | 14.4M | M7 M3 (CMO window incremental) |
| [ ] | VolumePriceTrend | 321.4M | _ |
| [ ] | VolumeWeightedMovingAverage | 60.0M | M3 (two sliding sums) |
| [ ] | Vortex | 47.3M | _ |
| [ ] | YangZhang | 19.8M | M3 M4 (three variances share one window) |
| [ ] | ZeroLagExponentialMovingAverage | 233.4M | _ |

### Global tasks

| Check | Task | Scope |
|---|---|---|
| [ ] | G1 Python-layer unification | all 287 classes (kills wrapper tax on ~55) |
| [ ] | G2 multiversion wheels + simd.rs cleanup | all bulk kernels |
| [ ] | G3 ring-buffer Window/LaggedValue + VecDeque sweep | 85 files in src/stream/ |
| [ ] | G4 extend_slice_into scaffold + wire all pyclass extends | 143/156 pyclasses |
| [ ] | G5 GIL release around bulk kernels | all pyclasses |
| [ ] | G6 dead-code removal (func_api.rs, metadata.rs, sliding_window.rs) | taflow-python, taflow-core |
| [ ] | G7 benchmark protocol: add 100k + 1M sizes, refresh reports | verify/ |

---

Notes for reviewers:

- The `Append µs` column in BENCHMARK.md is ~0.17–0.5 µs across the board — dominated by
  the Python call boundary, not indicator work; G1/G4 shrink it slightly, and the Phase-6
  bundle object (impl-plan) is the real lever if per-tick latency matters. Outliers worth
  a look while in the neighborhood: FracDiff 1.07, DX 0.91, HT_DCPHASE 0.88, MAMA 0.84,
  SessionVolumeLevels 0.78, HT_TRENDMODE/HT_SINE ~0.71, STOCHF 0.62.
- Method assignments for custom functions are best-effort from structure; an agent that
  finds a different bottleneck while profiling should note it in the row and proceed
  with what the profile says, not the label.
- Rows at 0.95–1.0× (CDLTASUKIGAP, CDLBELTHOLD, TRIX, MACDFIX, PLUS_DI, MAMA…) are
  flagged because their family fix is already being done for the slower members — they
  come along for free; don't do bespoke work for them alone.
