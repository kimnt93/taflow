# TAFlow implementation rules

These rules apply to the entire repository. They define the required shape for
new indicators and for repairs to existing indicators.

## One indicator, one state machine

- The canonical implementation is a persistent Rust state type in
  `crates/taflow-core/src/stream/`. Python is an adapter only; it must not
  implement indicator arithmetic, rolling windows, warm-up, or output repair.
- One canonical TA class lives in one same-named Rust file and one same-named
  Python file. The Rust module owns that indicator's state type, associated
  multi-output value type, private bulk helper, and focused tests. Small helper
  types used only by that indicator stay in the same file; helpers genuinely
  shared by several indicators live in a narrowly named helper module.
  Aggregation files such as `operator_states.rs` must not contain public TA
  state classes, and `mod.rs` contains declarations and re-exports only.
- A state type must expose the same lifecycle:
  - `new(...) -> TaResult<Self>` for validated configuration;
  - `append(...)` for one chronological bar;
  - `value(&self)` for the latest result (`None` during scalar warm-up);
  - `reset(&mut self)` that restores fresh-state behavior without reallocating;
  - a slice/bulk extension path when it can improve throughput without changing
    post-run state or output bits.
- Implement `StreamingIndicator` for single-input/single-output states. For
  multi-input or multi-output states, use equivalent inherent methods and a
  typed `*Value` struct rather than unlabelled internal tuples.
- The public indicator surface is class/state-only. Batch kernels may exist as
  private implementation details, but users construct the persistent state and
  call its lifecycle; do not add or export a parallel free-function indicator
  API. A private bulk kernel must delegate to the same state machine or be
  bit-equivalent, and it must never become a second Python execution path.
- Warm-up belongs in Rust. Scalar warm-up is `None`; aligned histories contain
  `f64::NAN` at those positions. Batch, repeated `extend`, and scalar `append`
  must leave identical state and bitwise-identical histories.

## Rust performance rules

- `append` must be allocation-free after construction. Retain bounded state;
  never retain full input history merely to calculate the next value.
- Use an O(1) recurrence or rolling add/remove state when the formula permits.
  Use fixed rings/`Window`, monotonic extrema, sorted rings, seeded EMA helpers,
  and existing moving-average states before creating another implementation.
- Do not assume a theoretically better algorithm is faster. Benchmark the
  canonical Python adapter and the Rust kernel at cold start, 1k/10k/100k/1M
  vectors, warmed scalar continuation, chunked continuation, and independent
  streams.
- Empty-state bulk acceleration must leave the exact state produced by scalar
  replay so later `append` calls remain chunk invariant. If floating-point
  reassociation changes the contract, keep the stable order or prove the oracle
  tolerance and document why bitwise equality is impossible.
- Release the GIL only for bulk work. Do not add `unsafe` or architecture-only
  build flags for ordinary kernels; use the repository's portable runtime
  dispatch where applicable.

## Python adapter contract

- The public API is the canonical CamelCase class. TA-Lib names are metadata
  aliases/mappings (for example `MathSin` maps to TA-Lib `SIN`); do not expose
  a second numerical implementation or a public snake-case batch function.
- Every indicator class exposes the same operations:
  `append`, `extend`, `value`, `compute`, `reset`, and `__len__`.
  `append`, `extend`, and `reset` mutate and return `self` for fluent use.
  Callers read the latest scalar/tuple through `value`.
- Constructors require every input series. Use empty aligned arrays to create
  a fresh state for later streaming; do not make series optional or use
  `None` as an implicit empty stream. Configuration parameters have documented
  defaults unless the caller must make a genuine semantic choice. `extend`
  accepts the same ordered input streams as the constructor; `append` accepts
  one scalar per stream in that order. Reject misaligned multi-series input
  before mutating native state.
- Convert supported containers once at the Python/Rust boundary using the
  shared series adapters. Never loop over bars or calculate indicator values in
  Python. Native `extend` must release the GIL around the Rust loop.
- Docstrings must be specific enough for IDE completion and generated docs:
  describe the formula/contract, every parameter and default, input ordering,
  warm-up, output names/order and type, fluent lifecycle returns, validation,
  oracle/name mapping, and any causal alignment or definition variant. Avoid
  placeholders such as “object”, “values”, or “updated adapter or output”.
- Type annotations must reflect the concrete contract: `append`, `extend`, and
  `reset` return the quoted concrete class name (for example
  `-> "ParabolicMovingAverageStop"`), never `object` or a generic base class;
  `compute` returns `np.ndarray` or an explicit tuple of arrays; `value`
  returns the corresponding scalar/tuple or `None`.

## Correctness and source mapping

- Record both names whenever they differ: canonical TAFlow class/name and
  oracle name (for example `MathSin` ⇔ `SIN`, `RollingCov` ⇔ Polars
  `rolling_cov`). Never infer equivalence from similar names alone.
- Choose the independent oracle in this priority order:
  1. TA-Lib;
  2. Polars;
  3. pandas;
  4. pandas-ta-classic;
  5. a pinned public GitHub implementation with license and commit/version.
- Compare every output, NaN/warm-up placement, defaults, output ordering, and a
  parameter matrix. Include random, constant, monotonic, repeated-extrema,
  minimum-length, reset/replay, chunked, cold-start, and warmed-continuation
  cases. Oracle import/errors are failures, never skipped self-checks.
- Mark a result `MATCH` only for numerical parity within the declared tolerance.
  Use `VARIANT` for a proven contract/definition difference and explain it.
  Native lifecycle equality is `INVARIANT`, not external correctness evidence.
- Correctness reports and benchmark reports must name the oracle/source and its
  version. A function with no external equivalent must explicitly say so.

## Performance comparison

- Benchmark against the same highest-priority library used for correctness.
  Do not benchmark against TA-Lib when Polars is the correctness oracle, or
  against self when an external oracle exists.
- Report final TAFlow-versus-oracle results only. Do not advertise improvement
  relative to an earlier TAFlow implementation in README comparison sections.
- Correctness is a gate: a timing row cannot be `MATCH` unless its configured
  oracle comparison passes first.

## Required gates

Before declaring an indicator complete, run its focused Rust tests, canonical
Python lifecycle audit, external oracle comparison, and focused full-protocol
benchmark. Before repository-wide completion, run:

```bash
cargo test --workspace
uv run pytest -q
make check
cd verify && uv run python all_interfaces.py
cd verify && uv run python benchmark.py
cargo fmt --all --check
git diff --check
```

Generated reports and README claims must agree with those results.
