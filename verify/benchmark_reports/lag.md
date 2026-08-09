# Lag benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 185.19M | 0.005 | 192.60M | 0.061 | 11.31× | 11.76× |
| 10,000 | 0.029 | 347.81M | 0.025 | 393.41M | 0.066 | 2.30× | 2.61× |
| 100,000 | 0.257 | 389.62M | 0.229 | 436.95M | 0.109 | 0.43× | 0.48× |
| 1,000,000 | 2.749 | 363.70M | 2.383 | 419.62M | 0.908 | 0.33× | 0.38× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.117 | 0.170 | 1.45× |
| 1 | 5 | 0.266 | 0.702 | 2.64× |
| 1 | 10 | 0.445 | 1.298 | 2.92× |
| 10 | 1 | 0.049 | 0.129 | 2.63× |
| 10 | 5 | 0.224 | 0.635 | 2.84× |
| 10 | 10 | 0.442 | 1.305 | 2.95× |
| 100 | 1 | 0.048 | 0.130 | 2.68× |
| 100 | 5 | 0.219 | 0.637 | 2.91× |
| 100 | 10 | 0.458 | 1.307 | 2.85× |
| 1,000 | 1 | 0.051 | 0.133 | 2.60× |
| 1,000 | 5 | 0.232 | 0.657 | 2.83× |
| 1,000 | 10 | 0.464 | 1.386 | 2.99× |

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.233 | 0.163 | 6.13M | nan | — | — |
| 100,000 | 10 | 0.976 | 0.589 | 16.99M | nan | — | — |
| 100,000 | 1,000 | 4.687 | 3.701 | 270.19M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 210.80M | 331.49M | 1.00× | 3.59M | 3.85M | 1.00× | — |
| 5 | 323.18M | 669.56M | 2.02× | 3.03M | 3.33M | 0.86× | — |
| 10 | 515.83M | 1.06G | 3.21× | 2.76M | 2.81M | 0.73× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
