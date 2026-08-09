# MedianPrice benchmark (`MEDPRICE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 173.57M | 0.004 | 230.06M | 0.028 | 4.80× | 6.36× |
| 10,000 | 0.020 | 496.16M | 0.017 | 602.74M | 0.032 | 1.61× | 1.96× |
| 100,000 | 0.156 | 641.37M | 0.135 | 739.09M | 0.067 | 0.43× | 0.50× |
| 1,000,000 | 2.301 | 434.54M | 1.784 | 560.65M | 0.940 | 0.41× | 0.53× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.117 | 0.166 | 1.42× |
| 1 | 5 | 0.290 | 0.490 | 1.69× |
| 1 | 10 | 0.475 | 0.914 | 1.92× |
| 10 | 1 | 0.051 | 0.090 | 1.76× |
| 10 | 5 | 0.215 | 0.424 | 1.97× |
| 10 | 10 | 0.487 | 0.905 | 1.86× |
| 100 | 1 | 0.050 | 0.087 | 1.73× |
| 100 | 5 | 0.231 | 0.438 | 1.89× |
| 100 | 10 | 0.478 | 0.890 | 1.86× |
| 1,000 | 1 | 0.054 | 0.090 | 1.67× |
| 1,000 | 5 | 0.231 | 0.421 | 1.83× |
| 1,000 | 10 | 0.503 | 0.909 | 1.81× |

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.327 | 0.196 | 5.10M | 76.943 | 392.73× | 134.96× |
| 100,000 | 10 | 1.507 | 0.676 | 14.79M | 77.227 | 114.23× | 39.53× |
| 100,000 | 1,000 | 4.637 | 3.034 | 329.60M | 72.847 | 24.01× | 9.49× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 335.29M | 327.57M | 1.00× | 2.60M | 3.13M | 1.00× | 656.73M |
| 5 | 690.79M | 1.12G | 3.43× | 2.32M | 2.95M | 0.94× | 575.67M |
| 10 | 689.05M | 1.25G | 3.82× | 2.13M | 2.83M | 0.91× | 581.70M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
