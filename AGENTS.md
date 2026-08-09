# TAFlow implementation rules

These rules apply to the entire repository. They define the required shape for
new indicators and for repairs to existing indicators.

## One indicator, one class, one implementation

- The canonical implementation is a persistent Rust state type in
  `crates/taflow-core/src/stream/`. Python is an adapter only; it must not
  implement indicator arithmetic, rolling windows, warm-up, or output repair.
- Canonical class names must use complete descriptive words. Never shorten,
  abbreviate, contract, or preserve an abbreviated external-library name in a
  class name. For example, `RollingCorr`, `RollingCov`, `RollingStd`, and
  `Var` are invalid; use `RollingCorrelation`, `RollingCovariance`,
  `RollingStandardDeviation`, and `Variance`. An external short name such as
  TA-Lib `CORREL` belongs only in oracle/alias metadata.
- Implement each indicator exactly once, as its canonical CamelCase class/state.
  For example, implement `VariablePeriodMovingAverage`; do not also implement,
  retain, or export `fn variable_period_moving_average(...)` in Rust or
  `def variable_period_moving_average(...)` in Python. This prohibition applies
  even when the free function is described as a batch kernel, compatibility
  helper, convenience wrapper, or test oracle. Put scalar, slice, and bulk
  behavior on the class/state through inherent methods or the shared lifecycle
  trait. Private associated methods are allowed when they support the class;
  parallel free-function indicator implementations are not.
- Derive the module filename mechanically from the full canonical class name
  using snake_case; do not use abbreviations, TA-Lib aliases, or family buckets.
  `VariablePeriodMovingAverage` therefore lives in
  `crates/taflow-core/src/stream/variable_period_moving_average.rs` and
  `python/taflow/variable_period_moving_average.py`.
- Each canonical class must have its own implementation file. Do not place two
  canonical indicator classes in one file, split one canonical class across
  multiple implementation files, or define a canonical class in `mod.rs`,
  `lib.rs`, `__init__.py`, an aggregation module, or a shared helper module.
  Rust and Python must use the same full canonical class name and corresponding
  snake_case filename.
- Keep production implementation and tests in separate same-named files. The
  Rust tests for the example above live in
  `crates/taflow-core/src/stream/variable_period_moving_average_test.rs`; its
  Python tests live in `tests/variable_period_moving_average_test.py`. An
  indicator implementation file must not contain an inline `#[cfg(test)] mod
  tests`, and a package/module initializer must not contain tests.
- `mod.rs`, `lib.rs`, Python `__init__.py`, and equivalent aggregation files are
  import surfaces only: module declarations, imports, and re-exports. They must
  not contain indicator logic, class definitions, free-function wrappers,
  tests, inline test helpers, or compatibility implementations. Rust test
  modules may be declared from `mod.rs` with `#[cfg(test)] mod
  variable_period_moving_average_test;`, but all test code stays in that test
  file.
- The Rust implementation module owns the class/state and its associated
  multi-output value type. Small helper types used only by that class stay in
  its implementation file; helpers genuinely shared by several classes live in
  a narrowly named helper module. Aggregation files such as
  `operator_states.rs` must not contain public TA state classes.
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
- The public and internal indicator surface is class/state-only. Users and
  internal callers construct the persistent state and use its lifecycle. A
  specialized bulk path, when justified, must be a method of that same state,
  must leave the same post-run state as scalar replay, and must never become a
  second implementation or Python execution path.
- Warm-up belongs in Rust. Scalar warm-up is `None`; aligned histories contain
  `f64::NAN` at those positions. Batch, repeated `extend`, and scalar `append`
  must leave identical state and bitwise-identical histories.

## Canonical structure for AI agents

For a class named `VariablePeriodMovingAverage`, create or maintain this shape:

```text
crates/taflow-core/src/stream/
├── variable_period_moving_average.rs
├── variable_period_moving_average_test.rs
└── mod.rs                                      # declarations/re-exports only

python/taflow/
├── variable_period_moving_average.py
└── __init__.py                                 # imports/re-exports only

tests/
└── variable_period_moving_average_test.py

verify/
└── ...                                         # Python class vs external oracle
```

GOOD Rust production shape:

