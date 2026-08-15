# CandleHomingPigeon benchmark (`CDLHOMINGPIGEON` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 170.83M | 0.003 | 365.87M | 0.033 | 5.60× | 11.99× |
| 10,000 | 0.047 | 213.54M | 0.042 | 240.06M | 0.096 | 2.04× | 2.29× |
| 100,000 | 0.637 | 156.91M | 0.625 | 159.98M | 0.768 | 1.21× | 1.23× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.070 | 0.107 | 1.52× |
| 1 | 5 | 0.260 | 0.487 | 1.87× |
| 1 | 10 | 0.404 | 0.952 | 2.36× |
| 10 | 1 | 0.039 | 0.092 | 2.36× |
| 10 | 5 | 0.184 | 0.433 | 2.35× |
| 10 | 10 | 0.429 | 0.929 | 2.17× |
| 100 | 1 | 0.040 | 0.093 | 2.33× |
| 100 | 5 | 0.202 | 0.415 | 2.05× |
| 100 | 10 | 0.389 | 1.020 | 2.62× |
| 1,000 | 1 | 0.062 | 0.103 | 1.66× |
| 1,000 | 5 | 0.228 | 0.585 | 2.56× |
| 1,000 | 10 | 0.405 | 1.040 | 2.57× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
