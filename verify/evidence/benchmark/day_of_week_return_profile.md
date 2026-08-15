# DayOfWeekReturnProfile benchmark (`DayOfWeekProfile` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.039 | 25.34M | 0.034 | 29.48M | 0.860 | 21.80× | 25.36× |
| 10,000 | 0.369 | 27.10M | 0.328 | 30.49M | 7.386 | 20.02× | 22.52× |
| 100,000 | 3.838 | 26.05M | 3.241 | 30.86M | 80.467 | 20.96× | 24.83× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.073 | 0.298 | 4.07× |
| 1 | 5 | 0.261 | 1.104 | 4.24× |
| 1 | 10 | 0.389 | 2.424 | 6.23× |
| 10 | 1 | 0.050 | 0.232 | 4.64× |
| 10 | 5 | 0.199 | 1.327 | 6.66× |
| 10 | 10 | 0.494 | 2.525 | 5.11× |
| 100 | 1 | 0.055 | 0.302 | 5.47× |
| 100 | 5 | 0.199 | 1.773 | 8.92× |
| 100 | 10 | 0.452 | 3.206 | 7.09× |
| 1,000 | 1 | 0.084 | 1.200 | 14.30× |
| 1,000 | 5 | 0.218 | 5.297 | 24.33× |
| 1,000 | 10 | 0.433 | 10.623 | 24.56× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
