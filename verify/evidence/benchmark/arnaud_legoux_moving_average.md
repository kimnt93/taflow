# ArnaudLegouxMovingAverage benchmark (`ALMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.018 | 54.59M | 0.016 | 61.43M | 0.255 | 13.94× | 15.68× |
| 10,000 | 0.156 | 63.99M | 0.154 | 64.76M | 0.596 | 3.81× | 3.86× |
| 100,000 | 1.505 | 66.46M | 1.487 | 67.23M | 4.361 | 2.90× | 2.93× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.078 | 0.311 | 4.00× |
| 1 | 5 | 0.214 | 1.477 | 6.92× |
| 1 | 10 | 0.425 | 3.000 | 7.06× |
| 10 | 1 | 0.048 | 0.270 | 5.57× |
| 10 | 5 | 0.210 | 1.518 | 7.24× |
| 10 | 10 | 0.419 | 3.128 | 7.47× |
| 100 | 1 | 0.050 | 0.288 | 5.77× |
| 100 | 5 | 0.209 | 1.485 | 7.10× |
| 100 | 10 | 0.438 | 3.188 | 7.27× |
| 1,000 | 1 | 0.063 | 0.327 | 5.22× |
| 1,000 | 5 | 0.221 | 1.760 | 7.98× |
| 1,000 | 10 | 0.432 | 3.540 | 8.20× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
