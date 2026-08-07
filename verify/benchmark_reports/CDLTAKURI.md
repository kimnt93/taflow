# CandleTakuri benchmark (`CDLTAKURI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 75.51M | 0.012 | 86.41M | 0.038 | 2.84× | 3.24× |
| 10,000 | 0.115 | 86.61M | 0.115 | 86.86M | 0.109 | 0.94× | 0.95× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
