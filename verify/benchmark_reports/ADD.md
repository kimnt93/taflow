# MathAdd benchmark (`ADD` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 216.98M | 0.003 | 296.75M | 0.029 | 6.22× | 8.50× |
| 10,000 | 0.010 | 1.00G | 0.007 | 1.45G | 0.033 | 3.33× | 4.81× |
| 100,000 | 0.068 | 1.48G | 0.041 | 2.44G | 0.067 | 0.99× | 1.64× |
| 1,000,000 | 1.126 | 888.19M | 0.758 | 1.32G | 0.789 | 0.70× | 1.04× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.187 | 0.165 | 0.88× |
| 1 | 5 | 0.317 | 0.535 | 1.69× |
| 1 | 10 | 0.486 | 0.924 | 1.90× |
| 10 | 1 | 0.050 | 0.088 | 1.75× |
| 10 | 5 | 0.221 | 0.426 | 1.93× |
| 10 | 10 | 0.475 | 0.875 | 1.84× |
| 100 | 1 | 0.048 | 0.086 | 1.77× |
| 100 | 5 | 0.223 | 0.430 | 1.93× |
| 100 | 10 | 0.486 | 0.918 | 1.89× |
| 1,000 | 1 | 0.050 | 0.086 | 1.72× |
| 1,000 | 5 | 0.220 | 0.427 | 1.94× |
| 1,000 | 10 | 0.453 | 0.916 | 2.02× |

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.335 | 0.199 | 5.02M | 69.120 | 347.24× | 144.94× |
| 100,000 | 10 | 1.808 | 0.640 | 15.63M | 67.518 | 105.50× | 43.06× |
| 100,000 | 1,000 | 3.721 | 2.009 | 497.72M | 71.041 | 35.36× | 14.07× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 276.39M | 505.02M | 1.00× | 2.63M | 3.12M | 1.00× | 635.73M |
| 5 | 544.98M | 1.95G | 3.86× | 2.39M | 3.23M | 1.04× | 646.58M |
| 10 | 606.66M | 1.49G | 2.95× | 2.37M | 2.98M | 0.96× | 627.60M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
