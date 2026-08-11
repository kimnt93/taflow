# Oracle and benchmark research

Research date: 2026-08-11. Versions and commits below are pins for reproducible
verification, not runtime dependencies.

## Recommended reference stack

### 1. Empyrical Reloaded 0.5.12 — primary standard-metric oracle

- Package: [`empyrical-reloaded` 0.5.12 on PyPI](https://pypi.org/project/empyrical-reloaded/)
- Source: [`stefan-jansen/empyrical-reloaded`](https://github.com/stefan-jansen/empyrical-reloaded)
- Pin: tag `0.5.12`, commit
  `b767a023cbd23f47298c9a6868d8ce159cdbf609`
- License: Apache-2.0
- Relevant source:
  [`src/empyrical/stats.py`](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py)

Use it first for total/annual return, annual volatility, maximum drawdown,
Sharpe, Sortino, Calmar, Omega, downside risk, alpha, beta, capture ratios,
tail ratio, historical VaR, and historical conditional VaR/expected shortfall.
It is the best primary oracle because its functions accept non-cumulative
NumPy return arrays directly, expose annualization parameters, and have a
small calculation surface.

Important adapters:

- Empyrical Sharpe expects a per-period risk-free return. TAFlow accepts an
  annual effective rate; the verifier converts it before calling Empyrical.
- Empyrical returns NaN/Inf in several zero-denominator cases. Normalize only
  at the verification boundary to TAFlow's documented `None`/infinity rule.
- Empyrical ignores NaNs in many reductions. The verifier explicitly applies
  TAFlow's selected `nan_policy` first.
- Empyrical maximum drawdown and VaR are signed return-space values; retain
  those signs in TAFlow P1.

### 2. QuantStats 0.0.81 — primary trade/quality oracle, secondary return oracle

- Package: [`quantstats` 0.0.81 on PyPI](https://pypi.org/project/quantstats/)
- Source: [`ranaroussi/quantstats`](https://github.com/ranaroussi/quantstats)
- Pin: tag `v0.0.81`, commit
  `fbd10daed0227aa0d10da6513f1b15e7e98d7fae`
- License: Apache-2.0
- Relevant source:
  [`quantstats/stats.py`](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py)

Use it for win rate, average win/loss, payoff ratio, profit factor, streaks,
Kelly criterion, gain-to-pain ratio, common-sense ratio, recovery factor,
Ulcer Index, Ulcer Performance Index, information ratio, and R-squared.

QuantStats is not the first oracle for standard return metrics because many
functions call pandas-oriented `_prepare_returns`/`_prepare_prices` helpers and
some contain input heuristics. When a function offers `prepare_returns=False`,
the verifier must use it. Otherwise construct a pandas Series already known to
be returns and inspect the pinned implementation before accepting parity.

QuantStats 0.0.81 requires Python 3.10+. Keep it in the development verifier,
not the runtime package. TAFlow's published Python floor does not need to move
solely for an oracle.

### 3. NumPy/pandas — elementary estimator oracle

Use pinned NumPy/pandas expressions only when no higher-priority financial
library exposes the exact contract, such as tracking error, breakeven rate, or
gross/net P&L. The expression belongs in the metrics verification script, not
production Python and not the metric's unit test as a self-oracle.

Record the exact NumPy version and percentile `method`. A local formula is not
enough evidence for a named financial metric when Empyrical or QuantStats is
available.

### 4. vectorbt 1.1.0 — secondary cross-check and optional performance peer

- Package: [`vectorbt` 1.1.0 on PyPI](https://pypi.org/project/vectorbt/)
- Documentation: [returns accessor](https://vectorbt.dev/api/returns/accessors/)
  and [trade metrics](https://vectorbt.dev/api/portfolio/trades/)
- Source: [`polakowo/vectorbt`](https://github.com/polakowo/vectorbt)
- Pin: tag `v1.1.0`, commit
  `259d2d89fe2e7638baf3ca76c394937cd32b656d`
- License: Apache-2.0 with Commons Clause (not plain Apache-2.0)

vectorbt has broad return accessors, rolling metrics, Numba/Rust acceleration,
trade profit factor, and System Quality Number. Use it as a second opinion for
ambiguous results and as the primary System Quality Number oracle.

Do not make it the only oracle or copy source/formulas from it: the current
license has a Commons Clause, the dependency is heavy, and accessor setup/JIT
warm-up complicates fair small-input benchmarks. If benchmarked, report cold
and warm Numba paths separately.

### 5. PerformanceAnalytics 2.1.0 — specification cross-check

- Package: [CRAN `PerformanceAnalytics`](https://cran.r-project.org/package=PerformanceAnalytics)
- Manual: [PerformanceAnalytics reference PDF](https://stat.ethz.ch/CRAN/web/packages/PerformanceAnalytics/PerformanceAnalytics.pdf)
- Version: 2.1.0, published 2026-04-11
- License: GPL-2 or GPL-3

This R package has unusually broad and well-documented risk/performance
coverage, including Sharpe variants, downside deviation, drawdown tables,
Pain Index/Ratio, Treynor, VaR, expected shortfall, and conditional drawdown at
risk. Use it to settle definitions that the Python libraries do not cover.

It is not the default automated performance competitor: R process startup,
serialization, xts index handling, and GPL source constraints make that
comparison noisy. Never translate or copy its implementation into TAFlow.

### 6. Riskfolio-Lib 7.3.0 — advanced-risk cross-check

- Package: [`riskfolio-lib` 7.3.0 on PyPI](https://pypi.org/project/riskfolio-lib/)
- Source: [`dcajasn/Riskfolio-Lib`](https://github.com/dcajasn/Riskfolio-Lib)
- Documentation: [Riskfolio-Lib docs](https://riskfolio-lib.readthedocs.io/)
- License: BSD-3-Clause

Riskfolio-Lib covers a large family of downside, tail, and drawdown risk
measures. Use it later for parametric VaR/ES, conditional drawdown at risk, and
entropic risk. It is an optimization-focused, heavy dependency and is not a
fair primary speed baseline for simple scalar reducers.

## Libraries not selected as primary

- Original `quantopian/empyrical`: useful historical source, but Empyrical
  Reloaded is the maintained package and retains the familiar API.
- Pyfolio: it delegates many statistics to Empyrical and adds tear-sheet
  concerns, so it is not an independent numerical oracle for those metrics.
- `ffn` and portfolio report packages: convenient, but add another layer of
  preprocessing and do not improve oracle independence over the stack above.
- Young Rust metric crates (`quant-metrics`, `stochastic-rs-quant`, `quantix`):
  useful ecosystem comparisons, but currently less established than the
  pinned Python/R references. They can be added to a Rust-only exploratory
  benchmark, never used as the sole correctness proof.

## Oracle selection rule per metric

1. Use Empyrical Reloaded if it exposes the exact agreed definition.
2. Otherwise use QuantStats with preprocessing disabled or explicitly
   reproduced at the verification boundary.
3. Otherwise use vectorbt for its native returns/trades metric.
4. Otherwise use PerformanceAnalytics or Riskfolio-Lib as a cross-language
   specification oracle.
5. Use NumPy/pandas only for elementary reductions or as a second independent
   calculation.
6. If no source matches exactly, mark `VARIANT`, cite the nearest source, and
   explain the semantic difference. Never label lifecycle parity as external
   correctness.

## Correctness comparison matrix

Each registry entry records:

```text
TAFlow class
accepted input factory used by the test
oracle package, version, function, and source URL
TAFlow parameters
oracle parameter transformation
NaN filtering/alignment rule
sign/output normalization
minimum sample and zero-denominator behavior
tolerance
MATCH or VARIANT with reason
```

The TAFlow side is always `CanonicalMetric.from_*(...).compute()`. Do not call
a Rust helper, Python reference formula, or free-function compatibility layer
as the actual value.

## Performance comparison plan

Only run this after explicit benchmark authorization and after correctness for
the exact parameter row passes.

### Published comparator

Use the same highest-priority library selected for correctness:

- Empyrical for standard return/risk metrics.
- QuantStats for its P&L/trade-quality metrics.
- vectorbt for System Quality Number.

Do not publish a speedup versus NumPy if correctness is claimed against
Empyrical, and do not switch to a slower library merely to inflate a ratio.

### Workloads

- Valid return arrays of 1K, 10K, 100K, and 1M observations.
- Constant, alternating, skewed, fat-tailed, autocorrelated, and drawdown-heavy
  deterministic datasets.
- NumPy contiguous float64 end-to-end construction plus `compute()`.
- Python list, pandas, Polars, and Arrow conversion reported separately.
- Empty-state full `extend`, chunks of 32/1K, warmed continuation, and scalar
  append latency.
- Repeated cached `compute()` for O(1) metrics and dirty `compute()` for exact
  quantiles.
- Independent metric objects to expose construction overhead.
- Optional multi-metric orchestration only after it exists canonically.

### Measurement rules

- Pin CPU, OS, compiler, Rust flags, Python, NumPy, and oracle versions in the
  report.
- Build TAFlow in release mode and verify the loaded extension path/version.
- Warm both implementations; report median and a dispersion statistic across
  repeated samples. Keep cold-start numbers separate.
- Include input conversion in the public Python benchmark. Add a separate
  native/core benchmark to explain kernel cost, never substitute it for the
  public result.
- For vectorbt/Numba, report first-call compile time separately from warm
  execution.
- Report peak allocation/memory for exact tail metrics in addition to time.
- Correctness failure suppresses the speedup row.

## Research-based cautions

- “Sharpe” is not a single formula unless annualization, risk-free conversion,
  return type, and degrees of freedom are fixed.
- Sortino libraries differ on whether downside deviation averages over all
  observations or only negative observations. TAFlow follows Empyrical's
  lower-partial-moment-over-all-observations definition.
- Calmar libraries differ on arithmetic versus compounded annual return and on
  drawdown sign. TAFlow uses CAGR / `abs(maximum_drawdown)`.
- Information ratio is often reported annualized, while Empyrical
  `excess_sharpe` and QuantStats `information_ratio` are unannualized. TAFlow
  exposes one class with an annualization parameter and transforms the oracle.
- QuantStats return/price heuristics are convenience behavior, not a semantic
  contract. TAFlow uses named factories instead.
- P&L cannot become a return without a capital base. `from_pnl` therefore
  requires initial equity and processes P&L chronologically.
- Exact historical quantiles are not bounded-memory streaming reducers. State
  and benchmark reports must say so plainly.