```rust
// variable_period_moving_average.rs
pub struct VariablePeriodMovingAverage {
    // bounded persistent state
}

impl VariablePeriodMovingAverage {
    pub fn new(/* configuration */) -> TaResult<Self> { /* ... */ }
    pub fn append(&mut self, value: f64, period: f64) -> Option<f64> { /* ... */ }
    pub fn extend_slices_into(
        &mut self,
        values: &[f64],
        periods: &[f64],
        output: &mut Vec<f64>,
    ) -> TaResult<()> { /* ... */ }
    pub fn value(&self) -> Option<f64> { /* ... */ }
    pub fn reset(&mut self) { /* ... */ }
}
```

BAD Rust shapes:

```rust
pub struct Mavp { /* abbreviated canonical class */ }
pub struct VariablePeriodMovingAverage { /* ... */ }

pub fn variable_period_moving_average(/* ... */) -> Vec<f64> {
    /* parallel free-function implementation */
}

#[cfg(test)]
mod tests { /* test code inside the production implementation file */ }
```

GOOD Rust module and test shape:

```rust
// mod.rs — declarations and re-exports only
mod variable_period_moving_average;
pub use variable_period_moving_average::VariablePeriodMovingAverage;

#[cfg(test)]
mod variable_period_moving_average_test;
```

```rust
// variable_period_moving_average_test.rs
use super::variable_period_moving_average::VariablePeriodMovingAverage;

#[test]
fn matches_expected_state_lifecycle() {
    // Exercise the class methods; do not call a free-function implementation.
}
```

GOOD Python production and correctness shape:

```python
# python/taflow/variable_period_moving_average.py
class VariablePeriodMovingAverage:
    def __init__(self, values, periods): ...
    def append(self, value: float, period: float) -> "VariablePeriodMovingAverage": ...
    def extend(self, values, periods) -> "VariablePeriodMovingAverage": ...
    def compute(self) -> np.ndarray: ...
    def reset(self) -> "VariablePeriodMovingAverage": ...
```

```python
# tests/variable_period_moving_average_test.py
def test_variable_period_moving_average_matches_reference():
    actual = VariablePeriodMovingAverage(values, periods).compute()
    expected = external_library_reference(values, periods)
    np.testing.assert_allclose(actual, expected, equal_nan=True)
```

BAD Python shapes:

```python
class Mavp:  # abbreviated canonical class
    ...

def variable_period_moving_average(values, periods):
    return VariablePeriodMovingAverage(values, periods).compute()

# BAD correctness comparison: compares a local function instead of the class.
actual = variable_period_moving_average(values, periods)
```

The snippets describe structure, not permission to use placeholder bodies in
finished code. Production methods, docstrings, types, validation, and tests must
be complete.

## Required implementation and scan workflow

Before editing an indicator, an AI agent must inspect the existing canonical
surface and all aliases instead of assuming the requested name is unused:

```bash
rg -n "VariablePeriodMovingAverage|variable_period_moving_average|Mavp|MAVP" \
  crates python tests verify
rg --files crates/taflow-core/src/stream python/taflow tests \
  | sort
```

Then follow this order:

1. Resolve the full canonical class name and the independent external oracle.
   Reject shortened names before creating files.
2. Locate every existing implementation, export, binding, test, registry entry,
   documentation entry, and compatibility alias. Determine which code is the
   canonical class and which duplicate/free-function paths must be removed.
3. Put the Rust class in exactly one full-name implementation file and its Rust
   tests in the matching `_test.rs` file.
4. Put the Python adapter class in exactly one full-name implementation file and
   its Python tests in the matching `_test.py` file under `tests/`.
5. Keep `mod.rs`, `lib.rs`, and `__init__.py` limited to declarations, imports,
   and re-exports. Update registries as metadata only; do not place calculations
   there.
6. Compare `CanonicalClass(...).compute()` with the selected external library.
   Exercise `append`, `extend`, `value`, and `reset` separately for lifecycle
   invariance.
7. Remove obsolete abbreviated classes, indicator free functions, inline tests,
   duplicate implementations, and imports of those removed symbols. Do not keep
   deprecated forwarding wrappers unless the user explicitly requires a
   compatibility transition that overrides this file.

After editing, scan the changed indicator again. Adapt the names in these
commands to the class being changed:

```bash
# Exactly one Rust and one Python production class definition should remain.
rg -n "(struct|class) VariablePeriodMovingAverage" crates python

# No same-named indicator free function should remain.
rg -n "(fn|def) variable_period_moving_average" crates python tests verify

# Tests must be in separate files; implementation/import files must contain no tests.
rg -n "#\[cfg\(test\)\]|#\[test\]|def test_" \
  crates/taflow-core/src/stream/variable_period_moving_average.rs \
  python/taflow/variable_period_moving_average.py \
  python/taflow/__init__.py

# Review every shortened/external alias occurrence; only metadata mappings may remain.
rg -n "\b(Mavp|MAVP)\b" crates python tests verify docs
```

