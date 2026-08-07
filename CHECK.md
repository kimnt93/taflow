# CHECK.md — AI implementation review gates

This file is the review contract for taflow. An implementing agent works
through the checklists in `plans/`; a reviewing AI (or the implementer,
before checking any box) walks EVERY gate below for each function. The
checklists in `plans/` are updated continuously — always re-read them and
this file before reviewing. A checklist box may only be `[x]` when every
gate here passes.

---

## 1. Module contract

Two public surfaces, ONE implementation:

- **`taflow`** — our style. Descriptive snake_case names, persistent state
  objects with continue-update compute:
  `append(...) / extend(...) / value / compute() / reset()`.
- **`taflow.talib`** — TA-Lib-compatible surface. Uppercase TA-Lib names
  (`BBANDS`, `MIN`, `MAX`, …), TA-Lib parameter names, order, defaults, and
  warm-up/NaN semantics. Thin aliases only.

Rules:

- A numerical kernel exists exactly once, in Rust
  (`crates/taflow-core`). Both surfaces bind the same kernel — never a
  second implementation, never Python math.
- Functions with no TA-Lib counterpart (SMC, rolling_median, …) exist only
  in `taflow` and never get a fake uppercase alias.

## 2. Grouping and rename map

### Groups

| Group | Contents | taflow naming |
|---|---|---|
| `rolling` | rolling-window math/statistics (from TA-Lib Math Operators + Statistic families and the operator checklist) | **mandatory `rolling_` prefix** |
| `ma` | moving averages: sma, ema, wma, dema, tema, trima, kama, t3, mama, hma, zlema, alma, vwma, jma, vidya, mcginley | indicator names (trader vocabulary), lowercase |
| `momentum` | rsi, macd, stoch, adx, cci, willr, tsi, fisher, … | lowercase indicator names |
| `volatility` | atr, natr, true_range, keltner, donchian, ulcer_index, chaikin_volatility + `rv` estimators (parkinson, garman_klass, rogers_satchell, yang_zhang) | lowercase |
| `volume` | ad, adosc, obv, cmf, force_index, eom, kvo, vpt, nvi, pvi, rolling_vwap | lowercase |
| `price` | avg_price, median_price, typical_price, weighted_close | lowercase descriptive |
| `math` | pointwise transforms (acos … tanh) — NOT rolling, no prefix | lowercase |
| `pattern` | 61 candle patterns | `cdl_` prefix: `cdl_doji`, `cdl_engulfing`, … |
| `cycle` | Hilbert family | `ht_` prefix |
| `smc` | fvg, swing_highs_lows, bos_choch, ob, liquidity, … | per recommend checklist P1 |
| `session` | anchored_vwap, pivot_points, opening_range, session volume levels | lowercase |
| `quant` | kalman_hedge_ratio, ou_half_life, spread_zscore, cusum, frac_diff, amihud, roll_spread | lowercase |
| `ops` | ts_rank, signedpower, lag, crossover helpers | lowercase |

### Rolling rename rule (the important one)

Rolling-window operators are implemented ONCE and exposed as
`taflow.rolling_*`; the un-prefixed name does NOT exist in `taflow` (it
would shadow Python builtins like `min`/`max`/`sum` anyway). The TA-Lib
uppercase name lives only in `taflow.talib`, bound to the same kernel:

| taflow (canonical) | taflow.talib alias |
|---|---|
| `rolling_min` | `MIN` |
| `rolling_max` | `MAX` |
| `rolling_sum` | `SUM` |
| `rolling_argmin` | `MININDEX` |
| `rolling_argmax` | `MAXINDEX` |
| `rolling_minmax` | `MINMAX` |
| `rolling_minmax_index` | `MINMAXINDEX` |
| `rolling_midpoint` | `MIDPOINT` |
| `rolling_std` | `STDDEV` |
| `rolling_var` | `VAR` |
| `rolling_avgdev` | `AVGDEV` |
| `rolling_corr` | `CORREL` |
| `rolling_beta` | `BETA` |
| `rolling_linreg` (+ `_slope`, `_intercept`, `_angle`, `rolling_tsf`) | `LINEARREG` (+ variants, `TSF`) |
| `rolling_median`, `rolling_quantile`, `rolling_rank`, `rolling_zscore`, `rolling_skew`, `rolling_kurtosis`, `rolling_cov`, … | — (no TA-Lib counterpart) |

