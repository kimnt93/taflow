# YangZhang benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.054 | 18.52M | 0.052 | 19.28M | nan | — | — |
| 10,000 | 0.509 | 19.64M | 0.501 | 19.95M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
