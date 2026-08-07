# WilliamsPercentR benchmark (`WILLR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.086 | 11.59M | 0.041 | 24.22M | 0.036 | 0.41× | 0.86× |
| 10,000 | 0.847 | 11.81M | 0.444 | 22.54M | 0.115 | 0.14× | 0.26× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
