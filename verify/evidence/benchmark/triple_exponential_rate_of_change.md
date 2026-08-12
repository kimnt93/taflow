# TripleExponentialRateOfChange benchmark (`TRIX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 144.15M | 0.005 | 189.65M | 0.040 | 5.72× | 7.53× |
| 10,000 | 0.030 | 330.36M | 0.032 | 315.94M | 0.131 | 4.33× | 4.14× |
| 100,000 | 0.257 | 388.40M | 0.228 | 438.29M | 1.002 | 3.89× | 4.39× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.118 | 0.105 | 0.89× |
| 1 | 5 | 0.248 | 0.456 | 1.84× |
| 1 | 10 | 0.461 | 0.967 | 2.10× |
| 10 | 1 | 0.051 | 0.091 | 1.80× |
| 10 | 5 | 0.213 | 0.428 | 2.01× |
| 10 | 10 | 0.442 | 0.923 | 2.09× |
| 100 | 1 | 0.062 | 0.100 | 1.63× |
| 100 | 5 | 0.255 | 0.456 | 1.79× |
| 100 | 10 | 0.505 | 0.934 | 1.85× |
| 1,000 | 1 | 0.060 | 0.102 | 1.71× |
| 1,000 | 5 | 0.226 | 0.492 | 2.18× |
| 1,000 | 10 | 0.492 | 0.994 | 2.02× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
