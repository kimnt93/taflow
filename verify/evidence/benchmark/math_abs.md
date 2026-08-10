# MathAbs benchmark (`numpy.abs` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.003 | 304.96M | 0.003 | 396.73M | 0.011 | 3.44× | 4.48× |
| 10,000 | 0.008 | 1.33G | 0.005 | 1.97G | 0.015 | 1.98× | 2.93× |
| 100,000 | 0.056 | 1.78G | 0.033 | 3.08G | 0.045 | 0.80× | 1.39× |
| 1,000,000 | 0.886 | 1.13G | 0.558 | 1.79G | 0.642 | 0.72× | 1.15× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.071 | 0.070 | 1.00× |
| 1 | 5 | 0.333 | 0.276 | 0.83× |
| 1 | 10 | 0.440 | 0.559 | 1.27× |
| 10 | 1 | 0.049 | 0.057 | 1.15× |
| 10 | 5 | 0.212 | 0.272 | 1.28× |
| 10 | 10 | 0.449 | 0.575 | 1.28× |
| 100 | 1 | 0.045 | 0.053 | 1.18× |
| 100 | 5 | 0.212 | 0.268 | 1.27× |
| 100 | 10 | 0.468 | 0.561 | 1.20× |
| 1,000 | 1 | 0.050 | 0.057 | 1.14× |
| 1,000 | 5 | 0.218 | 0.277 | 1.27× |
| 1,000 | 10 | 0.465 | 0.599 | 1.29× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
