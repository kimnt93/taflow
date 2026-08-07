# CandleDojiStar benchmark (`CDLDOJISTAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.031 | 32.72M | 0.028 | 35.89M | 0.036 | 1.18× | 1.29× |
| 10,000 | 0.311 | 32.19M | 0.306 | 32.63M | 0.133 | 0.43× | 0.43× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
