# RollingMedianAbsoluteDeviation benchmark (`MedianAbsoluteDeviation` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.304 | 3.28M | 0.316 | 3.17M | 0.505 | 1.66× | 1.60× |
| 10,000 | 3.489 | 2.87M | 3.285 | 3.04M | 3.666 | 1.05× | 1.12× |
| 100,000 | 37.586 | 2.66M | 32.723 | 3.06M | 34.889 | 0.93× | 1.07× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.096 | 0.272 | 2.83× |
| 1 | 5 | 0.245 | 1.352 | 5.51× |
| 1 | 10 | 0.401 | 2.203 | 5.49× |
| 10 | 1 | 0.050 | 0.230 | 4.59× |
| 10 | 5 | 0.214 | 1.327 | 6.20× |
| 10 | 10 | 0.402 | 2.366 | 5.89× |
| 100 | 1 | 0.081 | 0.249 | 3.08× |
| 100 | 5 | 0.231 | 1.414 | 6.13× |
| 100 | 10 | 0.459 | 2.709 | 5.90× |
| 1,000 | 1 | 0.371 | 0.589 | 1.59× |
| 1,000 | 5 | 0.619 | 3.110 | 5.03× |
| 1,000 | 10 | 0.886 | 5.936 | 6.70× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
