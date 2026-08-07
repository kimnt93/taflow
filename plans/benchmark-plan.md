# Python-interface benchmark plan: TAFlow vs TA-Lib

Successor to the ad-hoc scripts in `benches/python_benches/` and the
schema-v2 reports. One harness, one function registry, four scenarios,
per-function reports plus one aggregate report.

## Ground rules

- **Python interface only.** Both sides are measured through their public
  Python APIs: `talib.SMA(...)` (C TA-Lib via its Python binding) versus
  `taflow.talib.SMA(...)` and the taflow state classes. We never benchmark
  the Rust core directly against C — the product is the Python surface, so
  boundary cost is part of the number. (Criterion Rust-core benches stay,
  but they are out of scope for these reports.)
- **TA-Lib is the baseline.** Every comparable cell reports
  `speedup = talib_time / taflow_time` (>1 means TAFlow is faster).
- **Averages over repeated runs.** Default `repeats = 20` for the timed
  bulk cells (expensive one-shot baselines like the S2 full recompute and
  S5 thread sweeps cap at 3), each cell in a forked child process (the
  isolated-process pattern kills cache/JIT/allocator carryover between
  modes). Within a repeat: 1 untimed
  warm-up call, `gc.disable()`, `time.perf_counter_ns()`. Report the
  **mean** as the headline (user contract) and keep min/p50 in the JSON.
- **Deterministic data.** Seeded random-walk OHLCV
  (`make_ohlcv(n, seed=42+n)` as today). Same arrays feed both libraries in
  every scenario.

## Scenarios

### S1 — Bulk compute

Sizes: **100, 1 000, 10 000, 100 000, 1 000 000, 10 000 000**.

Modes per size:

| Mode | What is timed |
|---|---|
| `talib_batch` | `talib.FN(*arrays, **params)` |
| `taflow_batch` | `taflow.talib.FN(*arrays, **params)` |
| `taflow_state_cold` | construct state + `extend(arrays)` — the "backfill from empty" path |

Metrics per cell: mean wall ms, **ops/sec** (`bars / mean_seconds`),
speedup vs `talib_batch`. Latency percentiles are redundant with ops/sec
here (single call over one array) — skipped, per the contract.

The 10M size is guarded by `--max-size` (default includes it; CI quick mode
stops at 1M). 10M float64 is 80 MB/series — fine on a dev box, wasteful in
CI.

### S2 — Warm-up / live continuation

This is the headline realtime scenario: a state that already holds a base
history receives a small chunk of new bars.

Grid: base history **H ∈ {100, 1 000, 10 000, 100 000, 1 000 000,
10 000 000}** × append chunk **k ∈ {1, 10, 100, 1 000}**.

- **TAFlow**: build the state once, feed H bars (untimed), then time
  `append(x)` for k = 1 or `extend(chunk)` for k > 1. Repeat the timed
  update many times (fresh data each time, state keeps rolling forward) to
  get a distribution.
- **TA-Lib baseline**: TA-Lib has no state, so the honest live-update cost
  is *recompute over the window it needs*: `talib.FN(history[-(H+k):])` on
  the full H+k array. That O(H) vs O(k) gap **is the product claim**, so it
  must be measured, not asserted. (The JSON also records
  `talib_tail_window` — recompute over just the last `lookback+k` bars — as
  a fairer "expert user" baseline; the md table shows the full-recompute
  number, the JSON has both.)

Metrics per cell: mean **latency per update** (µs) for both sides,
**updates/sec** for taflow, speedup. For k = 1 also record p50/p99 append
latency (this is the one place latency is not redundant — tail latency of a
single `append` is a realtime SLA number that ops/sec hides).

Expectation to verify: taflow latency is flat in H (O(1) state), talib
latency grows linearly in H — the table makes the crossover visible.

### S3 — Correctness vs TA-Lib

At n = 100 000 (one size is enough; warm-up bugs show at any size):

- `taflow_batch` vs `talib_batch`: NaN-placement mismatches, max absolute
  error, `allclose(rtol=1e-8, atol=1e-10)` per output.
- `taflow_state_cold` (extend over full array) vs `talib_batch`: same
  checks — proves the stateful path matches, not just the batch path.
- **Chunk invariance**: feed the same array via chunk sizes {1, 10, 1 000};
  all three output sequences must be bitwise identical to the single-extend
  result. This is taflow-vs-taflow but belongs here because it validates
  that S2 numbers describe the same math as S1.

A function's report is marked `correctness: PASS` only if all checks pass;
speed tables still render on failure but the md gets a red warning line.

