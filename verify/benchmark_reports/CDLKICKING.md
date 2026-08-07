# CandleKicking benchmark (`CDLKICKING` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.044 | 22.58M | 0.044 | 22.77M | 0.040 | 0.90× | 0.91× |
| 10,000 | 0.455 | 21.99M | 0.449 | 22.25M | 0.176 | 0.39× | 0.39× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
