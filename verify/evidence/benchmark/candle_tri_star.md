# CandleTriStar benchmark (`CDLTRISTAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 160.95M | 0.003 | 325.66M | 0.035 | 5.58× | 11.29× |
| 10,000 | 0.046 | 217.97M | 0.042 | 240.32M | 0.099 | 2.17× | 2.39× |
| 100,000 | 0.542 | 184.64M | 0.522 | 191.45M | 0.637 | 1.18× | 1.22× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.067 | 0.118 | 1.77× |
| 1 | 5 | 0.273 | 0.496 | 1.81× |
| 1 | 10 | 0.377 | 0.907 | 2.41× |
| 10 | 1 | 0.048 | 0.097 | 2.04× |
| 10 | 5 | 0.193 | 0.463 | 2.39× |
| 10 | 10 | 0.396 | 0.907 | 2.29× |
| 100 | 1 | 0.041 | 0.100 | 2.42× |
| 100 | 5 | 0.188 | 0.448 | 2.39× |
| 100 | 10 | 0.434 | 0.938 | 2.16× |
| 1,000 | 1 | 0.049 | 0.093 | 1.88× |
| 1,000 | 5 | 0.208 | 0.463 | 2.22× |
| 1,000 | 10 | 0.417 | 1.019 | 2.44× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
