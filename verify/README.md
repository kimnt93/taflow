# taflow verification project

Standalone [uv](https://docs.astral.sh/uv/) project that checks and benchmarks
every canonical taflow class. External TA-Lib is an independent oracle and
baseline only; taflow does not provide or import a `taflow.talib` namespace.

## Protocol (per function)

With 10,000 seeded bars:

1. **Oracle full pass** — reference library over all 10k bars
   (TA-Lib for TA-Lib-named functions; pandas for `rolling_*`/`ewm_*`;
   self-oracle when no reference exists).
2. **Warm-up / continue** — feed the first **9,000** bars into the
   persistent state (`extend`), then continue with the last **1,000** bars
   through scalar `append` calls (the live-update path). The concatenated
   9k+1k output is compared to:
   - the oracle 10k result;
   - the one-shot native result, bitwise, including repeated `extend` chunks.

The mapping is read from the master table in `../CHECK.md`: descriptive
CamelCase classes are paired explicitly with original TA-Lib names. Missing
classes, constructor mappings, and input mappings are reported, never skipped.

## Performance protocol

`benchmark.py` uses the same registry and produces per-function JSON/Markdown:

- whole-vector native `extend` (the compiled Rust SIMD-capable path) versus
  original TA-Lib over the identical contiguous arrays;
- construction plus warm-up history;
- continuation after warm-up, for scalar and chunked updates, versus TA-Lib
  full-history and lookback-window recomputation;
- independent-stream thread scaling for native vector and continuation paths;
- correctness before performance, with raw timing samples retained in JSON.

The report calls the path SIMD-*capable*, not unconditionally SIMD: actual CPU
features depend on the wheel and `RUSTFLAGS` used to build it.

Verdict is `MATCH` when NaN placement is identical and values agree within
`rtol=1e-8, atol=1e-10`; the report also records the max absolute error so
tolerance-scale drift is visible even on matches.

## Run

```bash
cd verify
uv sync              # builds taflow from the repo root via maturin
uv run python verify.py            # all functions -> REPORT.md
uv run python verify.py EMA ATR    # subset
uv run python verify.py --bars 10000 --warmup-split 9000
uv run python benchmark.py EMA ATR --quick
uv run python benchmark.py --quick     # every CHECK.md function
uv run python benchmark.py --list
```

Output: `verify/REPORT.md` (summary + one row per function/output) and
`verify/report.json` (machine-readable detail).

Benchmark output is written under `verify/benchmark_reports/`, including the
aggregate `BENCHMARK.md`. The historical `python benches/bench.py ...` command
is retained as a launcher for this same runner.

Independent extension oracles (pandas-ta-classic, Polars, and
smartmoneyconcepts):

```bash
uv sync --extra extra-oracles
uv run python external_oracles.py
uv run python external_oracles.py --oracle polars --bars 10000
```

Output: `EXTERNAL_ORACLES.md` and `EXTERNAL_ORACLES.json`. The runner records
each output separately and exits non-zero for unexplained mismatches or oracle
errors. Intentional contract differences are reported as `VARIANT` rather than
being silently counted as matches (for example, taflow's causal SMC swing
events versus the package's retroactive lookahead markers).

`source_comparison.py` merges the primary and external JSON artifacts into
`SOURCE_COMPARISON.md`. Its `INVARIANT` verdict means only that native batch,
append, and chunked execution agree; it is never presented as external parity.
