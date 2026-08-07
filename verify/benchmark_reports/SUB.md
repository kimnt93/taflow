# MathSubtract benchmark (`SUB` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.251 | 3.98M | 0.001 | 791.36M | 0.030 | 0.12× | 24.05× |
| 10,000 | 2.490 | 4.02M | 0.005 | 2.19G | 0.035 | 0.01× | 7.74× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
