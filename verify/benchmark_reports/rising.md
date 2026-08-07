# Rising benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 135.99M | 0.006 | 159.67M | nan | — | — |
| 10,000 | 0.053 | 188.51M | 0.050 | 201.06M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
