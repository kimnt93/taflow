# Unified Python indicator API and deferred validation plan

## Decision

TAFlow has one public indicator model: a persistent, extendable indicator
object.  It replaces the former split between a TA-Lib-style batch namespace
and separately named streaming wrappers.

`taflow.indicators` and the root `taflow` namespace expose the canonical
CamelCase classes. There is no `taflow.talib` compatibility package. There
must be one Rust state machine and one Python adapter per TA function.

The canonical descriptive names are explicit and readable, for example
`ExponentialMovingAverage`, `RelativeStrengthIndex`, and
`AverageTrueRange`.  TA-Lib uppercase names such as `EMA`, `RSI`, and `ATR`
are aliases to those same classes in both namespaces.  `Ema`, `StatefulEma`,
and the current one-file-per-descriptive-wrapper surface are transitional
names and will not be extended.

```python
from taflow import ExponentialMovingAverage

ema = ExponentialMovingAverage(series_numpy_or_list_or_polars_or_df, timeperiod=20)
values = ema.compute()                    # full, aligned NumPy result

ema.append(next_value)                    # persistent O(1) state update
# or: ema.extend(next_series)
updated_values = ema.compute()            # no numerical replay of old bars
latest = ema.value                         # latest value without materializing history
```

`compute()` returns the complete output accumulated by that object, with NaN
in the TA-Lib warm-up positions.  It does not recalculate already processed
bars.  Returning a full NumPy result is necessarily O(n) to expose n values;
the calculation for each newly appended scalar is O(1) where the indicator
has a constant-space recurrence.  `value` is the realtime O(1) latest-value
path.  Multi-output indicators return a tuple of aligned NumPy arrays from
`compute()` and a tuple of latest optional scalars from `value`.

## Inputs and lifecycle

- Constructors accept initial data plus the indicator's TA-Lib parameters;
  initial data is optional, so `ExponentialMovingAverage(timeperiod=20)` is a
  valid empty realtime state.
- `append(...)` accepts one scalar per required input stream and mutates the
  object. `extend(...)` accepts the same supported collection types as the
  constructor and processes only the supplied new bars.
- NumPy one-dimensional float64 input is read without a copy when contiguous.
  Python lists, Polars `Series`, and supported one-column dataframe selections
  normalize to float64 once at the API boundary.
- For OHLCV indicators, callers may provide positional series
  (`AverageTrueRange(high, low, close, timeperiod=14)`) or a dataframe plus
  explicit column names.  The public API never guesses columns silently;
  standard names are defaults and ambiguous/missing columns raise `ValueError`.
- `reset()` clears state and accumulated outputs. Input length mismatches,
  invalid periods, and non-numeric input fail before any partial mutation.

The public output contract is NumPy float64 arrays and tuples of those arrays,
regardless of whether the source was list, NumPy, Polars, or dataframe. This
keeps numerical behavior and downstream interoperability consistent.  Native
Polars output and broader dataframe ergonomics are explicitly postponed.

## Implementation rules

1. Each TA-Lib function has one persistent Rust state type under
   `crates/taflow-core/src/stream/`. It supports construction, scalar append,
   chunked extend, current value, reset, and a bulk-initialization path.
2. The Python extension owns the output cache and exposes the unified class;
   no Python numerical loops and no duplicate batch wrapper are permitted.
3. Empty-state `extend` may use the optimized bulk path. Once history exists,
   `append` and `extend` continue the exact same state without replaying prior
   input.
4. TA-Lib aliases are generated from metadata, so names, defaults, input
   arity, output ordering, and warm-up rules cannot drift between namespaces.
5. Only TA-Lib inventory items are in the current implementation plan. The
   separate `operator-library-checklist.md` is deferred and is not a release
   gate.

## Implementation-first order

Do not pause the implementation for per-function TA-Lib comparison reports.
Build every TA-Lib indicator first, then run a single exhaustive comparison and
benchmark pass across the completed surface.

1. Establish the shared state, output-cache, input-normalization, metadata,
   and alias-generation infrastructure with EMA as the reference class.
2. Migrate all overlap, price-transform, math-transform, math-operator, and
   statistic functions.
3. Implement all momentum, volatility, and volume functions, including each
   multi-input and multi-output state.
4. Implement the Hilbert/cycle and candlestick-pattern families with bounded
   per-bar state.
5. Export every descriptive CamelCase class from `taflow.indicators` and the
   root `taflow` namespace. No compatibility alias package is shipped.
6. Only after every item is implemented: execute exhaustive TA-Lib oracle
   comparisons, batch/stream parity, real-data alignment, and benchmarks.

## Final validation and benchmark pass

The final pass is deliberately global rather than an implementation blocker.
It must cover random, constant, NaN/Inf-policy, minimum-period, chunked
`extend`, reset/replay, and backfill-then-append cases for every function.
Compare all outputs and exact warm-up placement with the original TA-Lib
package where it defines the function.

Benchmark initial bulk `compute`, backfill via `extend`, and realtime scalar
`append` independently at 1K, 10K, 100K, and 1M bars.  Record Python-to-Rust
and Rust-only append latency separately.  Per-function reports and generated
benchmark JSON are produced only after the entire TA-Lib surface passes this
final validation.
