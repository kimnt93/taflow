# CandleCounterAttack benchmark (`CDLCOUNTERATTACK` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.033 | 30.37M | 0.030 | 33.06M | 0.036 | 1.09× | 1.18× |
| 10,000 | 0.331 | 30.23M | 0.321 | 31.16M | 0.137 | 0.42× | 0.43× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
