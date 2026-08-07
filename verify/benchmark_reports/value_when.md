# ValueWhen benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 222.19M | 0.003 | 300.74M | nan | — | — |
| 10,000 | 0.025 | 395.28M | 0.022 | 463.75M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
