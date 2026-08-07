# CandleThreeLineStrike benchmark (`CDL3LINESTRIKE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.017 | 59.39M | 0.015 | 66.03M | 0.033 | 1.97× | 2.19× |
| 10,000 | 0.220 | 45.36M | 0.217 | 46.10M | 0.109 | 0.49× | 0.50× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
