# RollingStandardDeviation benchmark (`STDDEV` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 133.46M | 0.007 | 142.14M | 0.040 | 5.39× | 5.74× |
| 10,000 | 0.050 | 198.22M | 0.047 | 214.27M | 0.067 | 1.33× | 1.44× |
| 100,000 | 0.471 | 212.25M | 0.440 | 227.38M | 0.343 | 0.73× | 0.78× |
| 1,000,000 | 4.922 | 203.16M | 4.429 | 225.79M | 3.395 | 0.69× | 0.77× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.078 | 0.131 | 1.68× |
| 1 | 5 | 0.299 | 0.485 | 1.62× |
| 1 | 10 | 0.517 | 0.971 | 1.88× |
| 10 | 1 | 0.057 | 0.098 | 1.73× |
| 10 | 5 | 0.218 | 0.441 | 2.02× |
| 10 | 10 | 0.656 | 1.131 | 1.72× |
| 100 | 1 | 0.058 | 0.096 | 1.65× |
| 100 | 5 | 0.232 | 0.470 | 2.02× |
| 100 | 10 | 0.494 | 1.140 | 2.31× |
| 1,000 | 1 | 0.060 | 0.104 | 1.75× |
| 1,000 | 5 | 0.266 | 0.516 | 1.94× |
| 1,000 | 10 | 0.514 | 1.095 | 2.13× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
