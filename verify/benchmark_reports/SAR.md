# ParabolicSar benchmark (`SAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.049 | 20.40M | 0.010 | 99.78M | 0.038 | 0.78× | 3.82× |
| 10,000 | 0.506 | 19.78M | 0.111 | 90.32M | 0.094 | 0.19× | 0.85× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
