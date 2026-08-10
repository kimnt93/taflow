# RollingZScore benchmark (`ZScore` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.034 | 29.17M | 0.034 | 29.23M | 0.210 | 6.12× | 6.14× |
| 10,000 | 0.503 | 19.90M | 0.360 | 27.80M | 0.757 | 1.51× | 2.10× |
| 100,000 | 3.143 | 31.82M | 3.066 | 32.62M | 4.090 | 1.30× | 1.33× |
| 1,000,000 | 28.484 | 35.11M | 28.113 | 35.57M | 39.416 | 1.38× | 1.40× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.078 | 0.357 | 4.56× |
| 1 | 5 | 0.335 | 1.313 | 3.92× |
| 1 | 10 | 0.464 | 2.251 | 4.86× |
| 10 | 1 | 0.061 | 0.221 | 3.63× |
| 10 | 5 | 0.217 | 1.258 | 5.80× |
| 10 | 10 | 0.504 | 2.409 | 4.78× |
| 100 | 1 | 0.057 | 0.213 | 3.73× |
| 100 | 5 | 0.256 | 1.316 | 5.13× |
| 100 | 10 | 0.569 | 2.307 | 4.06× |
| 1,000 | 1 | 0.086 | 0.245 | 2.85× |
| 1,000 | 5 | 0.259 | 1.447 | 5.58× |
| 1,000 | 10 | 0.543 | 2.702 | 4.97× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
