# RollingAverageDeviation benchmark (`AVGDEV` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.064 | 15.56M | 0.024 | 41.06M | 0.047 | 0.73× | 1.92× |
| 10,000 | 0.648 | 15.44M | 0.243 | 41.21M | 0.182 | 0.28× | 0.75× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