Moving averages keep their indicator names (`sma`, not `rolling_mean`) —
they are named indicators, and `SMA`/`EMA`/… remain the talib aliases.

### Duplicate consolidations (found reviewing the plans checklists)

- `swing_high` / `swing_low` (operator checklist, pattern section) and the
  P0 causal swing state (recommend checklist) are the SAME function —
  implement once in `smc`/P0, re-export under `ops` naming if wanted.
- Rolling `VWAP` (operator checklist, volume) vs anchored/session VWAP
  (recommend P4) are two distinct functions in one family: `rolling_vwap`
  and `anchored_vwap`. Never a bare `vwap` (ambiguous).
- Extended MAs are one group: HMA/VWMA/ZLEMA/ALMA (operator checklist) +
  JMA/VIDYA/McGinley (recommend P2) all live in `ma`.
- `hedge_ratio` (rolling OLS, operator checklist) vs `kalman_hedge_ratio`
  (recommend P7) are different estimators — both in `quant`, distinct
  names, docstrings cross-referencing each other.
- Price-transform descriptive names (`typical_price` ↔ `TYPPRICE`, etc.)
  follow the same one-kernel/two-surfaces rule as rolling ops.

## 2.5 Master function table (live status)

One row per function: canonical Rust/Python name and the `taflow.talib`
alias (`_` = no TA-Lib counterpart — taflow-only). `[x]` means both the
batch alias and the persistent state exist in the current build; it does
NOT yet certify the §3 gates. The implementing agent updates this table
as functions land; regenerate statuses any time by running the checks in
§5.

### TA-Lib inventory

