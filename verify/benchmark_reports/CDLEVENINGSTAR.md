# CandleEveningStar benchmark (`CDLEVENINGSTAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.038 | 26.59M | 0.039 | 25.79M | 0.040 | 1.05× | 1.02× |
| 10,000 | 0.404 | 24.77M | 0.397 | 25.22M | 0.116 | 0.29× | 0.29× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
