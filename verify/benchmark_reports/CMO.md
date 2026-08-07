# ChandeMomentumOscillator benchmark (`CMO` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.049 | 20.36M | 0.007 | 147.21M | 0.038 | 0.76× | 5.53× |
| 10,000 | 0.465 | 21.48M | 0.062 | 162.16M | 0.091 | 0.19× | 1.47× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