| Done | implement function rs | implement function py | implement talib |
|---|---|---|---|
| [x] | accbands | accbands | ACCBANDS |
| [x] | bbands | bbands | BBANDS |
| [x] | dema | dema | DEMA |
| [x] | ema | ema | EMA |
| [x] | ht_trendline | ht_trendline | HT_TRENDLINE |
| [x] | kama | kama | KAMA |
| [x] | ma | ma | MA |
| [x] | mama | mama | MAMA |
| [x] | mavp | mavp | MAVP |
| [x] | rolling_midpoint | rolling_midpoint | MIDPOINT |
| [x] | rolling_midprice | rolling_midprice | MIDPRICE |
| [x] | sar | sar | SAR |
| [x] | sarext | sarext | SAREXT |
| [x] | sma | sma | SMA |
| [x] | t3 | t3 | T3 |
| [x] | tema | tema | TEMA |
| [x] | trima | trima | TRIMA |
| [x] | wma | wma | WMA |
| [x] | atr | atr | ATR |
| [x] | natr | natr | NATR |
| [x] | true_range | true_range | TRANGE |
| [x] | rolling_avgdev | rolling_avgdev | AVGDEV |
| [x] | rolling_beta | rolling_beta | BETA |
| [x] | rolling_corr | rolling_corr | CORREL |
| [x] | rolling_linreg | rolling_linreg | LINEARREG |
| [x] | rolling_linreg_angle | rolling_linreg_angle | LINEARREG_ANGLE |
| [x] | rolling_linreg_intercept | rolling_linreg_intercept | LINEARREG_INTERCEPT |
| [x] | rolling_linreg_slope | rolling_linreg_slope | LINEARREG_SLOPE |
| [x] | rolling_std | rolling_std | STDDEV |
| [x] | rolling_tsf | rolling_tsf | TSF |
| [x] | rolling_var | rolling_var | VAR |
| [x] | adx | adx | ADX |
| [x] | adxr | adxr | ADXR |
| [x] | apo | apo | APO |
| [x] | aroon | aroon | AROON |
| [x] | aroonosc | aroonosc | AROONOSC |
| [x] | bop | bop | BOP |
| [x] | cci | cci | CCI |
| [x] | cmo | cmo | CMO |
| [x] | dx | dx | DX |
| [x] | imi | imi | IMI |
| [x] | macd | macd | MACD |
| [x] | macdext | macdext | MACDEXT |
| [x] | macdfix | macdfix | MACDFIX |
| [ ] | mfi | mfi | MFI |
| [ ] | minus_di | minus_di | MINUS_DI |
| [ ] | minus_dm | minus_dm | MINUS_DM |
| [x] | mom | mom | MOM |
| [ ] | plus_di | plus_di | PLUS_DI |
| [ ] | plus_dm | plus_dm | PLUS_DM |
| [x] | ppo | ppo | PPO |
| [x] | roc | roc | ROC |
| [x] | rocp | rocp | ROCP |
| [x] | rocr | rocr | ROCR |
| [x] | rocr100 | rocr100 | ROCR100 |
| [x] | rsi | rsi | RSI |
| [x] | stoch | stoch | STOCH |
| [x] | stochf | stochf | STOCHF |
| [x] | stochrsi | stochrsi | STOCHRSI |
| [ ] | trix | trix | TRIX |
| [ ] | ultosc | ultosc | ULTOSC |
| [x] | willr | willr | WILLR |
| [x] | add | add | ADD |
| [x] | div | div | DIV |
| [x] | rolling_max | rolling_max | MAX |
| [x] | rolling_argmax | rolling_argmax | MAXINDEX |
| [x] | rolling_min | rolling_min | MIN |
| [x] | rolling_argmin | rolling_argmin | MININDEX |
| [x] | rolling_minmax | rolling_minmax | MINMAX |
| [x] | rolling_minmax_index | rolling_minmax_index | MINMAXINDEX |
| [x] | mult | mult | MULT |
| [x] | sub | sub | SUB |
| [x] | rolling_sum | rolling_sum | SUM |
| [x] | avg_price | avg_price | AVGPRICE |
| [x] | median_price | median_price | MEDPRICE |
| [x] | typical_price | typical_price | TYPPRICE |
| [x] | weighted_close | weighted_close | WCLPRICE |
| [ ] | ht_dcperiod | ht_dcperiod | HT_DCPERIOD |
| [ ] | ht_dcphase | ht_dcphase | HT_DCPHASE |
| [ ] | ht_phasor | ht_phasor | HT_PHASOR |
| [ ] | ht_sine | ht_sine | HT_SINE |
| [ ] | ht_trendmode | ht_trendmode | HT_TRENDMODE |
| [ ] | cdl_2crows | cdl_2crows | CDL2CROWS |
| [ ] | cdl_3blackcrows | cdl_3blackcrows | CDL3BLACKCROWS |
| [ ] | cdl_3inside | cdl_3inside | CDL3INSIDE |
| [ ] | cdl_3linestrike | cdl_3linestrike | CDL3LINESTRIKE |
| [ ] | cdl_3outside | cdl_3outside | CDL3OUTSIDE |
| [ ] | cdl_3starsinsouth | cdl_3starsinsouth | CDL3STARSINSOUTH |
| [ ] | cdl_3whitesoldiers | cdl_3whitesoldiers | CDL3WHITESOLDIERS |
| [ ] | cdl_abandonedbaby | cdl_abandonedbaby | CDLABANDONEDBABY |
| [ ] | cdl_advanceblock | cdl_advanceblock | CDLADVANCEBLOCK |
| [ ] | cdl_belthold | cdl_belthold | CDLBELTHOLD |
| [ ] | cdl_breakaway | cdl_breakaway | CDLBREAKAWAY |
| [ ] | cdl_closingmarubozu | cdl_closingmarubozu | CDLCLOSINGMARUBOZU |
| [ ] | cdl_concealbabyswall | cdl_concealbabyswall | CDLCONCEALBABYSWALL |
| [ ] | cdl_counterattack | cdl_counterattack | CDLCOUNTERATTACK |
| [ ] | cdl_darkcloudcover | cdl_darkcloudcover | CDLDARKCLOUDCOVER |
| [ ] | cdl_doji | cdl_doji | CDLDOJI |
| [ ] | cdl_dojistar | cdl_dojistar | CDLDOJISTAR |
| [ ] | cdl_dragonflydoji | cdl_dragonflydoji | CDLDRAGONFLYDOJI |
| [ ] | cdl_engulfing | cdl_engulfing | CDLENGULFING |
| [ ] | cdl_eveningdojistar | cdl_eveningdojistar | CDLEVENINGDOJISTAR |
| [ ] | cdl_eveningstar | cdl_eveningstar | CDLEVENINGSTAR |
| [ ] | cdl_gapsidesidewhite | cdl_gapsidesidewhite | CDLGAPSIDESIDEWHITE |
| [ ] | cdl_gravestonedoji | cdl_gravestonedoji | CDLGRAVESTONEDOJI |
| [ ] | cdl_hammer | cdl_hammer | CDLHAMMER |
| [ ] | cdl_hangingman | cdl_hangingman | CDLHANGINGMAN |
| [ ] | cdl_harami | cdl_harami | CDLHARAMI |
| [ ] | cdl_haramicross | cdl_haramicross | CDLHARAMICROSS |
| [ ] | cdl_highwave | cdl_highwave | CDLHIGHWAVE |
| [ ] | cdl_hikkake | cdl_hikkake | CDLHIKKAKE |
| [ ] | cdl_hikkakemod | cdl_hikkakemod | CDLHIKKAKEMOD |
| [ ] | cdl_homingpigeon | cdl_homingpigeon | CDLHOMINGPIGEON |
| [ ] | cdl_identical3crows | cdl_identical3crows | CDLIDENTICAL3CROWS |
| [ ] | cdl_inneck | cdl_inneck | CDLINNECK |
| [ ] | cdl_invertedhammer | cdl_invertedhammer | CDLINVERTEDHAMMER |
| [ ] | cdl_kicking | cdl_kicking | CDLKICKING |
| [ ] | cdl_kickingbylength | cdl_kickingbylength | CDLKICKINGBYLENGTH |
| [ ] | cdl_ladderbottom | cdl_ladderbottom | CDLLADDERBOTTOM |
| [ ] | cdl_longleggeddoji | cdl_longleggeddoji | CDLLONGLEGGEDDOJI |
| [ ] | cdl_longline | cdl_longline | CDLLONGLINE |
| [ ] | cdl_marubozu | cdl_marubozu | CDLMARUBOZU |
| [ ] | cdl_matchinglow | cdl_matchinglow | CDLMATCHINGLOW |
| [ ] | cdl_mathold | cdl_mathold | CDLMATHOLD |
| [ ] | cdl_morningdojistar | cdl_morningdojistar | CDLMORNINGDOJISTAR |
| [ ] | cdl_morningstar | cdl_morningstar | CDLMORNINGSTAR |
| [ ] | cdl_onneck | cdl_onneck | CDLONNECK |
| [ ] | cdl_piercing | cdl_piercing | CDLPIERCING |
| [ ] | cdl_rickshawman | cdl_rickshawman | CDLRICKSHAWMAN |
| [ ] | cdl_risefall3methods | cdl_risefall3methods | CDLRISEFALL3METHODS |
| [ ] | cdl_separatinglines | cdl_separatinglines | CDLSEPARATINGLINES |
| [ ] | cdl_shootingstar | cdl_shootingstar | CDLSHOOTINGSTAR |
| [ ] | cdl_shortline | cdl_shortline | CDLSHORTLINE |
| [ ] | cdl_spinningtop | cdl_spinningtop | CDLSPINNINGTOP |
| [ ] | cdl_stalledpattern | cdl_stalledpattern | CDLSTALLEDPATTERN |
| [ ] | cdl_sticksandwich | cdl_sticksandwich | CDLSTICKSANDWICH |
| [ ] | cdl_takuri | cdl_takuri | CDLTAKURI |
| [ ] | cdl_tasukigap | cdl_tasukigap | CDLTASUKIGAP |
| [ ] | cdl_thrusting | cdl_thrusting | CDLTHRUSTING |
| [ ] | cdl_tristar | cdl_tristar | CDLTRISTAR |
| [ ] | cdl_unique3river | cdl_unique3river | CDLUNIQUE3RIVER |
| [ ] | cdl_upsidegap2crows | cdl_upsidegap2crows | CDLUPSIDEGAP2CROWS |
| [ ] | cdl_xsidegap3methods | cdl_xsidegap3methods | CDLXSIDEGAP3METHODS |
| [x] | ad | ad | AD |
| [x] | adosc | adosc | ADOSC |
| [x] | obv | obv | OBV |
| [x] | acos | acos | ACOS |
| [x] | asin | asin | ASIN |
| [x] | atan | atan | ATAN |
| [x] | ceil | ceil | CEIL |
| [x] | cos | cos | COS |
| [x] | cosh | cosh | COSH |
| [x] | exp | exp | EXP |
| [x] | floor | floor | FLOOR |
| [x] | ln | ln | LN |
| [x] | log10 | log10 | LOG10 |
| [x] | sin | sin | SIN |
| [x] | sinh | sinh | SINH |
| [x] | sqrt | sqrt | SQRT |
| [x] | tan | tan | TAN |
| [x] | tanh | tanh | TANH |

