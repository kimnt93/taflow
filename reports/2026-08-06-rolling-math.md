# Rolling math state validation — 2026-08-06

## Implemented functions

| Function | Stateful method | Warm-up output | Complexity |
|---|---|---|---|
| MAX | monotonic maximum queue | `None` / NumPy `NaN` | amortized O(1) |
| MIN | monotonic minimum queue | `None` / NumPy `NaN` | amortized O(1) |
| SUM | rolling accumulator and fixed window | `None` / NumPy `NaN` | O(1) |
| MINMAX | fused maximum/minimum queues | paired `None` / `NaN` | amortized O(1) |
| MAXINDEX | exact tracked index with expiry rescan | `0`, matching TA-Lib | amortized O(1) |
| MININDEX | exact tracked index with expiry rescan | `0`, matching TA-Lib | amortized O(1) |
| MINMAXINDEX | fused exact index tracking | paired `0`, matching TA-Lib | amortized O(1) |

All functions expose Rust and Python `append`, `extend`, `value`, and `reset`.
Index outputs are absolute series indices, matching the original TA-Lib API.

## Correctness evidence

| Test | Series | Result |
|---|---:|---|
| Rust per-bar parity with duplicate extrema and expiry rescans | 17 values, period 4 | pass |
| Python scalar extend and append/reset parity | 128 values, period 7 | pass |
| Python paired-output duplicate/tie parity | 17 values, period 4 | pass |
| Complete exhaustive batch plus state suite | 295 tests | pass |
| Rust workspace | 63 tests | pass |

The duplicate series specifically exercises TA-Lib's index rule: initial and
expiry scans retain the first equal extremum, while a newly appended equal
extremum replaces the tracked index on the fast path.

## One-million-update benchmark

Criterion `--quick`, release build, period 20, after 10,000 warm-up values.
The range includes initialization.

| Function | Total time | Approx. ns/update |
|---|---:|---:|
| MAX | 11.83–12.21 ms | 11.9 |
| MAXINDEX | 18.70–18.81 ms | 18.7 |
| MIN | 12.31–12.61 ms | 12.4 |
| MININDEX | 18.67–19.00 ms | 18.7 |
| SUM | 3.36–3.38 ms | 3.4 |
| MINMAX | 15.97–16.33 ms | 16.0 |
| MINMAXINDEX | 18.06–18.16 ms | 18.1 |
