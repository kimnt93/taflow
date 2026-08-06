# Adaptive moving-average and oscillator report — 2026-08-06

## Implemented functions

| Function | Incremental method | Oracle series | Result |
|---|---|---:|---|
| MAMA/FAMA | four-bar WMA smoother, alternating even/odd Hilbert transforms, phase-driven alpha | 128 values | pass |
| T3 | six cascaded SMA-seeded EMA layers and Tillson coefficient combination | 128 values | pass |
| APO | fast MA minus slow MA | 200 values × 9 MA types | pass |
| PPO | normalized fast/slow MA difference | 200 values × 9 MA types | pass |

APO and PPO cover SMA, EMA, WMA, DEMA, TEMA, TRIMA, KAMA, MAMA, and T3.
Their tests verify aligned warm-up, bulk `extend`, scalar `append`, `value`, and
reset/replay behavior against original TA-Lib 0.7.1.

Each new public indicator is isolated in its own English-documented source
file and re-exported by `stream/mod.rs`.  The private `moving_average.rs`
dispatcher contains only shared routing used by APO and PPO.

## Verification

| Gate | Result |
|---|---|
| Rust batch parity for APO/PPO across all MA types | pass |
| `cargo test --workspace` | 72 passed |
| `python -m pytest tests/test_stateful.py -q` | 85 passed |
| exhaustive batch plus state suite | 334 passed |

## Streaming benchmark

Criterion `--quick`; each measurement initializes from 10,000 values and then
processes 1,000,000 appended values.

| Function | Total time | Approx. ns/bar |
|---|---:|---:|
| MAMA(0.5, 0.05) | 45.18–46.39 ms | 46.1 |
| T3(20, 0.7) | 21.25–21.27 ms | 21.3 |
| APO(12, 26, EMA) | 9.49–9.71 ms | 9.7 |
| PPO(12, 26, EMA) | 9.49–9.65 ms | 9.6 |
