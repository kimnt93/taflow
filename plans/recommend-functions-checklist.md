# Recommended-functions checklist

Implementation checklist derived from
[`recommend-functions.md`](recommend-functions.md), ordered by priority
(high → low), **restricted to functions that satisfy the taflow contract**:

> Input: one or more aligned time series (+ scalar params). Output: one or
> more time series of the **same length** as the input, NaN/None during
> warm-up, fully **causal** (bar `i` output uses only bars `≤ i`), and
> **chunk-invariant** (stream and batch produce identical values).

Functions from the research that violate this contract are listed at the
bottom as explicit non-goals — do not implement them under this API.

Every item must additionally pass all review gates in
[`/CHECK.md`](../CHECK.md) (module placement + naming map, no
builtin-shadowing params, typed enums, docs, multi-line style, one function
one file) before its box is checked; the master status table lives there.

Reference legend: **Impl** = concrete open-source implementation to match
(or align to) numerically; **Theory** = defining paper/book. Read the Impl
source before coding — secondhand descriptions of these algorithms are
often wrong (one LuxAlgo description was refuted 0-3 in verification).
Every row with an Impl reference gets an oracle parity test against that
package (version pinned); rows without one use state-vs-batch parity +
chunk invariance (benchmark plan S4).

## P0 — shared infrastructure (build once, unblocks everything)

- [x] **Causal swing/pivot state** — a bar is a confirmed swing high when
  `swing_length` bars have passed without a higher high. Outputs (same
  length as input, all causal): `swing_signal` (+1/-1/NaN **at the
  confirmation bar**), `swing_level` (price of the confirmed swing),
  `bars_since_swing`.
  Impl to align to: `smartmoneyconcepts/smc.py::swing_highs_lows` — but
  note it is **non-causal** (centered window via `swing_length *= 2` +
  negative shift, marker written at the swing bar itself). taflow
  implements the causal variant; the oracle test aligns by shifting the
  package's markers forward `swing_length` bars.
  Note: two monotonic deques (reuse `RollingExtrema`, split per §3.2 of
  `optimize-methods.md`) + candidate slot — O(1) amortized. Every SMC row
  consumes this state.
- [x] **Session/anchor input handling** — session-scoped functions need to
  know boundaries. Contract-compatible design: accept an extra aligned
  input series (`timestamp: i64` or precomputed `session_id`/`new_session`
  flags), never an internal calendar. Python layer provides helpers to
  build the flag series from datetime indexes. Keeps the Rust core
  timestamp-free and the API pure series-in/series-out.
- [x] **Bounded active-zone list** — internal state (not an output):
  fixed-capacity `Vec<Zone>` (top, bottom, birth index, flags) with
  O(active) linear scan per bar and eviction on fill/expiry/overflow
  (configurable cap, default 64). Used by FVG/OB/liquidity mitigation
  flags. Linear scan beats any tree at this size.

## P1 — Smart Money Concepts (`smc`)

Default semantics: joshyattridge `smartmoneyconcepts`
(github.com/joshyattridge/smart-money-concepts, `smartmoneyconcepts/smc.py`,
MIT, ~1.9k stars). The package returns same-length DataFrames, so the
family fits the contract — **but taflow's outputs are the causal
re-encoding**, documented per row. Two systematic differences from the
package, stated once here and in every docstring:

1. Swing-derived markers appear at the **confirmation bar**, not
   retroactively at the swing bar (the package uses lookahead).
2. Index-valued columns pointing to *future* bars (`MitigatedIndex`,
   `BrokenIndex`) are replaced by same-size **flag series emitted at the
   event bar** (`mitigated: ±1/NaN at the fill bar`, with the zone id/price
   as companion series).

