# CandleAbandonedBaby benchmark (`CDLABANDONEDBABY` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 137.77M | 0.004 | 263.67M | 0.037 | 5.09× | 9.74× |
| 10,000 | 0.083 | 120.01M | 0.077 | 129.10M | 0.144 | 1.73× | 1.86× |
| 100,000 | 0.952 | 104.99M | 0.885 | 113.02M | 1.033 | 1.08× | 1.17× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.096 | 0.111 | 1.15× |
| 1 | 5 | 0.305 | 0.486 | 1.60× |
| 1 | 10 | 0.404 | 0.956 | 2.37× |
| 10 | 1 | 0.042 | 0.087 | 2.06× |
| 10 | 5 | 0.184 | 0.425 | 2.30× |
| 10 | 10 | 0.386 | 0.949 | 2.46× |
| 100 | 1 | 0.048 | 0.091 | 1.90× |
| 100 | 5 | 0.183 | 0.454 | 2.49× |
| 100 | 10 | 0.384 | 0.939 | 2.45× |
| 1,000 | 1 | 0.053 | 0.102 | 1.92× |
| 1,000 | 5 | 0.190 | 0.496 | 2.61× |
| 1,000 | 10 | 0.423 | 1.027 | 2.43× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
