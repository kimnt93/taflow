# OvernightGap benchmark (`OvernightGap` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.015 | 65.92M | 0.011 | 87.57M | 0.363 | 23.96× | 31.83× |
| 10,000 | 0.050 | 201.48M | 0.045 | 222.77M | 2.254 | 45.40× | 50.20× |
| 100,000 | 0.392 | 255.37M | 0.368 | 271.43M | 21.455 | 54.79× | 58.24× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.096 | 0.269 | 2.81× |
| 1 | 5 | 0.365 | 1.204 | 3.30× |
| 1 | 10 | 0.559 | 2.619 | 4.69× |
| 10 | 1 | 0.058 | 0.236 | 4.09× |
| 10 | 5 | 0.289 | 1.167 | 4.04× |
| 10 | 10 | 0.569 | 2.458 | 4.32× |
| 100 | 1 | 0.065 | 0.256 | 3.95× |
| 100 | 5 | 0.266 | 1.466 | 5.51× |
| 100 | 10 | 0.552 | 2.711 | 4.91× |
| 1,000 | 1 | 0.068 | 0.475 | 6.96× |
| 1,000 | 5 | 0.286 | 2.506 | 8.75× |
| 1,000 | 10 | 0.611 | 4.854 | 7.95× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
