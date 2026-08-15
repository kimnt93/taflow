# CandleStickSandwich benchmark (`CDLSTICKSANDWICH` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 141.41M | 0.003 | 302.43M | 0.033 | 4.73× | 10.11× |
| 10,000 | 0.046 | 217.30M | 0.040 | 249.13M | 0.088 | 1.91× | 2.19× |
| 100,000 | 0.609 | 164.10M | 0.570 | 175.44M | 0.610 | 1.00× | 1.07× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.144 | 0.180 | 1.26× |
| 1 | 5 | 0.279 | 0.461 | 1.65× |
| 1 | 10 | 0.392 | 0.900 | 2.30× |
| 10 | 1 | 0.042 | 0.089 | 2.13× |
| 10 | 5 | 0.188 | 0.450 | 2.39× |
| 10 | 10 | 0.448 | 0.904 | 2.02× |
| 100 | 1 | 0.042 | 0.088 | 2.07× |
| 100 | 5 | 0.185 | 0.448 | 2.42× |
| 100 | 10 | 0.427 | 0.984 | 2.30× |
| 1,000 | 1 | 0.054 | 0.096 | 1.79× |
| 1,000 | 5 | 0.197 | 0.459 | 2.33× |
| 1,000 | 10 | 0.430 | 0.973 | 2.26× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
