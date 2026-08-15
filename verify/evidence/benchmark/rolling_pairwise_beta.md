# RollingPairwiseBeta benchmark (`PairwiseBeta` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.034 | 29.03M | 0.034 | 29.60M | 0.216 | 6.27× | 6.39× |
| 10,000 | 0.345 | 28.96M | 0.325 | 30.75M | 0.994 | 2.88× | 3.05× |
| 100,000 | 3.181 | 31.43M | 3.220 | 31.06M | 8.704 | 2.74× | 2.70× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.151 | 0.261 | 1.73× |
| 1 | 5 | 0.342 | 1.026 | 3.00× |
| 1 | 10 | 0.427 | 2.240 | 5.24× |
| 10 | 1 | 0.047 | 0.214 | 4.54× |
| 10 | 5 | 0.194 | 1.250 | 6.43× |
| 10 | 10 | 0.408 | 2.257 | 5.54× |
| 100 | 1 | 0.047 | 0.213 | 4.49× |
| 100 | 5 | 0.195 | 1.331 | 6.81× |
| 100 | 10 | 0.436 | 2.299 | 5.27× |
| 1,000 | 1 | 0.083 | 0.347 | 4.18× |
| 1,000 | 5 | 0.211 | 1.708 | 8.08× |
| 1,000 | 10 | 0.448 | 3.210 | 7.16× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
