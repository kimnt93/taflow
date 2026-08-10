# RollingQuantile benchmark (`RollingQuantile` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.049 | 20.22M | 0.046 | 21.73M | 0.339 | 6.85× | 7.37× |
| 10,000 | 0.491 | 20.38M | 0.480 | 20.85M | 1.744 | 3.55× | 3.64× |
| 100,000 | 4.882 | 20.48M | 4.851 | 20.62M | 16.403 | 3.36× | 3.38× |
| 1,000,000 | 50.055 | 19.98M | 48.198 | 20.75M | 165.088 | 3.30× | 3.43× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.151 | 0.307 | 2.04× |
| 1 | 5 | 0.285 | 1.375 | 4.83× |
| 1 | 10 | 0.541 | 2.730 | 5.05× |
| 10 | 1 | 0.054 | 0.254 | 4.67× |
| 10 | 5 | 0.242 | 1.472 | 6.08× |
| 10 | 10 | 0.579 | 2.553 | 4.41× |
| 100 | 1 | 0.061 | 0.260 | 4.27× |
| 100 | 5 | 0.267 | 1.569 | 5.87× |
| 100 | 10 | 0.527 | 2.976 | 5.64× |
| 1,000 | 1 | 0.132 | 0.445 | 3.36× |
| 1,000 | 5 | 0.241 | 2.289 | 9.51× |
| 1,000 | 10 | 0.591 | 4.470 | 7.57× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
