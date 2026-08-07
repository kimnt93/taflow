# CandleShootingStar benchmark (`CDLSHOOTINGSTAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.028 | 36.34M | 0.025 | 39.59M | 0.040 | 1.45× | 1.58× |
| 10,000 | 0.284 | 35.27M | 0.280 | 35.68M | 0.170 | 0.60× | 0.61× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
