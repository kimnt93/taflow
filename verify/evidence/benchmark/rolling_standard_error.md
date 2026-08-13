# RollingStandardError benchmark (`StandardError` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.078 | 12.86M | 0.070 | 14.37M | 0.173 | 2.22× | 2.48× |
| 10,000 | 0.646 | 15.47M | 0.622 | 16.08M | 0.626 | 0.97× | 1.01× |
| 100,000 | 6.329 | 15.80M | 6.543 | 15.28M | 5.389 | 0.85× | 0.82× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.099 | 0.258 | 2.60× |
| 1 | 5 | 0.361 | 1.068 | 2.96× |
| 1 | 10 | 0.603 | 2.214 | 3.67× |
| 10 | 1 | 0.070 | 0.207 | 2.93× |
| 10 | 5 | 0.291 | 1.272 | 4.37× |
| 10 | 10 | 0.612 | 2.177 | 3.56× |
| 100 | 1 | 0.077 | 0.220 | 2.85× |
| 100 | 5 | 0.312 | 1.247 | 4.00× |
| 100 | 10 | 0.760 | 2.294 | 3.02× |
| 1,000 | 1 | 0.147 | 0.267 | 1.81× |
| 1,000 | 5 | 0.298 | 1.462 | 4.91× |
| 1,000 | 10 | 0.643 | 2.770 | 4.30× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
