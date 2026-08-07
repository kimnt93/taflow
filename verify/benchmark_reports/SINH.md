# MathSinh benchmark (`SINH` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.050 | 19.94M | 0.008 | 121.93M | 0.034 | 0.67× | 4.09× |
| 10,000 | 0.480 | 20.84M | 0.073 | 136.40M | 0.091 | 0.19× | 1.24× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
