# BetterVolume benchmark (`BetterVolume` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.027 | 37.15M | 0.024 | 41.32M | 0.307 | 11.39× | 12.67× |
| 10,000 | 0.190 | 52.62M | 0.182 | 54.88M | 1.616 | 8.50× | 8.87× |
| 100,000 | 1.815 | 55.11M | 1.744 | 57.34M | 14.630 | 8.06× | 8.39× |
| 1,000,000 | 18.888 | 52.94M | 18.524 | 53.99M | 140.110 | 7.42× | 7.56× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.166 | 0.538 | 3.23× |
| 1 | 5 | 0.451 | 1.176 | 2.61× |
| 1 | 10 | 0.549 | 2.475 | 4.50× |
| 10 | 1 | 0.063 | 0.247 | 3.90× |
| 10 | 5 | 0.275 | 1.305 | 4.74× |
| 10 | 10 | 0.538 | 2.462 | 4.57× |
| 100 | 1 | 0.062 | 0.237 | 3.81× |
| 100 | 5 | 0.275 | 1.561 | 5.67× |
| 100 | 10 | 0.588 | 2.781 | 4.73× |
| 1,000 | 1 | 0.075 | 0.400 | 5.35× |
| 1,000 | 5 | 0.318 | 2.074 | 6.52× |
| 1,000 | 10 | 0.607 | 4.059 | 6.69× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
