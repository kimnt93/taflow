# Metrics architecture and repository layout

## Target layout

```text
crates/
├── taflow-core/                         # existing TA indicators
├── taflow-metrics/                      # new Rust metric domain
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs                       # declarations/re-exports only
│       ├── input/                       # shared domain conversion state
│       ├── primitives/                  # narrowly shared sufficient stats
│       └── metrics/
│           ├── mod.rs                   # declarations/re-exports only
│           ├── sharpe_ratio.rs
│           ├── sharpe_ratio_test.rs
│           ├── maximum_drawdown.rs
│           └── maximum_drawdown_test.rs
└── taflow-python/
    └── src/
        ├── lib.rs                       # registration only
        └── metrics/
            ├── mod.rs                   # registration/re-exports only
            ├── sharpe_ratio.rs
            └── maximum_drawdown.rs

python/taflow/
└── metrics/
    ├── __init__.py                      # imports/re-exports only
    ├── sharpe_ratio.py
    └── maximum_drawdown.py

tests/metrics/
├── sharpe_ratio_test.py
└── maximum_drawdown_test.py

scripts/verification/metrics/
├── registry.py
├── correctness.py
├── interfaces.py
└── benchmark.py

verify/metrics/
├── CORRECTNESS.md
├── BENCHMARK.md
└── evidence/
```

Add `crates/taflow-metrics` to the workspace. It may depend on `taflow-core`
(whose Rust crate name is currently `taflow`) for shared error/SIMD utilities,
but indicator code must never depend on metrics. `taflow-python` depends on
both crates. Do not create a second wheel or second extension shared library.

## Native Python module

Register metric binding classes under `taflow._native.metrics`, then wrap them
in `taflow.metrics`. Keeping a native submodule allows the same visible class
name to exist in separate domains without colliding with an existing indicator
binding. Python users should not need to import the native submodule directly.

Top-level `taflow.__init__` may expose only the package object:

```python
from . import metrics
```

It must not re-export every metric class. The canonical user imports are
`from taflow.metrics import SharpeRatio` and
`taflow.metrics.SharpeRatio`.

## One metric, one class, one implementation

Apply the indicator philosophy mechanically to metrics:

- `SharpeRatio` lives in `sharpe_ratio.rs` and `sharpe_ratio.py` in each
  relevant layer.
- Tests live in matching `_test` files, never inline in production files.
- Aggregation files contain declarations, imports, re-exports, and class
  registration only.
- No `fn sharpe_ratio(...)`, `def sharpe_ratio(...)`, compatibility wrapper,
  or second batch kernel is public or retained.
- Batch work is an inherent method of `SharpeRatio`; scalar and batch modes
  update the same state.
- Multi-field output uses a named `*Value` struct/dataclass, never an unlabeled
  tuple. Most planned metrics return one scalar.

The user's “one file one function” goal is therefore implemented in the
repository's class-only style as one file per canonical metric class, not as a
new functional API.

## Shared primitives are allowed, formulas are not duplicated

Narrow internal helpers should include only reusable state, for example:

- `MetricInputState`: returns, log returns, levels, or period P&L routing;
  return/path metrics convert period P&L with capital state, while P&L-native
  metrics receive raw P&L.
- `OnlineMoments`: count, mean, second central moment, and reset.
- `PairedMoments`: pairwise mean, covariance, and benchmark variance.
- `CompoundedGrowth`: stable `log1p` accumulation and total-loss state.
- `DrawdownState`: wealth, running peak, current drawdown, and maximum
  drawdown.
- `GainLossState`: positive/negative sums and counts.
- `DownsideMomentState`: required-return shortfall accumulator.
- `ExactOrderStatistics`: retained values and dirty cached selection for exact
  VaR/expected shortfall/tail metrics.

A helper must not decide a public metric's final formula or return sentinel.
For example, `DrawdownState` may expose maximum drawdown, while
`CalmarRatio` owns the Calmar division and undefined-result rule in
`calmar_ratio.rs`.

Do not create a public `MetricBase` that performs formulas in Python. Repeated
adapter boilerplate may be generated from metadata, but generated classes and
concrete return annotations must appear in their own files. The generator is a
development tool, not a runtime implementation.

## Persistent state and complexity

| Family | Persistent state | append | compute | retained history |
|---|---|---:|---:|---:|
| Mean/variance ratios | online moments | O(1) | O(1) | O(1) |
| Compound growth | log-growth/count | O(1) | O(1) | O(1) |
| Drawdown/path | wealth/peak/drawdown accumulators | O(1) | O(1) | O(1) |
| Benchmark regression | paired moments | O(1) | O(1) | O(1) |
| Gain/loss/trade quality | sums/counts/streaks | O(1) | O(1) | O(1) |
| Exact VaR/ES/tail ratio | observation vector + cache | amortized O(1) | O(n) when dirty | O(n) |
| Deflated/probabilistic Sharpe | moments through fourth order | O(1) | O(1) | O(1) |

Reserve capacity for a non-empty batch where possible. `reset` clears vectors
without shrinking them. Exact order-statistic metrics must disclose O(n)
memory; do not market them as bounded streaming metrics.

## Bulk and vectorization policy

`extend` receives contiguous slices and releases the GIL for the entire Rust
loop. Optimize in this order:

1. one Python-to-Rust call and no Python loop;
2. no temporary return array when level/P&L input can feed the metric state
   directly;
3. allocation-free chronological updates for fixed-state metrics;
4. LLVM auto-vectorization or existing portable runtime dispatch for truly
   independent pointwise/classification work;
5. specialized reduction only if it leaves state and output bitwise identical
   to scalar replay.

Welford updates, compounding, and drawdown are order-dependent recurrences.
Do not reassociate them merely to advertise SIMD. Benchmark before adding a
specialized path. A fast, single native chronological loop usually provides
the important Python-facing speedup.

## Configuration ownership

Keep configuration on the metric class and validate it in `new`:
annualization, risk-free/required rates, degrees of freedom, quantile cutoff,
and missing policy. The input input method selects an internal `MetricInputKind`.
The Rust state is the source of truth for processed length and converter
continuation state.

If many classes repeat a configuration shape, use a narrowly named internal
config value type. Do not expose a generic dictionary of parameters.

## Multi-metric computation (later phase)

After all individual classes are correct, add an orchestration class such as
`MetricSet` only if profiling shows repeated scans matter. It may:

- normalize input once;
- share read-only sufficient statistics;
- dispatch input once to several canonical states;
- return a typed mapping from class/name to scalar.

It must call or consume the canonical metric states and must not reproduce
Sharpe, drawdown, VaR, or any other formula. Do not block the first release on
this optimization.