Interpret scan results rather than relying only on counts. A native binding may
also expose a Rust struct named `VariablePeriodMovingAverage`; that is part of
the Python adapter boundary, not permission for a second numerical
implementation. Any short alias found outside explicit metadata must be removed
or renamed. Finish by inspecting `git diff --check`, `git diff --stat`, and the
full diff for unintended generated or unrelated changes.

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
  a second numerical implementation or any snake-case indicator function.
- The same class-only rule applies inside Python: constructors, lifecycle
  methods, converters, and bindings may delegate to the native state, but no
  module-level function may calculate or wrap an indicator result. The canonical
  batch call is `ClassName(...).compute()`.
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

- Compare implementations only through the public Python class API. The TAFlow
  side of every external-oracle assertion must be produced by constructing the
  canonical class and calling `.compute()` (plus its lifecycle methods for
  invariance tests). Do not compare an external library against a Rust free
  function, Python free function, private batch helper, or a duplicated local
  formula. Rust tests establish state-machine behavior; Python-versus-reference
  tests establish external numerical correctness.
- Record both names whenever they differ: canonical TAFlow class/name and
  oracle name (for example `MathSin` ⇔ `SIN`, `RollingCov` ⇔ Polars
  `rolling_cov`). Never infer equivalence from similar names alone.
- Choose the independent oracle in this priority order:
  1. TA-Lib;
  2. Polars;
  3. pandas;
  4. pandas-ta-classic;
  5. a pinned public GitHub implementation with license and commit/version.
- For numerical correctness, compute actual values with
  `CanonicalClass(...).compute()` and expected values with the external
  reference library. Compare every output, NaN/warm-up placement, defaults,
  output ordering, and a parameter matrix. Include random, constant, monotonic,
  repeated-extrema, and minimum-length inputs. Test reset/replay, chunking,
  cold-start, and warmed continuation separately through the same class
  lifecycle. Oracle import/errors are failures, never skipped self-checks.
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

Before declaring an indicator complete, run its separate
`<class_name>_test.rs` Rust tests, separate `<class_name>_test.py` Python tests,
canonical Python lifecycle audit, and Python-class-versus-external-oracle
comparison. Run benchmark gates only when the user explicitly authorizes
benchmarking. Before repository-wide completion, run:

```bash
cargo test --workspace
uv run pytest -q
make check
cd verify && uv run python all_interfaces.py
cd verify && uv run python benchmark.py  # only with explicit user authorization
cargo fmt --all --check
git diff --check
```

Generated reports and README claims must agree with those results.

## Repository-wide naming and binding normalization

- Every indicator implementation and its tests use the same mechanically
  derived full-name path. Rust files are `<snake_case_class>.rs` with a
  matching `<snake_case_class>_test.rs`; Python files are
  `python/taflow/indicators/<snake_case_class>.py` with matching tests under
  `tests/`. A class may not be implemented in a differently named alias file.
- Python indicator modules under `taflow/indicators/` are the canonical adapter
  implementations. `taflow/indicators/__init__.py` and `taflow/__init__.py`
  are import surfaces only and may contain imports/re-exports, but no numerical
  logic or compatibility implementation. Each canonical class is exported from
  `taflow/indicators` and from the top-level `taflow` surface.
- Python adapters do not maintain a duplicate length counter. `__len__` must
  delegate to the native state (or an existing shared protocol), while Rust
  remains the source of truth for processed-bar counts and histories.
- The PyO3 binding follows the same one-file/one-class rule. A binding module
  such as `crates/taflow-python/src/indicators/candle_hikkake.rs` owns the
  `CandleHikkake` operator binding; do not place multiple public indicator
  classes in `state_api.rs`, `state_helpers.rs`, or another aggregation module.
  Binding `mod.rs` and `lib.rs` contain declarations, imports, and registration
  only.
- The canonical Rust core indicator directory is
  `crates/taflow-core/src/indicators/`; migrate legacy `stream/` modules and
  their matching tests there while preserving one class per file and updating
  declarations/re-exports. Do not leave duplicate implementations in both
  directories.
