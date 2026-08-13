<p align="center">
  <h1 align="center">TAFlow</h1>
  <p align="center">
    Persistent technical analysis for Rust and Python, with configuration-only
    construction and causal streaming updates.
  </p>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/indicators-393-blue" alt="393 indicators" />
  <img src="https://img.shields.io/badge/TA--Lib_parity-161-blue" alt="161 TA-Lib functions" />
  <img src="https://img.shields.io/badge/correctness-393%2F393_MATCH-brightgreen" alt="393/393 externally matched" />
  <img src="https://img.shields.io/badge/unsafe-zero-brightgreen" alt="zero unsafe" />
  <img src="https://img.shields.io/badge/C_deps-zero-orange" alt="zero C dependencies" />
  <img src="https://img.shields.io/badge/license-MIT-lightgrey" alt="MIT" />
</p>

TAFlow provides 393 canonical indicator classes as native Rust states and
Python adapters. It covers the complete 161-function TA-Lib surface plus
rolling statistics, EWM operators, volatility estimators, market structure,
patterns, signals, and portfolio metrics—without linking to TA-Lib or another
C library.

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

## Documentation

| Document | Contents |
|---|---|
| [Python API](docs/PYTHON.md) | Installation, indicator lifecycle, streaming, outputs, pipelines, metrics, and data adapters |
| [Rust API](docs/RUST.md) | Crate setup, state lifecycle, bulk methods, multi-output values, errors, and metrics |
| [Indicator catalog](docs/INDICATORS.md) | All 393 classes, input order, constructor configuration, and oracle mapping |
| [Streaming](docs/STREAMING.md) | Warm-up, continuation, chunk invariance, reset, and per-tick behavior |
| [Pipelines](docs/PIPELINES.md) | Causal Python graphs, expressions, evaluation, and limitations |
| [Metric pipeline](docs/METRIC_PIPELINE.md) | Metric input domains, fan-out, lifecycle, and results |
| [Data input/output](docs/DATA.md) | NumPy, lists, pandas, Polars, Arrow, and custom adapters |
| [Correctness](verify/CORRECTNESS.md) | External evidence for all 393 registered indicators |
| [Benchmark](verify/BENCHMARK.md) | Per-indicator vector timings and reference-library versions |
| [Performance](docs/PERFORMANCE.md) | Kernel design, runtime dispatch, and optimization decisions |

## Benchmark summary

The 2026-08-13 correctness-gated run measured all 393 indicators against the
same external implementation used for correctness. The table summarizes native
kernel speedup (`reference time / TAFlow time`); values above 1× favor TAFlow.
Because the reference set mixes compiled TA-Lib/NumPy kernels with Python
libraries, the aggregate median is descriptive rather than a claim that every
indicator has the same competitor.

| Bars | Median speedup | Indicators at or above 1× |
|---:|---:|---:|
| 1,000 | 0.98× | 195 / 393 |
| 10,000 | 0.36× | 136 / 393 |
| 100,000 | 0.27× | 132 / 393 |

The full report records every class, oracle, environment version, repeated
sample, and 1/5/10-thread warm-state matrix in
[`verify/BENCHMARK.md`](verify/BENCHMARK.md) and
[`verify/evidence/benchmark/`](verify/evidence/benchmark/).

```bash
make check                   # Python tests plus all external oracle checks
make test                    # Rust workspace tests plus Python tests
make bench                   # full correctness-gated benchmark
make bench ARGS="EMA ATR"    # selected indicators
```

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
