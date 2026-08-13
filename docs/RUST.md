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

## Metrics

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

`MetricPipeline` owns several configured metrics and fans out one selected
returns, log-returns, equity, or period-P&L stream. See
[Metric pipeline](METRIC_PIPELINE.md) for its domain rules and Python mapping.

## Modules and names

- Indicator states: `taflow::indicators::*`
- Shared single-input trait: `taflow::stream::StreamingIndicator`
- Moving-average selection: `taflow::MaType`
- Indicator errors: `taflow::{TaError, TaResult}`
- Metrics: `taflow_metrics::metrics::*`
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
