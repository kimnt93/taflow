# RollingMedian benchmark (`MedianMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.053 | 18.94M | 0.041 | 24.27M | 0.461 | 8.73× | 11.19× |
| 10,000 | 0.461 | 21.71M | 0.456 | 21.92M | 2.375 | 5.16× | 5.20× |
| 100,000 | 4.646 | 21.52M | 4.745 | 21.08M | 27.203 | 5.86× | 5.73× |
| 1,000,000 | 45.715 | 21.87M | 44.221 | 22.61M | 202.396 | 4.43× | 4.58× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.129 | 0.457 | 3.54× |
| 1 | 5 | 0.240 | 1.084 | 4.52× |
| 1 | 10 | 0.523 | 3.020 | 5.77× |
| 10 | 1 | 0.075 | 0.307 | 4.08× |
| 10 | 5 | 0.398 | 1.451 | 3.65× |
| 10 | 10 | 0.561 | 2.241 | 3.99× |
| 100 | 1 | 0.064 | 0.240 | 3.77× |
| 100 | 5 | 0.310 | 1.403 | 4.53× |
| 100 | 10 | 0.529 | 2.766 | 5.23× |
| 1,000 | 1 | 0.113 | 0.427 | 3.77× |
| 1,000 | 5 | 0.285 | 2.270 | 7.96× |
| 1,000 | 10 | 0.613 | 4.312 | 7.03× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
