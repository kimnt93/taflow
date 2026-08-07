# TAFlow operator-library checklist

This is a deferred, post-TA-Lib inventory derived from the shared design
discussion. It is outside the current unified realtime TA-Lib delivery and
must not block it. Its entries are retained for future consideration only;
benchmarks and reports are a later pass.

Every entry must satisfy the taflow contract: aligned time series in, same-
length series out, causal (bar `i` uses only bars `<= i`), chunk-invariant.

Additionally, an entry earns its place only if it is **not** already a
TA-Lib function under another name (those live in
`full-ta-checklist.md`) and **not** a trivial one-liner the user can write
with numpy on taflow outputs. Removed entries are recorded at the bottom so
they are not re-proposed later.

## Core series operations

- [x] lag (delay line; `shift` is its alias)
- [x] log_return (`ln(x_t / x_{t-n})`; kept because it is the standard
      return definition and warm-up/NaN handling should match the library,
      not ad-hoc numpy)
- [x] cumulative_sum / cumulative_product

## Rolling operators

- [x] rolling_median / rolling_mode
- [x] rolling_quantile / rolling_percentile / rolling_rank / rolling_zscore
- [x] rolling_skew / rolling_kurtosis / rolling_iqr
- [x] rolling_cov (plain covariance; TA-Lib has only CORREL/BETA)
- [x] rolling_winsorize
- [x] rolling_apply (explicitly non-streaming unless an incremental reducer is supplied)

## Exponentially weighted operators

- [x] ewm_var / ewm_std
- [x] ewm_cov / ewm_corr

## Return and risk features

- [x] drawdown (needs a running-max state; also expose cumulative_maximum / cumulative_minimum as
      the underlying primitives)
- [x] rolling_sharpe / rolling_sortino / rolling_calmar

## Extended trend indicators

- [x] hull_moving_average / volume_weighted_moving_average /
      zero_lag_exponential_moving_average / arnaud_legoux_moving_average

## Extended momentum and oscillators

- [x] true_strength_index
- [x] Awesome Oscillator
- [x] Fisher Transform

## Extended volatility indicators

- [x] Keltner Channels
- [x] Donchian Channels (3 outputs: upper/lower/mid; named channel, worth
      more than its MAX/MIN composition)
- [x] Chaikin Volatility
- [x] Ulcer Index

## Extended volume indicators

- [x] rolling_volume_weighted_average_price (session/anchored variant is in
      `recommend-functions-checklist.md` P4)
- [x] Force Index
- [x] Ease of Movement

## Signal and strategy helpers

- [x] crossover
- [x] crossunder (aliases: `cross_up` / `cross_down`)
- [x] cross (either direction)
- [x] rising
- [x] falling
- [x] higher_high
- [x] lower_low
- [x] inside_bar
- [x] outside_bar
- [x] gap_up
- [x] gap_down
- [x] bars_since
- [x] value_when
- [x] highest_since
- [x] lowest_since
- [x] signal_delay
- [x] position_hold
- [x] entry_exit

## Extended pattern recognition

- [x] swing_high / swing_low (causal confirmation-lag semantics — shares
      the P0 swing state in `recommend-functions-checklist.md`; the
      textbook centered-window definition is lookahead and is not
      implemented. `pivot_high` / `pivot_low` are aliases of the same
      state, not separate functions)

## Cross-series and quantitative features

Two aligned input series are within the contract (like BETA/CORREL).

- [x] hedge_ratio (rolling OLS slope of y on x price levels; distinct from
      TA-Lib BETA, which regresses percent returns)
- [x] rolling_entropy / rolling_autocorr
- [x] hurst / fractal_dimension (rolling-window variants)
- [x] rolling_alpha / rolling_information_ratio

## Execution and adapters

- [x] indicator pipeline with one dispatch per input bar (`taflow.Pipeline`)
- [x] dependency graph and common-subexpression sharing (identity-memoized nodes)
- [x] expression engine (`taflow.Expr` arithmetic composition)
- [x] NumPy input/output adapter performance checks (`benches/adapter_bench.py`;
      the historical test file was removed with the tests directory)
- [x] Arrow adapter feature (optional `pyarrow` extra, lazy import)
- [x] Polars adapter feature (optional `polars` extra, lazy import)
- [x] Python list adapter and conversion benchmark (`benches/adapter_bench.py`)

## Implementation gates for every new operator

For this execution/adapters entry, the gates below are satisfied against the
pipeline and adapter implementation; indicator-specific numerical oracles
remain covered by the operator tests.

