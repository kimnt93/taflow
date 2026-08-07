# MathTanh benchmark (`TANH` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.045 | 22.00M | 0.004 | 235.21M | 0.030 | 0.66× | 7.04× |
| 10,000 | 0.443 | 22.56M | 0.035 | 288.87M | 0.054 | 0.12× | 1.57× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
