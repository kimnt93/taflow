# MathCot benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.060 | 16.54M | 0.019 | 52.59M | nan | — | — |
| 10,000 | 0.633 | 15.80M | 0.213 | 46.94M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
