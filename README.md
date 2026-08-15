<p align="center">
  <h1 align="center">TAFlow</h1>
  <p align="center">
    Persistent technical analysis for Rust and Python, with O(1) per-tick
    updates where the formula permits.
  </p>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/indicators-393-blue" alt="393 indicators" />
  <img src="https://img.shields.io/badge/metrics-57-blue" alt="57 metrics" />
  <img src="https://img.shields.io/badge/TA--Lib_parity-161-blue" alt="161 TA-Lib functions" />
  <img src="https://img.shields.io/badge/correctness-393%2F393_MATCH-brightgreen" alt="393/393 externally matched" />
  <img src="https://img.shields.io/badge/unsafe-zero-brightgreen" alt="zero unsafe" />
  <img src="https://img.shields.io/badge/C_deps-zero-orange" alt="zero C dependencies" />
  <img src="https://img.shields.io/badge/license-MIT-lightgrey" alt="MIT" />
</p>

TAFlow provides 393 canonical indicator classes and 57 strategy and portfolio
metrics as native Rust states with Python adapters. It covers the complete
161-function TA-Lib surface plus rolling statistics, EWM operators, volatility
estimators, market structure, patterns, and signals—without linking to TA-Lib
or another C library.

## Why TAFlow

- **More coverage than TA-Lib:** 393 indicators versus TA-Lib's 161, plus 57
  strategy, risk, trade, and portfolio metrics.
- **Verified parity:** all 161 TA-Lib-equivalent indicators match TA-Lib in the
  [correctness report](verify/CORRECTNESS.md).
- **Faster than TA-Lib:** at 100k rows, TAFlow wins 130 of 161 comparisons and
  reaches up to **11.82× faster**. See the [benchmark report](verify/BENCHMARK.md).
- **Built for real time:** persistent Rust states use the same API for bulk
  history and per-tick updates, without recomputing the full series.

## Features

- First-class Rust crate and native-backed Python package.
- Configuration-only constructors: create state first, then ingest data with
  bulk `extend` methods or scalar `append`.
- Persistent, bounded Rust state with O(1) scalar updates where the formula
  permits it.
- Optimized bulk kernels that leave the same state as chronological scalar
  replay.
- Bitwise chunk invariance across cold bulk, chunked continuation, scalar
  continuation, and reset/replay.
- `None` during scalar warm-up and aligned `NaN` values in bulk histories.
- Typed Rust value structs and Python tuples for multi-output indicators.
- 393/393 external correctness matches against TA-Lib, Wickra,
  pandas-ta-classic, NumPy, or Smart Money Concepts.
- NumPy, Python list, pandas, Polars, and Arrow inputs at the Python boundary.
- Causal Python indicator graphs through `TAPipeline`.
- Persistent strategy and portfolio metrics, including `MetricPipeline`.
- Portable runtime CPU dispatch, zero `unsafe`, and zero runtime C
  dependencies.

## Install

Python 3.9 or newer:

```bash
pip install "git+https://github.com/kimnt93/taflow"
# or, from a checkout:
make install
```

Rust from the repository:

```toml
[dependencies]
taflow = { git = "https://github.com/kimnt93/taflow" }
taflow-metrics = { git = "https://github.com/kimnt93/taflow" }
```

The packages are not yet published to PyPI or crates.io. Building the Python
extension from source requires a Rust toolchain.

## Short examples

Python constructors contain configuration only; historical and live values use
the same state:

```python
from taflow import AverageTrueRange, ExponentialMovingAverage

ema = ExponentialMovingAverage(timeperiod=12).extend(close)
ema.append(next_close)
ema_history = ema.compute()
ema_latest = ema.value

atr = AverageTrueRange(timeperiod=14).extend(high, low, close)
atr_history = atr.compute()
```

Rust follows the same lifecycle. Core Rust writes aligned history into
caller-owned vectors and retains only the state needed for continuation:

```rust
use taflow::indicators::{AverageTrueRange, ExponentialMovingAverage};
use taflow::stream::StreamingIndicator;

fn calculate(
    high: &[f64],
    low: &[f64],
    close: &[f64],
    next_close: f64,
) -> taflow::TaResult<()> {
    let mut ema = ExponentialMovingAverage::new(12)?;
    let mut ema_history = Vec::new();
    ema.extend_slice_into(close, &mut ema_history);
    ema.append(next_close);
    let ema_latest = ema.value();

    let mut atr = AverageTrueRange::new(14)?;
    let mut atr_history = Vec::new();
    atr.extend_slices_into(high, low, close, &mut atr_history)?;
    Ok(())
}
```

