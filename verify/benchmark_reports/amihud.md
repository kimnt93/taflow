# Amihud benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 99.52M | 0.008 | 119.64M | nan | — | — |
| 10,000 | 0.074 | 134.61M | 0.074 | 135.91M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
