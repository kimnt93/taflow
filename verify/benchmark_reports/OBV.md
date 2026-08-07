# OnBalanceVolume benchmark (`OBV` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.044 | 22.93M | 0.002 | 501.26M | 0.032 | 0.73× | 15.91× |
| 10,000 | 0.442 | 22.61M | 0.035 | 286.76M | 0.064 | 0.14× | 1.84× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
