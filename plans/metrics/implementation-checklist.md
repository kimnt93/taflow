# Metrics implementation checklist for agents

This is the execution document. Before every implementation batch, reread
`AGENTS.md`, [the API contract](api-and-input-contract.md), the active phase in
[the metric catalog](metric-catalog.md), and this checklist. The catalog is a
report, not proof; independently scan the repository before checking a row.

Implement small batches. The first batch should contain the P0 foundation and
no more than these five metrics: `TotalReturn`, `AnnualizedReturn`,
`AnnualizedVolatility`, `MaximumDrawdown`, and `SharpeRatio`. Later batches may
contain up to ten metrics only after shared primitives and verifier gates are
stable.

## Before editing a metric

- [x] Resolve the full descriptive canonical class name. Reject abbreviations.
- [x] Search every possible existing name/alias in Rust, Python, tests, plans,
      verification, and docs.
- [x] Confirm it is a whole-history scalar metric rather than an existing
      aligned rolling indicator.
- [x] Read the pinned oracle source for the exact function and version.
- [x] Write the formula, input domains, annualization/rate convention,
      estimator/ddof, sign, minimum sample, NaN policy, and edge results in the
      registry entry before production code.
- [x] Mark an unavoidable oracle definition difference `VARIANT` before
      implementation; do not discover semantics by tuning until arrays match.
- [x] Identify reusable primitive state without moving the public formula into
      a shared helper.
- [x] Inspect the working tree and preserve unrelated user changes.

Example scan for Sharpe:

```bash
rg -n "SharpeRatio|sharpe_ratio|Sharpe|SHARPE|RollingSharpe" \
  crates python tests scripts verify plans
rg --files crates/taflow-metrics crates/taflow-python/src/metrics \
  python/taflow/metrics tests/metrics 2>/dev/null | sort
```

## File and class work

- [x] Add exactly one core production file
      `crates/taflow-metrics/src/metrics/<canonical_name>.rs`.
- [x] Add exactly one separate core test file with `_test.rs`.
- [x] Add exactly one PyO3 binding file under
      `crates/taflow-python/src/metrics/`.
- [x] Add exactly one Python adapter file under `python/taflow/metrics/`.
- [x] Add exactly one Python test file under `tests/metrics/`.
- [x] Update Rust/Python module declarations and re-exports only.
- [x] Register the native class only under `taflow._native.metrics`.
- [x] Export the public class from `taflow.metrics`; do not top-level-export it.
- [x] Add one metrics registry record with pinned oracle metadata.
- [x] Keep production files free of inline tests and package initializers free
      of logic.

## Core lifecycle

- [x] `new(...) -> TaResult<Self>` validates every configuration value.
- [x] Associated `from_*` constructors or a validated input-mode constructor
      select one native semantic domain without duplicating the metric formula.
- [x] `append(...)` accepts one observation in the selected domain.
- [x] `extend_slices_into`/equivalent bulk method uses the same state and checks
      paired lengths before mutation.
- [x] `value(&self)` returns the current `Option<f64>` or named value.
- [x] `compute(&mut/self)` does not replay processed inputs; exact tail metrics
      may refresh a dirty cache.
- [x] `reset(&mut self)` preserves allocation and input configuration.
- [x] `len()` reports valid derived metric observations from native state.
- [x] Fixed-statistic append performs no allocation after construction.
- [x] P&L/equity conversion happens in Rust and supports correct continuation.
- [x] Batch, chunks, and scalar replay leave identical post-run state/results.

## Python adapter

- [x] Normalize each supported container exactly once with shared adapters.
- [x] Do not calculate returns, P&L conversion, annualization, missing-value
      repair, or metric arithmetic in Python.
- [x] Release the GIL for native bulk work.
- [x] `append`, `extend`, and `reset` return the quoted concrete class type.
- [x] `value` and `compute` have `float | None` or explicit named-value types.
- [x] `__len__` delegates to native state.
- [x] Every accepted input method is documented, including later append semantics.
- [x] The docstring names the oracle/function/version and every semantic choice.
- [x] Unsupported domains do not appear as input methods.

