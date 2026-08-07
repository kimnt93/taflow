# CandleThreeBlackCrows benchmark (`CDL3BLACKCROWS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 94.99M | 0.008 | 119.71M | 0.033 | 3.11× | 3.91× |
| 10,000 | 0.119 | 84.32M | 0.115 | 86.81M | 0.088 | 0.74× | 0.76× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
