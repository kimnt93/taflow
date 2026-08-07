# RelativeMomentumIndex benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 103.23M | 0.009 | 111.15M | nan | — | — |
| 10,000 | 0.081 | 123.79M | 0.079 | 127.35M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
