# AveragePrice benchmark (`AVGPRICE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 114.24M | 0.007 | 149.87M | 0.031 | 3.50× | 4.59× |
| 10,000 | 0.031 | 324.40M | 0.026 | 377.98M | 0.037 | 1.19× | 1.38× |
| 100,000 | 0.247 | 404.88M | 0.218 | 458.05M | 0.096 | 0.39× | 0.44× |
| 1,000,000 | 3.380 | 295.89M | 2.921 | 342.34M | 1.808 | 0.53× | 0.62× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.080 | 0.133 | 1.67× |
| 1 | 5 | 0.294 | 0.509 | 1.73× |
| 1 | 10 | 0.525 | 1.017 | 1.94× |
| 10 | 1 | 0.059 | 0.096 | 1.62× |
| 10 | 5 | 0.234 | 0.428 | 1.83× |
| 10 | 10 | 0.484 | 0.915 | 1.89× |
| 100 | 1 | 0.052 | 0.096 | 1.85× |
| 100 | 5 | 0.285 | 0.480 | 1.68× |
| 100 | 10 | 0.508 | 0.922 | 1.81× |
| 1,000 | 1 | 0.056 | 0.095 | 1.70× |
| 1,000 | 5 | 0.264 | 0.470 | 1.78× |
| 1,000 | 10 | 0.531 | 0.949 | 1.79× |

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.396 | 0.263 | 3.81M | 89.233 | 339.88× | 112.17× |
| 100,000 | 10 | 2.932 | 1.523 | 6.56M | 97.842 | 64.23× | 19.80× |
| 100,000 | 1,000 | 10.229 | 4.247 | 235.48M | 96.122 | 22.63× | 7.12× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 165.84M | 247.54M | 1.00× | 1.71M | 2.47M | 1.00× | 300.68M |
| 5 | 433.54M | 1.03G | 4.18× | 2.01M | 2.45M | 0.99× | 409.15M |
| 10 | 484.83M | 939.08M | 3.79× | 1.93M | 2.16M | 0.88× | 395.03M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
