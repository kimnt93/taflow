# MathDivide benchmark (`DIV` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.256 | 3.91M | 0.002 | 642.06M | 0.030 | 0.12× | 19.26× |
| 10,000 | 2.442 | 4.09M | 0.006 | 1.62G | 0.036 | 0.01× | 5.82× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
