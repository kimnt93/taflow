# CandleTriStar benchmark (`CDLTRISTAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.020 | 50.37M | 0.017 | 58.80M | 0.041 | 2.06× | 2.40× |
| 10,000 | 0.132 | 75.86M | 0.120 | 83.05M | 0.095 | 0.72× | 0.79× |
| 100,000 | 1.154 | 86.63M | 1.139 | 87.80M | 0.674 | 0.58× | 0.59× |
| 1,000,000 | 11.944 | 83.73M | 11.207 | 89.23M | 6.732 | 0.56× | 0.60× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.120 | 0.143 | 1.19× |
| 1 | 5 | 0.308 | 0.521 | 1.69× |
| 1 | 10 | 0.581 | 0.967 | 1.67× |
| 10 | 1 | 0.056 | 0.095 | 1.71× |
| 10 | 5 | 0.251 | 0.438 | 1.75× |
| 10 | 10 | 0.559 | 1.007 | 1.80× |
| 100 | 1 | 0.060 | 0.092 | 1.55× |
| 100 | 5 | 0.283 | 0.452 | 1.60× |
| 100 | 10 | 0.527 | 0.926 | 1.76× |
| 1,000 | 1 | 0.064 | 0.101 | 1.57× |
| 1,000 | 5 | 0.294 | 0.481 | 1.63× |
| 1,000 | 10 | 0.535 | 0.957 | 1.79× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
