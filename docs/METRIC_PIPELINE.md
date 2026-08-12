# Metric pipeline

`MetricPipeline` is a Rust-owned fan-out engine for computing several
whole-history metrics from one semantic input stream. It is useful when the
same P&L, equity, return, or log-return series feeds many metrics.

```python
import numpy as np
from taflow.metrics import MetricPipeline

period_pnl = np.array([500.0, -250.0, 300.0, 100.0])
capital = 100_000.0
returns = np.array([0.005, -0.0025, 0.003, 0.001])
log_returns = np.log1p(returns)
equity = capital * np.r_[1.0, np.cumprod(1.0 + returns)]
report = MetricPipeline.from_pnl(
    period_pnl,
    initial_equity=100_000.0,
    metrics=(
        "TotalReturn",
        "AnnualizedReturn",
        "SharpeRatio",
        "SortinoRatio",
        "MaximumDrawdown",
        "CalmarRatio",
    ),
    periods_per_year=252.0,
    annual_risk_free_rate=0.03,
)

values = report.compute()
values["SharpeRatio"]
```

## What is shared

The native pipeline owns one `MetricInputState`. Each input observation passes
through that converter exactly once:

- simple returns are validated by the shared converter before fan-out;
- log returns call `expm1` once;
- equity levels produce one causal simple return;
- period P&L updates one capital path and produces one causal return.

The resulting simple return is dispatched inside Rust to the selected
canonical metric states. Python converts the input container once and receives
the final named scalar mapping. It does not loop over observations or perform
financial arithmetic.

Each metric still owns its canonical formula and sufficient statistics. The
pipeline does not duplicate formulas, combine unlike definitions, or replace
the standalone classes.

## Factories and lifecycle

The input domain is explicit:

```text
MetricPipeline.from_returns(returns, metrics=(...))
MetricPipeline.from_log_returns(log_returns, metrics=(...))
MetricPipeline.from_equity(equity, metrics=(...))
MetricPipeline.from_pnl(pnl, initial_equity=capital, metrics=(...))
```

An empty input constructs streaming state. `append`, `extend`, and `reset`
return the same Python adapter. `value` and `compute` return an ordered
`dict[str, float | None]`. `len(pipeline)` is the number of usable normalized
returns. The first equity level establishes a baseline and is not counted as a
return.

```python
live = MetricPipeline.from_pnl(
    [],
    initial_equity=100_000.0,
    metrics=("SharpeRatio", "MaximumDrawdown"),
)
feed = [500.0, -250.0, 300.0]
backfill = period_pnl
for event in feed:
    live.append(event)
current = live.value
live.reset().extend(backfill)
```

## Selection and configuration

`metrics=None` selects every compatible metric. Names must be unique canonical
class names; result order follows selection order. Inspect the exact native
list with `MetricPipeline.supported_metrics()`.

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
cannot safely share this converter. Use their standalone semantic factories;
the pipeline will not guess or reinterpret inputs.

## Performance model

Bulk `extend` releases the GIL and performs conversion plus fan-out in one
native loop. Fixed-state metrics retain O(1) memory. Exact historical quantile
and entropic metrics retain their documented histories. For one metric, the
standalone class remains the simpler API; the pipeline is intended for several
metrics sharing one input conversion and one native boundary crossing.
