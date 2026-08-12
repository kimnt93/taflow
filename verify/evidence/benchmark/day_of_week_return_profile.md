# DayOfWeekReturnProfile benchmark (`DayOfWeekProfile` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.042 | 23.60M | 0.036 | 27.58M | 0.897 | 21.17× | 24.75× |
| 10,000 | 0.481 | 20.79M | 0.297 | 33.66M | 7.427 | 15.44× | 25.00× |
| 100,000 | 5.228 | 19.13M | 2.954 | 33.86M | 81.128 | 15.52× | 27.47× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.100 | 0.306 | 3.05× |
| 1 | 5 | 0.321 | 1.193 | 3.71× |
| 1 | 10 | 0.701 | 2.587 | 3.69× |
| 10 | 1 | 0.058 | 0.248 | 4.29× |
| 10 | 5 | 0.276 | 1.202 | 4.35× |
| 10 | 10 | 0.607 | 2.559 | 4.21× |
| 100 | 1 | 0.060 | 0.328 | 5.46× |
| 100 | 5 | 0.282 | 1.732 | 6.13× |
| 100 | 10 | 0.605 | 3.343 | 5.53× |
| 1,000 | 1 | 0.101 | 1.111 | 11.00× |
| 1,000 | 5 | 0.292 | 5.346 | 18.31× |
| 1,000 | 10 | 0.653 | 11.015 | 16.86× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
