# TAFlow streaming extension plan

## Product contract

Keep the current `taflow.talib` batch API byte-for-byte compatible in results and
lookback placement with the C TA-Lib Python wrapper.  Add a separate, opt-in
stateful API for live data:

```rust
let mut rsi = stream::Rsi::new(14)?;
assert_eq!(rsi.append(close), None); // warm-up
let value = rsi.append(next_close);  // O(1) after warm-up
```

The shared design discussion establishes these non-negotiable choices:

- Rust core has no Python, NumPy, Arrow, or dataframe dependency.
- Batch and streaming use the same numerical definitions and warm-up rules.
- Streaming state owns only the minimum history required; no re-computation of
  the complete price history on each tick.
- Python is an adapter layer.  It exposes `append()` and `extend()` and keeps
  the established functional TA-Lib-compatible API unchanged.
- Arrow, Polars, and pipeline scheduling are later adapters/features, not the
  initial in-memory representation.

## Current baseline (2026-08-06)

- Workspace contains a pure-Rust core and PyO3/NumPy bindings.
- The existing batch surface exposes all 158 functions reported by TA-Lib's
  function inventory and has comprehensive Python comparison tests.
- There is no stateful/incremental API, no streaming compatibility suite, and
  no streaming benchmark.  The `sliding_window` module is batch-only.
- Existing `BENCHMARK.md` covers 90 batch indicators against C TA-Lib; it must
  remain labelled with its original Apple M4 environment rather than being
  treated as a result from the current machine.

## Architecture

```text
taflow (core)
  stream/
    mod.rs          shared primitives and explicit module re-exports
    cmo.rs          one English-documented TA state per source file
    ...             one file per TA function for maintainability
  overlap/, momentum/, volatility/  existing batch compatibility API

taflow-python
  functional TA-Lib-compatible functions (unchanged)
  stateful classes: Sma, Ema, Rsi, Atr, Macd

tests/
  batch TA-Lib oracle tests (existing)
  streaming == batch regression tests (new)
benches/
  batch comparison (existing)
  streaming append and growing-history comparison (new)
```

`Window` is deliberately small and safe: `VecDeque<f64>` initially, with a
fixed capacity and no allocation once constructed.  A hand-rolled ring buffer
is only warranted after profiling proves `VecDeque` is a bottleneck.

## Compatibility rules

1. Constructors validate the same periods as the batch implementation.
2. `append` returns `None` before the exact TA-Lib lookback is satisfied;
   `Some(value)` otherwise.
3. `extend` is defined as repeated `append`, so chunk boundaries cannot change
   results.
4. Streaming-to-batch tests compare every output, including the warm-up area;
   C TA-Lib remains the external oracle for batch behavior.
5. Stateful output is not inserted into the existing functional namespace, so
   a user cannot accidentally replace TA-Lib-compatible batch semantics.

## Delivery phases and parallel ownership

Each workstream changes a disjoint directory.  Integrate in the listed order;
an agent must not silently change another workstream's public contract.

| Phase | Owner/workstream | Scope | Gate |
|---|---|---|---|
| 0 | architecture | this plan; API contract; baseline commands | reviewed plan |
| 1 | core primitives | `stream/window.rs`, trait and scalar `Sma`/`Ema` | Rust unit + batch parity |
| 2 | core momentum | `Rsi`, then MACD-family state machines | parity for randomized/chunked inputs |
| 3 | core OHLC | `Trange`, `Atr`, NATR, directional movement family | parity + OHLC edge cases |
| 4 | Python adapter | PyO3 classes, `append`, `extend`, `value`, `reset` | Python API + parity |
| 5 | engine | type-erased pipeline and one-dispatch-per-bar executor | independent-vs-engine parity |
| 6 | expansion | rolling stats, extrema, volume, trend, stochastic, patterns | family-by-family parity |
| 7 | operator library | `shift`, `diff`, returns, cross/rising/falling, rolling ops | semantic/unit suite |
| 8 | data adapters | NumPy zero-copy where sound; Arrow/Polars optional features | adapter tests + no core deps |
| 9 | quantitative extensions | cross-sectional, factors, ML features | licensing/design review |

Work in phases 1--4 is the first releasable streaming vertical slice.  Phases
5--9 are intentionally sequenced: pipeline scheduling is unsafe to build
before indicator state semantics are proved, and dataframes must not dictate
the core memory model.

The per-function implementation state lives in
[`full-ta-checklist.md`](full-ta-checklist.md). Mark an item after its isolated
implementation, parity tests, and Python adapters land. Benchmark and report
coverage follows in a dedicated pass after all mappings are implemented. This
avoids treating the inherited batch implementation as completed streaming
work.

The post-compatibility operator roadmap is tracked separately in
[`operator-library-checklist.md`](operator-library-checklist.md).  It covers
the rolling, signal, cross-sectional, and adapter layers proposed for TAFlow
without conflating them with TA-Lib compatibility.

### Source-layout rule

Every newly implemented TA state belongs in its own source file under
`crates/taflow-core/src/stream/`, starts with an English module/function
description, and is imported and re-exported explicitly by `stream/mod.rs`.
Existing monolithic states will be migrated family by family without changing
their public names or numerical behavior.

## Test and benchmark matrix

### Required correctness commands

```bash
cargo test --workspace
python -m pytest tests/accuracy -q
python -m pytest tests/test_full_coverage.py tests/test_exhaustive.py -q
```

The Python checks require a built extension plus the original `TA-Lib` package.
If either is absent, report that explicitly rather than calling compatibility
verified.

### Streaming-specific tests

- deterministic trend, flat, alternating, and random-walk prices;
- period 1 / minimum valid period / invalid periods;
- `append` versus batch for every index;
- `extend(all)` versus arbitrary chunk partitions;
- `reset` followed by replay;
- OHLC high/low/close length and first-bar behavior;
- multi-output MACD warm-up alignment.

### Benchmarks

Record machine, Rust/Python versions, commit, and command with every result.

1. Batch: 1K, 10K, 100K, 1M; compare selected existing indicators with C
   TA-Lib as the current benchmark does.
2. Streaming: after a 10K-bar warm-up, measure 100K and 1M `append` updates
   for each stateful indicator (ns/update and allocations).
3. Growing history: recompute batch TA-Lib-style history after each append
   versus persistent state to demonstrate the expected asymptotic difference.
4. Pipeline (after phase 5): 20 indicators and 1M ticks; report p50/p95/p99,
   allocations, total time, and peak RSS.

Do not compare a Python method call to an in-Rust loop as though they are the
same measurement: publish both Python-to-Rust and Rust-only append results.

## Completion criteria

The implementation phase is complete only when the phase 1--4 indicators have
exact streaming-to-batch parity, all workspace tests pass, and C TA-Lib oracle
tests pass when its dependency is available. Reproducible benchmarks and
reports are generated afterward as a separate phase; no claim of "all
strategies" is made before both phases have passed.
