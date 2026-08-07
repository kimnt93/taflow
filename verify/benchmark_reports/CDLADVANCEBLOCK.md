# CandleAdvanceBlock benchmark (`CDLADVANCEBLOCK` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.040 | 25.09M | 0.038 | 26.48M | 0.048 | 1.21× | 1.28× |
| 10,000 | 0.383 | 26.11M | 0.388 | 25.75M | 0.224 | 0.58× | 0.58× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
