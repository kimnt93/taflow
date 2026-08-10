# RollingMedianAbsoluteDeviation benchmark (`MedianAbsoluteDeviation` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.345 | 2.90M | 0.343 | 2.92M | 0.502 | 1.46× | 1.47× |
| 10,000 | 3.158 | 3.17M | 3.313 | 3.02M | 4.072 | 1.29× | 1.23× |
| 100,000 | 33.374 | 3.00M | 32.318 | 3.09M | 36.135 | 1.08× | 1.12× |
| 1,000,000 | 326.995 | 3.06M | 319.868 | 3.13M | 354.551 | 1.08× | 1.11× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.122 | 0.276 | 2.25× |
| 1 | 5 | 0.253 | 1.214 | 4.79× |
| 1 | 10 | 0.475 | 2.500 | 5.26× |
| 10 | 1 | 0.055 | 0.218 | 3.93× |
| 10 | 5 | 0.230 | 1.047 | 4.54× |
| 10 | 10 | 0.510 | 2.372 | 4.65× |
| 100 | 1 | 0.079 | 0.243 | 3.07× |
| 100 | 5 | 0.261 | 1.518 | 5.81× |
| 100 | 10 | 0.562 | 2.662 | 4.74× |
| 1,000 | 1 | 0.419 | 0.646 | 1.54× |
| 1,000 | 5 | 0.680 | 3.340 | 4.91× |
| 1,000 | 10 | 1.115 | 6.530 | 5.85× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
