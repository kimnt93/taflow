# RogersSatchell benchmark (`RogersSatchellVolatility` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.108 | 9.26M | 0.101 | 9.91M | 0.309 | 2.86× | 3.06× |
| 10,000 | 0.884 | 11.32M | 0.893 | 11.20M | 1.638 | 1.85× | 1.83× |
| 100,000 | 9.530 | 10.49M | 8.574 | 11.66M | 14.818 | 1.55× | 1.73× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.136 | 0.346 | 2.54× |
| 1 | 5 | 0.480 | 1.424 | 2.96× |
| 1 | 10 | 0.683 | 2.483 | 3.63× |
| 10 | 1 | 0.077 | 0.241 | 3.15× |
| 10 | 5 | 0.321 | 1.469 | 4.58× |
| 10 | 10 | 0.673 | 2.787 | 4.14× |
| 100 | 1 | 0.094 | 0.247 | 2.62× |
| 100 | 5 | 0.319 | 1.487 | 4.66× |
| 100 | 10 | 0.901 | 3.959 | 4.40× |
| 1,000 | 1 | 0.275 | 0.576 | 2.10× |
| 1,000 | 5 | 0.613 | 3.304 | 5.39× |
| 1,000 | 10 | 0.833 | 5.170 | 6.21× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
