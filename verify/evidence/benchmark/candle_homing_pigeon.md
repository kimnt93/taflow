# CandleHomingPigeon benchmark (`CDLHOMINGPIGEON` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 72.22M | 0.010 | 97.89M | 0.032 | 2.34× | 3.17× |
| 10,000 | 0.111 | 89.92M | 0.110 | 91.31M | 0.099 | 0.89× | 0.90× |
| 100,000 | 1.108 | 90.26M | 1.067 | 93.72M | 0.747 | 0.67× | 0.70× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.090 | 0.119 | 1.32× |
| 1 | 5 | 0.268 | 0.447 | 1.67× |
| 1 | 10 | 0.386 | 0.979 | 2.54× |
| 10 | 1 | 0.046 | 0.104 | 2.24× |
| 10 | 5 | 0.203 | 0.448 | 2.20× |
| 10 | 10 | 0.399 | 0.895 | 2.24× |
| 100 | 1 | 0.044 | 0.085 | 1.93× |
| 100 | 5 | 0.175 | 0.427 | 2.44× |
| 100 | 10 | 0.400 | 0.902 | 2.26× |
| 1,000 | 1 | 0.054 | 0.097 | 1.80× |
| 1,000 | 5 | 0.195 | 0.469 | 2.41× |
| 1,000 | 10 | 0.405 | 1.032 | 2.55× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
