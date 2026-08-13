# MathAcos benchmark (`ACOS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.033 | 30.07M | 0.027 | 37.40M | 0.032 | 0.95× | 1.18× |
| 10,000 | 0.215 | 46.50M | 0.199 | 50.37M | 0.093 | 0.43× | 0.47× |
| 100,000 | 1.925 | 51.96M | 1.862 | 53.70M | 0.688 | 0.36× | 0.37× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.140 | 0.115 | 0.82× |
| 1 | 5 | 0.345 | 0.505 | 1.46× |
| 1 | 10 | 0.636 | 0.904 | 1.42× |
| 10 | 1 | 0.061 | 0.082 | 1.36× |
| 10 | 5 | 0.267 | 0.408 | 1.53× |
| 10 | 10 | 0.600 | 0.867 | 1.45× |
| 100 | 1 | 0.068 | 0.087 | 1.29× |
| 100 | 5 | 0.306 | 0.426 | 1.39× |
| 100 | 10 | 0.619 | 0.888 | 1.43× |
| 1,000 | 1 | 0.082 | 0.101 | 1.23× |
| 1,000 | 5 | 0.294 | 0.466 | 1.59× |
| 1,000 | 10 | 0.613 | 0.974 | 1.59× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
