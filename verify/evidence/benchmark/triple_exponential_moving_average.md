# TripleExponentialMovingAverage benchmark (`TEMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 161.67M | 0.006 | 169.65M | 0.041 | 6.64× | 6.97× |
| 10,000 | 0.046 | 217.51M | 0.049 | 203.64M | 0.120 | 2.60× | 2.44× |
| 100,000 | 0.442 | 226.44M | 0.472 | 211.75M | 0.906 | 2.05× | 1.92× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.055 | 0.108 | 1.97× |
| 1 | 5 | 0.319 | 0.465 | 1.46× |
| 1 | 10 | 0.417 | 1.035 | 2.48× |
| 10 | 1 | 0.050 | 0.099 | 1.97× |
| 10 | 5 | 0.188 | 0.440 | 2.34× |
| 10 | 10 | 0.402 | 0.911 | 2.26× |
| 100 | 1 | 0.052 | 0.091 | 1.77× |
| 100 | 5 | 0.207 | 0.501 | 2.42× |
| 100 | 10 | 0.388 | 0.955 | 2.46× |
| 1,000 | 1 | 0.046 | 0.103 | 2.24× |
| 1,000 | 5 | 0.205 | 0.479 | 2.34× |
| 1,000 | 10 | 0.468 | 1.040 | 2.22× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
