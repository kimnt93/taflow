# AveragePrice benchmark (`AVGPRICE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.045 | 22.17M | 0.002 | 455.67M | 0.031 | 0.68× | 13.93× |
| 10,000 | 0.411 | 24.31M | 0.008 | 1.21G | 0.036 | 0.09× | 4.34× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
