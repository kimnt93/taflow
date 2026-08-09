# TrueRange benchmark (`TRANGE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 127.68M | 0.007 | 147.51M | 0.027 | 3.49× | 4.03× |
| 10,000 | 0.034 | 296.57M | 0.031 | 325.76M | 0.034 | 1.00× | 1.10× |
| 100,000 | 0.261 | 383.54M | 0.237 | 421.63M | 0.086 | 0.33× | 0.36× |
| 1,000,000 | 3.195 | 313.00M | 2.718 | 367.90M | 1.532 | 0.48× | 0.56× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.106 | 0.128 | 1.21× |
| 1 | 5 | 0.311 | 0.566 | 1.82× |
| 1 | 10 | 0.493 | 0.887 | 1.80× |
| 10 | 1 | 0.050 | 0.085 | 1.69× |
| 10 | 5 | 0.218 | 0.409 | 1.88× |
| 10 | 10 | 0.517 | 0.918 | 1.78× |
| 100 | 1 | 0.052 | 0.089 | 1.71× |
| 100 | 5 | 0.221 | 0.417 | 1.89× |
| 100 | 10 | 0.476 | 0.928 | 1.95× |
| 1,000 | 1 | 0.057 | 0.094 | 1.67× |
| 1,000 | 5 | 0.230 | 0.440 | 1.91× |
| 1,000 | 10 | 0.493 | 0.876 | 1.78× |

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.352 | 0.229 | 4.37M | 85.083 | 371.86× | 116.28× |
| 100,000 | 10 | 1.865 | 0.903 | 11.07M | 86.613 | 95.89× | 28.64× |
| 100,000 | 1,000 | 5.965 | 4.222 | 236.85M | 85.453 | 20.24× | 6.54× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 181.52M | 313.35M | 1.00× | 2.41M | 2.78M | 1.00× | 533.59M |
| 5 | 413.85M | 907.88M | 2.90× | 2.25M | 2.85M | 1.03× | 491.71M |
| 10 | 412.36M | 611.87M | 1.95× | 2.19M | 2.59M | 0.93× | 465.08M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
