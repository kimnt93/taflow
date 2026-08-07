# RollingMin benchmark (`MIN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.065 | 15.34M | 0.019 | 51.48M | 0.035 | 0.54× | 1.80× |
| 10,000 | 0.657 | 15.22M | 0.251 | 39.85M | 0.081 | 0.12× | 0.32× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
