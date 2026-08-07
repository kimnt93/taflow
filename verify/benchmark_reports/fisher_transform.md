# FisherTransform benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.056 | 17.82M | 0.052 | 19.08M | nan | — | — |
| 10,000 | 0.514 | 19.46M | 0.522 | 19.17M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
