# CandleThreeStarsInSouth benchmark (`CDL3STARSINSOUTH` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 80.48M | 0.010 | 104.47M | 0.033 | 2.66× | 3.45× |
| 10,000 | 0.069 | 144.65M | 0.067 | 149.93M | 0.110 | 1.59× | 1.65× |
| 100,000 | 0.696 | 143.62M | 0.684 | 146.10M | 0.867 | 1.24× | 1.27× |
| 1,000,000 | 8.077 | 123.81M | 7.721 | 129.52M | 8.986 | 1.11× | 1.16× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.133 | 0.139 | 1.05× |
| 1 | 5 | 0.269 | 0.463 | 1.72× |
| 1 | 10 | 0.606 | 0.919 | 1.52× |
| 10 | 1 | 0.053 | 0.095 | 1.77× |
| 10 | 5 | 0.301 | 0.462 | 1.54× |
| 10 | 10 | 0.546 | 0.883 | 1.62× |
| 100 | 1 | 0.059 | 0.090 | 1.52× |
| 100 | 5 | 0.263 | 0.433 | 1.65× |
| 100 | 10 | 0.588 | 0.919 | 1.56× |
| 1,000 | 1 | 0.068 | 0.096 | 1.42× |
| 1,000 | 5 | 0.259 | 0.455 | 1.76× |
| 1,000 | 10 | 0.547 | 1.115 | 2.04× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
