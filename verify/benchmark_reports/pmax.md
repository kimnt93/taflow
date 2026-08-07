# ParabolicMovingAverageStop benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.022 | 45.39M | 0.021 | 48.14M | nan | — | — |
| 10,000 | 0.192 | 52.17M | 0.189 | 52.88M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
