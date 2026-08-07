# CandleThreeOutside benchmark (`CDL3OUTSIDE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 93.19M | 0.009 | 112.23M | 0.029 | 2.75× | 3.31× |
| 10,000 | 0.074 | 134.90M | 0.070 | 142.94M | 0.084 | 1.13× | 1.20× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
