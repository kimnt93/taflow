# Unified realtime TA-Lib implementation plan

## Product contract

TAFlow implements the complete TA-Lib function inventory as one persistent,
fast indicator API. There is no separate batch TA-Lib implementation and
extension library: every public TA-Lib name resolves to an extendable state
object, and `taflow.talib` is an alias for that unified surface.

The first reference API is:

```python
ema = ExponentialMovingAverage(history, timeperiod=20)
full_history = ema.compute()

ema.append(next_close)
ema.extend(next_chunk)
latest = ema.value
full_history_after_updates = ema.compute()
```

The core must retain only the bounded history and recurrence state required by
the indicator. `append` is O(1) in the period for all mathematically
incremental TA-Lib functions; `extend(k)` is O(k). A returned full-history
array from `compute()` has unavoidable O(n) materialization cost, but it must
not rerun calculations over previously ingested bars.  Indicators whose
definition needs a period window use fixed-capacity ring buffers, monotonic
deques, or rolling accumulators rather than rescanning the window.

## Architecture

```text
Python input adapters (NumPy/list/Polars/dataframe)
             |
             v
generated metadata + unified Python class / TA-Lib alias
             |
             v
PyO3 state adapter + output cache
             |
             v
taflow-core stream::Indicator states
  - scalar append / current value / reset
  - optimized empty-state bulk extend
  - shared rolling, smoothing, extrema, regression primitives
```

The Rust core has no Python, NumPy, Arrow, Polars, or dataframe dependency.
Each state is in a dedicated English-documented source file under
`crates/taflow-core/src/stream/` and is explicitly re-exported by
`stream/mod.rs`.  Dataframe support belongs solely in the Python conversion
boundary.

There is no second numerical implementation hidden behind a functional API.
The initial bulk path and incremental path share state primitives, numerical
definitions, parameters, output ordering, and warm-up semantics.

## Performance requirements

- Scalar recurrence states use constant work and fixed storage after
  construction; no per-bar allocations and no full-history recomputation.
- Rolling sums/statistics use add/remove accumulators, numerically stable
  formulas where required, and compensated accumulation when it is necessary
  to meet the final TA-Lib comparison contract.
- Rolling extrema and index operations use monotonic deques; regression uses
  maintained moments; smoothing chains retain only their required values.
- Bulk initialization and pointwise functions use contiguous float64 loops
  that LLVM can auto-vectorize, plus the existing safe SIMD/reduction
  primitives when profiling proves a gain. Avoid Python loops and temporary
  allocation chains.
- NumPy contiguous float64 is zero-copy at the native boundary. Other accepted
  input types may pay one explicit conversion; that cost is reported rather
  than hidden in indicator timing.
- A claim of O(1) applies to state update work, not input conversion or
  returning the complete accumulated result.

## Delivery order: implement all, compare afterward

1. **Foundation** — finalize `StreamingIndicator`/multi-output counterparts,
   bounded windows, output cache, input conversion, metadata, and generated
   descriptive-name/uppercase-alias exports. Implement EMA as the API and
   performance reference.
2. **Single-series families** — overlap averages, price transforms, math
   transforms, math operators, rolling statistics, and their stateful bulk
   paths.
3. **OHLC/OHLCV families** — volatility, volume, directional movement,
   oscillators, stochastic, and MACD variants.
4. **Advanced bounded-state families** — adaptive averages, Hilbert/cycle
   indicators, and every candlestick pattern recognizer.
5. **Surface migration** — remove legacy duplicate Python wrappers, export all
   classes in `taflow.indicators`, and make `taflow.talib` forward to the same
   objects. Preserve the uppercase aliases but do not preserve functional
   return semantics that conflict with the object contract.
6. **Final comparison** — only when every checklist item is implemented, run
   exhaustive TA-Lib oracle comparison, state-to-bulk parity, chunk/replay
   cases, and real-data alignment for the entire inventory.
7. **Final performance pass** — profile, optimize measured bottlenecks, then
   produce all reports and benchmark artifacts together.

Non-TA operator work, pipeline scheduling, native Arrow output, and native
Polars output are outside this delivery. They must not block completion of the
TA-Lib realtime surface.

## Completion criteria

Implementation is complete when each item in
[`full-ta-checklist.md`](full-ta-checklist.md) has a unified Rust state,
Python class, descriptive name, TA-Lib alias, supported input conversion,
bounded continuation behavior, and basic local lifecycle tests. This first
phase intentionally finishes before full external comparison.

The release is complete only after the deferred global pass verifies all
functions against TA-Lib where applicable, confirms exact warm-up/output
placement and chunk invariance, and publishes separate bulk/backfill/realtime
measurements. No benchmark or parity claim should be made before then.
