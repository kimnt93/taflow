# RateOfChangeRatioPercent benchmark (`ROCR100` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 209.25M | 0.004 | 266.34M | 0.031 | 6.57× | 8.36× |
| 10,000 | 0.020 | 490.99M | 0.018 | 547.80M | 0.040 | 1.97× | 2.20× |
| 100,000 | 0.175 | 570.15M | 0.155 | 643.68M | 0.125 | 0.71× | 0.81× |
| 1,000,000 | 1.992 | 502.06M | 1.625 | 615.38M | 1.114 | 0.56× | 0.69× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.138 | 0.134 | 0.97× |
| 1 | 5 | 0.276 | 0.520 | 1.88× |
| 1 | 10 | 0.463 | 0.965 | 2.09× |
| 10 | 1 | 0.049 | 0.094 | 1.92× |
| 10 | 5 | 0.215 | 0.440 | 2.05× |
| 10 | 10 | 0.482 | 0.959 | 1.99× |
| 100 | 1 | 0.052 | 0.098 | 1.87× |
| 100 | 5 | 0.231 | 0.460 | 1.99× |
| 100 | 10 | 0.470 | 0.974 | 2.07× |
| 1,000 | 1 | 0.064 | 0.110 | 1.72× |
| 1,000 | 5 | 0.297 | 0.519 | 1.75× |
| 1,000 | 10 | 0.505 | 0.955 | 1.89× |

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.247 | 0.157 | 6.37M | 123.204 | 785.41× | 187.05× |
| 100,000 | 10 | 0.974 | 0.501 | 19.96M | 123.047 | 245.62× | 59.59× |
| 100,000 | 1,000 | 4.035 | 2.971 | 336.64M | 127.619 | 42.96× | 10.98× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 305.51M | 405.68M | 1.00× | 3.44M | 3.77M | 1.00× | 497.66M |
| 5 | 699.01M | 1.09G | 2.68× | 3.02M | 3.15M | 0.84× | 419.96M |
| 10 | 681.09M | 1.19G | 2.93× | 2.75M | 2.66M | 0.70× | 484.47M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
