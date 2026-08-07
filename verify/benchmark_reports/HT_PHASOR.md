# HilbertTransformPhasor benchmark (`HT_PHASOR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.049 | 20.28M | 0.046 | 21.52M | 0.075 | 1.53× | 1.62× |
| 10,000 | 0.469 | 21.32M | 0.454 | 22.01M | 0.488 | 1.04× | 1.07× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
