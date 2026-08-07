# KaufmanAdaptiveMovingAverage benchmark (`KAMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.055 | 18.25M | 0.013 | 74.89M | 0.045 | 0.82× | 3.37× |
| 10,000 | 0.541 | 18.50M | 0.127 | 78.53M | 0.064 | 0.12× | 0.50× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
