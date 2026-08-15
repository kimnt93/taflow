# KeltnerChannels benchmark (`Keltner` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.015 | 67.52M | 0.012 | 85.38M | 0.652 | 44.01× | 55.65× |
| 10,000 | 0.107 | 93.73M | 0.099 | 100.75M | 4.289 | 40.21× | 43.22× |
| 100,000 | 1.020 | 98.03M | 0.934 | 107.09M | 46.333 | 45.42× | 49.62× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.060 | 0.396 | 6.63× |
| 1 | 5 | 0.294 | 1.591 | 5.42× |
| 1 | 10 | 0.432 | 3.202 | 7.41× |
| 10 | 1 | 0.049 | 0.286 | 5.80× |
| 10 | 5 | 0.199 | 1.588 | 7.96× |
| 10 | 10 | 0.451 | 3.310 | 7.34× |
| 100 | 1 | 0.047 | 0.343 | 7.34× |
| 100 | 5 | 0.235 | 1.795 | 7.62× |
| 100 | 10 | 0.442 | 3.696 | 8.36× |
| 1,000 | 1 | 0.059 | 0.813 | 13.69× |
| 1,000 | 5 | 0.207 | 3.985 | 19.21× |
| 1,000 | 10 | 0.449 | 8.111 | 18.08× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
