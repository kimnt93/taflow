# RateOfChangePercent benchmark (`ROCP` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.037 | 26.93M | 0.030 | 33.30M | 0.033 | 0.90× | 1.11× |
| 10,000 | 0.242 | 41.31M | 0.254 | 39.45M | 0.041 | 0.17× | 0.16× |
| 100,000 | 2.289 | 43.68M | 2.241 | 44.63M | 0.124 | 0.05× | 0.06× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.121 | 0.144 | 1.19× |
| 1 | 5 | 0.375 | 0.490 | 1.30× |
| 1 | 10 | 0.604 | 0.960 | 1.59× |
| 10 | 1 | 0.064 | 0.091 | 1.42× |
| 10 | 5 | 0.295 | 0.476 | 1.61× |
| 10 | 10 | 0.739 | 0.911 | 1.23× |
| 100 | 1 | 0.066 | 0.096 | 1.46× |
| 100 | 5 | 0.289 | 0.431 | 1.49× |
| 100 | 10 | 0.642 | 0.917 | 1.43× |
| 1,000 | 1 | 0.090 | 0.095 | 1.05× |
| 1,000 | 5 | 0.313 | 0.443 | 1.42× |
| 1,000 | 10 | 0.630 | 0.937 | 1.49× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
