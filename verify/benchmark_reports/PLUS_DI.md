# PlusDirectionalIndicator benchmark (`PLUS_DI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 79.71M | 0.011 | 89.67M | 0.039 | 3.07× | 3.46× |
| 10,000 | 0.103 | 96.84M | 0.102 | 98.16M | 0.098 | 0.95× | 0.97× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
