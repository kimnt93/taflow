# CandleUpDownSideGapThreeMethods benchmark (`CDLXSIDEGAP3METHODS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 77.43M | 0.010 | 97.09M | 0.032 | 2.47× | 3.10× |
| 10,000 | 0.113 | 88.70M | 0.110 | 90.59M | 0.086 | 0.76× | 0.78× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
