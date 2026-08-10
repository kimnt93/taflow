# VolumePriceTrend benchmark (`VolumePriceTrend` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 132.27M | 0.006 | 164.27M | 0.157 | 20.76× | 25.78× |
| 10,000 | 0.032 | 316.89M | 0.027 | 364.91M | 0.674 | 21.35× | 24.59× |
| 100,000 | 0.258 | 387.52M | 0.237 | 422.53M | 6.265 | 24.28× | 26.47× |
| 1,000,000 | 3.120 | 320.49M | 2.602 | 384.27M | 61.363 | 19.67× | 23.58× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.095 | 0.210 | 2.22× |
| 1 | 5 | 0.279 | 0.849 | 3.04× |
| 1 | 10 | 0.502 | 1.960 | 3.90× |
| 10 | 1 | 0.054 | 0.170 | 3.13× |
| 10 | 5 | 0.230 | 0.822 | 3.57× |
| 10 | 10 | 0.499 | 2.054 | 4.12× |
| 100 | 1 | 0.051 | 0.177 | 3.50× |
| 100 | 5 | 0.224 | 0.862 | 3.85× |
| 100 | 10 | 0.470 | 1.727 | 3.67× |
| 1,000 | 1 | 0.053 | 0.230 | 4.33× |
| 1,000 | 5 | 0.244 | 1.629 | 6.67× |
| 1,000 | 10 | 0.534 | 2.415 | 4.52× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
