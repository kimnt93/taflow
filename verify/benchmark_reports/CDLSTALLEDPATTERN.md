# CandleStalledPattern benchmark (`CDLSTALLEDPATTERN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.058 | 17.32M | 0.055 | 18.08M | 0.041 | 0.72× | 0.75× |
| 10,000 | 0.562 | 17.78M | 0.554 | 18.03M | 0.164 | 0.29× | 0.29× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
