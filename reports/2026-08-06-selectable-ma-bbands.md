# Selectable MA and Bollinger Bands report — 2026-08-06

## Implemented functions

| Function | Incremental method | Oracle series | Result |
|---|---|---:|---|
| MA | dispatches each bar to the selected concrete MA state | 200 values × 9 MA types | pass |
| BBANDS | selected middle MA plus rolling population variance around the window SMA | 200 values × 9 MA types | pass |

Both functions cover SMA, EMA, WMA, DEMA, TEMA, TRIMA, KAMA, MAMA, and T3.
The BBANDS outer bands intentionally do not use the selected middle MA as their
deviation center; this matches original TA-Lib behavior for non-SMA types.

The public implementations and their Rust parity tests live in separate
English-documented `stream/ma.rs` and `stream/bbands.rs` files. `stream/mod.rs`
contains only declarations and re-exports for these indicators.

## Verification

| Gate | Result |
|---|---|
| Rust batch parity over all nine MA types | pass |
| Python `extend` and reset/replay against TA-Lib 0.7.1 | 18 passed |
| `cargo test --workspace` | 74 passed |
| `python -m pytest tests/test_stateful.py -q` | 103 passed |
| exhaustive batch plus state suite | 352 passed |

## Streaming benchmark

Criterion `--quick`; each sample initializes from 10,000 values and processes
1,000,000 appended values.

| Function | Total time | Approx. ns/bar |
|---|---:|---:|
| MA(20, EMA) | 4.84–5.01 ms | 5.0 |
| BBANDS(20, 2, 2, SMA) | 17.26–17.65 ms | 17.3 |
