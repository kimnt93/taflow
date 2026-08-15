# RollingQuantile benchmark (`RollingQuantile` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.043 | 23.50M | 0.042 | 23.58M | 0.359 | 8.43× | 8.45× |
| 10,000 | 0.503 | 19.89M | 0.440 | 22.73M | 1.828 | 3.64× | 4.15× |
| 100,000 | 4.407 | 22.69M | 4.841 | 20.66M | 18.108 | 4.11× | 3.74× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.066 | 0.286 | 4.35× |
| 1 | 5 | 0.260 | 1.471 | 5.67× |
| 1 | 10 | 0.418 | 2.493 | 5.96× |
| 10 | 1 | 0.050 | 0.262 | 5.20× |
| 10 | 5 | 0.236 | 1.434 | 6.07× |
| 10 | 10 | 0.394 | 2.819 | 7.15× |
| 100 | 1 | 0.055 | 0.258 | 4.73× |
| 100 | 5 | 0.193 | 1.474 | 7.63× |
| 100 | 10 | 0.446 | 2.847 | 6.38× |
| 1,000 | 1 | 0.090 | 0.416 | 4.64× |
| 1,000 | 5 | 0.215 | 2.454 | 11.42× |
| 1,000 | 10 | 0.493 | 4.600 | 9.33× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
