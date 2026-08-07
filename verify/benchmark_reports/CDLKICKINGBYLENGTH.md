# CandleKickingByLength benchmark (`CDLKICKINGBYLENGTH` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.045 | 22.34M | 0.043 | 23.14M | 0.039 | 0.87× | 0.90× |
| 10,000 | 0.453 | 22.06M | 0.451 | 22.18M | 0.176 | 0.39× | 0.39× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
