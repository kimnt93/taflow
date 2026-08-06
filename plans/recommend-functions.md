# Recommended functions beyond TA-Lib

> **Note:** this is the research record. The actionable, contract-filtered
> inventory is [`recommend-functions-checklist.md`](recommend-functions-checklist.md)
> — several families below (bar transforms, cross-sectional operators,
> trendlines, triple-barrier, tick microstructure) do **not** fit taflow's
> same-size causal series contract and are listed there as explicit
> non-goals. Implement from the checklist, not from this file.

Result of a deep-research pass (2026-08-06): 5 search angles, 23 sources
fetched, 108 claims extracted, top 25 adversarially verified (22 confirmed,
3 refuted). Classified by family and prioritized by popularity × streaming
feasibility for taflow's persistent-state model.

Confidence legend:
- **[verified]** — survived 3-vote adversarial verification against primary
  sources (repo source code, official listings).
- **[extracted]** — pulled from a fetched source but not verified in this
  round; re-check the cited source before implementing.
- Streaming-cost notes marked *(own analysis)* are my assessment against
  taflow's existing primitives, not a research claim.

Everything already on `full-ta-checklist.md` or
`operator-library-checklist.md` is excluded here.

---

## Priority tiers at a glance

| Tier | Family | Why |
|---|---|---|
| 1 | SMC / market structure | Exceptional demand; no fast native implementation exists anywhere |
| 1 | Modern mainstream indicators (Supertrend, Ichimoku, VWAP, …) | Converged demand across pandas-ta / ta / freqtrade-technical; all cheaply streamable |
| 2 | Bar transforms (Heikin-Ashi, Renko, volume/dollar/imbalance bars) | Inherently streaming constructs; mlfinlab-verified inventory; differentiator vs every batch library |
| 2 | Range-based volatility estimators | Simple rolling formulas, strong quant credibility, trivial to stream |
| 2 | Session/anchored functions (anchored VWAP, pivots, opening range) | Every TradingView user expects them; research round left details unverified |
| 3 | Alpha operators (101-Alphas ts_* set) | Small operator basis, mostly already planned; completes a quant-feature story |
| 3 | Stat-arb primitives (Kalman hedge ratio, half-life, spread z-score) | O(1) streamable, high quant value, smaller audience |
| 4 | Financial-ML transforms (fracdiff, CUSUM, triple-barrier) | Well-defined but shifts toward ML-pipeline territory |
| 4 | Microstructure (Kyle λ, Amihud, VPIN) | Weakest demand evidence; needs tick/quote data taflow doesn't ingest yet |

---

## 1. Smart Money Concepts (`smc` family) — Tier 1

**Demand [verified]:** LuxAlgo's SMC indicator on TradingView: 156,552
likes, 4.63M views (fetched 2026-08-06), reportedly TradingView's most-liked
community indicator. The `smartmoneyconcepts` Python package
(joshyattridge, ~1.9k stars, MIT) is the canonical open-source reference.
No compiled/streaming implementation of this family exists — a Rust one
would be a genuine differentiator.

**Canonical 8-function inventory [verified]** (signatures from
`smartmoneyconcepts/smc.py`):

| Function | Definition (verified from source) | Streaming cost |
|---|---|---|
| `fvg(join_consecutive)` | Bullish: `high[i-1] < low[i+1]` AND `close[i] > open[i]` (mirrored bearish). 3-candle pattern. | **O(1), 1-bar lag** [verified]. Mitigation tracking (`MitigatedIndex`) needs a list of active gaps — O(active gaps) per bar. |
| `swing_highs_lows(swing_length=50)` | Centered window: bar is a swing high if it is the max of `swing_length` bars **before and after** (code: `swing_length *= 2` + negative shift). | O(1) via monotonic deque, but **confirms only after `swing_length` future bars** [verified]. This is lookahead by definition — see semantics note below. |
| `bos_choch(close_break=True)` | Fixed inequality patterns over the last 4 alternating swings; break confirmed when close (or high/low) crosses the stored level. Bullish BOS: swings `[-1,1,-1,1]` with `low[-4] < low[-2] < high[-3] < high[-1]`. [verified] | O(1) state machine over confirmed swings + crossover detection *(own analysis)*; inherits swing confirmation lag. |
| `ob(close_mitigation=False)` | Order block located at the extreme within the structure interval. LuxAlgo variant [verified]: two pivot scales (swing 50 / internal 5), excludes volatile bars where `(high-low) >= 2 × ATR(200)` (or cumulative-mean-range) threshold. | O(1) amortized with extrema deque + active-block list *(own analysis)*; inherits swing lag. |
| `liquidity(range_percent=0.01)` | Clusters of swing highs/lows within a price tolerance band. | O(active pools) per bar *(own analysis)*. |
| `previous_high_low(time_frame)` | Prior higher-timeframe bar's high/low. | O(1); needs session/HTF calendar state *(own analysis)*. |
| `sessions(session, times)` | Session high/low/open tracking (Asia/London/NY). | O(1); timestamp-aware state *(own analysis)*. |
| `retracements(...)` | % retracement from the last confirmed swing leg. | O(1) after swing confirmation *(own analysis)*. |

