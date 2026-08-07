# FibonacciRetracement benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.194 | 5.16M | 0.187 | 5.35M | nan | — | — |
| 10,000 | 2.008 | 4.98M | 1.950 | 5.13M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
