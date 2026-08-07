# MovingAverageConvergenceDivergenceFixed benchmark (`MACDFIX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.211 | 4.73M | 0.016 | 62.99M | 0.047 | 0.22× | 2.96× |
| 10,000 | 2.143 | 4.67M | 0.149 | 67.18M | 0.137 | 0.06× | 0.92× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
