# CandleRiseFallThreeMethods benchmark (`CDLRISEFALL3METHODS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.062 | 16.19M | 0.057 | 17.64M | 0.034 | 0.55× | 0.60× |
| 10,000 | 0.611 | 16.37M | 0.603 | 16.58M | 0.121 | 0.20× | 0.20× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
