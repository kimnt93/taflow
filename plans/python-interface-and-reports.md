# Python interface and per-function report plan

## Public namespaces

TAFlow exposes two deliberate Python surfaces:

1. `taflow.talib` is the compatibility namespace. It preserves TA-Lib's
   uppercase function names, argument names, NumPy array inputs, aligned `NaN`
   warm-up, and tuple-of-array outputs.
2. `taflow` is the descriptive stateful namespace. It exposes CamelCase types
   such as `MovingAverage`, `BollingerBands`, and `ParabolicSar` with
   `append`, `extend`, `value`, and `reset`.

The installed distribution and native extension are `taflow` and
`taflow._native`; new implementation code must not depend on the former
package name.

## Python source-layout rule

Every descriptive public indicator belongs in one file under `python/taflow/`.
The file name is snake_case and the public type is imported explicitly by
`python/taflow/__init__.py`. Implementation details do not belong in either
package `__init__.py`.

The compatibility namespace is a thin re-export layer over the verified native
batch API; it must not duplicate numerical implementations.

## Naming and behavior

| TA-Lib name | Descriptive name | Python file | Mode |
|---|---|---|---|
| MA | MovingAverage | `moving_average.py` | stateful |
| BBANDS | BollingerBands | `bollinger_bands.py` | stateful |
| ACCBANDS | AccelerationBands | `acceleration_bands.py` | stateful |
| SAR | ParabolicSar | `parabolic_sar.py` | stateful |
| SAREXT | ParabolicSarExtended | `parabolic_sar_extended.py` | stateful |
| IMI | IntradayMomentumIndex | `intraday_momentum_index.py` | stateful |
| MACDFIX | MovingAverageConvergenceDivergenceFixed | `moving_average_convergence_divergence_fixed.py` | stateful |
| STOCHF | FastStochasticOscillator | `fast_stochastic_oscillator.py` | stateful |
| STOCH | StochasticOscillator | `stochastic_oscillator.py` | stateful |
| STOCHRSI | StochasticRelativeStrengthIndex | `stochastic_relative_strength_index.py` | stateful |

Each descriptive wrapper uses composition around the PyO3 state class. This
gives stable public names without subclassing extension types and keeps all
numerical work in the Rust implementation.

## Required tests per function

- compatibility output from `taflow.talib` versus original TA-Lib;
- descriptive `extend` output versus the same oracle;
- scalar `append` and reset/replay parity;
- exact warm-up placement and multi-output ordering;
- invalid parameters and unequal input lengths where applicable;
- Rust batch/state parity and a one-million-update Rust benchmark.

## Per-function reporting contract

Every completed or updated function owns two files in `reports/`:

- `<FUNCTION>.md`: algorithm summary, correctness evidence, stream behavior,
  benchmark method, speed, series sizes, and limitations;
- `<FUNCTION>.json`: machine-readable numerical results using the schema below.

The benchmark schema is append-only so a later aggregator can combine every
function without parsing Markdown. Required top-level fields are
`schema_version`, `function`, `date`, `environment`, `correctness`,
`benchmark_protocol`, `benchmark_matrix`, and `rust_stream_benchmark`.
Every numerical result records its unit; unavailable modes use `available:
false` plus a reason and never a fabricated zero.

For each size, a mode records repeated end-to-end call latency (`min`, `mean`,
`p50`, `p95`, `p99`, and `max` milliseconds), CPU time, throughput in bars per
second, and peak resident-memory delta. Input-array allocation and deterministic
data generation happen before measurement. The raw repeated samples are kept in
JSON so aggregate reports can recompute percentiles.

## Reproducible benchmark matrix

Use deterministic float64 inputs and the sizes `100`, `1_000`, `10_000`,
`100_000`, and `1_000_000`; additionally run `10_000_000` in the dedicated
scaling job rather than making it a per-commit gate. Run every timed scenario
at least five times after one untimed process warm-up.

| Mode | Definition | Per-function availability |
|---|---|---|
| TA-Lib | original `talib` batch call | required |
| TAFlow | `taflow.talib` batch call | required |
| TAFlow state extend | one descriptive-state `extend` call | required for checked states |
| TAFlow streaming | 10,000-bar backfill, then scalar `append` updates | required for checked states |
| TAFlow Pipeline | type-erased multi-indicator Rust execution plan | unavailable until pipeline phase |

`extend` is a state backfill and must not be mislabeled as the future Pipeline.
The Pipeline row remains explicitly unavailable until it performs a shared
execution plan in Rust. Once implemented, benchmark 1, 20, 50, and 100
indicators over the same sizes, plus 1, 10, and 100 symbols.

For live-update latency, collect per-append samples independently from the
throughput loop and report mean/p50/p95/p99/max. Also maintain the continuous
backfill comparison: begin with 10,000 history bars, append new bars, and
compare TA-Lib full-history recomputation with persistent TAFlow state. Cap
that quadratic oracle case separately and record the cap.

Correctness gates cover random, constant, NaN/Inf policy, warm-up alignment,
float64, chunked `extend`, and backfill-then-append continuation. Float32 input
behavior is recorded as conversion/API compatibility because the numerical
engine intentionally computes float64.

Checklist items are checked only after both report files, both Python
interfaces, native parity, oracle parity, and benchmark evidence exist.

## Migration order

1. Establish the package and report contract with MA, BBANDS, ACCBANDS, SAR,
   and SAREXT.
2. Add one descriptive Python file and report pair whenever an existing
   checked state is next touched.
3. Backfill the remaining checked states family-by-family.
4. Complete the unchecked TA-Lib states, then the operator-library extensions.
5. Audit that every checked function has native source, both Python surfaces,
   tests, benchmark registration, and its Markdown/JSON report pair.
