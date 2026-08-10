# RollingMinimumIndex benchmark (`MININDEX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 110.55M | 0.007 | 148.03M | 0.039 | 4.35× | 5.82× |
| 10,000 | 0.058 | 172.08M | 0.054 | 184.39M | 0.104 | 1.79× | 1.92× |
| 100,000 | 0.739 | 135.35M | 0.728 | 137.38M | 0.872 | 1.18× | 1.20× |
| 1,000,000 | 7.499 | 133.35M | 6.213 | 160.95M | 8.664 | 1.16× | 1.39× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.104 | 0.225 | 2.15× |
| 1 | 5 | 0.311 | 0.561 | 1.80× |
| 1 | 10 | 0.656 | 1.332 | 2.03× |
| 10 | 1 | 0.071 | 0.100 | 1.41× |
| 10 | 5 | 0.323 | 0.701 | 2.17× |
| 10 | 10 | 0.651 | 1.174 | 1.80× |
| 100 | 1 | 0.051 | 0.100 | 1.95× |
| 100 | 5 | 0.306 | 0.528 | 1.73× |
| 100 | 10 | 0.523 | 1.095 | 2.09× |
| 1,000 | 1 | 0.063 | 0.096 | 1.53× |
| 1,000 | 5 | 0.317 | 0.630 | 1.99× |
| 1,000 | 10 | 0.601 | 1.212 | 2.02× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
