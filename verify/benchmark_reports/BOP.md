# BalanceOfPower benchmark (`BOP` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 126.39M | 0.007 | 152.77M | 0.027 | 3.47× | 4.19× |
| 10,000 | 0.031 | 327.63M | 0.026 | 379.21M | 0.036 | 1.17× | 1.36× |
| 100,000 | 0.244 | 410.66M | 0.219 | 456.40M | 0.119 | 0.49× | 0.54× |
| 1,000,000 | 3.059 | 326.92M | 2.551 | 392.01M | 1.617 | 0.53× | 0.63× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.136 | 0.143 | 1.06× |
| 1 | 5 | 0.316 | 0.482 | 1.53× |
| 1 | 10 | 0.482 | 0.883 | 1.83× |
| 10 | 1 | 0.052 | 0.088 | 1.69× |
| 10 | 5 | 0.217 | 0.420 | 1.94× |
| 10 | 10 | 0.494 | 0.875 | 1.77× |
| 100 | 1 | 0.049 | 0.084 | 1.71× |
| 100 | 5 | 0.235 | 0.429 | 1.83× |
| 100 | 10 | 0.476 | 0.891 | 1.87× |
| 1,000 | 1 | 0.052 | 0.097 | 1.85× |
| 1,000 | 5 | 0.230 | 0.408 | 1.78× |
| 1,000 | 10 | 0.484 | 0.851 | 1.76× |

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.410 | 0.253 | 3.96M | 119.684 | 473.64× | 100.21× |
| 100,000 | 10 | 2.306 | 1.104 | 9.06M | 118.667 | 107.50× | 22.48× |
| 100,000 | 1,000 | 6.491 | 4.299 | 232.60M | 119.093 | 27.70× | 6.38× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 164.01M | 233.71M | 1.00× | 2.12M | 2.53M | 1.00× | 395.61M |
| 5 | 471.98M | 915.93M | 3.92× | 1.92M | 2.52M | 1.00× | 404.52M |
| 10 | 545.41M | 1.06G | 4.52× | 2.00M | 2.39M | 0.94× | 437.47M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