## Correctness gates

- [x] Rust edge/lifecycle test passes.
- [x] Python adapter/lifecycle test passes.
- [x] Deterministic dataset matrix passes through public class API.
- [x] Parameter matrix passes against the independent oracle.
- [x] Return/equity/P&L/log-return equivalence passes when supported.
- [x] Scalar, chunked, warmed continuation, and reset/replay are invariant.
- [x] NaN and infinity behavior matches the contract.
- [x] The registry row says `MATCH` only for numerical parity; otherwise it
      says `VARIANT` with the exact reason.
- [x] Interface audit passes.

## Performance preparation (do not execute without authorization)

- [x] Add benchmark metadata only after correctness passes.
- [x] Choose the same library as the correctness oracle.
- [x] Define public end-to-end, native core, append, chunk, and conversion rows.
- [x] Ensure exact quantile memory is included.
- [x] Keep vectorbt JIT cold/warm rows separate if it is the selected oracle.
      System Quality Number remains benchmark-ineligible because vectorbt is
      unavailable; no NumPy timing is mislabeled as a vectorbt comparison.
- [x] Do not write speed claims or generated `BENCHMARK.md` rows before a real
      authorized run.

## After editing each metric

Adapt the names below:

```bash
# Review every implementation/binding class occurrence.
rg -n "(struct|class) SharpeRatio" crates python

# No public/free parallel implementation.
rg -n "(pub )?fn sharpe_ratio|def sharpe_ratio" \
  crates python tests scripts verify

# Production and aggregation surfaces contain no tests.
rg -n "#\[cfg\(test\)\]|#\[test\]|def test_" \
  crates/taflow-metrics/src/metrics/sharpe_ratio.rs \
  crates/taflow-python/src/metrics/sharpe_ratio.rs \
  python/taflow/metrics/sharpe_ratio.py \
  python/taflow/metrics/__init__.py

# Review external/short aliases; only oracle metadata may retain them.
rg -n "\bSharpe\b|SHARPE" crates python tests scripts verify docs plans
```

Then run focused tests/verifiers, `cargo fmt --all --check`,
`git diff --check`, `git diff --stat`, and inspect the complete diff for
unrelated/generated changes.

## Native metric pipeline

- [x] `MetricPipeline` owns one Rust `MetricInputState` and normalizes each
      return/log-return/equity/period-P&L observation once.
- [x] Selected compatible metric states are constructed and fanned out in
      Rust; Python performs container conversion and result mapping only.
- [x] Canonical standalone metric states remain the sole formula
      implementations.
- [x] Selection order is stable; duplicates and unsupported semantic domains
      are rejected before processing.
- [x] P&L pipeline values match standalone public metric classes.
- [x] Return, log-return, equity, and P&L input methods are equivalent.
- [x] Scalar, chunked, reset/replay, cached compute, and length behavior pass.
- [x] Bulk fan-out releases the GIL.
- [x] Paired, raw-total, trade-only, and matrix metrics remain explicit rather
      than being reinterpreted as one-return metrics.
- [x] Public README and the metric-pipeline guide document the API and limits.

## Batch completion

- [x] Re-scan class/file names and all aliases independently of the checklist.
- [x] Confirm `mod.rs`, `lib.rs`, and `__init__.py` are import surfaces only.
- [x] Confirm no production Python arithmetic or per-observation loop exists.
- [x] Confirm reports name actual oracle versions and do not mix lifecycle
      invariance with correctness.
- [x] Run all focused metric gates.
- [x] Run repository-wide gates when completing a release phase.
- [x] Update only rows actually proven complete.

## Decisions that require user/product approval before expansion

Stop and ask before:

- changing the existing indicator names or semantics;
- exporting metric classes at top-level `taflow`;
- raising the package's supported Python floor for an oracle dependency;
- adding automatic timestamp frequency inference;
- supporting external cash flows or cumulative P&L heuristics;
- adding portfolio optimization/backtesting/reporting scope;
- changing signed VaR/drawdown conventions after release;
- running benchmarks or publishing performance claims.
