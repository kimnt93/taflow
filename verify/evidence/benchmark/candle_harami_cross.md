# CandleHaramiCross benchmark (`CDLHARAMICROSS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.023 | 42.93M | 0.018 | 56.31M | 0.039 | 1.68× | 2.21× |
| 10,000 | 0.159 | 62.76M | 0.158 | 63.40M | 0.148 | 0.93× | 0.94× |
| 100,000 | 1.610 | 62.11M | 1.619 | 61.77M | 1.245 | 0.77× | 0.77× |
| 1,000,000 | 16.022 | 62.42M | 15.671 | 63.81M | 12.037 | 0.75× | 0.77× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.088 | 0.117 | 1.33× |
| 1 | 5 | 0.346 | 0.512 | 1.48× |
| 1 | 10 | 0.552 | 1.027 | 1.86× |
| 10 | 1 | 0.078 | 0.098 | 1.25× |
| 10 | 5 | 0.311 | 0.555 | 1.78× |
| 10 | 10 | 0.583 | 0.989 | 1.69× |
| 100 | 1 | 0.066 | 0.095 | 1.45× |
| 100 | 5 | 0.361 | 0.522 | 1.44× |
| 100 | 10 | 0.616 | 0.945 | 1.53× |
| 1,000 | 1 | 0.068 | 0.101 | 1.48× |
| 1,000 | 5 | 0.334 | 0.696 | 2.08× |
| 1,000 | 10 | 0.661 | 1.046 | 1.58× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
