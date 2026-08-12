# Metric pipeline

`MetricPipeline` coordinates configured whole-history metric instances under
caller-provided names. It is useful when the same P&L, equity, return, or
log-return series feeds several compatible metrics.

```python
import numpy as np
from taflow.metrics import (
    AnnualizedReturn, CalmarRatio, MaximumDrawdown, MetricPipeline,
    SharpeRatio, SortinoRatio, TotalReturn,
)

period_pnl = np.array([500.0, -250.0, 300.0, 100.0])
capital = 100_000.0
returns = np.array([0.005, -0.0025, 0.003, 0.001])
log_returns = np.log1p(returns)
equity = capital * np.r_[1.0, np.cumprod(1.0 + returns)]
report = MetricPipeline()
report.add("total", TotalReturn())
report.add("annual", AnnualizedReturn(252.0))
report.add("sharpe", SharpeRatio(252.0, 0.03))
report.add("sortino", SortinoRatio())
report.add("drawdown", MaximumDrawdown())
report.add("calmar", CalmarRatio(252.0))
report.from_pnl(period_pnl, initial_capital=capital)

values = report.compute()
values["sharpe"]
```

## Ownership and lifecycle

Each metric remains the sole owner of its native conversion and numerical
state. The Python pipeline performs container normalization and lifecycle
fan-out only; it performs no financial arithmetic.

The selected instance `from_*` method is called on every metric. Subsequent
`append` and `extend` calls keep that domain, and selecting a different domain
is rejected.

Each metric still owns its canonical formula and sufficient statistics. The
pipeline does not duplicate formulas, combine unlike definitions, or replace
the standalone classes.

## Input methods

The input domain is explicit:

```text
pipeline.from_returns(returns)
pipeline.from_log_returns(log_returns)
pipeline.from_equity(equity)
pipeline.from_pnl(pnl, initial_capital=capital)
```

An empty input constructs streaming state. `append`, `extend`, and `reset`
return the same Python adapter. `value` and `compute` return an ordered
`dict[str, float | None]`. `len(pipeline)` is the number of usable normalized
returns. The first equity level establishes a baseline and is not counted as a
return.

```python
live = MetricPipeline()
live.add("sharpe", SharpeRatio()).add("drawdown", MaximumDrawdown())
live.from_pnl([], initial_capital=100_000.0)
feed = [500.0, -250.0, 300.0]
backfill = period_pnl
for event in feed:
    live.append(event)
current = live.value
live.reset().extend(backfill)
```

## Selection and configuration

`add(name, metric)` requires a unique non-empty caller-provided name and a
configured metric instance. Metrics must be added before input is selected;
result order follows insertion order.

| Parameter | Default | Consumers |
|---|---:|---|
| `periods_per_year` | `252.0` | annualized return/risk metrics |
| `annual_risk_free_rate` | `0.0` | Sharpe, pain, modified/probabilistic Sharpe |
| `annual_required_return` | `0.0` | downside, Sortino, Omega |
| `annual_benchmark_sharpe_ratio` | `0.0` | probabilistic Sharpe |
| `cutoff` | `0.05` | historical, parametric, and entropic tail risk |
| `confidence_level` | `0.95` | modified Sharpe and conditional drawdown risk |
| `nan_policy` | `"omit"` | shared native input converter |

## Deliberate boundary

This pipeline accepts metrics whose canonical input is one normalized return
stream. It deliberately excludes paired benchmark metrics, raw P&L totals,
trade-only metrics, portfolio matrix/weight metrics, and metrics with required
metric-specific configuration such as `DeflatedSharpeRatio`. Those domains
cannot safely share this converter. Use their standalone semantic input methods;
the pipeline will not guess or reinterpret inputs.

## Performance model

Bulk `extend` releases the GIL and performs conversion plus fan-out in one
native loop. Fixed-state metrics retain O(1) memory. Exact historical quantile
and entropic metrics retain their documented histories. For one metric, the
standalone class remains the simpler API; the pipeline is intended for several
metrics sharing one input conversion and one native boundary crossing.
