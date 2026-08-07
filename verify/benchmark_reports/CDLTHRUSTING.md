# CandleThrusting benchmark (`CDLTHRUSTING` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.027 | 37.73M | 0.024 | 42.08M | 0.034 | 1.30× | 1.45× |
| 10,000 | 0.259 | 38.61M | 0.252 | 39.64M | 0.121 | 0.47× | 0.48× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
