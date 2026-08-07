# HilbertTransformDominantCyclePeriod benchmark (`HT_DCPERIOD` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.045 | 22.14M | 0.044 | 22.72M | 0.074 | 1.63× | 1.68× |
| 10,000 | 0.446 | 22.44M | 0.445 | 22.46M | 0.466 | 1.04× | 1.05× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