**Plus two LuxAlgo primitives worth adding [verified]:**

- `equal_highs_lows(eq_len=3, eq_threshold=0.1)` — two pivots are "equal"
  when their difference `< eq_threshold × ATR(200)`; 3-bar pivot
  confirmation. Deterministic ATR-scaled tolerance comparison; O(1).
- `premium_discount_zones` — zones relative to the midpoint of the current
  swing range. Simple once swings exist. *(Definition beyond the 50%
  midpoint is an open question from the research — verify against LuxAlgo
  Pine before implementing.)*

**Critical design decisions for taflow** (both flagged by verification):

1. **Confirmation-lag semantics.** Swing-dependent functions are
   *non-causal by definition* (centered windows). A streaming
   implementation must emit "swing confirmed at bar `i` (as of bar
   `i + swing_length`)" — either as late-emitting events or as a value with
   an explicit `confirmed_at` index. Document this loudly; do not present
   them as ordinary causal indicators. FVG is the exception (1-bar lag).
2. **There is no single standard definition.** joshyattridge and LuxAlgo
   use materially different FVG encodings (direction-filtered 3-candle vs
   wick-vs-body) and different BOS/CHoCH encodings (4-swing inequality
   pattern vs trend-state machine on pivot crosses). A claim that LuxAlgo
   uses a rolling-mean FVG magnitude threshold was **refuted (0-3)** —
   don't trust secondhand descriptions of LuxAlgo internals; read the Pine
   source. Recommendation: implement the joshyattridge semantics as the
   default (it's the pip-installable de-facto standard people will diff
   against) and expose a `variant=` parameter for LuxAlgo-style rules.

Sources: github.com/joshyattridge/smart-money-concepts (+ smc.py),
tradingview.com/script/CnB3fSph, LuxAlgo Pine mirrors
(gist niquedegraaff, github acepriority/PineScript).

---

## 2. Modern mainstream indicators — Tier 1

Three independent library inventories converge on the same beyond-TA-Lib
core; anything appearing in ≥2 of them is safe to prioritize.

**pandas-ta [verified]** (canonical repo taken down; evidence from fork
0xAVX/pandas-ta and successor pandas-ta-classic, matching v0.3.14b):
10-family taxonomy — Candles (64), Cycles (1), Momentum (41), Overlap (33),
Performance (3), Statistics (11), Trend (18), Utility (5), Volatility (14),
Volume (15). The Trend/Performance/Utility families don't exist in TA-Lib —
worth copying as taxonomy. Named beyond-TA-Lib members: Supertrend, VWAP,
Ichimoku, HMA, ALMA, Jurik MA (JMA), Squeeze / Squeeze Pro, Even Better
Sinewave, Klinger Volume Oscillator, Schaff Trend Cycle, TD Sequential.

**bukosabino/ta [verified]** (~4k stars, 43 indicators in 5 families):
adds Ichimoku, Keltner, Donchian, Ulcer Index, VWAP, KST, Schaff Trend
Cycle, Vortex, Mass Index, DPO, Awesome Oscillator; volume family adds CMF,
Force Index, Ease of Movement, VPT, NVI.

**freqtrade/technical [verified]** (validated by a large live-trading
community): Ichimoku, VWMA, Supertrend, ALMA, PMAX, SSL Channel, Schaff
Trend Cycle, Laguerre RSI, VIDYA, RMI, Volume-Weighted MACD, TKE, Volume
Flow Indicator, VPCI — plus **structure tooling**: two trendline-detection
algorithms, `fibonacci_retracements`, `pivots_points`.