- Remove duplicate indicator modules and aliases when their names represent the
  same calculation (for example `rolling_vwap.py`,
  `rolling_volume_weighted_average_price.py`, and `vwap.py`). Keep the complete
  canonical class/module name and retain shorter names only as explicit oracle
  or metadata aliases. Update all exports, bindings, registries, tests, and
  documentation to point at that canonical implementation.

Good Python adapter style:

```python
class EqualHighsLows:
    """Causal equal-high/equal-low detection.

    Rust owns the persistent state and arithmetic; Python converts input
    containers once. ``append``, ``extend``, and ``reset`` return ``self``;
    ``value`` exposes the latest tuple and ``compute`` returns aligned arrays.
    Required inputs are ``high``, ``low``, and ``close``; warm-up is represented
    by ``NaN`` in history. The oracle/name mapping is pandas-ta ``equal_re``.
    """

    def __init__(self, high: Any, low: Any, close: Any, eq_len: int = 3) -> None:
        """Initialize and process aligned chronological input histories.

        Parameters
        ----------
        high, low, close : object
            Required aligned price histories; empty arrays create a fresh state.
        eq_len : int, default 3
            Equality lookback in bars.
        """
        self._state = _Native(eq_len)
        self.extend(high, low, close)

    def append(self, high: float, low: float, close: float) -> "EqualHighsLows":
        """Append one high/low/close bar and return this adapter."""
        self._state.append(float(high), float(low), float(close))
        return self
```

- Before each batch, reread this file, inspect `verify/FUNCTION_CHECKLIST.md`,
  scan all aliases with `rg`, select ten remaining or structurally nonconforming
  functions, refactor them, run correctness and interface/style checks, update
  the checklist, and push the verified commit to `main`. Do not run benchmark
  commands unless the user explicitly changes that instruction.
- Python correctness tests must compare the canonical class API against the
  selected independent target library. Generate deterministic random series
  (along with constant, monotonic, repeated-extrema, and minimum-length cases),
  compare every output and warm-up position with the target, and exercise reset,
  chunked `extend`, and scalar `append` through the same class. A lifecycle-only
  test is not sufficient when an external oracle exists.

## Batch execution clarifications

- Before each ten-indicator batch, reread this file and inspect the checklist,
  then independently scan the repository. The generated checklist is a useful
  report but is not authoritative: a row can be stale, checked too early, or
  miss a duplicate/alias. Verify every claimed completion from source files,
  exports, bindings, tests, and the external-oracle report before updating it.
- The canonical Rust implementation directory is
  `crates/taflow-core/src/indicators/`; legacy `stream/` files must be migrated
  as part of the normalization and must not remain as duplicate implementations.
  Python implementations live in `python/taflow/indicators/`, with matching
  snake_case implementation and test filenames. Bindings live in
  `crates/taflow-python/src/indicators/`, one public binding class per file.
- Every public Python adapter should follow this complete lifecycle shape; the
  native state remains the source of truth for arithmetic, history, and length:

```python
class PositiveVolumeIndex:
    """Persistent Positive Volume Index.

    Rust owns the persistent state and arithmetic; Python converts containers
    only. ``append``, ``extend``, and ``reset`` are fluent, ``value`` exposes
    the latest result, and ``compute`` returns aligned history. Required input
    histories are ``close`` and ``volume``; warm-up is represented by ``NaN``.
    The oracle/name mapping is TA-Lib ``PVI`` when available.
    """

    def __init__(self, close: Any, volume: Any) -> None:
        """Initialize the adapter and process aligned input histories."""
        self._state = _Native()
        self.extend(close, volume)

    def append(self, close: float, volume: float) -> "PositiveVolumeIndex":
        """Append one close/volume observation and return this adapter."""
        self._state.append(float(close), float(volume))
        return self

    def extend(self, close: Any, volume: Any) -> "PositiveVolumeIndex":
        """Append aligned close and volume series and return this adapter."""
        self._state.extend(as_float64_series(close), as_float64_series(volume))
        return self

    @property
    def value(self) -> float | None:
        """Return the latest native result, or ``None`` during warm-up."""
        return self._state.value

    def compute(self) -> np.ndarray:
        """Return the aligned native output history as a NumPy array."""
        return self._state.compute()

    def reset(self) -> "PositiveVolumeIndex":
        """Reset native state and return this adapter."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the processed-bar count delegated to native state."""
        return len(self._state)
```
