# ParabolicSarExtended benchmark (`SAREXT` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.051 | 19.53M | 0.011 | 88.83M | 0.052 | 1.03× | 4.66× |
| 10,000 | 0.509 | 19.64M | 0.115 | 86.90M | 0.091 | 0.18× | 0.80× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
