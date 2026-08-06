# TAFlow operator-library checklist

This is the post-TA-Lib extension inventory derived from the shared design
discussion. It is deliberately separate from `full-ta-checklist.md`: classic
TA-Lib compatibility is implemented first. During the current phase, a check
means Rust/Python implementation and tests exist; benchmarks and reports are a
later pass.

## Core series operations

- [ ] shift / lag / lead
- [ ] diff / change / pct_change / log_return
- [ ] cumsum / cumprod
- [ ] clip / fill_nan / replace_inf
- [ ] where / sign / normalize / standardize

## Rolling operators

- [x] rolling_sum / rolling_mean / rolling_min / rolling_max
- [ ] rolling_argmin / rolling_argmax / rolling_range
- [ ] rolling_var / rolling_std / rolling_median / rolling_mode
- [ ] rolling_quantile / rolling_percentile / rolling_rank / rolling_zscore
- [ ] rolling_skew / rolling_kurtosis / rolling_iqr / rolling_mad
- [ ] rolling_cov / rolling_corr / rolling_beta
- [ ] rolling_gain / rolling_loss / ratio
- [ ] rolling_apply (explicitly non-streaming unless an incremental reducer is supplied)

## Exponentially weighted operators

- [ ] ewm_mean / ewm_var / ewm_std
- [ ] ewm_cov / ewm_corr
- [ ] Wilder smoothing primitive

## Price and return features

- [x] hl2 / median_price
- [x] hlc3 / typical_price
- [x] ohlc4
- [x] weighted_close
- [x] true_range
- [ ] log_price
- [ ] return / rolling_return / annual_return
- [ ] cumulative_return / drawdown
- [ ] rolling_sharpe / rolling_sortino / rolling_calmar

## Extended trend indicators

- [ ] HMA / VWMA / LSMA / ZLEMA / ALMA

## Extended momentum and oscillators

- [ ] TSI
- [ ] Awesome Oscillator
- [ ] Fisher Transform

## Extended volatility indicators

- [ ] Keltner Channels
- [ ] Donchian Channels
- [ ] Chaikin Volatility
- [ ] Ulcer Index

## Extended volume indicators

- [ ] VWAP / VWMA
- [ ] Force Index
- [ ] Ease of Movement

## Signal and strategy helpers

- [ ] cross / cross_up / cross_down
- [ ] crossover / crossunder
- [ ] rising / falling
- [ ] higher_high / lower_low / inside_bar / outside_bar
- [ ] bars_since / value_when
- [ ] highest_since / lowest_since
- [ ] turning_point
- [ ] signal_delay / position_hold / entry_exit

## Extended pattern recognition

- [ ] engulfing / pinbar / doji
- [ ] gap_up / gap_down
- [ ] swing_high / swing_low
- [ ] pivot_high / pivot_low

## Cross-series and quantitative features

- [ ] spread / hedge_ratio / cointegration
- [ ] rank / zscore / winsorize / neutralize / group_rank
- [ ] industry_rank / market_rank
- [ ] rolling_entropy / rolling_autocorr
- [ ] hurst / fractal_dimension
- [ ] rolling_alpha / rolling_information_ratio / rolling_ic

## Factor library

- [ ] Alpha101
- [ ] Alpha158
- [ ] licensed WorldQuant-style operators
- [ ] academic quantitative factors

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
