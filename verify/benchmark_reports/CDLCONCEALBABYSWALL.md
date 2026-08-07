# CandleConcealBabySwall benchmark (`CDLCONCEALBABYSWALL` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.031 | 32.49M | 0.028 | 35.77M | 0.034 | 1.11× | 1.23× |
| 10,000 | 0.276 | 36.26M | 0.297 | 33.71M | 0.091 | 0.33× | 0.31× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
