# TAFlow operator-library checklist

This is the post-TA-Lib extension inventory derived from the shared design
discussion.  It is deliberately separate from `full-ta-checklist.md`: the
classic TA-Lib 161-function compatibility gate must pass first.  An unchecked
operator has not been designed, implemented, and tested yet.

## Core series operations

- [ ] shift / lag / lead
- [ ] diff / change / pct_change / log_return
- [ ] cumsum / cumprod
- [ ] clip / fill_nan / replace_inf
- [ ] where / sign / normalize

## Rolling operators

- [x] rolling_sum / rolling_mean / rolling_min / rolling_max
- [ ] rolling_argmin / rolling_argmax / rolling_range
- [ ] rolling_var / rolling_std / rolling_median
- [ ] rolling_quantile / rolling_rank / rolling_zscore
- [ ] rolling_skew / rolling_kurtosis / rolling_mad
- [ ] rolling_cov / rolling_corr / rolling_beta
- [ ] rolling_apply (explicitly non-streaming unless an incremental reducer is supplied)

## Exponentially weighted operators

- [ ] ewm_mean / ewm_var / ewm_std
- [ ] ewm_cov / ewm_corr
- [ ] Wilder smoothing primitive

## Signal and strategy helpers

- [ ] cross / cross_up / cross_down
- [ ] rising / falling
- [ ] higher_high / lower_low / inside_bar / outside_bar
- [ ] bars_since / value_when
- [ ] highest_since / lowest_since
- [ ] turning_point

## Cross-series and quantitative features

- [ ] spread / hedge_ratio / cointegration
- [ ] rank / zscore / winsorize / neutralize / group_rank
- [ ] rolling_entropy / rolling_autocorr
- [ ] hurst / fractal_dimension

## Execution and adapters

- [ ] indicator pipeline with one dispatch per input bar
- [ ] dependency graph and common-subexpression sharing
- [ ] NumPy input/output adapter performance tests
- [ ] Arrow adapter feature
- [ ] Polars adapter feature
- [ ] Python list adapter and conversion benchmark

## Gates for every new operator

- [ ] numerical definition and warm-up rule documented
- [ ] batch implementation and property tests
- [ ] stateful implementation when mathematically possible
- [ ] NumPy/Python API test
- [ ] dataset-size benchmark (1K, 10K, 100K, 1M)
- [ ] checklist and validation report updated
