# CandleOnNeck benchmark (`CDLONNECK` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.025 | 39.97M | 0.023 | 43.90M | 0.034 | 1.36× | 1.49× |
| 10,000 | 0.263 | 37.96M | 0.259 | 38.60M | 0.122 | 0.46× | 0.47× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
