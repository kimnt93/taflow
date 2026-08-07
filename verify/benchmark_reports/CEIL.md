# MathCeil benchmark (`CEIL` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.045 | 22.15M | 0.004 | 278.99M | 0.029 | 0.64× | 8.00× |
| 10,000 | 0.441 | 22.70M | 0.027 | 366.43M | 0.041 | 0.09× | 1.50× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
