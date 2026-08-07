# CandleGapSideSideWhite benchmark (`CDLGAPSIDESIDEWHITE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.018 | 57.01M | 0.016 | 63.14M | 0.049 | 2.77× | 3.07× |
| 10,000 | 0.163 | 61.35M | 0.161 | 62.09M | 0.231 | 1.41× | 1.43× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
