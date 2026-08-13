# RollingPercentile benchmark (`rolling percentile` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.307 | 3.26M | 0.307 | 3.26M | 0.402 | 1.31× | 1.31× |
| 10,000 | 2.963 | 3.38M | 2.918 | 3.43M | 2.192 | 0.74× | 0.75× |
| 100,000 | 29.570 | 3.38M | 29.671 | 3.37M | 20.617 | 0.70× | 0.69× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.145 | 0.253 | 1.75× |
| 1 | 5 | 0.530 | 1.023 | 1.93× |
| 1 | 10 | 0.600 | 2.107 | 3.51× |
| 10 | 1 | 0.073 | 0.201 | 2.75× |
| 10 | 5 | 0.311 | 0.989 | 3.18× |
| 10 | 10 | 0.646 | 2.143 | 3.32× |
| 100 | 1 | 0.097 | 0.258 | 2.66× |
| 100 | 5 | 0.312 | 1.203 | 3.86× |
| 100 | 10 | 0.635 | 2.509 | 3.95× |
| 1,000 | 1 | 0.379 | 0.440 | 1.16× |
| 1,000 | 5 | 0.537 | 1.536 | 2.86× |
| 1,000 | 10 | 0.983 | 2.967 | 3.02× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
