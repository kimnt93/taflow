# CandleAbandonedBaby benchmark (`CDLABANDONEDBABY` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 137.67M | 0.004 | 244.10M | 0.039 | 5.37× | 9.52× |
| 10,000 | 0.090 | 110.75M | 0.082 | 121.23M | 0.136 | 1.51× | 1.65× |
| 100,000 | 0.941 | 106.24M | 0.947 | 105.54M | 1.067 | 1.13× | 1.13× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.099 | 0.140 | 1.41× |
| 1 | 5 | 0.215 | 0.475 | 2.21× |
| 1 | 10 | 0.401 | 0.996 | 2.49× |
| 10 | 1 | 0.056 | 0.128 | 2.30× |
| 10 | 5 | 0.214 | 0.487 | 2.27× |
| 10 | 10 | 0.392 | 1.034 | 2.64× |
| 100 | 1 | 0.043 | 0.089 | 2.06× |
| 100 | 5 | 0.187 | 0.509 | 2.73× |
| 100 | 10 | 0.479 | 1.006 | 2.10× |
| 1,000 | 1 | 0.049 | 0.112 | 2.29× |
| 1,000 | 5 | 0.203 | 0.514 | 2.53× |
| 1,000 | 10 | 0.436 | 1.172 | 2.69× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
