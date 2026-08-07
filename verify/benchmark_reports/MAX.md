# RollingMax benchmark (`MAX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.064 | 15.51M | 0.020 | 50.92M | 0.037 | 0.57× | 1.88× |
| 10,000 | 0.646 | 15.47M | 0.245 | 40.84M | 0.083 | 0.13× | 0.34× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
