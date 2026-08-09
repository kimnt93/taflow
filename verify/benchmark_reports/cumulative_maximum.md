# CumulativeMaximum benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 184.37M | 0.005 | 218.02M | 0.048 | 8.81× | 10.42× |
| 10,000 | 0.032 | 313.68M | 0.028 | 355.46M | 0.088 | 2.77× | 3.14× |
| 100,000 | 0.299 | 334.07M | 0.268 | 373.27M | 0.466 | 1.56× | 1.74× |
| 1,000,000 | 3.107 | 321.87M | 2.720 | 367.68M | 4.436 | 1.43× | 1.63× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.068 | 0.302 | 4.43× |
| 1 | 5 | 0.259 | 0.670 | 2.59× |
| 1 | 10 | 0.497 | 1.296 | 2.60× |
| 10 | 1 | 0.049 | 0.154 | 3.15× |
| 10 | 5 | 0.228 | 0.560 | 2.46× |
| 10 | 10 | 0.453 | 1.221 | 2.70× |
| 100 | 1 | 0.049 | 0.157 | 3.21× |
| 100 | 5 | 0.228 | 0.584 | 2.56× |
| 100 | 10 | 0.457 | 1.239 | 2.71× |
| 1,000 | 1 | 0.054 | 0.172 | 3.21× |
| 1,000 | 5 | 0.253 | 0.588 | 2.33× |
| 1,000 | 10 | 0.500 | 1.145 | 2.29× |

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.231 | 0.162 | 6.16M | nan | — | — |
| 100,000 | 10 | 0.925 | 0.530 | 18.88M | nan | — | — |
| 100,000 | 1,000 | 5.623 | 4.418 | 226.35M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 241.97M | 283.45M | 1.00× | 3.49M | 4.00M | 1.00× | — |
| 5 | 576.42M | 969.09M | 3.42× | 3.02M | 3.19M | 0.80× | — |
| 10 | 612.13M | 1.21G | 4.28× | 2.75M | 3.12M | 0.78× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
