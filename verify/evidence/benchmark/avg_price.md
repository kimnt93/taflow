# AveragePrice benchmark (`AVGPRICE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 110.59M | 0.008 | 132.96M | 0.036 | 4.03× | 4.84× |
| 10,000 | 0.030 | 328.13M | 0.027 | 375.59M | 0.034 | 1.12× | 1.28× |
| 100,000 | 0.323 | 309.18M | 0.212 | 472.57M | 0.095 | 0.29× | 0.45× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.116 | 0.181 | 1.56× |
| 1 | 5 | 0.347 | 0.468 | 1.35× |
| 1 | 10 | 0.528 | 0.927 | 1.75× |
| 10 | 1 | 0.056 | 0.091 | 1.63× |
| 10 | 5 | 0.225 | 0.427 | 1.90× |
| 10 | 10 | 0.481 | 0.895 | 1.86× |
| 100 | 1 | 0.050 | 0.090 | 1.80× |
| 100 | 5 | 0.243 | 0.424 | 1.74× |
| 100 | 10 | 0.478 | 0.867 | 1.81× |
| 1,000 | 1 | 0.052 | 0.091 | 1.75× |
| 1,000 | 5 | 0.236 | 0.445 | 1.88× |
| 1,000 | 10 | 0.528 | 0.931 | 1.76× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