### S5 — Parallel-thread continuation

Models an N-symbol live feed: N Python threads, each owning its own warmed
state (taflow) or its own recompute loop (talib), all updating
concurrently. Thread counts **{1, 2, 5, 10, 20}**, base history 100k bars
per thread, chunk = 1.

Metrics per thread count: aggregate updates/sec across threads, scaling
factor vs the 1-thread row, and taflow-vs-talib speedup at equal thread
count. Scaling above 1× requires the underlying native call to release the
GIL — measured result (2026-08-07): **neither taflow (PyO3 holds the GIL)
nor TA-Lib's binding scales with threads; both are flat ~1× to 20
threads.** TAFlow's multi-symbol advantage is therefore its ~1000×+ lower
per-update cost, not parallelism. Releasing the GIL inside bulk `extend`
is the corresponding taflow improvement (optimize-methods §2.3); scalar
`append` is too short (~0.2 µs) to benefit from GIL release.

### S4 — Functions with no TA-Lib counterpart

Same S1 sizes and S2 grid, minus the talib modes and speedup columns.
Correctness becomes self-oracle only: state-vs-batch parity and chunk
invariance (the S3 machinery minus the talib comparison). The report
template renders "baseline: none (not in TA-Lib)" so aggregate tooling can
filter them.

## Zero-edit extensibility (the registry)

The current `FUNCTIONS = {...}` dict in `benchmark_function_reports.py`
must die: it means every new indicator needs a bench-script edit.

Replace it with discovery from taflow's own metadata. The Rust side already
generates per-function metadata (`crates/taflow-python/src/metadata.rs`);
expose it as `taflow.metadata.functions()` returning one record per public
function:

```python
{
  "name": "EMA",                    # uppercase TA-Lib alias
  "class": taflow.ExponentialMovingAverage,
  "batch": taflow.talib.EMA,
  "params": {"timeperiod": 20},     # benchmark defaults = TA-Lib defaults
  "inputs": ("close",),             # which OHLCV series it consumes
  "outputs": 1,
  "in_talib": True,                 # False → scenario S4
  "input_domain": "prices",         # prices | unit | signed | positive | periods
}
```

The harness iterates this registry — **adding a new indicator requires zero
benchmark-code changes**, because the metadata record is already mandatory
for the Python class to exist at all. `input_domain` covers the special
input ranges the old scripts hand-coded (`close_safe` for ACOS/ASIN,
positive series for LN/SQRT, `periods` array for MAVP); the data generator
owns the domain → array mapping.