### taflow-only inventory (no TA-Lib counterpart)

| Done | implement function rs | implement function py | implement talib |
|---|---|---|---|
| [x] | lag | lag | _ |
| [x] | log_return | log_return | _ |
| [x] | cumsum | cumsum | _ |
| [x] | cumprod | cumprod | _ |
| [x] | rolling_median | rolling_median | _ |
| [x] | rolling_mode | rolling_mode | _ |
| [x] | rolling_quantile | rolling_quantile | _ |
| [x] | rolling_rank | rolling_rank | _ |
| [x] | rolling_zscore | rolling_zscore | _ |
| [x] | rolling_skew | rolling_skew | _ |
| [x] | rolling_kurtosis | rolling_kurtosis | _ |
| [x] | rolling_cov | rolling_cov | _ |
| [x] | rolling_winsorize | rolling_winsorize | _ |
| [x] | ewm_var | ewm_var | _ |
| [x] | ewm_std | ewm_std | _ |
| [x] | ewm_cov | ewm_cov | _ |
| [x] | ewm_corr | ewm_corr | _ |
| [x] | drawdown | drawdown | _ |
| [x] | cummax | cummax | _ |
| [x] | cummin | cummin | _ |
| [x] | rolling_sharpe | rolling_sharpe | _ |
| [x] | rolling_sortino | rolling_sortino | _ |
| [x] | rolling_calmar | rolling_calmar | _ |
| [x] | hma | hma | _ |
| [x] | vwma | vwma | _ |
| [x] | zlema | zlema | _ |
| [x] | alma | alma | _ |
| [x] | tsi | tsi | _ |
| [x] | awesome_oscillator | awesome_oscillator | _ |
| [x] | fisher_transform | fisher_transform | _ |
| [x] | keltner_channels | keltner_channels | _ |
| [ ] | donchian_channels | donchian_channels | _ |
| [x] | chaikin_volatility | chaikin_volatility | _ |
| [x] | ulcer_index | ulcer_index | _ |
| [ ] | rolling_vwap | rolling_vwap | _ |
| [x] | force_index | force_index | _ |
| [x] | ease_of_movement | ease_of_movement | _ |
| [x] | crossover | crossover | _ |
| [x] | crossunder | crossunder | _ |
| [x] | rising | rising | _ |
| [x] | falling | falling | _ |
| [x] | higher_high | higher_high | _ |
| [x] | lower_low | lower_low | _ |
| [x] | inside_bar | inside_bar | _ |
| [x] | outside_bar | outside_bar | _ |
| [x] | gap_up | gap_up | _ |
| [x] | gap_down | gap_down | _ |
| [x] | bars_since | bars_since | _ |
| [x] | value_when | value_when | _ |
| [x] | highest_since | highest_since | _ |
| [x] | lowest_since | lowest_since | _ |
| [x] | signal_delay | signal_delay | _ |
| [ ] | swing_highs_lows | swing_highs_lows | _ |
| [x] | hedge_ratio | hedge_ratio | _ |
| [x] | rolling_entropy | rolling_entropy | _ |
| [x] | rolling_autocorr | rolling_autocorr | _ |
| [x] | hurst | hurst | _ |
| [x] | fractal_dimension | fractal_dimension | _ |
| [x] | rolling_alpha | rolling_alpha | _ |
| [x] | rolling_information_ratio | rolling_information_ratio | _ |
| [x] | fvg | fvg | _ |
| [x] | bos_choch | bos_choch | _ |
| [ ] | ob | ob | _ |
| [ ] | liquidity | liquidity | _ |
| [ ] | equal_highs_lows | equal_highs_lows | _ |
| [ ] | previous_high_low | previous_high_low | _ |
| [ ] | sessions | sessions | _ |
| [ ] | retracements | retracements | _ |
| [ ] | premium_discount | premium_discount | _ |
| [ ] | supertrend | supertrend | _ |
| [ ] | ichimoku | ichimoku | _ |
| [ ] | squeeze | squeeze | _ |
| [ ] | schaff_trend_cycle | schaff_trend_cycle | _ |
| [ ] | vortex | vortex | _ |
| [ ] | kst | kst | _ |
| [ ] | mass_index | mass_index | _ |
| [ ] | dpo | dpo | _ |
| [ ] | cmf | cmf | _ |
| [ ] | kvo | kvo | _ |
| [ ] | vpt | vpt | _ |
| [ ] | nvi | nvi | _ |
| [ ] | pvi | pvi | _ |
| [ ] | mcginley | mcginley | _ |
| [ ] | vidya | vidya | _ |
| [ ] | laguerre_rsi | laguerre_rsi | _ |
| [ ] | rmi | rmi | _ |
| [ ] | jma | jma | _ |
| [ ] | ssl_channel | ssl_channel | _ |
| [ ] | pmax | pmax | _ |
| [ ] | td_sequential | td_sequential | _ |
| [ ] | even_better_sinewave | even_better_sinewave | _ |
| [ ] | fib_retracement | fib_retracement | _ |
| [ ] | heikin_ashi | heikin_ashi | _ |
| [ ] | anchored_vwap | anchored_vwap | _ |
| [ ] | pivot_points | pivot_points | _ |
| [ ] | opening_range | opening_range | _ |
| [ ] | session_volume_levels | session_volume_levels | _ |
| [ ] | parkinson | parkinson | _ |
| [ ] | garman_klass | garman_klass | _ |
| [ ] | rogers_satchell | rogers_satchell | _ |
| [ ] | gk_yang_zhang | gk_yang_zhang | _ |
| [ ] | yang_zhang | yang_zhang | _ |
| [ ] | ts_rank | ts_rank | _ |
| [ ] | signedpower | signedpower | _ |
| [ ] | kalman_hedge_ratio | kalman_hedge_ratio | _ |
| [ ] | ou_half_life | ou_half_life | _ |
| [ ] | spread_zscore | spread_zscore | _ |
| [ ] | cusum | cusum | _ |
| [ ] | frac_diff | frac_diff | _ |
| [ ] | amihud | amihud | _ |
| [ ] | roll_spread | roll_spread | _ |

