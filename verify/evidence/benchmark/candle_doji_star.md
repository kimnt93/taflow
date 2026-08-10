# CandleDojiStar benchmark (`CDLDOJISTAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.023 | 43.69M | 0.018 | 55.18M | 0.044 | 1.91× | 2.42× |
| 10,000 | 0.188 | 53.26M | 0.180 | 55.52M | 0.157 | 0.84× | 0.87× |
| 100,000 | 1.820 | 54.94M | 2.134 | 46.86M | 1.260 | 0.69× | 0.59× |
| 1,000,000 | 18.465 | 54.16M | 18.764 | 53.29M | 12.265 | 0.66× | 0.65× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.092 | 0.145 | 1.58× |
| 1 | 5 | 0.401 | 0.701 | 1.75× |
| 1 | 10 | 0.731 | 1.088 | 1.49× |
| 10 | 1 | 0.058 | 0.091 | 1.57× |
| 10 | 5 | 0.324 | 0.569 | 1.76× |
| 10 | 10 | 0.638 | 1.169 | 1.83× |
| 100 | 1 | 0.104 | 0.129 | 1.23× |
| 100 | 5 | 0.340 | 0.539 | 1.59× |
| 100 | 10 | 0.634 | 0.984 | 1.55× |
| 1,000 | 1 | 0.073 | 0.108 | 1.48× |
| 1,000 | 5 | 0.366 | 0.593 | 1.62× |
| 1,000 | 10 | 0.655 | 1.100 | 1.68× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
