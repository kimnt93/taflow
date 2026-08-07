# KnowSureThing benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.037 | 27.32M | 0.035 | 28.50M | nan | — | — |
| 10,000 | 0.337 | 29.65M | 0.332 | 30.11M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