## 3. Per-function review checklist

For every function claimed done, verify ALL of:

### 3.1 Implemented enough?

- [ ] Rust persistent state (`append` O(1), `reset`, `value`)
- [ ] Optimized bulk path (`extend_slice`-style; not per-bar `append` loop)
- [ ] Python class in `taflow` with `append/extend/value/compute/reset`
- [ ] Canonical taflow name per the map above; talib alias if applicable
- [ ] Lifecycle tests (construct, append, extend, compute, reset-replay)
- [ ] Oracle parity test (TA-Lib or the pinned reference impl from the
      recommend checklist); self-oracle (state-vs-batch + chunk
      invariance) where no reference exists
- [ ] Appears in `benches/bench.py` output (auto-discovered; run it) with a
      `reports/<FN>.md` + `.json`
- [ ] Same-size causal contract: N bars in → N values out, NaN warm-up,
      no lookahead, chunk-invariant

### 3.2 Optimized? (see `plans/optimize-methods.md`)

- [ ] No per-bar heap allocation; fixed ring buffers, not VecDeque scans
- [ ] Rolling extrema via monotonic deque (single-sided where only one
      side is consumed — §3.2); never O(period) rescans (§3.3)
- [ ] Recurrences use `mul_add`; bulk loops split warm-up prologue from
      branch-free steady state (§4.5); EMA chains fused in bulk (§4.3)
