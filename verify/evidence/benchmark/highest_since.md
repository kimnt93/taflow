# HighestSince benchmark (`highest since condition` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.043 | 23.41M | 0.037 | 26.68M | 0.264 | 6.17× | 7.04× |
| 10,000 | 0.294 | 34.07M | 0.277 | 36.12M | 2.644 | 9.01× | 9.55× |
| 100,000 | 2.692 | 37.15M | 2.718 | 36.79M | 25.603 | 9.51× | 9.42× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.136 | 0.083 | 0.61× |
| 1 | 5 | 0.417 | 0.352 | 0.85× |
| 1 | 10 | 0.633 | 0.637 | 1.01× |
| 10 | 1 | 0.066 | 0.072 | 1.10× |
| 10 | 5 | 0.301 | 0.341 | 1.13× |
| 10 | 10 | 0.592 | 0.695 | 1.17× |
| 100 | 1 | 0.064 | 0.097 | 1.52× |
| 100 | 5 | 0.296 | 0.448 | 1.52× |
| 100 | 10 | 0.608 | 0.903 | 1.49× |
| 1,000 | 1 | 0.091 | 0.342 | 3.76× |
| 1,000 | 5 | 0.284 | 1.747 | 6.15× |
| 1,000 | 10 | 0.601 | 3.412 | 5.68× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
