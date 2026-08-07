# VariablePeriodMovingAverage benchmark (`MAVP` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.259 | 3.86M | 0.223 | 4.48M | 0.116 | 0.45× | 0.52× |
| 10,000 | 2.539 | 3.94M | 2.058 | 4.86M | 0.854 | 0.34× | 0.42× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
