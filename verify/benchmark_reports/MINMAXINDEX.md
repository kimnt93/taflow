# RollingMinMaxIndex benchmark (`MINMAXINDEX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.340 | 2.94M | 0.014 | 72.71M | 0.045 | 0.13× | 3.26× |
| 10,000 | 3.451 | 2.90M | 0.173 | 57.76M | 0.155 | 0.04× | 0.89× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
