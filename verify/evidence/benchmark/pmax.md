# ParabolicMovingAverageStop benchmark (`pmax` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.025 | 40.48M | 0.024 | 42.04M | 3.298 | 133.50× | 138.62× |
| 10,000 | 0.190 | 52.63M | 0.193 | 51.92M | 17.683 | 93.07× | 91.81× |
| 100,000 | 1.963 | 50.95M | 1.895 | 52.77M | 160.560 | 81.80× | 84.73× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.097 | 0.299 | 3.10× |
| 1 | 5 | 0.282 | 1.180 | 4.19× |
| 1 | 10 | 0.517 | 2.393 | 4.63× |
| 10 | 1 | 0.054 | 1.845 | 34.11× |
| 10 | 5 | 0.273 | 8.798 | 32.20× |
| 10 | 10 | 0.582 | 18.280 | 31.42× |
| 100 | 1 | 0.061 | 1.772 | 29.07× |
| 100 | 5 | 0.356 | 9.602 | 26.94× |
| 100 | 10 | 0.599 | 19.273 | 32.18× |
| 1,000 | 1 | 0.081 | 3.247 | 40.15× |
| 1,000 | 5 | 0.278 | 16.989 | 61.17× |
| 1,000 | 10 | 0.544 | 33.765 | 62.07× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
