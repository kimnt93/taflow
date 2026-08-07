# CandleUpsideGapTwoCrows benchmark (`CDLUPSIDEGAP2CROWS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.029 | 34.68M | 0.027 | 36.71M | 0.033 | 1.13× | 1.20× |
| 10,000 | 0.296 | 33.81M | 0.289 | 34.58M | 0.124 | 0.42× | 0.43× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
