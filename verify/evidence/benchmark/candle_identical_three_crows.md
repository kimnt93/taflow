# CandleIdenticalThreeCrows benchmark (`CDLIDENTICAL3CROWS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.024 | 41.53M | 0.021 | 48.24M | 0.038 | 1.60× | 1.86× |
| 10,000 | 0.166 | 60.26M | 0.160 | 62.37M | 0.125 | 0.75× | 0.78× |
| 100,000 | 1.524 | 65.62M | 1.521 | 65.75M | 0.993 | 0.65× | 0.65× |
| 1,000,000 | 17.246 | 57.98M | 15.376 | 65.04M | 9.801 | 0.57× | 0.64× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.097 | 0.128 | 1.33× |
| 1 | 5 | 0.347 | 0.824 | 2.38× |
| 1 | 10 | 0.517 | 1.049 | 2.03× |
| 10 | 1 | 0.100 | 0.133 | 1.33× |
| 10 | 5 | 0.318 | 0.551 | 1.73× |
| 10 | 10 | 0.613 | 0.984 | 1.60× |
| 100 | 1 | 0.081 | 0.179 | 2.21× |
| 100 | 5 | 0.330 | 0.490 | 1.48× |
| 100 | 10 | 0.593 | 0.969 | 1.63× |
| 1,000 | 1 | 0.079 | 0.105 | 1.34× |
| 1,000 | 5 | 0.330 | 0.538 | 1.63× |
| 1,000 | 10 | 0.684 | 1.075 | 1.57× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
