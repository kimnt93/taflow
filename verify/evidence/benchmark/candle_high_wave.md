# CandleHighWave benchmark (`CDLHIGHWAVE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.022 | 44.88M | 0.015 | 68.62M | 0.037 | 1.64× | 2.51× |
| 10,000 | 0.152 | 65.61M | 0.146 | 68.32M | 0.168 | 1.10× | 1.14× |
| 100,000 | 1.476 | 67.75M | 1.477 | 67.68M | 1.540 | 1.04× | 1.04× |
| 1,000,000 | 16.335 | 61.22M | 16.179 | 61.81M | 13.855 | 0.85× | 0.86× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.097 | 0.126 | 1.29× |
| 1 | 5 | 0.312 | 0.508 | 1.63× |
| 1 | 10 | 0.548 | 0.941 | 1.72× |
| 10 | 1 | 0.056 | 0.093 | 1.67× |
| 10 | 5 | 0.270 | 0.471 | 1.75× |
| 10 | 10 | 0.536 | 0.923 | 1.72× |
| 100 | 1 | 0.056 | 0.090 | 1.62× |
| 100 | 5 | 0.260 | 0.494 | 1.90× |
| 100 | 10 | 0.582 | 0.961 | 1.65× |
| 1,000 | 1 | 0.066 | 0.109 | 1.65× |
| 1,000 | 5 | 0.272 | 0.495 | 1.82× |
| 1,000 | 10 | 0.627 | 1.167 | 1.86× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
