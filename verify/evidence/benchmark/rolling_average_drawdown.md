# RollingAverageDrawdown benchmark (`AverageDrawdown` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.049 | 20.36M | 0.048 | 20.65M | 0.203 | 4.13× | 4.19× |
| 10,000 | 0.476 | 21.03M | 0.481 | 20.77M | 1.021 | 2.15× | 2.12× |
| 100,000 | 5.009 | 19.96M | 4.904 | 20.39M | 8.727 | 1.74× | 1.78× |
| 1,000,000 | 49.481 | 20.21M | 49.544 | 20.18M | 88.756 | 1.79× | 1.79× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.073 | 0.232 | 3.16× |
| 1 | 5 | 0.317 | 1.016 | 3.20× |
| 1 | 10 | 0.453 | 2.071 | 4.57× |
| 10 | 1 | 0.050 | 0.185 | 3.71× |
| 10 | 5 | 0.227 | 0.928 | 4.08× |
| 10 | 10 | 0.471 | 2.101 | 4.46× |
| 100 | 1 | 0.053 | 0.199 | 3.75× |
| 100 | 5 | 0.237 | 0.992 | 4.19× |
| 100 | 10 | 0.514 | 2.157 | 4.19× |
| 1,000 | 1 | 0.113 | 0.290 | 2.56× |
| 1,000 | 5 | 0.238 | 1.445 | 6.08× |
| 1,000 | 10 | 0.545 | 3.091 | 5.67× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
