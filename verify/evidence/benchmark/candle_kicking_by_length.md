# CandleKickingByLength benchmark (`CDLKICKINGBYLENGTH` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.023 | 42.94M | 0.020 | 49.25M | 0.046 | 1.98× | 2.27× |
| 10,000 | 0.190 | 52.62M | 0.177 | 56.44M | 0.207 | 1.09× | 1.17× |
| 100,000 | 1.951 | 51.27M | 1.891 | 52.87M | 1.994 | 1.02× | 1.05× |
| 1,000,000 | 19.769 | 50.58M | 18.297 | 54.65M | 17.073 | 0.86× | 0.93× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.096 | 0.174 | 1.80× |
| 1 | 5 | 0.330 | 0.593 | 1.80× |
| 1 | 10 | 0.635 | 1.103 | 1.74× |
| 10 | 1 | 0.067 | 0.097 | 1.45× |
| 10 | 5 | 0.389 | 0.648 | 1.67× |
| 10 | 10 | 0.719 | 1.142 | 1.59× |
| 100 | 1 | 0.061 | 0.103 | 1.69× |
| 100 | 5 | 0.379 | 0.595 | 1.57× |
| 100 | 10 | 0.708 | 1.097 | 1.55× |
| 1,000 | 1 | 0.079 | 0.117 | 1.49× |
| 1,000 | 5 | 0.374 | 0.768 | 2.05× |
| 1,000 | 10 | 0.708 | 1.218 | 1.72× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
