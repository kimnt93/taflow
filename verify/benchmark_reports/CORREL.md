# RollingCorrelation benchmark (`CORREL` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.277 | 3.62M | 0.022 | 45.36M | 0.041 | 0.15× | 1.87× |
| 10,000 | 2.703 | 3.70M | 0.213 | 46.87M | 0.092 | 0.03× | 0.43× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
