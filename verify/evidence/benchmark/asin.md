# MathAsin benchmark (`ASIN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.030 | 33.42M | 0.026 | 38.46M | 0.033 | 1.09× | 1.25× |
| 10,000 | 0.200 | 49.97M | 0.195 | 51.34M | 0.089 | 0.45× | 0.46× |
| 100,000 | 1.853 | 53.97M | 1.858 | 53.82M | 0.652 | 0.35× | 0.35× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.112 | 0.157 | 1.41× |
| 1 | 5 | 0.428 | 0.443 | 1.03× |
| 1 | 10 | 0.597 | 0.885 | 1.48× |
| 10 | 1 | 0.060 | 0.083 | 1.37× |
| 10 | 5 | 0.295 | 0.414 | 1.40× |
| 10 | 10 | 0.593 | 0.892 | 1.51× |
| 100 | 1 | 0.067 | 0.082 | 1.24× |
| 100 | 5 | 0.290 | 0.427 | 1.47× |
| 100 | 10 | 0.582 | 0.899 | 1.55× |
| 1,000 | 1 | 0.080 | 0.094 | 1.17× |
| 1,000 | 5 | 0.295 | 0.461 | 1.56× |
| 1,000 | 10 | 0.623 | 0.929 | 1.49× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