- [x] numerical definition and warm-up rule documented (causal one-row
      dispatch and NaN warm-up are documented in `taflow.execution`)
- [x] batch implementation and property tests (`Pipeline.extend`, alignment,
      and chunk-invariance tests)
- [x] stateful implementation when mathematically possible (`append` and
      persistent indicator-node state)
- [x] NumPy/Python API smoke coverage (gateway conversion, pipeline
      append/extend/reset, chunk invariance, and optional-adapter error paths;
      the historical `tests/` tree was removed)
- [x] **all applicable review gates in `/CHECK.md` pass** (module placement,
      typed/documented public gateway, multi-line style, and diff/compile
      checks; rolling-indicator gates do not apply)
- [x] checklist updated

## Deferred benchmark and report gates

- [x] dataset-size benchmarks (1K, 10K, 100K, 1M)
- [x] continuous-backfill and streaming latency benchmarks (`adapter_bench.py`)
- [ ] per-function reports and aggregate validation report

## Renamed, not duplicated (one kernel, two surfaces — see `CHECK.md` §2)

These are NOT separate checklist items: each is one Rust kernel already in
`full-ta-checklist.md`, exposed under its canonical `taflow` name with the
TA-Lib uppercase name living only in `taflow.talib`. Per the rolling
rename rule, the un-prefixed name (`min`, `max`, `sum`, …) does not exist
in `taflow` at all.

| taflow canonical name | taflow.talib alias |
|---|---|
| rolling_min / rolling_max | MIN / MAX |
| rolling_sum | SUM |
| rolling_argmin / rolling_argmax | MININDEX / MAXINDEX |
| rolling_minmax / rolling_minmax_index | MINMAX / MINMAXINDEX |
| rolling_midpoint | MIDPOINT |
| rolling_var / rolling_std | VAR / STDDEV |
| rolling_avgdev | AVGDEV |
| rolling_corr / rolling_beta | CORREL / BETA |
| rolling_linreg (+ slope/intercept/angle, rolling_tsf) | LINEARREG* / TSF |
| median_price / typical_price / avg_price / weighted_close | MEDPRICE / TYPPRICE / AVGPRICE / WCLPRICE |
| true_range | TRANGE |
| sma / ema / … (moving averages keep indicator names) | SMA / EMA / … |

## Removed entries (do not re-propose)

### Duplicates of TA-Lib functions (already in `full-ta-checklist.md`)

| Removed name | It is exactly |
|---|---|
| ewm_mean | EMA |
| Wilder smoothing | EMA with `alpha = 1/period` (stays as an internal shared primitive, not a public operator) |
| diff / change | MOM |
| pct_change | ROCP |
| ratio | DIV |
| spread | SUB (the hedged spread is `SUB(y, hedge_ratio*x)` — a composition) |
| log_price | LN |
| return / rolling_return | ROCP(1) / ROCP(n) |
| LSMA (least-squares MA) | LINEARREG |
| engulfing / doji / pinbar | CDLENGULFING / CDLDOJI / CDLHAMMER + CDLSHOOTINGSTAR (the 61 CDL patterns cover these) |

### Trivial one-liners (self-computable with numpy on existing outputs)

- clip / fill_nan / replace_inf / where / sign — pointwise numpy
  (`np.clip`, `np.nan_to_num`, `np.where`, `np.sign`); no state, no warm-up
  semantics, nothing for taflow to add.
- rolling_range — `MAX - MIN` (or one MINMAX call).
- rolling_gain / rolling_loss — internal building blocks of RSI/CMO; as
  public functions they are `SMA(max(diff,0))` compositions.
- rolling_annualized_return — pointwise scaling of ROCP output.
- cumulative_return — `price / price[anchor] - 1`.
- cross_up / cross_down as separate entries — same function as
  crossover/crossunder (kept once under signal helpers).
- turning_point — same state as swing_high/swing_low (kept once under
  pattern recognition).

### Out of contract (violates same-size causal series semantics)

- `lead` — lookahead by definition (`x[i+k]` at bar `i`).
- global `normalize` / `standardize` — full-series statistics are
  non-causal; `rolling_zscore` covers the causal use case.
- `annual_return` as a scalar aggregate — not a series.
- `cointegration` — batch statistical test with a scalar result; point
  users to statsmodels.
- `neutralize` / `group_rank` / `industry_rank` / `market_rank` —
  cross-sectional operators requiring a universe snapshot per timestamp.
- `rolling_ic` — correlates a signal with *forward* returns; bar `i`
  depends on bars `> i`. Research-notebook metric, not a causal indicator.
