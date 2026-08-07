# Retracements benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.054 | 18.37M | 0.053 | 18.93M | nan | — | — |
| 10,000 | 0.530 | 18.85M | 0.492 | 20.33M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
