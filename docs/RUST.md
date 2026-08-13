# Rust API

The `taflow` crate contains the canonical persistent indicator states. The
`taflow-metrics` crate contains persistent strategy, risk, and portfolio
metrics. Python delegates to these Rust implementations; it does not maintain a
second calculation path.

## Add the crates

Until the crates are published, use the repository or a local checkout:

```toml
[dependencies]
taflow = { git = "https://github.com/kimnt93/taflow" }
taflow-metrics = { git = "https://github.com/kimnt93/taflow" }

# Local development alternative:
# taflow = { path = "crates/taflow-core" }
# taflow-metrics = { path = "crates/taflow-metrics" }
```

Constructors validate configuration and return `TaResult<Self>` or
`MetricResult<Self>`.

## Reference map

| Area | Reference |
|---|---|
| Indicator classes | [All 393 full class names, inputs, configuration, outputs, and oracle mappings](INDICATORS.md) |
| Metric classes | [All 57 full class names, complete Python defaults, explicit Rust parameters, inputs, outputs, and definitions](METRICS.md) |
| Technical analysis streaming | [Indicator lifecycle](#indicator-lifecycle) |
| Metric streaming | [Metric streaming](#metric-streaming) |
| Technical analysis pipelines | [Technical analysis pipelines](#technical-analysis-pipelines) |
| Metric pipelines | [`MetricPipeline`](#metricpipeline) |
| Data input and output | [`Vec<f64>`, Apache Arrow, and Polars](#data-input-and-output) |

## Indicator lifecycle

Rust follows the same separation as Python: constructors accept configuration,
while slices and scalar observations are supplied afterward.

```rust
use taflow::indicators::ExponentialMovingAverage;
use taflow::stream::StreamingIndicator;

fn ema(close: &[f64], next_close: f64) -> taflow::TaResult<()> {
    let mut state = ExponentialMovingAverage::new(20)?;

    let mut aligned = Vec::with_capacity(close.len());
    state.extend_slice_into(close, &mut aligned);

    let next = state.append(next_close);
    let latest = state.value();
    assert_eq!(next, latest);

    state.reset();
    assert_eq!(state.value(), None);
    Ok(())
}
```

`StreamingIndicator` supplies the common single-input lifecycle:

```rust
pub trait StreamingIndicator {
    type Output: Copy;

    fn append(&mut self, input: f64) -> Option<Self::Output>;
    fn value(&self) -> Option<Self::Output>;
    fn reset(&mut self);
    fn extend_slice_into(&mut self, inputs: &[f64], output: &mut Vec<f64>)
    where
        Self::Output: Into<f64>;
}
```

The excerpt shows the practical contract; consult the crate source or generated
Rust API documentation for exact trait bounds and default implementation.

## Why Rust has no indicator `compute()`

Core indicator states retain bounded recurrence state rather than an unbounded
output history. Bulk methods write aligned results into vectors owned by the
caller, and `value` exposes the latest result. The Python binding owns its
output cache so Python can offer `compute()` without making every Rust state
retain all prior output.

This gives Rust callers explicit control over allocation and reuse:

```rust
use taflow::indicators::SimpleMovingAverage;
use taflow::stream::StreamingIndicator;

fn chunks(close: &[f64]) -> taflow::TaResult<Vec<f64>> {
    let mut state = SimpleMovingAverage::new(10)?;
    let mut output = Vec::with_capacity(close.len());

    let split = close.len().min(100);
    state.extend_slice_into(&close[..split], &mut output);
    state.extend_slice_into(&close[split..], &mut output);
    Ok(output)
}
```

The two chunks leave the same state and bits as one full slice or chronological
calls to `append`.

## Multiple inputs

Multi-input states expose typed inherent methods. They validate aligned lengths
before mutation and write `f64::NAN` at warm-up positions:

```rust
use taflow::indicators::AverageTrueRange;

fn atr(high: &[f64], low: &[f64], close: &[f64]) -> taflow::TaResult<Vec<f64>> {
    let mut state = AverageTrueRange::new(14)?;
    let mut output = Vec::with_capacity(close.len());
    state.extend_slices_into(high, low, close, &mut output)?;
    Ok(output)
}
```

Method names reflect arity:

- `extend_slice_into` for the shared single-input trait path;
- `extend_slices_into` for inherent multi-input or multi-output paths;
- a few specialized states expose an optimized `extend_slice` that returns its
  aligned values directly.

## Multiple outputs

Scalar multi-output values use named structs rather than unlabeled tuples.
Bulk paths receive one caller-owned vector per output:

```rust
use taflow::indicators::BollingerBands;
use taflow::stream::StreamingIndicator;
use taflow::MaType;

fn bands(close: &[f64]) -> taflow::TaResult<()> {
    let mut bands = BollingerBands::new(
        20,
        2.0,
        2.0,
        MaType::SimpleMovingAverage,
    )?;
    let mut upper = Vec::new();
    let mut middle = Vec::new();
    let mut lower = Vec::new();

    bands.extend_slices_into(close, &mut upper, &mut middle, &mut lower);

    if let Some(value) = bands.value() {
        println!("{} {} {}", value.upper, value.middle, value.lower);
    }
    Ok(())
}
```

## Warm-up and errors

- `append` and `value` return `None` while a scalar state is warming up.
- Bulk vectors contain `f64::NAN` at aligned warm-up positions.
- Invalid periods and configuration return `TaError`.
- Misaligned slice inputs return `TaError::LengthMismatch` when the method is
  fallible.
- `reset` restores fresh behavior while retaining reusable allocations owned by
  the state.

Use `?` to propagate errors or match on `TaError` when an application needs
parameter-specific handling.

## Metric reference

Metric constructors also contain configuration only. A `from_*` method binds
the semantic input domain and ingests the initial slice:

```rust
use taflow_metrics::metrics::SharpeRatio;
use taflow_metrics::NanPolicy;

fn sharpe(returns: &[f64], next: f64) -> taflow_metrics::MetricResult<Option<f64>> {
    let mut state = SharpeRatio::new(252.0, 0.03, NanPolicy::Omit)?;
    state.from_returns(returns)?;
    state.append(next)?;
    Ok(state.compute())
}
```

All metric states are exported under `taflow_metrics::metrics::*`. The
generated [metric class reference](METRICS.md) lists all 57 full class names in
alphabetical order with complete Python defaults, explicit Rust constructor
and semantic-input parameters, outputs, minimum observations, and definitions.

### Metric streaming

Select the input domain once with `from_returns`, `from_log_returns`,
`from_equity`, or `from_pnl`. An empty initial slice creates a live state;
`append` adds one observation, `extend` adds a slice, `value` and `compute`
return the current result, and `reset` preserves configuration while restoring
fresh-state behavior.

```rust
use taflow_metrics::metrics::SharpeRatio;
use taflow_metrics::NanPolicy;

fn live_metric(returns: &[f64]) -> taflow_metrics::MetricResult<Option<f64>> {
    let mut sharpe_ratio = SharpeRatio::new(252.0, 0.03, NanPolicy::Omit)?;
    sharpe_ratio.from_returns(&[])?;

    for &period_return in returns {
        sharpe_ratio.append(period_return)?;
    }

    let current = sharpe_ratio.value();
    sharpe_ratio.reset();
    sharpe_ratio.extend(returns)?;
    assert_eq!(sharpe_ratio.compute(), current);
    Ok(current)
}
```

## Technical analysis pipelines

The Rust core currently exposes indicator states rather than a graph-owning
pipeline type. Compose full indicator classes directly and advance each state
in dependency order. The Python-only [`TAPipeline`](PIPELINES.md) adds named
sources, expressions, shared-node evaluation, and dictionary outputs over
these same native states.

```rust
use taflow::indicators::ExponentialMovingAverage;
use taflow::stream::StreamingIndicator;

fn moving_average_spread(close: &[f64]) -> taflow::TaResult<Vec<f64>> {
    let mut fast_moving_average = ExponentialMovingAverage::new(12)?;
    let mut slow_moving_average = ExponentialMovingAverage::new(26)?;
    let mut spread = Vec::with_capacity(close.len());

    for &price in close {
        let fast_value = fast_moving_average.append(price);
        let slow_value = slow_moving_average.append(price);
        spread.push(match (fast_value, slow_value) {
            (Some(fast_value), Some(slow_value)) => fast_value - slow_value,
            _ => f64::NAN,
        });
    }

    Ok(spread)
}
```

## `MetricPipeline`

The native `MetricPipeline` owns several configured metric states and fans out
one selected returns, log-returns, equity, or period-profit-and-loss stream.

```rust
use taflow_metrics::metrics::{SharpeRatio, SortinoRatio};
use taflow_metrics::{MetricPipeline, NanPolicy};

fn metric_report(
    returns: &[f64],
) -> taflow_metrics::MetricResult<Vec<(String, Option<f64>)>> {
    let mut metric_pipeline = MetricPipeline::new();
    metric_pipeline
        .add(
            "sharpe_ratio",
            SharpeRatio::new(252.0, 0.03, NanPolicy::Omit)?,
        )?
        .add(
            "sortino_ratio",
            SortinoRatio::new(252.0, 0.0, NanPolicy::Omit)?,
        )?
        .from_returns(returns)?;

    Ok(metric_pipeline
        .compute()
        .into_iter()
        .map(|(name, value)| (name.to_owned(), value))
        .collect())
}
```

See the [`MetricPipeline` guide](METRIC_PIPELINE.md) for input-domain rules,
supported metric classes, continuation, and reset behavior.

## Data input and output

The core crates deliberately depend on slices and caller-owned `Vec` output,
so applications can interoperate with their chosen container libraries without
making Polars or Apache Arrow mandatory TAFlow dependencies.

### Rust vectors

Borrow a `Vec<f64>` as a slice and write directly into another reusable
`Vec<f64>`:

```rust
use taflow::indicators::SimpleMovingAverage;
use taflow::stream::StreamingIndicator;

fn vector_output(close: &Vec<f64>) -> taflow::TaResult<Vec<f64>> {
    let mut simple_moving_average = SimpleMovingAverage::new(10)?;
    let mut output = Vec::with_capacity(close.len());
    simple_moving_average.extend_slice_into(close.as_slice(), &mut output);
    Ok(output)
}
```

### Apache Arrow

A null-free `Float64Array` exposes its values as a contiguous slice. Moving the
output `Vec<f64>` into a `Float64Array` reuses its allocation.

```rust
use arrow_array::{Array, Float64Array};
use taflow::indicators::SimpleMovingAverage;
use taflow::stream::StreamingIndicator;

fn arrow_output(close: &Float64Array) -> taflow::TaResult<Float64Array> {
    assert_eq!(close.null_count(), 0, "TAFlow inputs must not contain Arrow nulls");

    let mut simple_moving_average = SimpleMovingAverage::new(10)?;
    let mut output = Vec::with_capacity(close.len());
    simple_moving_average.extend_slice_into(close.values(), &mut output);
    Ok(Float64Array::from(output))
}
```

Use Arrow nulls only after defining an application-level null policy. TAFlow
uses `f64::NAN` for aligned indicator warm-up, which is distinct from an Arrow
validity bitmap.

### Polars

Rechunk a `Float64` `Series` before borrowing a contiguous slice. `cont_slice`
rejects nulls or a non-contiguous layout; constructing the result `Series` from
the output vector transfers it to Polars.

```rust
use polars::prelude::*;
use taflow::indicators::SimpleMovingAverage;
use taflow::stream::StreamingIndicator;

fn polars_output(close: &Series) -> PolarsResult<Series> {
    let close = close.rechunk();
    let close = close.f64()?;
    let values = close.cont_slice()?;

    let mut simple_moving_average =
        SimpleMovingAverage::new(10).expect("the fixed period is valid");
    let mut output = Vec::with_capacity(values.len());
    simple_moving_average.extend_slice_into(values, &mut output);
    Ok(Series::new("simple_moving_average_10".into(), output))
}
```

Polars and Apache Arrow are application dependencies, not features of the
`taflow` or `taflow-metrics` crates. See the current
[Polars `Series` documentation](https://docs.rs/polars/latest/polars/series/struct.Series.html)
and [Apache Arrow `Float64Array` documentation](https://docs.rs/arrow-array/latest/arrow_array/type.Float64Array.html)
for container-specific version details.

## Modules and names

- Indicator states: `taflow::indicators::*`
- Shared single-input trait: `taflow::stream::StreamingIndicator`
- Moving-average selection: `taflow::MaType`
- Indicator errors: `taflow::{TaError, TaResult}`
- Metrics: `taflow_metrics::metrics::*`
- Metric pipeline: `taflow_metrics::MetricPipeline`
- Metric errors and input policy: `taflow_metrics::{MetricError, MetricResult,
  NanPolicy}`

Canonical names use complete descriptive words and map one-to-one to files in
`crates/taflow-core/src/indicators/`. The [indicator catalog](INDICATORS.md)
provides the shared class inventory and external oracle mapping; Rust method
signatures are authoritative in the corresponding implementation module.

## Build, test, and benchmark

```bash
cargo test --workspace
cargo fmt --all --check
cargo bench -p taflow
```

Repository-wide correctness and cross-library benchmarks are driven through the
canonical Python boundary so each Rust kernel is checked together with its
binding and public adapter:

```bash
make check
make bench
```

See [correctness](../verify/CORRECTNESS.md),
[benchmark](../verify/BENCHMARK.md), and [performance](PERFORMANCE.md) for the
current evidence and optimization contract.