See the [Python guide](docs/PYTHON.md) and [Rust guide](docs/RUST.md) for the
complete lifecycle, multi-input and multi-output examples, pipelines, metrics,
errors, and warm-up behavior.

## Python documentation

- [Python guide](docs/PYTHON.md) — installation, class lifecycle, outputs,
  metrics, and data adapters.
- [Indicator class reference](docs/INDICATORS.md) — all 393 full class names,
  ordered inputs, constructor configuration, outputs, and oracle mappings.
- [Metric class reference](docs/METRICS.md) — all 57 full class names with
  Python defaults, Rust parameters, semantic inputs, outputs, and definitions.
- [Streaming lifecycle](docs/STREAMING.md) — warm-up, continuation, chunk
  invariance, reset, and per-tick behavior.
- [`TAPipeline` reference](docs/PIPELINES.md) — causal graphs, expressions,
  evaluation, and limitations.
- [`MetricPipeline` reference](docs/METRIC_PIPELINE.md) — metric input
  domains, fan-out, lifecycle, and results.
- [Data input and output](docs/DATA.md) — NumPy, Python lists, pandas, Polars,
  Arrow, and custom adapters.

## Rust documentation

- [Rust guide](docs/RUST.md) — crate setup, lifecycle contracts, errors, and
  native integration.
- [Indicator class reference](docs/INDICATORS.md) — all 393 full class names,
  configuration, ordered inputs, outputs, and oracle mappings.
- [Metric class reference](docs/METRICS.md) — all 57 full class names with
  Python defaults, explicit Rust parameters, semantic inputs, and definitions.
- [Technical analysis streaming](docs/RUST.md#indicator-lifecycle) — scalar
  updates, bulk slices, warm-up, continuation, and caller-owned output.
- [Metric streaming](docs/RUST.md#metric-streaming) — semantic input selection,
  scalar updates, bulk slices, current values, and reset.
- [Technical analysis pipelines](docs/RUST.md#technical-analysis-pipelines) —
  native state composition and the boundary of the Python-only `TAPipeline`.
- [`MetricPipeline` reference](docs/RUST.md#metricpipeline) — native metric
  ownership, named fan-out, input domains, continuation, and results.
- [Data input and output](docs/RUST.md#data-input-and-output) — direct
  `Vec<f64>`, Apache Arrow `Float64Array`, and Polars `Series` conversion.

## Indicators, metrics, and pipelines

| Surface | Coverage | API |
|---|---:|---|
| [Indicator classes](docs/INDICATORS.md) | 393 canonical classes, including all 161 TA-Lib functions | Configure a class, then use `extend`, `append`, `value`, `compute`, and `reset` |
| [Metric classes](docs/METRICS.md) | 57 strategy, risk, trade, and portfolio metrics with complete Python defaults and Rust parameters | Import standalone states from `taflow.metrics` and select the documented semantic input domain before streaming or computing |
| [`TAPipeline`](docs/PIPELINES.md) | Causal graphs of sources, indicators, expressions, and named outputs | Shares stateful nodes across historical `extend` and live `append` evaluation |
| [`MetricPipeline`](docs/METRIC_PIPELINE.md) | Multiple compatible metrics over one normalized input stream | Provides named fan-out, aligned lifecycle operations, and dictionary results |

The standalone class APIs are ideal for one calculation at a time. The two
pipeline APIs coordinate several calculations while keeping each indicator or
metric as the sole owner of its numerical state.

## Evaluation

| Evaluation | Indicator classes | Metric classes |
|---|---|---|
| Correctness | [External-oracle results for all 393 indicator classes](verify/CORRECTNESS.md) | [External-oracle results for all 57 metric classes](verify/metrics/CORRECTNESS.md) |
| Benchmark | [Correctness-gated indicator timings](verify/BENCHMARK.md) | [Correctness-gated metric timings](verify/metrics/BENCHMARK.md) |
| Performance | [Kernel design, runtime dispatch, and optimization decisions](docs/PERFORMANCE.md) | Native persistent states and `MetricPipeline` fan-out are covered by the metric benchmark report |

## Repository layout

```text
crates/taflow-core/     Rust indicator states and kernels
crates/taflow-metrics/  Rust strategy and portfolio metrics
crates/taflow-python/   PyO3 bindings
python/taflow/          Python adapters, pipelines, and converters
docs/                   Language guides and focused documentation
tests/                  Python API and correctness tests
verify/                 Generated correctness and benchmark reports
```

## License

MIT—see [LICENSE](LICENSE).
