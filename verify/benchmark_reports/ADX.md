# AverageDirectionalIndex benchmark (`ADX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.057 | 17.52M | 0.015 | 68.10M | 0.041 | 0.71× | 2.76× |
| 10,000 | 0.537 | 18.62M | 0.133 | 75.20M | 0.124 | 0.23× | 0.93× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
