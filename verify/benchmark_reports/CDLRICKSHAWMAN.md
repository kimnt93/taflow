# CandleRickshawman benchmark (`CDLRICKSHAWMAN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 78.22M | 0.011 | 94.21M | 0.037 | 2.86× | 3.45× |
| 10,000 | 0.110 | 90.87M | 0.106 | 93.98M | 0.127 | 1.15× | 1.19× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