Escape hatch: `benches/overrides.py`, a small optional dict keyed by
function name for genuinely irregular cases (extra param sets to sweep,
custom talib call shape like MAVP's second array). It must stay short; if
an override is needed routinely, the metadata schema is missing a field.

Per-function param policy: benchmark at TA-Lib defaults only. Param sweeps
(period 5/20/200) are a later, separate report — they cube the matrix.

## Runner CLI

```
python -m benches.run [FN ...]        # default: every function in registry
    --repeats 5
    --max-size 10_000_000             # S1/S2 size cap
    --quick                           # sizes ≤ 100k, repeats 3, skip 10M
    --scenarios s1,s2,s3              # default: all applicable
    --reports-dir reports/
```

Per-function wall budget at full settings should stay under ~2 minutes;
`--quick` is the CI/pre-commit mode.

## Outputs

### `reports/<FN>.json` — machine-readable (schema v3)

```json
{
  "schema_version": 3,
  "function": "EMA",
  "in_talib": true,
  "date": "2026-08-06",
  "environment": {
    "platform": "Linux-6.18.7-x86_64", "python": "3.12.3",
    "numpy": "2.5.1", "taflow": "0.1.2", "talib_python": "0.7.1",
    "cpu": "…", "rustflags": "…"
  },
  "protocol": {"repeats": 5, "seed_base": 42, "isolated_processes": true},
  "correctness": {
    "oracle": "TA-Lib 0.7.1", "bars": 100000,
    "batch_vs_talib":  {"max_abs_error": 1.4e-14, "nan_mismatches": 0, "passed": true},
    "state_vs_talib":  {"max_abs_error": 0.0, "nan_mismatches": 0, "passed": true},
    "chunk_invariance": {"chunks": [1, 10, 1000], "bitwise_identical": true},
    "passed": true
  },
  "bulk": [
    {"size": 1000000,
     "talib_batch":       {"mean_ms": 2.855, "min_ms": 2.71, "p50_ms": 2.81, "ops_per_sec": 3.50e8},
     "taflow_batch":      {"mean_ms": 2.306, "ops_per_sec": 4.34e8, "speedup": 1.24},
     "taflow_state_cold": {"mean_ms": 6.256, "ops_per_sec": 1.60e8, "speedup": 0.46}}
  ],
  "continuation": [
    {"base": 1000000, "chunk": 1,
     "taflow": {"mean_latency_us": 0.24, "p50_us": 0.22, "p99_us": 0.41, "updates_per_sec": 4.2e6},
     "talib_full_recompute": {"mean_latency_us": 2855.0},
     "talib_tail_window":    {"mean_latency_us": 8.1},
     "speedup_vs_full": 11895.0, "speedup_vs_tail": 33.7}
  ]
}
```

Full raw samples per cell go under a `"samples"` key (kept in JSON only,
never in md).

### `reports/<FN>.md` — human-readable

```markdown
# EMA — benchmark vs TA-Lib 0.7.1

Correctness: **PASS** (max abs err 1.4e-14 @100k bars; NaN placement exact;
chunk replay bitwise-identical for chunks 1/10/1000).

## Bulk compute (mean of 5 runs)

| Bars | TA-Lib ms | TAFlow batch ms | Speedup | TAFlow ops/s | State-cold ms | Speedup |
|---:|---:|---:|---:|---:|---:|---:|
| 100 | 0.0053 | 0.0046 | 1.15× | 21.7M | 0.0072 | 0.74× |
| 1 000 | 0.0086 | 0.0073 | 1.18× | 137M | 0.0098 | 0.88× |
| 10 000 | 0.0302 | 0.0251 | 1.20× | 398M | 0.0630 | 0.48× |
| 100 000 | 0.277 | 0.238 | 1.16× | 421M | 0.594 | 0.47× |
| 1 000 000 | 2.855 | 2.306 | 1.24× | 434M | 6.256 | 0.46× |
| 10 000 000 | 29.1 | 23.5 | 1.24× | 426M | 63.0 | 0.46× |

## Live continuation (latency per update, mean; TA-Lib = full recompute)

| Base bars | Chunk | TAFlow µs | TA-Lib µs | Speedup | TAFlow updates/s |
|---:|---:|---:|---:|---:|---:|
| 100 | 1 | 0.24 | 5.3 | 22× | 4.2M |
| 10 000 | 1 | 0.24 | 30.2 | 126× | 4.2M |
| 1 000 000 | 1 | 0.24 | 2 855 | 11 895× | 4.2M |
| 1 000 000 | 1 000 | 89 | 2 861 | 32× | 11.2M bars/s |

Append latency (base 1M, chunk 1): p50 0.22 µs, p99 0.41 µs.
```

(Numbers above are illustrative, formatted from the current MA report's
real magnitudes.)

### `reports/BENCHMARK.md` — aggregate

Regenerated by `python -m benches.aggregate` from the JSON files only (never
re-runs anything). One row per function; enables spotting outliers at a
glance:

```markdown
| Function | In TA-Lib | Correct | Bulk speedup @1M | Bulk ops/s @1M | Append p50 | Speedup @1M base | 
|---|---|---|---:|---:|---:|---:|
| EMA | yes | PASS | 1.24× | 434M | 0.22 µs | 11 895× |
| SMA | yes | PASS | 1.31× | 468M | 0.21 µs | 12 400× |
| SUPERTREND | no | self-PASS | — | 122M | 0.35 µs | — |
```

Plus summary stats: median bulk speedup, functions <1× (regression list),
correctness failures, missing reports vs the checklist inventory.

## Migration and gates

1. ✅ Done (2026-08-07): single runner `benches/bench.py` with
   registry-driven discovery, S1/S2/S3/S4/S5, and per-function + aggregate
   reports. The legacy scripts (`bench_all_indicators.py`,
   `bench_vs_talib.py`, `generate_report.py`,
   `benchmark_function_reports.py`) are deleted — their measurement ideas
   are absorbed; overlapping scripts were how the FUNCTIONS drift happened.
2. Existing schema-v2 JSONs are regenerated, not migrated.
3. Checklist gate addition: a function is done only when
   `reports/<FN>.md` + `.json` exist at schema v3 — which, with the
   registry, requires no bench-code work, only running the runner.
4. Known caveat to state in every report footer: results measure the
   *Python-visible* cost including conversion/boundary overhead by design;
   Rust-core-only numbers live in criterion benches and are not comparable.
```