- [ ] Long-stream drift considered for `sum += new - old` accumulators
      (periodic reseed policy, §6.2)
- [ ] Bench evidence: S2 append latency flat in base size; bulk ops/s not
      below TA-Lib without a noted reason

### 3.3 API and typing

- [ ] **No parameter shadows a Python builtin or keyword.** Prefix with
      underscore: `_input`, `_open`, `_min`, `_max`, `_filter`. Applies to
      Python signatures and `#[pyo3(signature = ...)]`. (`high`, `low`,
      `close`, `volume` are fine — they shadow nothing.)
- [ ] Full static type hints on every public Python signature (inputs
      `ArrayLike`/`np.ndarray`, scalars typed, return types explicit)
- [ ] Selector parameters use enums, not magic ints: `MaType` is an
      `enum.IntEnum` (exported from `taflow`), mirrored by a Rust `MaType`
      enum. `taflow.talib` still accepts raw ints for TA-Lib compat and
      converts at the boundary.

### 3.4 Docs and style

- [ ] Rust: `///` doc comments on every public item — one-line summary,
      formula/definition, warm-up rule, complexity, reference matched
- [ ] Python: docstring on every public class/function (NumPy style) with
      the same content; module docstring per file
- [ ] Multi-line formatting only — `rustfmt` and `black`-clean; never
      `def f(self,x):self._s.append(x);return self` one-liners
