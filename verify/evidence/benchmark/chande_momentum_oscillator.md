# ChandeMomentumOscillator benchmark (`CMO` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 137.65M | 0.006 | 164.06M | 0.043 | 5.92× | 7.06× |
| 10,000 | 0.056 | 177.25M | 0.053 | 188.44M | 0.091 | 1.61× | 1.71× |
| 100,000 | 0.536 | 186.62M | 0.497 | 201.27M | 0.613 | 1.14× | 1.23× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.069 | 0.110 | 1.61× |
| 1 | 5 | 0.200 | 0.518 | 2.60× |
| 1 | 10 | 0.401 | 0.987 | 2.46× |
| 10 | 1 | 0.041 | 0.094 | 2.30× |
| 10 | 5 | 0.180 | 0.474 | 2.63× |
| 10 | 10 | 0.385 | 1.039 | 2.70× |
| 100 | 1 | 0.051 | 0.096 | 1.89× |
| 100 | 5 | 0.197 | 0.501 | 2.54× |
| 100 | 10 | 0.429 | 1.022 | 2.38× |
| 1,000 | 1 | 0.061 | 0.107 | 1.75× |
| 1,000 | 5 | 0.224 | 0.516 | 2.31× |
| 1,000 | 10 | 0.427 | 1.010 | 2.36× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
