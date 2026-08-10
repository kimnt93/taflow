# RollingPercentile benchmark (`rolling percentile` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.043 | 23.03M | 0.043 | 23.10M | 0.350 | 8.05× | 8.08× |
| 10,000 | 0.464 | 21.53M | 0.461 | 21.67M | 2.257 | 4.86× | 4.89× |
| 100,000 | 4.612 | 21.68M | 4.576 | 21.85M | 20.554 | 4.46× | 4.49× |
| 1,000,000 | 46.611 | 21.45M | 46.082 | 21.70M | 209.861 | 4.50× | 4.55× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.147 | 0.237 | 1.61× |
| 1 | 5 | 0.294 | 1.074 | 3.65× |
| 1 | 10 | 0.505 | 2.484 | 4.92× |
| 10 | 1 | 0.050 | 0.192 | 3.81× |
| 10 | 5 | 0.258 | 1.038 | 4.02× |
| 10 | 10 | 0.522 | 2.441 | 4.67× |
| 100 | 1 | 0.063 | 0.266 | 4.23× |
| 100 | 5 | 0.243 | 1.251 | 5.14× |
| 100 | 10 | 0.508 | 2.612 | 5.14× |
| 1,000 | 1 | 0.101 | 0.445 | 4.41× |
| 1,000 | 5 | 0.238 | 1.472 | 6.17× |
| 1,000 | 10 | 0.530 | 3.006 | 5.67× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
