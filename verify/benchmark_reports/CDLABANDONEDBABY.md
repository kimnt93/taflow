# CandleAbandonedBaby benchmark (`CDLABANDONEDBABY` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.043 | 23.37M | 0.040 | 24.82M | 0.042 | 0.97× | 1.03× |
| 10,000 | 0.409 | 24.43M | 0.410 | 24.41M | 0.136 | 0.33× | 0.33× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
