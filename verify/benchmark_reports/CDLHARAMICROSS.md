# CandleHaramiCross benchmark (`CDLHARAMICROSS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.030 | 33.48M | 0.028 | 35.60M | 0.036 | 1.19× | 1.27× |
| 10,000 | 0.318 | 31.43M | 0.312 | 32.10M | 0.138 | 0.43× | 0.44× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