| Done | Function | Outputs (same-size series) | Reference | Implementation & speed note |
|---|---|---|---|---|
| [x] | `fvg` | `fvg` (+1/-1/NaN), `top`, `bottom`, `mitigated` flag | Impl: `smc.py::fvg` — bullish: `high[i-2] < low[i]` AND middle candle `close > open`, mirrored bearish | Fully causal with 1-bar lag already (3-bar pattern detected at bar `i`, attributed to bar `i-1` in the package — taflow emits at `i`; oracle test shifts by 1). O(1): 3-bar shift registers; mitigation via zone list. Bulk path = one pass over 3 shifted slices. |
| [x] | `swing_highs_lows` | `swing_signal`, `swing_level`, `bars_since_swing` | see P0 | Thin wrapper over the P0 swing state. |
| [x] | `bos_choch` | `bos` (+1/-1/NaN at confirmation), `choch` (same), `level`, `broken` flag at break bar | Impl: `smc.py::bos_choch` lines ~222-370 — 4-swing inequality patterns (bullish BOS: swings `[-1,1,-1,1]` with `low[-4]<low[-2]<high[-3]<high[-1]`), break = close (or high/low) crossing the level. LuxAlgo variant (Pine mirror, gist niquedegraaff): same trigger, classified by trend state | Ring of last 4 confirmed swings; per bar O(1) pattern + level-cross check. `variant=` selects classifier; crossover detector shared. |
| [x] | `ob` | `ob` (+1/-1/NaN), `top`, `bottom`, `ob_volume`, `mitigated` flag | Impl: `smc.py::ob`; LuxAlgo variant: dual pivot scales (50/5), volatile-bar exclusion `(high-low) ≥ 2×ATR(200)` | Reuse `Atr(200)` for the filter; block located at structure break from extrema state; zone list for mitigation. Inherits swing confirmation lag. |
| [x] | `liquidity` | `liquidity` (+1/-1/NaN), `level`, `swept` flag at sweep bar | Impl: `smc.py::liquidity(range_percent=0.01)` | Cluster incrementally as swings confirm (nearest pool within tolerance else new pool) — avoids the package's O(n²) rescan and is chunk-invariant by construction. |
| [x] | `equal_highs_lows` | `eqh`/`eql` flags, `level` | Impl: LuxAlgo Pine — pivots equal when `max < min + ATR(200)×eq_threshold` (default 0.1), `eq_len=3` | O(1): compare consecutive confirmed pivots; needs `Atr(200)` + last-pivot slot. |
| [x] | `previous_high_low` | `prev_high`, `prev_low`, `broken_high`/`broken_low` flags | Impl: `smc.py::previous_high_low(time_frame)` | O(1) given session-flag input series: running HTF extrema, snapshot at boundary. Causal by nature. |
| [x] | `sessions` | `active` (0/1), `session_high`, `session_low` | Impl: `smc.py::sessions` | O(1) given session flags. Causal running extrema — matches package exactly. |
| [x] | `retracements` | `direction`, `current_retracement_pct`, `deepest_retracement_pct` | Impl: `smc.py::retracements` | O(1): two floats (leg high/low) updated on swing confirmation; inherits swing lag. |
| [ ] | `premium_discount` | `zone` (-1/0/+1), `equilibrium` level | Theory: zones relative to 50% of current swing range (LuxAlgo). Verify exact rule in Pine source — open question from research | O(1) once swings exist. |

## P2 — modern mainstream indicators

All are classic per-bar indicators — same-size, causal, no contract issues.
Oracle: pandas-ta-classic (canonical twopirllc repo deleted; pin
fork/successor version) unless noted.

