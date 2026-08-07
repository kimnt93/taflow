# FairValueGap benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.016 | 60.95M | 0.013 | 76.58M | nan | — | — |
| 10,000 | 0.120 | 83.42M | 0.109 | 91.62M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
