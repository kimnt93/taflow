# NewHighsNewLows benchmark (`NewHighsNewLows` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.034 | 29.50M | 0.029 | 34.14M | 7.818 | 230.62× | 266.96× |
| 10,000 | 0.210 | 47.56M | 0.204 | 48.93M | 78.027 | 371.12× | 381.75× |
| 100,000 | 1.925 | 51.95M | 1.860 | 53.75M | 791.338 | 411.10× | 425.37× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.130 | 0.301 | 2.30× |
| 1 | 5 | 0.441 | 1.448 | 3.29× |
| 1 | 10 | 0.598 | 2.050 | 3.43× |
| 10 | 1 | 0.070 | 0.283 | 4.04× |
| 10 | 5 | 0.286 | 1.700 | 5.93× |
| 10 | 10 | 0.640 | 2.819 | 4.40× |
| 100 | 1 | 0.072 | 1.021 | 14.21× |
| 100 | 5 | 0.292 | 5.489 | 18.81× |
| 100 | 10 | 0.610 | 10.694 | 17.52× |
| 1,000 | 1 | 0.100 | 8.383 | 83.48× |
| 1,000 | 5 | 0.383 | 54.335 | 141.99× |
| 1,000 | 10 | 0.751 | 97.907 | 130.37× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
