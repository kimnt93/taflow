# BreakOfStructureChangeOfCharacter benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.073 | 13.70M | 0.071 | 14.11M | nan | — | — |
| 10,000 | 0.703 | 14.23M | 0.701 | 14.26M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
