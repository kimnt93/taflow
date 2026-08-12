# Metrics testing and verification plan

## Required layers

Every metric needs three independent kinds of evidence:

1. Rust unit tests establish state-machine behavior and exact edge rules.
2. Python tests establish the public adapter, input methods, containers, and
   lifecycle.
3. The metrics correctness verifier compares the public Python class to the
   pinned external oracle.

An internal Rust formula compared with the same formula in a unit test is not
external correctness. Batch/scalar equality is an invariant, not an oracle.

## Rust test requirements per metric

The matching `<metric>_test.rs` file must cover:

- validated construction and every invalid parameter;
- minimum input length minus one, exact minimum, and minimum plus one;
- scalar append and latest `value`;
- one-shot `extend_slice`;
- multiple chunk boundaries, including empty chunks;
- one-shot versus scalar replay state/result equality;
- warmed continuation after a batch;
- reset and replay without capacity loss where observable;
- NaN policy and infinity rejection;
- signed zero and zero-denominator behavior;
- clone/debug behavior if those traits are public;
- P&L or equity conversion continuation when that input method is supported;
- paired length validation before mutation for benchmark metrics.

Use bitwise equality for paths promised bitwise invariant. If an oracle
tolerance is necessary, it belongs only in external numerical comparison and
must not weaken native lifecycle invariance.

## Python test requirements per metric

The matching `tests/metrics/<metric>_test.py` file must instantiate the
canonical class and cover:

- import from `taflow.metrics` and no accidental top-level class export;
- `from_returns` with NumPy and Python list input;
- supported `from_equity`, `from_pnl`, `from_log_returns`, or `from_trades`;
- fluent concrete returns from `append`, `extend`, and `reset`;
- `value`, `compute`, and `__len__` before and after each lifecycle action;
- constructor input method with an empty series for fresh streaming;
- scalar, chunked, and one-shot equality;
- reset/replay equality;
- list/NumPy/pandas/Polars/Arrow container equivalence where dependencies are
  available in the required development environment;
- one-column dataframe acceptance and multi-column ambiguity rejection;
- paired input length/alignment failures without partial mutation;
- documentation signature and concrete type annotations.

Python tests must not calculate the metric formula as their correctness
reference. They may use literal hand-calculated cases for edge behavior, but
the external comparison belongs in the verifier.

## Deterministic dataset matrix

Every external-oracle row uses all applicable datasets:

- empty, singleton, two observations, and exact minimum length;
- all zero;
- constant positive and constant negative;
- alternating equal gains/losses;
- monotonic equity growth and monotonic equity decline;
- one early deep drawdown, one late deep drawdown, and repeated equal peaks;
- all wins, all losses, and wins/losses/breakevens;
- near-zero variance with a large mean;
- one `-100%` terminal return;
- deterministic normal returns;
- skewed returns;
- fat-tailed returns with deterministic injected shocks;
- autocorrelated returns;
- NaNs at start, middle, and end under each supported missing policy;
- benchmark identical to primary, constant benchmark, inverse benchmark, and
  mismatched length;
- P&L with growing capital, eroding capital, zero P&L, terminal ruin, and
  invalid continuation after ruin;
- closed trades with positive, negative, and zero outcomes and long streaks.

Use fixed seeds and store dataset generation in the metrics verifier, not in
each production module.

## Cross-input equivalence fixtures

Construct one valid equity path and derive consistent representations outside
the TAFlow actual path:

```text
equity: [100, 102, 99.96, 104.958]
returns: [0.02, -0.02, 0.05]
period P&L: [2, -2.04, 4.998] with initial_capital=100
log returns: log1p(returns)
```

For every return/path metric, assert that:

```python
Metric().from_returns(returns).compute()
Metric().from_equity(equity).compute()
Metric().from_pnl(pnl, initial_capital=100.0).compute()
Metric().from_log_returns(log_returns).compute()
```

agree under the metric's tolerance and leave continuation in an equivalent
state. The actual TAFlow value must still be oracle-checked through
`from_returns`; cross-input agreement alone proves only conversion invariance.

## Oracle verifier organization

Create `scripts/verification/metrics/registry.py` with one explicit record per
class. Do not reuse the indicator registry: scalar output, input methods,
annualization, and missing policies are different contracts.

Each entry names:

