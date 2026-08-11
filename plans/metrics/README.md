# TAFlow metrics expansion plan

Research date: 2026-08-11.

## Decision

Add portfolio, strategy, and trade metrics as a separate product domain. Do
not add whole-history scalar metrics to `taflow.indicators`, and do not replace
the existing causal rolling indicator classes such as `RollingSharpe`,
`RollingSortino`, `RollingCalmar`, `RollingInformationRatio`, or `Drawdown`.

The proposed public surface is:

```python
from taflow.metrics import SharpeRatio

metric = SharpeRatio.from_returns(
    returns,
    periods_per_year=252.0,
    annual_risk_free_rate=0.02,
)
value = metric.compute()

same_metric = SharpeRatio.from_equity(equity, periods_per_year=252.0)
from_pnl = SharpeRatio.from_pnl(
    period_pnl,
    initial_equity=100_000.0,
    periods_per_year=252.0,
)
```

This is preferable to `SharpeRatio(series, series_type=...)`. A named factory
makes the financial meaning of the series reviewable at the call site and
prevents a price, equity, return, cumulative P&L, or period-P&L series from
being silently interpreted as another kind.

`from_pnl` has one exact input meaning: each value is the non-cumulative P&L
earned during one observation period. A return-based class such as
`SharpeRatio` therefore requires `initial_equity` to convert P&L to returns. A
P&L-native class such as `GrossProfit` consumes those same period P&L values
directly and does not ask for capital it would ignore. A cumulative P&L series
must first be expressed as equity (`initial_equity + cumulative_pnl`) and
passed to `from_equity`; TAFlow must not guess which P&L convention the caller
intended.

## Package boundary

- Add a workspace crate named `taflow-metrics` (`taflow_metrics` in Rust).
- Keep one wheel/distribution. Bind the crate through the existing
  `taflow-python` extension as the native submodule `taflow._native.metrics`.
- Add the Python package `taflow.metrics`.
- Export metric classes from `taflow.metrics`, not from top-level `taflow`.
  This keeps the domain visible and avoids collisions with rolling indicators,
  especially the existing `taflow.UlcerIndex` indicator.
- Keep external oracle libraries in development/verification dependencies
  only. The runtime dependency remains NumPy.

## Non-negotiable implementation philosophy

- One metric, one descriptive CamelCase class, one implementation file in
  each layer, and one matching test file. No public snake-case metric
  functions and no duplicate batch formula.
- Rust owns input-domain conversion, validation, accumulation, formulas,
  warm-up, undefined-result handling, and scalar output. Python normalizes a
  supported container exactly once and delegates.
- Every class supports persistent `append`, `extend`, `value`, `compute`,
  `reset`, and `__len__`. The input factory fixes the meaning of later
  `append`/`extend` calls.
- `append` is allocation-free after construction for metrics that have a
  fixed-size sufficient statistic. Exact historical quantile metrics are a
  documented exception because they must retain observations.
- Batch and scalar replay must leave identical state and results. Use native
  contiguous loops, stable recurrences, and measured optimization. Do not
  change reduction order merely to claim vectorization.
- Correctness against the pinned external oracle is a gate before any
  performance claim.

## Plan documents

- [API and input contract](api-and-input-contract.md)
- [Architecture and file layout](architecture.md)
- [Metric inventory and priority](metric-catalog.md)
- [Oracle and benchmark research](oracle-and-benchmark-research.md)
- [Testing and verification](testing-and-verification.md)
- [Implementation checklist](implementation-checklist.md)

## Recommended delivery order

1. Freeze the input, annualization, missing-data, sign, and undefined-result
   contracts.
2. Scaffold `taflow-metrics`, the native metrics submodule, Python package,
   and separate verification registry.
3. Implement the first essential return/drawdown batch in the order recorded
   in `metric-catalog.md`.
4. Add benchmark-relative metrics, then period/trade-P&L metrics.
5. Add exact tail metrics and advanced research metrics only after their
   estimator contract is explicit.
6. Add a multi-metric report/workspace only after individual metric classes
   are canonical and verified. It may orchestrate or share primitives; it may
   not contain a second implementation of any metric.

No benchmark command is authorized by this planning task. The benchmark plan
is ready for a later explicit authorization.
