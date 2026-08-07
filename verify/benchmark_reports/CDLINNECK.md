# CandleInNeck benchmark (`CDLINNECK` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.025 | 40.04M | 0.024 | 42.52M | 0.033 | 1.31× | 1.39× |
| 10,000 | 0.256 | 38.99M | 0.255 | 39.26M | 0.121 | 0.47× | 0.48× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
