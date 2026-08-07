# McGinleyDynamic benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.015 | 65.84M | 0.014 | 71.40M | nan | — | — |
| 10,000 | 0.134 | 74.79M | 0.130 | 77.06M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
