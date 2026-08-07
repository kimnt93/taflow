# CandleMarubozu benchmark (`CDLMARUBOZU` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 73.65M | 0.011 | 87.86M | 0.036 | 2.62× | 3.13× |
| 10,000 | 0.134 | 74.60M | 0.130 | 77.04M | 0.137 | 1.03× | 1.06× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
