# RateOfChangeRatio benchmark (`ROCR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.046 | 21.88M | 0.004 | 222.28M | 0.031 | 0.69× | 6.99× |
| 10,000 | 0.438 | 22.86M | 0.034 | 291.00M | 0.041 | 0.09× | 1.19× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
