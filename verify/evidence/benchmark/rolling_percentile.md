# RollingPercentile benchmark (`rolling percentile` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.038 | 25.99M | 0.037 | 27.21M | 0.350 | 9.09× | 9.51× |
| 10,000 | 0.459 | 21.77M | 0.421 | 23.74M | 2.116 | 4.61× | 5.03× |
| 100,000 | 4.185 | 23.90M | 4.047 | 24.71M | 21.140 | 5.05× | 5.22× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.112 | 0.353 | 3.17× |
| 1 | 5 | 0.308 | 1.046 | 3.39× |
| 1 | 10 | 0.425 | 2.103 | 4.95× |
| 10 | 1 | 0.048 | 0.189 | 3.94× |
| 10 | 5 | 0.188 | 0.983 | 5.22× |
| 10 | 10 | 0.395 | 2.070 | 5.24× |
| 100 | 1 | 0.047 | 0.256 | 5.46× |
| 100 | 5 | 0.197 | 1.234 | 6.27× |
| 100 | 10 | 0.426 | 2.512 | 5.89× |
| 1,000 | 1 | 0.088 | 0.438 | 5.00× |
| 1,000 | 5 | 0.200 | 1.481 | 7.41× |
| 1,000 | 10 | 0.446 | 2.934 | 6.58× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
