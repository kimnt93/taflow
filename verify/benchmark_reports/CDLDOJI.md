# CandleDoji benchmark (`CDLDOJI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 133.36M | 0.005 | 187.62M | 0.032 | 4.30× | 6.04× |
| 10,000 | 0.039 | 256.34M | 0.036 | 278.89M | 0.052 | 1.33× | 1.45× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
