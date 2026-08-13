# CandleInNeck benchmark (`CDLINNECK` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.104 | 9.63M | 0.096 | 10.46M | 0.032 | 0.31× | 0.33× |
| 10,000 | 0.858 | 11.65M | 0.857 | 11.66M | 0.117 | 0.14× | 0.14× |
| 100,000 | 8.270 | 12.09M | 8.149 | 12.27M | 0.891 | 0.11× | 0.11× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.125 | 0.134 | 1.07× |
| 1 | 5 | 0.547 | 0.454 | 0.83× |
| 1 | 10 | 0.650 | 0.954 | 1.47× |
| 10 | 1 | 0.070 | 0.090 | 1.29× |
| 10 | 5 | 0.319 | 0.424 | 1.33× |
| 10 | 10 | 0.645 | 0.918 | 1.42× |
| 100 | 1 | 0.076 | 0.099 | 1.30× |
| 100 | 5 | 0.322 | 0.432 | 1.34× |
| 100 | 10 | 0.636 | 0.889 | 1.40× |
| 1,000 | 1 | 0.152 | 0.099 | 0.65× |
| 1,000 | 5 | 0.338 | 0.475 | 1.40× |
| 1,000 | 10 | 0.698 | 0.994 | 1.42× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
