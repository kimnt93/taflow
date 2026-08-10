# CandleHaramiCross benchmark (`CDLHARAMICROSS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.018 | 56.75M | 0.018 | 55.03M | 0.034 | 1.91× | 1.86× |
| 10,000 | 0.140 | 71.48M | 0.152 | 65.61M | 0.135 | 0.96× | 0.88× |
| 100,000 | 1.307 | 76.52M | 1.462 | 68.42M | 1.075 | 0.82× | 0.74× |
| 1,000,000 | 13.665 | 73.18M | 14.926 | 67.00M | 11.053 | 0.81× | 0.74× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.151 | 0.141 | 0.93× |
| 1 | 5 | 0.413 | 0.444 | 1.07× |
| 1 | 10 | 0.528 | 0.922 | 1.75× |
| 10 | 1 | 0.062 | 0.089 | 1.44× |
| 10 | 5 | 0.246 | 0.419 | 1.70× |
| 10 | 10 | 0.523 | 0.919 | 1.76× |
| 100 | 1 | 0.068 | 0.135 | 1.98× |
| 100 | 5 | 0.293 | 0.448 | 1.53× |
| 100 | 10 | 0.562 | 0.942 | 1.68× |
| 1,000 | 1 | 0.071 | 0.098 | 1.38× |
| 1,000 | 5 | 0.312 | 0.540 | 1.73× |
| 1,000 | 10 | 0.662 | 0.981 | 1.48× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
