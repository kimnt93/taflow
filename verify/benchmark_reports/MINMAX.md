# RollingMinMax benchmark (`MINMAX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.330 | 3.03M | 0.022 | 45.67M | 0.043 | 0.13× | 1.97× |
| 10,000 | 3.196 | 3.13M | 0.250 | 39.96M | 0.117 | 0.04× | 0.47× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
