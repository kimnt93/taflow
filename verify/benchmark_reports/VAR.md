# RollingVariance benchmark (`VAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.049 | 20.45M | 0.007 | 138.50M | 0.035 | 0.72× | 4.90× |
| 10,000 | 0.500 | 19.98M | 0.063 | 157.96M | 0.054 | 0.11× | 0.86× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