| Done | Function | Reference (Impl / Theory) | Implementation & speed note |
|---|---|---|---|
| [x] | Supertrend | Impl: pandas-ta `overlap/supertrend.py` (pinned pandas-ta classic 0.6.52); Theory: Olivier Seban | O(1): band-ratchet state machine over inline Wilder RMA using the package's seed (mean of first `length−1` TRs at bar `length−1` — NOT the stream `Atr` seed, so the ATR is computed inside the state). Direction starts `+1`; oracle test compares from bar `length−1` (package seeds bar 0 with trend `0`/dir `+1`). |
| [x] | Ichimoku | Impl: pandas-ta `overlap/ichimoku.py`; Theory: Hosoda 9/26/52 | O(1) amortized: three `Midprice` (RollingExtrema pair) states; chikou/senkou displacement: **emit non-shifted values** (causal) plus the displacement constant in metadata — plotting shift is presentation, not computation. Document that pandas-ta's shifted columns need re-alignment in oracle tests. |
| [x] | Squeeze / Squeeze Pro | Impl: pandas-ta `momentum/squeeze.py`, `squeeze_pro.py`; Theory: John Carter | O(1): composition of BBANDS (SMA basis + population std via `Sma`+`Stddev`) and Keltner (SMA close basis + SMA-of-TR band via `SqueezeTrBand`). TR[0]=NaN is excluded from KC windows → first KC value at bar `kc_length` (pandas rolling semantics). Momentum line = SMA of `mom`. `on`/`off`/`no` are 0/1; during warm-up `no=1` (NaN `&` yields False). SqueezePro shares one basis/band across wide/normal/narrow scalars (validates wide>normal>narrow). |
| [x] | Schaff Trend Cycle | Impl: pandas-ta `momentum/stc.py` | O(1): MACD from two SMA-seeded `Ema`s + two cascaded stochastic recurrences over monotonic-deque rolling extrema (`RollingExtremum`); `round(..., 8)` smoothing replicated (round-half-to-even). `stc`/`stoch` seeded `0` and carried forward while windows are cold; `macd` NaN until EMAs warm. Documented deviation: pandas-ta `non_zero_range` adds f64 epsilon to the whole series when any element is exactly zero (common in the 8-decimal-rounded pf series) — not reproducible in a streaming state, observable only as ~1e-8 boundary flips (≤ ~2e-6 cumulative); parity tests use atol=1e-5. |
| [x] | Vortex (VI±) | Impl: bukosabino ta `trend.VortexIndicator`; Theory: Botes & Siepman, TASC Jan 2010 | O(1): three rolling `Sum` states (`RollingSum`, pandas `min_periods=n` skip-NaN semantics). +VI/−VI first defined at bar `n` (bar-0 movement terms are NaN). Bar-0 TR uses `close` as its own previous close — the package fills it with the global close mean, but that value only enters incomplete windows, so outputs are identical. |
| [ ] | KST | Impl: ta `trend.KSTIndicator`; Theory: Martin Pring | O(1): 4×(ROC→SMA) + signal SMA, all existing states. |
| [ ] | Mass Index | Impl: ta `trend.MassIndex`; Theory: Donald Dorsey | O(1): EMA(9) of range, EMA of that, `Sum(25)` of ratio. |
| [ ] | DPO | Impl: pandas-ta `trend/dpo.py` | **Causality trap**: pandas-ta default `centered=True` is non-causal. taflow implements `centered=False` only; docstring explains the difference. O(1): SMA + delay ring. |
| [ ] | CMF | Impl: ta `volume.ChaikinMoneyFlowIndicator` | O(1): two `Sum` states. |
| [ ] | Klinger VO | Impl: pandas-ta `volume/kvo.py` (definitions vary — pin pandas-ta's) | O(1): trend-flip state + two EMAs. |
| [ ] | VPT | Impl: ta `volume.VolumePriceTrendIndicator` | O(1) cumulative recurrence. |
| [ ] | NVI / PVI | Impl: ta `volume.NegativeVolumeIndexIndicator`; pandas-ta `pvi` | O(1): recurrence gated on volume direction. |
| [ ] | McGinley Dynamic | Impl: pandas-ta `overlap/mcgd.py`; Theory: McGinley, JoTA 1997 | O(1): `md += (x−md)/(k·n·(x/md)^4)`; guard `md→0`. |
| [ ] | VIDYA | Impl: pandas-ta / freqtrade technical; Theory: Chande, TASC 1992 | O(1): CMO-modulated EMA — reuse `Cmo`. |
| [ ] | Laguerre RSI | Impl: freqtrade technical; Theory: Ehlers, *Cybernetic Analysis*, ch. 14 | O(1): 4-float Laguerre filter recurrence. |
| [ ] | RMI | Impl: freqtrade technical `indicators.py::RMI` | O(1): Wilder smoothing + lag-k delay ring. |
| [ ] | JMA | Impl: pandas-ta `overlap/jma.py` (public reconstruction; true JMA proprietary — document we match pandas-ta) | O(1) multi-stage adaptive recurrence. |
| [ ] | SSL Channel | Impl: freqtrade technical `indicators.py::SSLChannels` | O(1): SMA(high), SMA(low) + side flip-flop. |
| [ ] | PMAX | Impl: freqtrade technical `indicators.py::PMAX` | O(1): Supertrend generalized over (MA, ATR multiple) — share ratchet code. |
| [ ] | TD Sequential | Impl: pandas-ta `momentum/td_seq.py`; Theory: Tom DeMark | O(1): setup/countdown counters + 4-bar delay ring; integer outputs like CDL patterns. |
| [ ] | Even Better Sinewave | Impl: pandas-ta `cycles/ebsw.py`; Theory: Ehlers, *Cycle Analytics for Traders* | O(1) recurrence; lives with Hilbert family. |
| [ ] | Fibonacci retracement levels | Impl: freqtrade technical `indicators.py::fibonacci_retracements(window=120)` | O(1) on rolling extrema; emit each level as its own same-size series. |

## P3 — per-bar transforms

| Done | Function | Reference | Implementation & speed note |
|---|---|---|---|
| [ ] | Heikin-Ashi | Impl: pandas-ta `candles/ha.py` | 1:1 OHLC→OHLC transform, O(1) 4-float recurrence. The only chart transform that fits the same-size contract. |

## P4 — session/anchored (`session`) — needs P0 session-flag input

All rows emit one value per input bar (running levels), so they fit the
contract; the session boundary arrives as an input series.

| Done | Function | Outputs | Reference | Implementation & speed note |
|---|---|---|---|---|
| [ ] | Anchored/session VWAP (+ σ-bands) | `vwap`, `upper`, `lower` | Impl: pandas-ta `overlap/vwap.py`; TradingView VWAP docs for band convention | O(1): cumulative Σpv, Σv, Σpv² reset on anchor flag; bands from running moments. |
| [ ] | Pivot points (classic, Fibonacci, Camarilla, Woodie) | one series per level (PP, R1-3, S1-3) | Impl: freqtrade technical `pivots_points.py` | O(1): snapshot prior-session OHLC at boundary; four variants share state, differ only in level formulas. Levels constant within a session — still emitted per bar (same-size). |
| [ ] | Opening range | `or_high`, `or_low`, `breakout` (+1/-1/0) | Theory: standard ORB (first N bars of session) | O(1): extrema until bar-count cutoff, then frozen + crossover flags. |
| [ ] | Session volume levels (POC, VAH, VAL) | `poc`, `vah`, `val` per bar | Impl: TradingView Volume Profile docs; Theory: CBOT Market Profile (value area 70%) | Histogram is **internal state** (fixed price bins, O(1) update/bar); outputs are the running POC/VAH/VAL as same-size series (O(bins) refresh per bar, bins bounded). Document bin-width sensitivity. |

## P5 — realized-volatility estimators (`rv`)

Rolling means of per-bar closed-form terms → all O(1) via
`RollingMoments`/`Sum`; one file, shared tests. Formulas: cross-check the
portfoliooptimizer.io summary against the original papers (extracted-only
evidence in the research round).

| Done | Function | Reference (Theory) | Implementation & speed note |
|---|---|---|---|
| [x] | Close-to-close σ | textbook | `RollingMoments` on log returns; baseline for the family's tests. |
| [x] | Parkinson | Parkinson (1980), *J. Business* 53:61-65 | rolling mean of `ln(H/L)²` × `1/(4 ln 2)`. Zero-drift assumption — document. |
| [x] | Garman-Klass | Garman & Klass (1980), *J. Business* 53:67-78 | rolling mean of `0.5·ln(H/L)² − (2ln2−1)·ln(C/O)²`. |
| [x] | Rogers-Satchell | Rogers & Satchell (1991), *Ann. Appl. Prob.* 1:504-512 | rolling mean of `ln(H/C)ln(H/O) + ln(L/C)ln(L/O)`; drift-independent. |
| [x] | GK-Yang-Zhang | Yang & Zhang (2000) extension | GK + overnight term `ln(O/C_prev)²`; one prev-close float. |
| [x] | Yang-Zhang | Yang & Zhang (2000), *J. Business* 73:477-491 | `σ²_YZ = σ²_on + k·σ²_oc + (1−k)·σ²_RS`, `k = 0.34/(1.34+(n+1)/(n−1))`; three moment sets + prev close. Highest efficiency (~14×) — headline function. |

## P6 — time-series alpha operators (`ops` extension)

Theory: Kakushadze, *101 Formulaic Alphas*, arXiv:1601.00991, Appendix A.1.
Impl: github.com/yli188/WorldQuant_alpha101_code. Only the per-series
(time-series) operators — they all fit the contract. The cross-sectional
tier does not (see non-goals).

| Done | Function | Reference | Implementation & speed note |
|---|---|---|---|
| [x] | `ts_rank(x, d)` | A.1; yli188 `ts_rank` | O(w) branchless SIMD count of `values < x` over ring buffer — beats trees for w≤64; Fenwick-over-ranks O(log w) only if large windows prove hot. Shares machinery with planned `rolling_rank`. |
| [x] | `signedpower(x, a)` | A.1 | pointwise `sign(x)·|x|^a`; special-case a=2 as `x·|x|` (no `powf`). |
| [x] | `decay_linear(x, d)` | A.1 | **Alias of WMA** (verified) — re-export, zero code. |
| [x] | `adv(d)` | paper §2 | SMA of `close×volume` — composition. |

## P7 — stat-arb primitives (`quant`)

| Done | Function | Reference | Implementation & speed note |
|---|---|---|---|
| [x] | Kalman hedge ratio (online regression) | Theory: state (α,β), random-walk transition, obs `y=α+βx+v`; Impl: QuantStart "Dynamic Hedge Ratio Between ETF Pairs Using the Kalman Filter"; letianzj.github.io/kalman-filter-pairs-trading.html (pykalman `filter_update`) | Two input series → same-size outputs α, β, innovation, `√S` (fits contract like BETA/CORREL). O(1)/bar: hand-rolled 2-state filter (~a dozen FLOPs), no linalg dep. Oracle: pykalman. |
| [x] | OU half-life | Theory: `−ln(2)/λ`, λ from regressing Δp on lagged p; Impl: robotwealth "Mean Reversion and Cointegration pt 2" | O(1): `RollingPairMoments(Δp, p_lag)` slope → closed form; λ≥0 → NaN. |
| [x] | Rolling spread z-score | composition | `(spread − mean)/std` over rolling window given hedge ratio — pure composition. |
| [x] | CUSUM event flags | Theory: *AFML* §2.5.2; Impl: mlfinlab `filters.cusum_filter` | Same-size 0/±1 flag series, O(1): two accumulators + reset. (Reclassified here from the ML family — as a flag series it fits the contract.) |
| [x] | Fractional differentiation (FFD) | Theory: *AFML* ch. 5; Impl: mlfinlab `frac_diff_ffd(d, thres)` | Same-size output. Precompute truncated weights (`w_k = −w_{k−1}(d−k+1)/k`, stop at |w|<thres); per bar = dot product over ring buffer — O(w) fixed, SIMD FMA loop shaped like WMA bulk. `min_ffd` (ADF scan) stays Python-layer. |
| [x] | Amihud illiquidity | Amihud (2002), *J. Fin. Markets* 5:31-56 | O(1): rolling mean of `|ret|/(close×volume)` — `Sma` composition. |
| [x] | Roll spread | Roll (1984), *J. Finance* 39:1127-1139 | O(1): `2√max(0, −cov(Δp_t, Δp_{t−1}))` via `RollingPairMoments`. |

## Out of scope under the same-size series contract (explicit non-goals)

Do **not** implement these in the indicator API; revisit only if taflow
ever grows a separate product surface:

| Function | Why it violates the contract |
|---|---|
| Renko / range bars / Kagi / Point & Figure | Emit fewer (or variable-count) bars than input — output length ≠ input length. |
| Tick/volume/dollar bars, imbalance/run bars (*AFML* ch. 2; mlfinpy) | Resampling transforms: N ticks → M bars, M ≪ N. A future `BarBuilder` API could host them, but that is a different product decision, not a checklist item. |
| Cross-sectional `rank`/`scale`/`indneutralize` (101-Alphas A.1) | Need the full universe snapshot per timestamp — multi-asset input, not a single aligned series set. |
| Volume-profile histogram output | Output is a histogram per query, not a series (per-bar POC/VAH/VAL series version IS in scope — P4). |
| Trendline detection (technical `trendline.py`) | Global batch fit, non-causal, not chunk-invariant. |
| Triple-barrier labeling (*AFML* ch. 3) | Event-driven labeling with forward-looking barriers — inherently non-causal (labels depend on future path). Python-layer ML utility at most. |
| Cointegration tests (Engle-Granger/CADF, Johansen) | Batch statistical tests (regression/eigen), scalar results, not per-bar series. Point users to statsmodels. |
| Kyle λ, VPIN, OFI/CVD, Lee-Ready | Require tick/quote/trade-classified input taflow doesn't model; also weakest demand evidence. |
| SMC lookahead semantics (package-exact swing markers, `MitigatedIndex`-style future indices) | Violates causality/chunk invariance — taflow ships the causal re-encoding (P1) and documents the alignment rule for oracle tests. |

## Cross-cutting implementation notes

1. Order of leverage: P0 → P1/P4; the SMC family is ~80% swing-state +
   zone-list once P0 exists.
2. Almost every P2/P5/P7 row is a composition of existing states (`Ema`,
   `Atr`, `Sum`, `RollingMoments`, `RollingPairMoments`, extrema deques).
   Resist writing new numerics; wiring states together means the
   `optimize-methods.md` wins (FMA, ring buffers, fused bulk chains) apply
   automatically.
3. Multi-output stays within the existing pattern (like MACD/BBANDS
   multi-output structs) — flags are f64 series with NaN/±1, matching how
   CDL patterns emit integers. No new output machinery needed.
4. Causality review is a per-row gate: before implementing, diff the
   reference implementation for `shift(-k)`, centered windows, or
   future-index columns (found in: smc swings, pandas-ta DPO
   `centered=True`, Ichimoku plotted displacement). Implement causal;
   document the alignment rule for the oracle test.
5. Definitions vary across sources for: Supertrend seeding, KVO, JMA, DPO,
   FVG (joshyattridge vs LuxAlgo). The reference column pins whose
   definition taflow matches — repeat that sentence in the docstring.
