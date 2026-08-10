# CandleMatchingLow benchmark (`CDLMATCHINGLOW` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.020 | 49.79M | 0.017 | 57.89M | 0.037 | 1.83× | 2.12× |
| 10,000 | 0.127 | 78.85M | 0.120 | 83.67M | 0.108 | 0.85× | 0.90× |
| 100,000 | 1.225 | 81.66M | 1.832 | 54.60M | 0.949 | 0.77× | 0.52× |
| 1,000,000 | 13.348 | 74.92M | 12.201 | 81.96M | 7.887 | 0.59× | 0.65× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.111 | 0.140 | 1.26× |
| 1 | 5 | 0.290 | 0.513 | 1.77× |
| 1 | 10 | 0.602 | 1.204 | 2.00× |
| 10 | 1 | 0.080 | 0.117 | 1.46× |
| 10 | 5 | 0.326 | 0.532 | 1.63× |
| 10 | 10 | 0.626 | 1.167 | 1.86× |
| 100 | 1 | 0.188 | 0.129 | 0.69× |
| 100 | 5 | 0.316 | 0.544 | 1.72× |
| 100 | 10 | 0.638 | 1.096 | 1.72× |
| 1,000 | 1 | 0.079 | 0.127 | 1.60× |
| 1,000 | 5 | 0.355 | 0.571 | 1.61× |
| 1,000 | 10 | 0.725 | 1.164 | 1.61× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
