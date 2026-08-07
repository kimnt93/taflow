# SessionVolumeLevels benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.065 | 15.31M | 0.062 | 16.03M | nan | — | — |
| 10,000 | 0.687 | 14.55M | 0.671 | 14.89M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
