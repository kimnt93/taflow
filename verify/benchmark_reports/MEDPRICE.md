# MedianPrice benchmark (`MEDPRICE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.042 | 23.86M | 0.001 | 812.28M | 0.029 | 0.68× | 23.22× |
| 10,000 | 0.408 | 24.49M | 0.004 | 2.28G | 0.033 | 0.08× | 7.50× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
