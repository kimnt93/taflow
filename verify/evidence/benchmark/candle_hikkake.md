# CandleHikkake benchmark (`CDLHIKKAKE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 164.61M | 0.003 | 355.41M | 0.031 | 5.03× | 10.85× |
| 10,000 | 0.054 | 186.24M | 0.049 | 205.18M | 0.075 | 1.40× | 1.54× |
| 100,000 | 0.576 | 173.59M | 0.567 | 176.30M | 0.489 | 0.85× | 0.86× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.073 | 0.126 | 1.73× |
| 1 | 5 | 0.236 | 0.466 | 1.97× |
| 1 | 10 | 0.443 | 0.953 | 2.15× |
| 10 | 1 | 0.044 | 0.091 | 2.06× |
| 10 | 5 | 0.193 | 0.420 | 2.18× |
| 10 | 10 | 0.364 | 0.988 | 2.72× |
| 100 | 1 | 0.044 | 0.091 | 2.06× |
| 100 | 5 | 0.223 | 0.450 | 2.02× |
| 100 | 10 | 0.376 | 0.897 | 2.39× |
| 1,000 | 1 | 0.046 | 0.100 | 2.17× |
| 1,000 | 5 | 0.197 | 0.505 | 2.56× |
| 1,000 | 10 | 0.391 | 0.981 | 2.51× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
