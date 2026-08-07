# CandleInvertedHammer benchmark (`CDLINVERTEDHAMMER` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.027 | 37.16M | 0.026 | 38.99M | 0.040 | 1.50× | 1.58× |
| 10,000 | 0.289 | 34.56M | 0.283 | 35.29M | 0.170 | 0.59× | 0.60× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
