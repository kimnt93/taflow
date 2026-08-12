# LinearRegressionChannel benchmark (`LinRegChannel` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.070 | 14.22M | 0.068 | 14.78M | 0.611 | 8.69× | 9.03× |
| 10,000 | 0.743 | 13.46M | 0.672 | 14.87M | 4.086 | 5.50× | 6.08× |
| 100,000 | 6.534 | 15.30M | 6.423 | 15.57M | 45.819 | 7.01× | 7.13× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.112 | 0.322 | 2.87× |
| 1 | 5 | 0.240 | 1.360 | 5.67× |
| 1 | 10 | 0.466 | 2.667 | 5.72× |
| 10 | 1 | 0.055 | 0.261 | 4.72× |
| 10 | 5 | 0.232 | 1.442 | 6.21× |
| 10 | 10 | 0.516 | 2.827 | 5.48× |
| 100 | 1 | 0.057 | 0.293 | 5.11× |
| 100 | 5 | 0.238 | 1.737 | 7.30× |
| 100 | 10 | 0.495 | 3.122 | 6.31× |
| 1,000 | 1 | 0.124 | 0.891 | 7.22× |
| 1,000 | 5 | 0.277 | 3.743 | 13.51× |
| 1,000 | 10 | 0.546 | 13.553 | 24.82× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
