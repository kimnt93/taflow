# RollingMedian benchmark (`MedianMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.034 | 29.05M | 0.034 | 29.28M | 0.330 | 9.59× | 9.66× |
| 10,000 | 0.394 | 25.39M | 0.383 | 26.10M | 1.883 | 4.78× | 4.92× |
| 100,000 | 3.959 | 25.26M | 3.922 | 25.50M | 18.075 | 4.57× | 4.61× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.097 | 0.239 | 2.48× |
| 1 | 5 | 0.249 | 1.087 | 4.37× |
| 1 | 10 | 0.401 | 2.569 | 6.41× |
| 10 | 1 | 0.045 | 0.213 | 4.72× |
| 10 | 5 | 0.204 | 1.097 | 5.39× |
| 10 | 10 | 0.486 | 2.272 | 4.67× |
| 100 | 1 | 0.051 | 0.226 | 4.47× |
| 100 | 5 | 0.214 | 1.404 | 6.57× |
| 100 | 10 | 0.442 | 2.453 | 5.55× |
| 1,000 | 1 | 0.097 | 0.406 | 4.20× |
| 1,000 | 5 | 0.212 | 2.143 | 10.12× |
| 1,000 | 10 | 0.453 | 4.214 | 9.30× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