- [ ] **One function, one file**: Rust
      `crates/taflow-core/src/stream/<name>.rs` with a single public
      state; `mod.rs` contains only `mod`/`pub use` lines. Python
      `python/taflow/<name>.py`, one indicator per file.

## 4. Known debt (found in review, 2026-08-07 — fix as encountered)

1. `crates/taflow-core/src/stream/mod.rs` defines ~31 states inline
   (Mom/Roc*/Max/Min/Sum/Minmax*/Midpoint/Midprice/Var/Stddev/Avgdev/
   Correl/Beta/Linearreg*/Ad/Adosc/Obv/Bop/Willr/Aroon*/Atr/Natr/Trange/
   math ops) — violates one-function-one-file; split them out.
2. One-liner style + builtin-shadowing `open` parameter in generated
   pattern wrappers (e.g. `python/taflow/abandoned_baby.py`) — violates
   §3.3/§3.4; regenerate with `_open` and multi-line bodies.
3. `MA_Type` in `taflow.talib` is a bare int class → replace with
   `MaType(IntEnum)` per §3.3.
4. TA-Lib compat gaps found by the benchmark: 7 patterns reject the
   `penetration` kwarg (CDLABANDONEDBABY, CDLDARKCLOUDCOVER,
   CDLEVENINGDOJISTAR, CDLEVENINGSTAR, CDLMATHOLD, CDLMORNINGDOJISTAR,
   CDLMORNINGSTAR).
5. Pattern logic disagrees with TA-Lib on real data for: CDL3LINESTRIKE,
   CDLADVANCEBLOCK, CDLGAPSIDESIDEWHITE, CDLHIKKAKEMOD, CDLLADDERBOTTOM,
   CDLTRISTAR, CDLUNIQUE3RIVER — diff against TA-Lib C source.
6. Statistics drift at 100k bars (STDDEV/VAR/CORREL/LINEARREG_SLOPE/
   LINEARREG_ANGLE ~1e-9..3e-7; CCI state-path only) — needs the parity
   contract decision + accumulator reseeding (optimize-methods §6.1/§6.2).

## 5. How to run the review

```bash
# build current code
source .venv/bin/activate && maturin develop --release -m crates/taflow-python/Cargo.toml
# correctness + performance evidence for changed functions
python benches/bench.py <FN...> --quick
# style
cargo fmt --check && black --check python/
```

Then walk §3 for each function touched; update the relevant checklist in
`plans/` and this file's §4 as debt is fixed or found.
