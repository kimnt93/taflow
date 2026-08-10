# AwesomeOscillator benchmark (`AwesomeOscillator` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.027 | 37.26M | 0.026 | 37.93M | 0.225 | 8.39× | 8.55× |
| 10,000 | 0.234 | 42.65M | 0.234 | 42.72M | 0.823 | 3.51× | 3.52× |
| 100,000 | 2.425 | 41.23M | 2.549 | 39.23M | 6.947 | 2.86× | 2.73× |
| 1,000,000 | 23.336 | 42.85M | 22.828 | 43.81M | 64.747 | 2.77× | 2.84× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.158 | 0.294 | 1.86× |
| 1 | 5 | 0.258 | 1.332 | 5.16× |
| 1 | 10 | 0.480 | 2.721 | 5.67× |
| 10 | 1 | 0.050 | 0.243 | 4.87× |
| 10 | 5 | 0.268 | 1.376 | 5.14× |
| 10 | 10 | 0.502 | 2.532 | 5.05× |
| 100 | 1 | 0.053 | 0.255 | 4.84× |
| 100 | 5 | 0.243 | 1.406 | 5.78× |
| 100 | 10 | 0.523 | 2.668 | 5.10× |
| 1,000 | 1 | 0.074 | 0.306 | 4.13× |
| 1,000 | 5 | 0.241 | 1.744 | 7.23× |
| 1,000 | 10 | 0.551 | 3.429 | 6.22× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
