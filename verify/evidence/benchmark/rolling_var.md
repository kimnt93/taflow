# RollingVariance benchmark (`VAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.036 | 28.12M | 0.033 | 29.93M | 0.038 | 1.08× | 1.15× |
| 10,000 | 0.200 | 49.99M | 0.220 | 45.42M | 0.054 | 0.27× | 0.24× |
| 100,000 | 2.027 | 49.34M | 1.935 | 51.68M | 0.253 | 0.12× | 0.13× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.144 | 0.119 | 0.83× |
| 1 | 5 | 0.307 | 0.517 | 1.69× |
| 1 | 10 | 0.721 | 1.065 | 1.48× |
| 10 | 1 | 0.081 | 0.103 | 1.28× |
| 10 | 5 | 0.406 | 0.505 | 1.24× |
| 10 | 10 | 0.660 | 1.008 | 1.53× |
| 100 | 1 | 0.069 | 0.096 | 1.38× |
| 100 | 5 | 0.327 | 0.453 | 1.39× |
| 100 | 10 | 0.644 | 1.042 | 1.62× |
| 1,000 | 1 | 0.100 | 0.101 | 1.02× |
| 1,000 | 5 | 0.323 | 0.486 | 1.51× |
| 1,000 | 10 | 0.704 | 1.147 | 1.63× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
