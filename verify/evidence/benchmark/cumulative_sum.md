# CumulativeSum benchmark (`numpy.cumsum` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.003 | 304.16M | 0.002 | 565.79M | 0.017 | 5.16× | 9.60× |
| 10,000 | 0.013 | 765.86M | 0.010 | 965.50M | 0.037 | 2.85× | 3.59× |
| 100,000 | 0.127 | 786.18M | 0.097 | 1.03G | 0.227 | 1.78× | 2.33× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.085 | 0.091 | 1.08× |
| 1 | 5 | 0.221 | 0.313 | 1.41× |
| 1 | 10 | 0.390 | 0.656 | 1.68× |
| 10 | 1 | 0.055 | 0.061 | 1.12× |
| 10 | 5 | 0.203 | 0.370 | 1.83× |
| 10 | 10 | 0.391 | 0.637 | 1.63× |
| 100 | 1 | 0.041 | 0.067 | 1.66× |
| 100 | 5 | 0.172 | 0.298 | 1.73× |
| 100 | 10 | 0.408 | 0.672 | 1.65× |
| 1,000 | 1 | 0.049 | 0.106 | 2.17× |
| 1,000 | 5 | 0.210 | 0.351 | 1.67× |
| 1,000 | 10 | 0.412 | 0.709 | 1.72× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
