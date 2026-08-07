# RollingMidprice benchmark (`MIDPRICE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.308 | 3.24M | 0.041 | 24.68M | 0.039 | 0.13× | 0.96× |
| 10,000 | 3.107 | 3.22M | 0.457 | 21.87M | 0.103 | 0.03× | 0.23× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
