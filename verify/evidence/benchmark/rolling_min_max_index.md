# RollingMinMaxIndex benchmark (`MINMAXINDEX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.026 | 37.95M | 0.026 | 38.45M | 0.052 | 1.96× | 1.99× |
| 10,000 | 0.304 | 32.90M | 0.297 | 33.69M | 0.169 | 0.56× | 0.57× |
| 100,000 | 2.996 | 33.38M | 2.824 | 35.42M | 1.300 | 0.43× | 0.46× |
| 1,000,000 | 29.691 | 33.68M | 29.075 | 34.39M | 12.382 | 0.42× | 0.43× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.110 | 0.128 | 1.17× |
| 1 | 5 | 0.257 | 0.555 | 2.16× |
| 1 | 10 | 0.586 | 1.149 | 1.96× |
| 10 | 1 | 0.057 | 0.100 | 1.74× |
| 10 | 5 | 0.253 | 0.635 | 2.51× |
| 10 | 10 | 0.542 | 1.146 | 2.11× |
| 100 | 1 | 0.057 | 0.099 | 1.74× |
| 100 | 5 | 0.290 | 0.592 | 2.04× |
| 100 | 10 | 0.566 | 1.148 | 2.03× |
| 1,000 | 1 | 0.089 | 0.115 | 1.29× |
| 1,000 | 5 | 0.334 | 0.651 | 1.95× |
| 1,000 | 10 | 0.657 | 1.220 | 1.86× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
