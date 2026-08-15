# VariableIndexDynamicAverage benchmark (`VIDYA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 76.07M | 0.011 | 88.87M | 0.199 | 15.13× | 17.67× |
| 10,000 | 0.112 | 89.25M | 0.110 | 91.28M | 0.530 | 4.73× | 4.84× |
| 100,000 | 1.172 | 85.32M | 1.106 | 90.38M | 3.758 | 3.21× | 3.40× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.083 | 0.300 | 3.60× |
| 1 | 5 | 0.313 | 1.462 | 4.67× |
| 1 | 10 | 0.396 | 2.838 | 7.17× |
| 10 | 1 | 0.044 | 0.248 | 5.68× |
| 10 | 5 | 0.197 | 1.531 | 7.79× |
| 10 | 10 | 0.482 | 2.659 | 5.51× |
| 100 | 1 | 0.047 | 0.256 | 5.47× |
| 100 | 5 | 0.196 | 1.607 | 8.21× |
| 100 | 10 | 0.397 | 2.802 | 7.06× |
| 1,000 | 1 | 0.054 | 0.284 | 5.30× |
| 1,000 | 5 | 0.189 | 1.658 | 8.77× |
| 1,000 | 10 | 0.413 | 2.973 | 7.20× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
