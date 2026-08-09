# HilbertTransformDominantCyclePeriod benchmark (`HT_DCPERIOD` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.047 | 21.13M | 0.048 | 20.78M | 0.078 | 1.65× | 1.62× |
| 10,000 | 0.449 | 22.28M | 0.451 | 22.16M | 0.488 | 1.09× | 1.08× |
| 100,000 | 4.553 | 21.96M | 4.722 | 21.18M | 4.926 | 1.08× | 1.04× |
| 1,000,000 | 47.286 | 21.15M | 46.307 | 21.60M | 43.750 | 0.93× | 0.94× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.095 | 0.129 | 1.36× |
| 1 | 5 | 0.306 | 0.531 | 1.74× |
| 1 | 10 | 0.497 | 1.028 | 2.07× |
| 10 | 1 | 0.049 | 0.094 | 1.92× |
| 10 | 5 | 0.261 | 0.487 | 1.87× |
| 10 | 10 | 0.503 | 0.996 | 1.98× |
| 100 | 1 | 0.053 | 0.091 | 1.70× |
| 100 | 5 | 0.237 | 0.454 | 1.92× |
| 100 | 10 | 0.552 | 0.970 | 1.76× |
| 1,000 | 1 | 0.120 | 0.137 | 1.15× |
| 1,000 | 5 | 0.242 | 0.671 | 2.77× |
| 1,000 | 10 | 0.540 | 1.444 | 2.67× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
