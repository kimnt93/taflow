# WeightedClose benchmark (`WCLPRICE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 141.21M | 0.005 | 187.21M | 0.030 | 4.25× | 5.63× |
| 10,000 | 0.022 | 457.79M | 0.018 | 560.10M | 0.035 | 1.62× | 1.98× |
| 100,000 | 0.171 | 584.19M | 0.138 | 725.62M | 0.090 | 0.53× | 0.65× |
| 1,000,000 | 2.382 | 419.83M | 2.070 | 483.17M | 1.280 | 0.54× | 0.62× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.147 | 0.157 | 1.07× |
| 1 | 5 | 0.293 | 0.469 | 1.60× |
| 1 | 10 | 0.474 | 0.897 | 1.89× |
| 10 | 1 | 0.048 | 0.087 | 1.80× |
| 10 | 5 | 0.228 | 0.425 | 1.87× |
| 10 | 10 | 0.479 | 0.923 | 1.93× |
| 100 | 1 | 0.050 | 0.089 | 1.78× |
| 100 | 5 | 0.239 | 0.442 | 1.85× |
| 100 | 10 | 0.573 | 1.123 | 1.96× |
| 1,000 | 1 | 0.055 | 0.095 | 1.73× |
| 1,000 | 5 | 0.234 | 0.446 | 1.91× |
| 1,000 | 10 | 0.508 | 0.959 | 1.89× |

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.360 | 0.221 | 4.52M | 83.665 | 377.76× | 123.25× |
| 100,000 | 10 | 2.083 | 0.885 | 11.30M | 81.558 | 92.20× | 30.82× |
| 100,000 | 1,000 | 5.163 | 3.189 | 313.56M | 80.312 | 25.18× | 9.11× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 278.15M | 394.41M | 1.00× | 2.22M | 2.92M | 1.00× | 447.96M |
| 5 | 639.89M | 986.41M | 2.50× | 2.08M | 2.56M | 0.88× | 501.36M |
| 10 | 636.60M | 1.09G | 2.77× | 1.80M | 2.10M | 0.72× | 453.20M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
