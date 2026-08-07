# CandleClosingMarubozu benchmark (`CDLCLOSINGMARUBOZU` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.015 | 67.01M | 0.012 | 80.38M | 0.035 | 2.33× | 2.79× |
| 10,000 | 0.141 | 71.03M | 0.134 | 74.88M | 0.129 | 0.92× | 0.97× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
