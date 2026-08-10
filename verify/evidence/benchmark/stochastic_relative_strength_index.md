# StochasticRelativeStrengthIndex benchmark (`STOCHRSI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.027 | 37.19M | 0.024 | 42.51M | 0.058 | 2.15× | 2.45× |
| 10,000 | 0.253 | 39.54M | 0.242 | 41.28M | 0.210 | 0.83× | 0.87× |
| 100,000 | 2.773 | 36.06M | 2.587 | 38.66M | 1.693 | 0.61× | 0.65× |
| 1,000,000 | 28.077 | 35.62M | 26.886 | 37.19M | 16.839 | 0.60× | 0.63× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.097 | 0.133 | 1.37× |
| 1 | 5 | 0.323 | 0.658 | 2.04× |
| 1 | 10 | 0.517 | 1.191 | 2.30× |
| 10 | 1 | 0.054 | 0.108 | 2.00× |
| 10 | 5 | 0.278 | 0.687 | 2.47× |
| 10 | 10 | 0.467 | 1.197 | 2.57× |
| 100 | 1 | 0.068 | 0.136 | 1.99× |
| 100 | 5 | 0.258 | 0.625 | 2.43× |
| 100 | 10 | 0.606 | 1.086 | 1.79× |
| 1,000 | 1 | 0.073 | 0.132 | 1.81× |
| 1,000 | 5 | 0.286 | 0.674 | 2.36× |
| 1,000 | 10 | 0.621 | 1.273 | 2.05× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