Recommended additions (excluding what's already on your operator checklist),
with streaming cost *(own analysis)*:

| Function | Appears in | Streaming cost |
|---|---|---|
| **Supertrend** | pandas-ta, technical (and pre-SMC was among TradingView's most-liked) | O(1): ATR state + band ratchet state machine |
| **Ichimoku** (5 lines) | all three | O(1) amortized: 3 monotonic-deque midpoints + fixed displacement buffers |
| **Squeeze / Squeeze Pro** (BB inside Keltner) | pandas-ta | O(1): composes BBANDS + Keltner you already have |
| **Schaff Trend Cycle** | all three | O(1): MACD + double stochastic recurrence |
| **Vortex Indicator** | ta | O(1): two rolling sums |
| **KST Oscillator** | ta | O(1): 4 ROC + 4 SMA compositions |
| **Mass Index** | ta | O(1): EMA-ratio rolling sum |
| **DPO** | ta, pandas-ta | O(1): SMA + fixed delay line |
| **Chaikin Money Flow (CMF)** | ta | O(1): two rolling sums |
| **Klinger Volume Oscillator** | pandas-ta | O(1): EMA chain on volume force |
| **NVI / PVI** | ta | O(1): scalar recurrence |
| **VPT (Volume-Price Trend)** | ta | O(1): cumulative recurrence |
| **Jurik MA (JMA)** | pandas-ta, technical | O(1) recurrence; definition is semi-proprietary — document which public reconstruction you follow |
| **VIDYA** | technical | O(1): CMO-modulated EMA |
| **Laguerre RSI** | technical | O(1): 4-stage recurrence |
| **RMI (Relative Momentum Index)** | technical | O(1): RSI with lag-k differences |
| **PMAX / SSL Channel** | technical | O(1): compositions of MA + ATR / rolling extrema |
| **McGinley Dynamic** | pandas-ta | O(1) recurrence |
| **TD Sequential** | pandas-ta | O(1) counter state machine |
| **Even Better Sinewave** (Ehlers) | pandas-ta | O(1) recurrence (fits your Hilbert family) |
| **Trendline detection** (gentrends/segtrends) | technical | batch-oriented; offer as non-streaming utility like `rolling_apply` |
| **Fibonacci retracements** | technical | O(1) on top of rolling extrema |

A refuted claim (0-3): "pandas-ta implements 130+ indicators + 60 patterns
exceeding TA-Lib" — the real counts are the family table above; don't quote
the 130+ figure.

---

## 3. Bar transforms (`bars` family) — Tier 2

These change the sampling clock rather than compute a value per input bar —
API-wise they consume a stream and emit a (sparser) stream of OHLCV bars.
That is inherently streaming: **a bar closes when a cumulative threshold is
hit, so every variant is O(1) per tick** [verified for imbalance bars;
*(own analysis)* for the chart types].

**Information-driven bars [verified]:** mlfinpy (mlfinlab lineage)
implements exactly six public imbalance-bar functions — EMA-based and
constant-threshold variants for **tick / volume / dollar imbalance** — each
emitting OHLCV plus `cum_buy_volume`, `cum_ticks`, `cum_dollar_value`.
Canonical source: López de Prado, *Advances in Financial Machine Learning*
(2018), p. 29 ("activity-based sampling has better statistical properties
than time bars"; one verified caveat: better statistics ≠ better strategy
returns). Also in the family: plain tick/volume/dollar bars and run bars.

**Chart-type transforms [extracted / own analysis]:** Heikin-Ashi (O(1)
recurrence — trivial, huge retail demand), Renko (O(1) brick state machine;
parameterize ATR-brick vs fixed-brick), Range bars, Kagi, Point & Figure
(all O(1) state machines). The research round returned no verified
popularity claims for these — but every charting platform ships them, and
they cost little.

---

## 4. Session / anchored calculations — Tier 2

Research gap: this angle's claims didn't reach verification, so demand
evidence is thin in this round — but `pivots_points` in freqtrade/technical
and VWAP presence in every modern library [verified] anchor the family.
All are O(1) with timestamp-aware session state *(own analysis)*:

- **Session VWAP / anchored VWAP** (anchor = session open, week, month, or
  arbitrary user timestamp) — cumulative `Σpv / Σv` reset at anchor; add
  VWAP standard-deviation bands (rolling moments you already have).
- **Pivot points**: classic, Fibonacci, Camarilla, Woodie — closed-form
  from prior session OHLC; needs only the session-boundary machinery.
- **Opening range** (first N minutes high/low) + breakout signals.
- **Volume profile / market profile** (POC, value area high/low): fixed
  price-bin histogram updated O(1) per bar; POC/VA queried on demand O(bins).
  The only member that needs real state; still bounded.

The session-calendar/anchor mechanism is shared infrastructure for this
whole family *and* for SMC `sessions`/`previous_high_low` — build it once.

---

## 5. Range-based realized-volatility estimators (`rv` family) — Tier 2

[extracted — sources: portfoliooptimizer.io overview (with formulas),
macrosynergy.com; not verified this round, but formulas are textbook.]

| Estimator | Inputs | Property (extracted) | Streaming cost *(own analysis)* |
|---|---|---|---|
| Close-to-close | C | baseline | O(1) rolling moments |
| **Parkinson (1980)** | H,L | up to 5.2× efficiency of close-to-close; assumes zero drift, misses gaps | O(1): rolling sum of `ln(H/L)²` |
| **Garman-Klass** | O,H,L,C | up to 7.4×; zero drift, misses overnight gaps | O(1): two rolling sums |
| **Rogers-Satchell** | O,H,L,C | up to 6×; drift-independent | O(1): one rolling sum |
| **Garman-Klass-Yang-Zhang** | O,H,L,C + prev C | GK + overnight-jump term | O(1) |
| **Yang-Zhang** | O,H,L,C + prev C | up to 14× (the max); drift-independent AND handles gaps; weighted combo of overnight var, open-to-close var, and RS | O(1): three rolling-moment sets |

Extracted design insight worth keeping in the docs: no single-period
estimator handles both non-zero drift and opening jumps — that's the reason
to ship the whole set rather than one "best" estimator. All six are one
afternoon of work each on top of `RollingMoments`; very high
credibility-per-effort. (These also directly upgrade your planned
`rolling_sharpe`/Keltner-style consumers.)

---

## 6. Cross-sectional / formulaic-alpha operators (`ops` extension) — Tier 3

[extracted — primary source: Kakushadze, *101 Formulaic Alphas*,
arXiv:1601.00991, Appendix A.1; corroborated by the yli188 pandas
implementation (~18 primitives cover ~80 of 101 alphas).]

The complete operator basis for all 101 alphas:

- **Time-series (per-instrument — fits taflow's streaming model):**
  `delay(x,d)`, `delta(x,d)` — you have these (MOM/lag);
  `ts_min/ts_max/ts_argmin/ts_argmax` — you have (MIN/MAX/MININDEX/MAXINDEX);
  `sum/product/stddev/correlation/covariance` over a window — planned;
  **`ts_rank(x,d)`** — rank of today's value within the trailing window:
  O(log w) per bar with an order-statistics structure (Fenwick over value
  ranks or a skiplist), or O(w) rescan for small windows — same machinery
  as your planned `rolling_rank`;
  **`decay_linear(x,d)`** — extracted claim, confirmed by inspection: it is
  exactly WMA, already O(1) in taflow;
  **`signedpower(x,a)`** = `sign(x)·|x|^a` — pointwise, trivial.
- **Cross-sectional (require the full universe snapshot per timestamp —
  do NOT fit a single-series stream):** `rank(x)` (cross-sectional),
  `scale(x,a)` (normalize so `Σ|x| = a`), `indneutralize(x, group)` (group
  demeaning). Extracted claim confirms ~20 of the 101 alphas need
  indneutralize/cap data unavailable from OHLCV alone.
- **Auxiliary inputs:** `vwap`, `adv{d}` (average daily dollar volume —
  just SMA(close×volume)), both trivial.

Recommendation: implement the time-series set as part of the operator
checklist (only `ts_rank` and `signedpower` are genuinely new), and expose
the cross-sectional trio in a separate **multi-series** API tier
(`taflow.xs`) that takes an aligned matrix/dict of streams — be explicit
that it breaks the per-series O(1) model (O(universe) or O(universe log
universe) per timestamp).

---

## 7. Stat-arb primitives — Tier 3

[extracted — sources: QuantStart, robotwealth, letianzj; all blog-grade,
formulas standard.]

- **Kalman-filter hedge ratio / online regression**: state = (intercept,
  slope), random-walk transition, observation `y = a + b·x + v`. Extracted
  claims confirm it is an online recursive regression with **O(1) per-tick
  updates** (`filter_update` pattern) — a perfect fit for taflow, and the
  standard replacement for static OLS hedge ratios which are time-varying.
  Also emit the innovation ± `δ√S` bands (z-score entry/exit) as outputs.
- **Half-life of mean reversion (OU)**: `-ln(2)/λ` where λ comes from
  regressing Δp on lagged p — implementable as a rolling pair-moments
  computation you already have (`RollingPairMoments`), O(1).
- **Rolling spread z-score** (given hedge ratio): composition of existing
  primitives, O(1).
- **Cointegration tests** (Engle-Granger/CADF, Johansen): batch statistical
  tests, not streaming indicators. Recommend: out of scope for the core;
  at most a Python-level utility. Johansen especially (eigen-decomposition,
  multi-asset) belongs in statsmodels territory.

---

## 8. Financial-ML transforms — Tier 4

[extracted — sources: Hudson & Thames (mlfinlab maintainers), deepwiki
mlfinlab; canonical reference López de Prado, *AFML* 2018.]

- **Fractional differentiation (FFD, fixed-width window)**: makes a series
  stationary while retaining memory; mlfinlab API `frac_diff_ffd(d,
  threshold)`. Streaming: precompute the truncated weight vector once, then
  each bar is a dot product over the last `w` values — **O(w) per bar with
  fixed w** *(own analysis)*, fits a ring buffer; genuinely useful as an ML
  feature and rare in native code.
- **CUSUM filter**: event-sampling on cumulative deviations,
  `filters.cusum_filter(series, threshold)` — O(1) recurrence, easy, pairs
  naturally with the bars family.
- **Triple-barrier labeling**: three stopping conditions (PT/SL/time),
  labels 1/−1/0, barrier widths scaled by rolling volatility; mlfinlab API
  `get_events`/`get_bins`. Extracted claim (correctly) notes the canonical
  workflow is event-driven (CUSUM → barriers → labels), not bar-by-bar, and
  no source documents streaming feasibility. It's a *labeling* utility for
  ML training, not an indicator — recommend Python-layer implementation
  only, or defer.

---

## 9. Microstructure features — Tier 4 (defer)

Weakest evidence in the round: the only implementing library found
(mqt-microstructure) has ~1 star and targets MQL5 — inventory evidence
only, zero demand evidence. The family (Kyle's lambda, Amihud illiquidity,
roll spread, VPIN, order-flow imbalance/CVD, with tick-rule/Lee-Ready trade
classification as a prerequisite primitive) also requires tick/quote-level
input taflow doesn't currently model. Revisit only if taflow grows a
tick-data story; Amihud (|return|/dollar volume, rolling mean) is the one
member computable from daily OHLCV today at O(1).

---

## Open questions from the research (worth a follow-up pass)

1. Exact deterministic definitions for liquidity *sweeps/grabs* as events
   and premium/discount zones beyond the 50% midpoint.
2. Verified formulas + demand evidence for realized-vol estimators and
   microstructure features (this round's claims are extracted-only).
3. Popularity ranking *within* the converged mainstream set (Supertrend vs
   Ichimoku vs VWAP variants vs Squeeze) to order Tier-1 implementation.
4. Whether any 101-alphas operator beyond rank/scale/indneutralize needs
   cross-asset state that breaks the per-series model (current evidence:
   no).

## Source notes

- Canonical pandas-ta repo (twopirllc) was removed from GitHub; family
  counts came from a fork and pandas-ta-classic and may drift by version.
- LuxAlgo evidence comes from community mirrors of their open-source Pine
  script, not TradingView directly; fine-grained LuxAlgo internals beyond
  the verified EQH/OB rules should be re-checked against Pine source (one
  such secondhand claim was refuted 0-3 in verification).
- TradingView likes measure retail popularity, not quant-desk demand.
