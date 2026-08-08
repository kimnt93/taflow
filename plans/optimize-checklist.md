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
| [ ] | AccelerationBands | ACCBANDS | 0.63× | M3 |
| [ ] | MathAcos | ACOS | 1.21× | _ |
| [ ] | AccumulationDistribution | AD | 2.77× | _ |
| [ ] | MathAdd | ADD | 7.52× | _ |
| [ ] | AccumulationDistributionOscillator | ADOSC | 0.82× | M2 |
| [ ] | AverageDirectionalIndex | ADX | 0.92× | M2 |
| [ ] | AverageDirectionalIndexRating | ADXR | 0.81× | M2 |
| [ ] | AbsolutePriceOscillator | APO | 0.77× | M2 |
| [ ] | Aroon | AROON | 0.33× | M1 M12 |
| [ ] | AroonOscillator | AROONOSC | 0.27× | M1 M12 |
| [ ] | MathAsin | ASIN | 1.34× | _ |
| [ ] | MathAtan | ATAN | 1.30× | _ |
| [ ] | AverageTrueRange | ATR | 1.57× | _ |
| [ ] | RollingAverageDeviation | AVGDEV | 0.84× | M3 (O(p) scan is inherent — fuse mean+dev passes on contiguous ring) |
| [ ] | AveragePrice | AVGPRICE | 4.56× | _ |
| [ ] | BollingerBands | BBANDS | 0.46× | M3 M4 |
| [ ] | RollingBeta | BETA | 1.00× | _ |
| [ ] | BalanceOfPower | BOP | 3.32× | _ |
| [ ] | CommodityChannelIndex | CCI | 1.21× | _ |
| [ ] | CandleTwoCrows | CDL2CROWS | 0.57× | M5 |
| [ ] | CandleThreeBlackCrows | CDL3BLACKCROWS | 0.76× | M5 |
| [ ] | CandleThreeInside | CDL3INSIDE | 0.36× | M5 |
| [ ] | CandleThreeLineStrike | CDL3LINESTRIKE | 0.50× | M5 |
| [ ] | CandleThreeOutside | CDL3OUTSIDE | 1.16× | _ |
| [ ] | CandleThreeStarsInSouth | CDL3STARSINSOUTH | 0.98× | M5 |
| [ ] | CandleThreeWhiteSoldiers | CDL3WHITESOLDIERS | 0.34× | M5 |
| [ ] | CandleAbandonedBaby | CDLABANDONEDBABY | 0.34× | M5 |
| [ ] | CandleAdvanceBlock | CDLADVANCEBLOCK | 0.59× | M5 |
| [ ] | CandleBeltHold | CDLBELTHOLD | 0.96× | M5 |
| [ ] | CandleBreakaway | CDLBREAKAWAY | 0.30× | M5 |
| [ ] | CandleClosingMarubozu | CDLCLOSINGMARUBOZU | 0.90× | M5 |
| [ ] | CandleConcealBabySwall | CDLCONCEALBABYSWALL | 0.32× | M5 |
| [ ] | CandleCounterAttack | CDLCOUNTERATTACK | 0.43× | M5 |
| [ ] | CandleDarkCloudCover | CDLDARKCLOUDCOVER | 0.55× | M5 |
| [ ] | CandleDoji | CDLDOJI | 1.49× | _ |
| [ ] | CandleDojiStar | CDLDOJISTAR | 0.41× | M5 |
| [ ] | CandleDragonflyDoji | CDLDRAGONFLYDOJI | 1.31× | _ |
| [ ] | CandleEngulfing | CDLENGULFING | 1.22× | _ |
| [ ] | CandleEveningDojiStar | CDLEVENINGDOJISTAR | 0.34× | M5 |
| [ ] | CandleEveningStar | CDLEVENINGSTAR | 0.27× | M5 |
| [ ] | CandleGapSideSideWhite | CDLGAPSIDESIDEWHITE | 1.43× | _ |
| [ ] | CandleGravestoneDoji | CDLGRAVESTONEDOJI | 1.17× | _ |
| [ ] | CandleHammer | CDLHAMMER | 1.41× | _ |
| [ ] | CandleHangingMan | CDLHANGINGMAN | 0.54× | M5 |
| [ ] | CandleHarami | CDLHARAMI | 0.44× | M5 |
| [ ] | CandleHaramiCross | CDLHARAMICROSS | 0.45× | M5 |
| [ ] | CandleHighWave | CDLHIGHWAVE | 1.11× | _ |
| [ ] | CandleHikkake | CDLHIKKAKE | 0.71× | M5 |
| [ ] | CandleHikkakeModified | CDLHIKKAKEMOD | 0.76× | M5 |
| [ ] | CandleHomingPigeon | CDLHOMINGPIGEON | 0.37× | M5 |
| [ ] | CandleIdenticalThreeCrows | CDLIDENTICAL3CROWS | 0.31× | M5 |
| [ ] | CandleInNeck | CDLINNECK | 0.48× | M5 |
| [ ] | CandleInvertedHammer | CDLINVERTEDHAMMER | 0.62× | M5 |
| [ ] | CandleKicking | CDLKICKING | 0.38× | M5 |
| [ ] | CandleKickingByLength | CDLKICKINGBYLENGTH | 0.41× | M5 |
| [ ] | CandleLadderBottom | CDLLADDERBOTTOM | 0.41× | M5 |
| [ ] | CandleLongLeggedDoji | CDLLONGLEGGEDDOJI | 1.14× | _ |
| [ ] | CandleLongLine | CDLLONGLINE | 1.23× | _ |
| [ ] | CandleMarubozu | CDLMARUBOZU | 1.04× | _ |
| [ ] | CandleMatchingLow | CDLMATCHINGLOW | 0.69× | M5 |
| [ ] | CandleMatHold | CDLMATHOLD | 0.23× | M5 |
| [ ] | CandleMorningDojiStar | CDLMORNINGDOJISTAR | 0.28× | M5 |
| [ ] | CandleMorningStar | CDLMORNINGSTAR | 0.25× | M5 |
| [ ] | CandleOnNeck | CDLONNECK | 0.47× | M5 |
| [ ] | CandlePiercing | CDLPIERCING | 0.49× | M5 |
| [ ] | CandleRickshawman | CDLRICKSHAWMAN | 0.74× | M5 |
| [ ] | CandleRiseFallThreeMethods | CDLRISEFALL3METHODS | 0.20× | M5 |
| [ ] | CandleSeparatingLines | CDLSEPARATINGLINES | 0.53× | M5 |
| [ ] | CandleShootingStar | CDLSHOOTINGSTAR | 0.60× | M5 |
| [ ] | CandleShortLine | CDLSHORTLINE | 1.04× | _ |
| [ ] | CandleSpinningTop | CDLSPINNINGTOP | 1.01× | _ |
| [ ] | CandleStalledPattern | CDLSTALLEDPATTERN | 0.29× | M5 |
| [ ] | CandleStickSandwich | CDLSTICKSANDWICH | 0.77× | M5 |
| [ ] | CandleTakuri | CDLTAKURI | 0.99× | M5 |
| [ ] | CandleTasukiGap | CDLTASUKIGAP | 0.95× | M5 |
| [ ] | CandleThrusting | CDLTHRUSTING | 0.48× | M5 |
| [ ] | CandleTriStar | CDLTRISTAR | 0.48× | M5 |
| [ ] | CandleUniqueThreeRiver | CDLUNIQUE3RIVER | 0.25× | M5 |
| [ ] | CandleUpsideGapTwoCrows | CDLUPSIDEGAP2CROWS | 0.42× | M5 |
| [ ] | CandleUpDownSideGapThreeMethods | CDLXSIDEGAP3METHODS | 0.76× | M5 |
| [ ] | MathCeil | CEIL | 1.53× | _ |
| [ ] | ChandeMomentumOscillator | CMO | 1.64× | _ |
| [ ] | RollingCorrelation | CORREL | 0.41× | M3 |
| [ ] | MathCos | COS | 1.16× | _ |
| [ ] | MathCosh | COSH | 1.31× | _ |
| [ ] | DoubleExponentialMovingAverage | DEMA | 1.19× | _ |
| [ ] | MathDivide | DIV | 5.70× | _ |
| [ ] | DirectionalMovementIndex | DX | 1.13× | _ |
| [ ] | ExponentialMovingAverage | EMA | 1.53× | _ |
| [ ] | MathExp | EXP | 1.38× | _ |
| [ ] | MathFloor | FLOOR | 1.64× | _ |
| [ ] | HilbertTransformDominantCyclePeriod | HT_DCPERIOD | 1.07× | _ |
| [ ] | HilbertTransformDominantCyclePhase | HT_DCPHASE | 1.00× (2.3M bars/s) | M13 |
| [ ] | HilbertTransformPhasor | HT_PHASOR | 1.05× | _ |
| [ ] | HilbertTransformSineWave | HT_SINE | 1.06× (2.1M bars/s) | M13 |
| [ ] | HilbertTransformTrendline | HT_TRENDLINE | 0.80× | M13 |
| [ ] | HilbertTransformTrendMode | HT_TRENDMODE | 0.97× (2.0M bars/s) | M13 |
| [ ] | IntradayMomentumIndex | IMI | 4.75× | _ |
| [ ] | KaufmanAdaptiveMovingAverage | KAMA | 0.58× | M7 M3 |
| [ ] | RollingLinearRegression | LINEARREG | 0.62× | M3 |
| [ ] | RollingLinearRegressionAngle | LINEARREG_ANGLE | 0.69× | M3 |
| [ ] | RollingLinearRegressionIntercept | LINEARREG_INTERCEPT | 0.65× | M3 |
| [ ] | RollingLinearRegressionSlope | LINEARREG_SLOPE | 0.54× | M3 |
| [ ] | MathLn | LN | 1.54× | _ |
| [ ] | MathLog10 | LOG10 | 1.17× | _ |
| [ ] | MovingAverage | MA | 1.01× | _ (inherits dispatched family's kernel) |
| [ ] | MovingAverageConvergenceDivergence | MACD | 0.89× | M2 |
| [ ] | MovingAverageConvergenceDivergenceExtended | MACDEXT | 0.55× | M2 |
| [ ] | MovingAverageConvergenceDivergenceFixed | MACDFIX | 0.96× | M2 |
| [ ] | MesaAdaptiveMovingAverage | MAMA | 0.98× | M7 |
| [ ] | VariablePeriodMovingAverage | MAVP | 0.37× | M11 |
| [ ] | RollingMax | MAX | 0.33× | M1 |
| [ ] | RollingArgmax | MAXINDEX | 0.69× | M1 M12 |
| [ ] | MedianPrice | MEDPRICE | 7.15× | _ |
| [ ] | MoneyFlowIndex | MFI | 0.75× | M3 M4 (signed-flow single ring) |
| [ ] | RollingMidpoint | MIDPOINT | 0.40× | M1 |
| [ ] | RollingMidprice | MIDPRICE | 0.24× | M1 |
| [ ] | RollingMin | MIN | 0.33× | M1 |
| [ ] | RollingArgmin | MININDEX | 0.61× | M1 M12 |
| [ ] | RollingMinMax | MINMAX | 0.48× | M1 |
| [ ] | RollingMinMaxIndex | MINMAXINDEX | 0.84× | M1 M12 |
| [ ] | MinusDirectionalIndicator | MINUS_DI | 1.04× | _ |
| [ ] | MinusDirectionalMovement | MINUS_DM | 1.58× | _ |
| [ ] | Momentum | MOM | 1.34× | _ |
| [ ] | MathMultiply | MULT | 4.01× | _ |
| [ ] | NormalizedAverageTrueRange | NATR | 1.38× | _ |
| [ ] | OnBalanceVolume | OBV | 2.09× | _ |
| [ ] | PlusDirectionalIndicator | PLUS_DI | 0.97× | M2 |
| [ ] | PlusDirectionalMovement | PLUS_DM | 1.56× | _ |
| [ ] | PercentagePriceOscillator | PPO | 0.77× | M2 |
| [ ] | RateOfChange | ROC | 1.33× | _ |
| [ ] | RateOfChangePercent | ROCP | 1.25× | _ |
| [ ] | RateOfChangeRatio | ROCR | 1.18× | _ |
| [ ] | RateOfChangeRatioPercent | ROCR100 | 1.34× | _ |
| [ ] | RelativeStrengthIndex | RSI | 1.41× | _ |
| [ ] | ParabolicSar | SAR | 0.83× | M7 |
| [ ] | ParabolicSarExtended | SAREXT | 0.83× | M7 |
| [ ] | MathSin | SIN | 1.21× | _ |
| [ ] | MathSinh | SINH | 1.17× | _ |
| [ ] | SimpleMovingAverage | SMA | 1.15× | _ |
| [ ] | MathSqrt | SQRT | 2.02× | _ |
| [ ] | RollingStandardDeviation | STDDEV | 0.80× | M3 |
| [ ] | StochasticOscillator | STOCH | 0.30× | M1 (batch O(n·p) loop + fused MA passes) |
| [ ] | FastStochasticOscillator | STOCHF | 0.29× | M1 |
| [ ] | StochasticRelativeStrengthIndex | STOCHRSI | 0.35× | M1 (reuse RSI output buffer) |
| [ ] | MathSubtract | SUB | 8.04× | _ |
| [ ] | RollingSum | SUM | 1.13× | _ |
| [ ] | TripleExponentialAverage | T3 | 0.30× | M2 (6 EMAs fused, the family's worked example) |
| [ ] | MathTan | TAN | 1.17× | _ |
| [ ] | MathTanh | TANH | 1.60× | _ |
| [ ] | TripleExponentialMovingAverage | TEMA | 1.06× | _ |
| [ ] | TrueRange | TRANGE | 2.78× | _ |
| [ ] | TriangularMovingAverage | TRIMA | 0.78× | M3 (fuse the SMA-of-SMA) |
| [ ] | TripleExponentialRateOfChange | TRIX | 0.98× | M2 |
| [ ] | RollingTimeSeriesForecast | TSF | 0.60× | M3 |
| [ ] | TypicalPrice | TYPPRICE | 6.01× | _ |
| [ ] | UltimateOscillator | ULTOSC | 0.55× | M3 M4 (3 periods share one bp ring + one tr ring) |
| [ ] | RollingVariance | VAR | 0.89× | M3 |
| [ ] | WeightedClose | WCLPRICE | 5.25× | _ |
| [ ] | WilliamsPercentR | WILLR | 0.26× | M1 |
| [ ] | WeightedMovingAverage | WMA | 1.00× | _ |

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
