# CandleShortLine benchmark (`CDLSHORTLINE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.017 | 59.30M | 0.015 | 67.65M | 0.035 | 2.09× | 2.38× |
| 10,000 | 0.186 | 53.82M | 0.184 | 54.39M | 0.199 | 1.07× | 1.08× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
