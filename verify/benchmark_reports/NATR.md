# NormalizedAverageTrueRange benchmark (`NATR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.050 | 20.20M | 0.008 | 123.61M | 0.039 | 0.78× | 4.80× |
| 10,000 | 0.468 | 21.38M | 0.066 | 150.64M | 0.089 | 0.19× | 1.34× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
