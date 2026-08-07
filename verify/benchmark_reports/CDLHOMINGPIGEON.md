# CandleHomingPigeon benchmark (`CDLHOMINGPIGEON` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.027 | 37.45M | 0.025 | 40.14M | 0.032 | 1.19× | 1.27× |
| 10,000 | 0.262 | 38.17M | 0.257 | 38.87M | 0.096 | 0.37× | 0.37× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
