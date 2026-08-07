# CandleHikkakeModified benchmark (`CDLHIKKAKEMOD` oracle)

Correctness: **MISMATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 83.02M | 0.010 | 102.06M | 0.033 | 2.73× | 3.36× |
| 10,000 | 0.111 | 89.99M | 0.108 | 92.87M | 0.083 | 0.75× | 0.77× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
