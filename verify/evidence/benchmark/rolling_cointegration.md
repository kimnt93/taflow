# RollingCointegration benchmark (`Cointegration` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.346 | 2.89M | 0.343 | 2.92M | 3.281 | 9.48× | 9.58× |
| 10,000 | 3.444 | 2.90M | 3.405 | 2.94M | 31.780 | 9.23× | 9.33× |
| 100,000 | 34.680 | 2.88M | 34.495 | 2.90M | 318.634 | 9.19× | 9.24× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.073 | 0.286 | 3.91× |
| 1 | 5 | 0.458 | 1.382 | 3.02× |
| 1 | 10 | 0.503 | 2.509 | 4.99× |
| 10 | 1 | 0.059 | 0.272 | 4.58× |
| 10 | 5 | 0.249 | 1.409 | 5.66× |
| 10 | 10 | 0.518 | 2.847 | 5.50× |
| 100 | 1 | 0.078 | 0.496 | 6.39× |
| 100 | 5 | 0.249 | 2.762 | 11.10× |
| 100 | 10 | 0.565 | 5.094 | 9.02× |
| 1,000 | 1 | 0.388 | 3.806 | 9.81× |
| 1,000 | 5 | 0.578 | 24.949 | 43.13× |
| 1,000 | 10 | 1.129 | 36.851 | 32.65× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
