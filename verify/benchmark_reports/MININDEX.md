# RollingArgmin benchmark (`MININDEX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.054 | 18.53M | 0.013 | 78.22M | 0.036 | 0.67× | 2.82× |
| 10,000 | 0.572 | 17.47M | 0.168 | 59.35M | 0.097 | 0.17× | 0.57× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
