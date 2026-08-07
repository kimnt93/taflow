# PlusDirectionalMovement benchmark (`PLUS_DM` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 136.36M | 0.006 | 167.94M | 0.037 | 5.02× | 6.18× |
| 10,000 | 0.056 | 179.05M | 0.053 | 189.78M | 0.083 | 1.48× | 1.57× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
