# KalmanHedgeRatio benchmark (`KalmanHedgeRatio` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.016 | 61.13M | 0.015 | 66.88M | 0.524 | 32.05× | 35.06× |
| 10,000 | 0.151 | 66.41M | 0.139 | 71.94M | 3.638 | 24.16× | 26.17× |
| 100,000 | 1.393 | 71.77M | 1.403 | 71.29M | 40.958 | 29.39× | 29.20× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.070 | 0.320 | 4.59× |
| 1 | 5 | 0.310 | 1.496 | 4.82× |
| 1 | 10 | 0.389 | 2.623 | 6.75× |
| 10 | 1 | 0.046 | 0.242 | 5.23× |
| 10 | 5 | 0.186 | 1.538 | 8.27× |
| 10 | 10 | 0.390 | 2.863 | 7.35× |
| 100 | 1 | 0.048 | 0.289 | 6.05× |
| 100 | 5 | 0.199 | 1.706 | 8.59× |
| 100 | 10 | 0.415 | 3.024 | 7.28× |
| 1,000 | 1 | 0.058 | 0.905 | 15.73× |
| 1,000 | 5 | 0.208 | 3.558 | 17.12× |
| 1,000 | 10 | 0.439 | 7.207 | 16.43× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
