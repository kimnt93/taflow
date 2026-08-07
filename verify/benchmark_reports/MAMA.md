# MesaAdaptiveMovingAverage benchmark (`MAMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.391 | 2.56M | 0.060 | 16.53M | 0.090 | 0.23× | 1.48× |
| 10,000 | 3.610 | 2.77M | 0.608 | 16.44M | 0.591 | 0.16× | 0.97× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
