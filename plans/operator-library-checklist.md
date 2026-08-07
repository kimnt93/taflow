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
- [x] cumsum / cumprod

## Rolling operators

- [x] rolling_median / rolling_mode
- [x] rolling_quantile / rolling_percentile / rolling_rank / rolling_zscore
- [x] rolling_skew / rolling_kurtosis / rolling_iqr
- [x] rolling_cov (plain covariance; TA-Lib has only CORREL/BETA)
- [x] rolling_winsorize
- [x] rolling_apply (explicitly non-streaming unless an incremental reducer is supplied)

## Exponentially weighted operators

- [ ] ewm_var / ewm_std
- [ ] ewm_cov / ewm_corr

## Return and risk features

- [ ] drawdown (needs a running-max state; also expose cummax / cummin as
      the underlying primitives)
- [ ] rolling_sharpe / rolling_sortino / rolling_calmar

## Extended trend indicators

- [ ] HMA / VWMA / ZLEMA / ALMA

## Extended momentum and oscillators

- [ ] TSI
- [ ] Awesome Oscillator
- [ ] Fisher Transform

## Extended volatility indicators

- [ ] Keltner Channels
- [ ] Donchian Channels (3 outputs: upper/lower/mid; named channel, worth
      more than its MAX/MIN composition)
- [ ] Chaikin Volatility
- [ ] Ulcer Index

## Extended volume indicators

- [ ] VWAP (rolling; session/anchored variant is in
      `recommend-functions-checklist.md` P4)
- [ ] Force Index
- [ ] Ease of Movement

## Signal and strategy helpers

- [ ] crossover / crossunder (aliases: cross_up / cross_down; plus plain
      `cross` = either direction)
- [ ] rising / falling
- [ ] higher_high / lower_low / inside_bar / outside_bar
- [ ] gap_up / gap_down
- [ ] bars_since / value_when
- [ ] highest_since / lowest_since
- [ ] signal_delay / position_hold / entry_exit

## Extended pattern recognition

- [ ] swing_high / swing_low (causal confirmation-lag semantics — shares
      the P0 swing state in `recommend-functions-checklist.md`; the
      textbook centered-window definition is lookahead and is not
      implemented. `pivot_high` / `pivot_low` are aliases of the same
      state, not separate functions)

## Cross-series and quantitative features

Two aligned input series are within the contract (like BETA/CORREL).

- [ ] hedge_ratio (rolling OLS slope of y on x price levels; distinct from
      TA-Lib BETA, which regresses percent returns)
- [ ] rolling_entropy / rolling_autocorr
- [ ] hurst / fractal_dimension (rolling-window variants)
- [ ] rolling_alpha / rolling_information_ratio

## Execution and adapters

- [ ] indicator pipeline with one dispatch per input bar
- [ ] dependency graph and common-subexpression sharing
- [ ] expression engine
- [ ] NumPy input/output adapter performance tests
- [ ] Arrow adapter feature
- [ ] Polars adapter feature
- [ ] Python list adapter and conversion benchmark

## Implementation gates for every new operator

- [ ] numerical definition and warm-up rule documented
- [ ] batch implementation and property tests
- [ ] stateful implementation when mathematically possible
- [ ] NumPy/Python API test
- [ ] checklist updated

## Deferred benchmark and report gates

- [ ] dataset-size benchmarks (1K, 10K, 100K, 1M)
- [ ] continuous-backfill and streaming latency benchmarks
- [ ] per-function reports and aggregate validation report

## Removed entries (do not re-propose)

### Duplicates of TA-Lib functions (already in `full-ta-checklist.md`)

| Removed name | It is exactly | 
|---|---|
| rolling_sum / rolling_mean | SUM / SMA |
| rolling_min / rolling_max | MIN / MAX |
| rolling_argmin / rolling_argmax | MININDEX / MAXINDEX |
| rolling_var / rolling_std | VAR / STDDEV |
| rolling_mad | AVGDEV |
| rolling_corr / rolling_beta | CORREL / BETA |
| ewm_mean | EMA |
| Wilder smoothing | EMA with `alpha = 1/period` (stays as an internal shared primitive, not a public operator) |
| diff / change | MOM |
| pct_change | ROCP |
| ratio | DIV |
| spread | SUB (the hedged spread is `SUB(y, hedge_ratio*x)` — a composition) |
| hl2 / median_price | MEDPRICE |
| hlc3 / typical_price | TYPPRICE |
| ohlc4 | AVGPRICE |
| weighted_close | WCLPRICE |
| true_range | TRANGE |
| log_price | LN |
| return / rolling_return | ROCP(1) / ROCP(n) |
| LSMA (least-squares MA) | LINEARREG |
| engulfing / doji / pinbar | CDLENGULFING / CDLDOJI / CDLHAMMER + CDLSHOOTINGSTAR (the 61 CDL patterns cover these) |

Descriptive aliases for TA-Lib names (e.g. `typical_price` → TYPPRICE) are
welcome as zero-cost Python-level re-exports, but they are naming, not
checklist items.

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
