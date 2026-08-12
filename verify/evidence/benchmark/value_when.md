# ValueWhen benchmark (`last value when condition` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 97.74M | 0.007 | 150.20M | 0.156 | 15.28× | 23.49× |
| 10,000 | 0.028 | 351.28M | 0.024 | 411.24M | 1.469 | 51.61× | 60.42× |
| 100,000 | 0.217 | 461.48M | 0.179 | 559.83M | 14.489 | 66.86× | 81.11× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.134 | 0.114 | 0.85× |
| 1 | 5 | 0.311 | 0.312 | 1.00× |
| 1 | 10 | 0.503 | 0.661 | 1.31× |
| 10 | 1 | 0.067 | 0.078 | 1.16× |
| 10 | 5 | 0.267 | 0.325 | 1.22× |
| 10 | 10 | 0.499 | 0.682 | 1.37× |
| 100 | 1 | 0.049 | 0.077 | 1.56× |
| 100 | 5 | 0.243 | 0.390 | 1.61× |
| 100 | 10 | 0.561 | 0.817 | 1.46× |
| 1,000 | 1 | 0.052 | 0.221 | 4.23× |
| 1,000 | 5 | 0.245 | 1.126 | 4.59× |
| 1,000 | 10 | 0.584 | 2.128 | 3.65× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
