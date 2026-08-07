# CandleTriStar benchmark (`CDLTRISTAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.022 | 46.29M | 0.019 | 51.73M | 0.034 | 1.59× | 1.77× |
| 10,000 | 0.197 | 50.79M | 0.190 | 52.57M | 0.089 | 0.45× | 0.47× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
