# CandleUniqueThreeRiver benchmark (`CDLUNIQUE3RIVER` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.030 | 33.57M | 0.028 | 36.19M | 0.032 | 1.06× | 1.14× |
| 10,000 | 0.298 | 33.61M | 0.292 | 34.30M | 0.080 | 0.27× | 0.28× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
