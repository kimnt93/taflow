# CandleBeltHold benchmark (`CDLBELTHOLD` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 82.03M | 0.010 | 101.23M | 0.037 | 3.02× | 3.73× |
| 10,000 | 0.167 | 59.96M | 0.136 | 73.67M | 0.132 | 0.79× | 0.98× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
