# CandleMatchingLow benchmark (`CDLMATCHINGLOW` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.015 | 65.40M | 0.013 | 75.99M | 0.031 | 2.05× | 2.38× |
| 10,000 | 0.134 | 74.50M | 0.132 | 75.64M | 0.091 | 0.68× | 0.69× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
