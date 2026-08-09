# DoubleExponentialMovingAverage benchmark (`DEMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 92.31M | 0.009 | 115.36M | 0.039 | 3.59× | 4.49× |
| 10,000 | 0.066 | 150.96M | 0.063 | 157.60M | 0.104 | 1.57× | 1.64× |
| 100,000 | 0.679 | 147.20M | 0.618 | 161.88M | 0.645 | 0.95× | 1.04× |
| 1,000,000 | 6.863 | 145.71M | 6.291 | 158.97M | 11.518 | 1.68× | 1.83× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.080 | 0.123 | 1.55× |
| 1 | 5 | 0.257 | 0.540 | 2.10× |
| 1 | 10 | 0.510 | 1.022 | 2.00× |
| 10 | 1 | 0.050 | 0.092 | 1.83× |
| 10 | 5 | 0.232 | 0.470 | 2.03× |
| 10 | 10 | 0.503 | 1.039 | 2.06× |
| 100 | 1 | 0.053 | 0.098 | 1.83× |
| 100 | 5 | 0.258 | 0.489 | 1.90× |
| 100 | 10 | 0.497 | 1.099 | 2.21× |
| 1,000 | 1 | 0.060 | 0.113 | 1.88× |
| 1,000 | 5 | 0.253 | 0.539 | 2.13× |
| 1,000 | 10 | 0.520 | 1.044 | 2.01× |

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | Reference full µs | vs full | vs bounded tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.267 | 0.184 | 5.45M | 689.969 | 3757.51× | 174.38× |
| 100,000 | 10 | 1.204 | 0.757 | 13.21M | 636.418 | 840.85× | 42.82× |
| 100,000 | 1,000 | 35.362 | 29.551 | 33.84M | 649.022 | 21.96× | 1.38× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | Reference vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 115.33M | 137.27M | 1.00× | 2.23M | 3.02M | 1.00× | 91.68M |
| 5 | 384.87M | 396.78M | 2.89× | 2.53M | 2.56M | 0.85× | 107.11M |
| 10 | 503.99M | 715.94M | 5.22× | 2.16M | 2.64M | 0.87× | 120.63M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
