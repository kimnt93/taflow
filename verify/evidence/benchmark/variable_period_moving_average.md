# VariablePeriodMovingAverage benchmark (`MAVP` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.139 | 7.17M | 0.137 | 7.31M | 0.118 | 0.85× | 0.86× |
| 10,000 | 1.395 | 7.17M | 1.588 | 6.30M | 0.829 | 0.59× | 0.52× |
| 100,000 | 14.567 | 6.87M | 13.342 | 7.50M | 7.523 | 0.52× | 0.56× |
| 1,000,000 | 128.464 | 7.78M | 127.579 | 7.84M | 104.084 | 0.81× | 0.82× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.193 | 0.196 | 1.01× |
| 1 | 5 | 0.306 | 0.558 | 1.83× |
| 1 | 10 | 0.554 | 1.009 | 1.82× |
| 10 | 1 | 0.060 | 0.109 | 1.83× |
| 10 | 5 | 0.238 | 0.490 | 2.06× |
| 10 | 10 | 0.491 | 1.091 | 2.22× |
| 100 | 1 | 0.068 | 0.119 | 1.74× |
| 100 | 5 | 0.262 | 0.519 | 1.98× |
| 100 | 10 | 0.536 | 1.119 | 2.09× |
| 1,000 | 1 | 0.180 | 0.199 | 1.11× |
| 1,000 | 5 | 0.327 | 0.888 | 2.71× |
| 1,000 | 10 | 0.792 | 1.908 | 2.41× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
