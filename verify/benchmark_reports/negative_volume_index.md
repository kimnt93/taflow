# NegativeVolumeIndex benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 129.58M | 0.006 | 157.82M | nan | — | — |
| 10,000 | 0.068 | 147.38M | 0.063 | 159.42M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
