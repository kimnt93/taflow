# ExponentiallyWeightedCovariance benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 126.21M | 0.007 | 153.16M | nan | — | — |
| 10,000 | 0.056 | 177.86M | 0.054 | 184.21M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
