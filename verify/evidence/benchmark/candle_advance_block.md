# CandleAdvanceBlock benchmark (`CDLADVANCEBLOCK` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.236 | 4.25M | 0.213 | 4.69M | 0.046 | 0.19× | 0.22× |
| 10,000 | 2.071 | 4.83M | 2.055 | 4.87M | 0.210 | 0.10× | 0.10× |
| 100,000 | 20.326 | 4.92M | 20.786 | 4.81M | 1.800 | 0.09× | 0.09× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.133 | 0.142 | 1.07× |
| 1 | 5 | 0.470 | 0.478 | 1.02× |
| 1 | 10 | 0.632 | 0.899 | 1.42× |
| 10 | 1 | 0.070 | 0.092 | 1.31× |
| 10 | 5 | 0.321 | 0.441 | 1.37× |
| 10 | 10 | 0.672 | 0.900 | 1.34× |
| 100 | 1 | 0.092 | 0.090 | 0.98× |
| 100 | 5 | 0.308 | 0.454 | 1.47× |
| 100 | 10 | 0.673 | 0.904 | 1.34× |
| 1,000 | 1 | 0.300 | 0.111 | 0.37× |
| 1,000 | 5 | 0.668 | 0.533 | 0.80× |
| 1,000 | 10 | 0.896 | 1.128 | 1.26× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
