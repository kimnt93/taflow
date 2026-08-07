# HilbertTransformSineWave benchmark (`HT_SINE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.438 | 2.28M | 0.444 | 2.25M | 0.502 | 1.15× | 1.13× |
| 10,000 | 4.611 | 2.17M | 4.533 | 2.21M | 4.651 | 1.01× | 1.03× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
