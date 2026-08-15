# HilbertTransformTrendline benchmark (`HT_TRENDLINE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.070 | 14.24M | 0.069 | 14.42M | 0.078 | 1.11× | 1.13× |
| 10,000 | 0.690 | 14.49M | 0.688 | 14.53M | 0.598 | 0.87× | 0.87× |
| 100,000 | 6.996 | 14.29M | 6.915 | 14.46M | 5.925 | 0.85× | 0.86× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.123 | 0.105 | 0.86× |
| 1 | 5 | 0.227 | 0.445 | 1.96× |
| 1 | 10 | 0.359 | 0.883 | 2.46× |
| 10 | 1 | 0.041 | 0.086 | 2.09× |
| 10 | 5 | 0.177 | 0.464 | 2.62× |
| 10 | 10 | 0.385 | 0.918 | 2.39× |
| 100 | 1 | 0.046 | 0.092 | 2.02× |
| 100 | 5 | 0.188 | 0.444 | 2.36× |
| 100 | 10 | 0.388 | 0.922 | 2.38× |
| 1,000 | 1 | 0.117 | 0.157 | 1.34× |
| 1,000 | 5 | 0.235 | 0.717 | 3.05× |
| 1,000 | 10 | 0.453 | 1.523 | 3.36× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
