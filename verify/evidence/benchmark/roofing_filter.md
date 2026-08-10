# RoofingFilter benchmark (`RoofingFilter` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 118.26M | 0.008 | 132.93M | 0.202 | 23.91× | 26.88× |
| 10,000 | 0.051 | 196.82M | 0.050 | 198.76M | 0.520 | 10.24× | 10.34× |
| 100,000 | 0.498 | 200.83M | 0.745 | 134.20M | 4.570 | 9.18× | 6.13× |
| 1,000,000 | 5.759 | 173.63M | 6.065 | 164.87M | 36.543 | 6.34× | 6.02× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.087 | 0.293 | 3.35× |
| 1 | 5 | 0.283 | 1.437 | 5.07× |
| 1 | 10 | 0.513 | 3.044 | 5.94× |
| 10 | 1 | 0.058 | 0.239 | 4.14× |
| 10 | 5 | 0.260 | 1.537 | 5.92× |
| 10 | 10 | 0.523 | 2.739 | 5.24× |
| 100 | 1 | 0.159 | 0.378 | 2.38× |
| 100 | 5 | 0.320 | 1.798 | 5.61× |
| 100 | 10 | 0.522 | 3.004 | 5.76× |
| 1,000 | 1 | 0.060 | 0.290 | 4.83× |
| 1,000 | 5 | 0.264 | 1.755 | 6.65× |
| 1,000 | 10 | 0.610 | 3.216 | 5.28× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
