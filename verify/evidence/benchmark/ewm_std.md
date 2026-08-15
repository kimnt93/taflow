# ExponentiallyWeightedStandardDeviation benchmark (`ewm standard deviation` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 132.36M | 0.005 | 193.00M | 1.224 | 161.96× | 236.16× |
| 10,000 | 0.061 | 163.68M | 0.045 | 220.17M | 15.526 | 254.13× | 341.84× |
| 100,000 | 0.606 | 165.04M | 0.442 | 226.26M | 135.379 | 223.42× | 306.31× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.058 | 0.117 | 2.03× |
| 1 | 5 | 0.350 | 0.481 | 1.37× |
| 1 | 10 | 0.392 | 0.860 | 2.19× |
| 10 | 1 | 0.044 | 0.103 | 2.31× |
| 10 | 5 | 0.198 | 0.549 | 2.78× |
| 10 | 10 | 0.390 | 1.093 | 2.80× |
| 100 | 1 | 0.070 | 0.215 | 3.08× |
| 100 | 5 | 0.185 | 1.132 | 6.13× |
| 100 | 10 | 0.476 | 2.215 | 4.66× |
| 1,000 | 1 | 0.049 | 1.345 | 27.50× |
| 1,000 | 5 | 0.264 | 6.869 | 26.06× |
| 1,000 | 10 | 0.434 | 14.588 | 33.60× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