- canonical class and module;
- phase/family;
- supported input methods;
- input method arguments and metric configuration;
- output type and minimum observations;
- oracle package/version/function/source;
- oracle argument transformation;
- output normalization and tolerance;
- expected `MATCH` or documented `VARIANT`;
- benchmark eligibility (false until correctness passes).

The correctness executable must fail, not skip, when a required oracle cannot
be imported. It writes `verify/metrics/CORRECTNESS.md` atomically only after
the complete selected run succeeds. Evidence files include parameters, versions,
maximum absolute/relative errors, dataset cases, and lifecycle results.

## Parameter matrices

At minimum test:

- `periods_per_year`: 1, 12, 52, 252, 365, 8760;
- annual risk-free rate: 0, positive, and negative above -1;
- annual required return: 0 and non-zero;
- degrees of freedom: 0 and 1 where supported, default fixed at 1 otherwise;
- VaR/ES cutoff: 0.01, 0.05, 0.10, 0.50;
- missing policy: omit and raise;
- benchmark metrics with both annualized and unannualized modes.

Do not add parameters solely because an oracle has them. For example,
Empyrical annual volatility's Levy alpha remains outside the P1 contract.

## Tolerances

Start with strict per-metric tolerances in the registry:

- counts, streaks, signs, None/finite/infinite classification: exact;
- simple sums/means and paired moments: `rtol=1e-13`, `atol=1e-15` where
  oracle order matches;
- compounded growth/drawdown: `rtol=1e-12`, `atol=1e-14`;
- ratios/regression: `rtol=1e-11`, `atol=1e-13`;
- quantiles/order statistics: exact when NumPy interpolation and filtering are
  identical, otherwise `rtol=1e-13`, `atol=1e-15`.

Relax only with a written numerical reason and an adversarial regression test.
Never use a loose global tolerance to hide a definition mismatch.

## Interface audit

`scripts/verification/metrics/interfaces.py` must check every exported class:

- class lives in a mechanically matching snake-case module;
- exactly one public class implementation exists per layer;
- no same-named free metric function exists;
- class is exported from `taflow.metrics` only;
- input methods require their series and have descriptive parameters;
- append/extend/reset return the concrete class;
- value/compute return type is `float | None` or an explicit named value;
- `__len__` delegates to native state;
- docstrings state formula, input domains, annualization, sign, missing-data,
  minimum sample, undefined behavior, and oracle mapping;
- package initializers contain imports/re-exports only;
- production Python performs no series arithmetic or Python bar loop;
- native bulk methods release the GIL.

## Metric-specific edge table

| Metric | Required edge result |
|---|---|
| Total/annualized return | empty `None`; terminal -100% handled; below -100% rejected for compounding |
| Annualized volatility | fewer than 2 `None`; constant series returns `0.0` |
| Sharpe/Sortino | insufficient or zero denominator `None` |
| Maximum drawdown | no decline returns `0.0`; result never positive |
| Calmar | maximum drawdown zero returns `None` |
| Omega | no below-threshold mass returns `None` |
| VaR/ES | empty `None`; cutoff strictly in `(0, 1)` |
| Beta | zero benchmark variance returns `None` |
| Information ratio | zero active-return deviation returns `None` |
| Capture ratios | zero/undefined benchmark CAGR returns `None` |
| Win/average win/average loss | empty subset returns `None`; zero is breakeven |
| Profit factor | gains and no losses `+inf`; no gains and no losses `None`; losses only `0.0` |
| Streaks | empty `None`; non-empty with no matching observations returns `0` |

Freeze this table before implementation. If an external library disagrees,
normalize only for documented sentinel behavior or record a `VARIANT`.

## Required commands before a metric is complete

Adapt package/test filters to the implemented class:

```bash
cargo test -p taflow-metrics sharpe_ratio
uv run pytest -q tests/metrics/sharpe_ratio_test.py
uv run python scripts/verification/metrics/interfaces.py --metric SharpeRatio
uv run python scripts/verification/metrics/correctness.py --metric SharpeRatio
cargo fmt --all --check
git diff --check
```

Before a metrics release, run the existing repository gates plus the full
metrics verifier:

```bash
cargo test --workspace
uv run pytest -q
make check
uv run python scripts/verification/interfaces.py
uv run python scripts/verification/metrics/interfaces.py
uv run python scripts/verification/metrics/correctness.py
cargo fmt --all --check
git diff --check
```

Do not run `scripts/verification/metrics/benchmark.py` until the user explicitly
authorizes benchmarking.
