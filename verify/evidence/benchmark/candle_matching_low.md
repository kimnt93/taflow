# CandleMatchingLow benchmark (`CDLMATCHINGLOW` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 173.73M | 0.002 | 428.16M | 0.033 | 5.75× | 14.16× |
| 10,000 | 0.042 | 238.26M | 0.036 | 279.25M | 0.090 | 2.15× | 2.52× |
| 100,000 | 0.446 | 224.37M | 0.440 | 227.51M | 0.656 | 1.47× | 1.49× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.072 | 0.105 | 1.45× |
| 1 | 5 | 0.234 | 0.453 | 1.94× |
| 1 | 10 | 0.442 | 0.948 | 2.14× |
| 10 | 1 | 0.043 | 0.097 | 2.24× |
| 10 | 5 | 0.172 | 0.440 | 2.56× |
| 10 | 10 | 0.389 | 0.957 | 2.46× |
| 100 | 1 | 0.050 | 0.093 | 1.86× |
| 100 | 5 | 0.209 | 0.422 | 2.02× |
| 100 | 10 | 0.409 | 0.906 | 2.22× |
| 1,000 | 1 | 0.052 | 0.095 | 1.81× |
| 1,000 | 5 | 0.227 | 0.490 | 2.15× |
| 1,000 | 10 | 0.426 | 0.972 | 2.28× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
