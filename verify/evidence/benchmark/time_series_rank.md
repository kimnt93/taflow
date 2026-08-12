# TimeSeriesRank benchmark (`rolling percentile rank` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.020 | 50.04M | 0.020 | 51.25M | 0.137 | 6.87× | 7.03× |
| 10,000 | 0.166 | 60.22M | 0.168 | 59.36M | 0.699 | 4.21× | 4.15× |
| 100,000 | 1.611 | 62.08M | 1.671 | 59.83M | 6.095 | 3.78× | 3.65× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.093 | 0.144 | 1.55× |
| 1 | 5 | 0.308 | 0.540 | 1.75× |
| 1 | 10 | 0.465 | 1.085 | 2.33× |
| 10 | 1 | 0.046 | 0.101 | 2.19× |
| 10 | 5 | 0.227 | 0.510 | 2.25× |
| 10 | 10 | 0.488 | 1.067 | 2.18× |
| 100 | 1 | 0.052 | 0.163 | 3.14× |
| 100 | 5 | 0.236 | 0.804 | 3.41× |
| 100 | 10 | 0.502 | 1.731 | 3.45× |
| 1,000 | 1 | 0.075 | 0.217 | 2.88× |
| 1,000 | 5 | 0.252 | 0.918 | 3.64× |
| 1,000 | 10 | 0.480 | 2.034 | 4.24× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
