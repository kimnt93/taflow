# RollingSum benchmark (`SUM` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.047 | 21.48M | 0.005 | 209.21M | 0.033 | 0.71× | 6.91× |
| 10,000 | 0.447 | 22.39M | 0.040 | 250.29M | 0.050 | 0.11× | 1.26× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
