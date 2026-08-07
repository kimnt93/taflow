# LaguerreRelativeStrengthIndex benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 82.94M | 0.009 | 108.80M | nan | — | — |
| 10,000 | 0.106 | 94.35M | 0.082 | 121.72M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
