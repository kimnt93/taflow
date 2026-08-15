# HedgeRatio benchmark (`rolling OLS hedge ratio` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.040 | 25.03M | 0.037 | 26.67M | 0.265 | 6.64× | 7.08× |
| 10,000 | 0.358 | 27.91M | 0.361 | 27.70M | 1.583 | 4.42× | 4.39× |
| 100,000 | 3.726 | 26.84M | 3.732 | 26.80M | 16.548 | 4.44× | 4.43× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.082 | 0.143 | 1.75× |
| 1 | 5 | 0.264 | 0.743 | 2.82× |
| 1 | 10 | 0.391 | 1.274 | 3.26× |
| 10 | 1 | 0.044 | 0.125 | 2.81× |
| 10 | 5 | 0.197 | 0.618 | 3.14× |
| 10 | 10 | 0.378 | 1.230 | 3.25× |
| 100 | 1 | 0.051 | 0.206 | 4.04× |
| 100 | 5 | 0.193 | 1.149 | 5.96× |
| 100 | 10 | 0.396 | 2.254 | 5.70× |
| 1,000 | 1 | 0.086 | 0.350 | 4.08× |
| 1,000 | 5 | 0.198 | 1.355 | 6.84× |
| 1,000 | 10 | 0.439 | 3.089 | 7.04× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
