# MathAsin benchmark (`ASIN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 100.05M | 0.009 | 116.06M | 0.032 | 3.22× | 3.74× |
| 10,000 | 0.072 | 138.87M | 0.070 | 142.01M | 0.088 | 1.22× | 1.25× |
| 100,000 | 0.743 | 134.59M | 0.725 | 137.95M | 0.632 | 0.85× | 0.87× |
| 1,000,000 | 8.147 | 122.74M | 7.313 | 136.74M | 6.099 | 0.75× | 0.83× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.110 | 0.106 | 0.96× |
| 1 | 5 | 0.256 | 0.448 | 1.75× |
| 1 | 10 | 0.516 | 0.953 | 1.85× |
| 10 | 1 | 0.053 | 0.089 | 1.67× |
| 10 | 5 | 0.237 | 0.440 | 1.86× |
| 10 | 10 | 0.505 | 0.917 | 1.82× |
| 100 | 1 | 0.066 | 0.104 | 1.59× |
| 100 | 5 | 0.342 | 0.479 | 1.40× |
| 100 | 10 | 0.510 | 0.908 | 1.78× |
| 1,000 | 1 | 0.060 | 0.098 | 1.64× |
| 1,000 | 5 | 0.219 | 0.448 | 2.04× |
| 1,000 | 10 | 0.515 | 0.986 | 1.92× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
