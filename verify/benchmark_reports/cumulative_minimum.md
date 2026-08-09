# CumulativeMinimum benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 182.78M | 0.005 | 218.43M | 0.050 | 9.12× | 10.89× |
| 10,000 | 0.031 | 317.63M | 0.030 | 338.66M | 0.087 | 2.77× | 2.95× |
| 100,000 | 0.291 | 343.10M | 0.276 | 362.59M | 0.478 | 1.64× | 1.73× |
| 1,000,000 | 3.115 | 321.03M | 2.724 | 367.16M | 4.367 | 1.40× | 1.60× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.064 | 0.206 | 3.22× |
| 1 | 5 | 0.238 | 0.642 | 2.70× |
| 1 | 10 | 0.478 | 1.338 | 2.80× |
| 10 | 1 | 0.050 | 0.159 | 3.15× |
| 10 | 5 | 0.248 | 0.659 | 2.65× |
| 10 | 10 | 0.471 | 1.257 | 2.67× |
| 100 | 1 | 0.059 | 0.152 | 2.59× |
| 100 | 5 | 0.218 | 0.575 | 2.63× |
| 100 | 10 | 0.468 | 1.186 | 2.54× |
| 1,000 | 1 | 0.053 | 0.161 | 3.03× |
| 1,000 | 5 | 0.231 | 0.552 | 2.39× |
| 1,000 | 10 | 0.480 | 1.117 | 2.32× |

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.223 | 0.160 | 6.23M | nan | — | — |
| 100,000 | 10 | 0.935 | 0.523 | 19.11M | nan | — | — |
| 100,000 | 1,000 | 5.277 | 4.618 | 216.52M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 237.24M | 312.25M | 1.00× | 3.41M | 4.42M | 1.00× | — |
| 5 | 634.94M | 1.05G | 3.37× | 3.03M | 3.34M | 0.76× | — |
| 10 | 640.08M | 1.24G | 3.97× | 2.73M | 2.90M | 0.66× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
