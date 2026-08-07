# MovingAverageConvergenceDivergenceExtended benchmark (`MACDEXT` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.218 | 4.59M | 0.025 | 39.98M | 0.057 | 0.26× | 2.28× |
| 10,000 | 2.297 | 4.35M | 0.227 | 44.08M | 0.118 | 0.05× | 0.52× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
