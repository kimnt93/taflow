# FisherTransform benchmark (`fisher` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.037 | 27.15M | 0.035 | 28.18M | 1.220 | 33.13× | 34.38× |
| 10,000 | 0.378 | 26.47M | 0.379 | 26.36M | 1.657 | 4.39× | 4.37× |
| 100,000 | 3.998 | 25.01M | 3.988 | 25.07M | 6.870 | 1.72× | 1.72× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.129 | 0.260 | 2.01× |
| 1 | 5 | 0.270 | 0.981 | 3.63× |
| 1 | 10 | 0.378 | 1.808 | 4.78× |
| 10 | 1 | 0.045 | 1.354 | 29.78× |
| 10 | 5 | 0.209 | 6.524 | 31.18× |
| 10 | 10 | 0.365 | 13.080 | 35.86× |
| 100 | 1 | 0.057 | 1.282 | 22.56× |
| 100 | 5 | 0.201 | 6.527 | 32.55× |
| 100 | 10 | 0.403 | 13.307 | 33.04× |
| 1,000 | 1 | 0.089 | 1.348 | 15.08× |
| 1,000 | 5 | 0.194 | 7.076 | 36.45× |
| 1,000 | 10 | 0.486 | 14.511 | 29.88× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
