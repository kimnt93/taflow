# MathAdd benchmark (`ADD` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.249 | 4.02M | 0.001 | 768.79M | 0.029 | 0.12× | 22.46× |
| 10,000 | 2.437 | 4.10M | 0.005 | 2.10G | 0.034 | 0.01× | 7.18× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
