# RollingSpearmanCorrelation benchmark (`SpearmanCorrelation` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.439 | 2.28M | 0.451 | 2.22M | 0.762 | 1.73× | 1.69× |
| 10,000 | 4.379 | 2.28M | 4.425 | 2.26M | 6.419 | 1.47× | 1.45× |
| 100,000 | 43.675 | 2.29M | 43.366 | 2.31M | 63.222 | 1.45× | 1.46× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.103 | 0.302 | 2.93× |
| 1 | 5 | 0.242 | 1.307 | 5.41× |
| 1 | 10 | 0.390 | 2.309 | 5.91× |
| 10 | 1 | 0.054 | 0.214 | 3.95× |
| 10 | 5 | 0.190 | 1.228 | 6.47× |
| 10 | 10 | 0.406 | 2.368 | 5.83× |
| 100 | 1 | 0.084 | 0.271 | 3.24× |
| 100 | 5 | 0.201 | 1.506 | 7.47× |
| 100 | 10 | 0.539 | 3.402 | 6.31× |
| 1,000 | 1 | 0.498 | 1.015 | 2.04× |
| 1,000 | 5 | 1.036 | 5.046 | 4.87× |
| 1,000 | 10 | 1.184 | 9.197 | 7.77× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
