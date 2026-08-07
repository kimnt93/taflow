# RollingAutocorr benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.082 | 12.26M | 0.079 | 12.58M | nan | — | — |
| 10,000 | 0.795 | 12.57M | 0.791 | 12.64M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
