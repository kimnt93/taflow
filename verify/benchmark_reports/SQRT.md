# MathSqrt benchmark (`SQRT` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.044 | 22.80M | 0.003 | 344.40M | 0.027 | 0.62× | 9.37× |
| 10,000 | 0.426 | 23.45M | 0.021 | 483.56M | 0.042 | 0.10× | 2.01× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
