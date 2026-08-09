# MathAcosh benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 80.53M | 0.011 | 90.23M | 0.010 | 0.84× | 0.94× |
| 10,000 | 0.098 | 102.48M | 0.094 | 106.22M | 0.094 | 0.97× | 1.00× |
| 100,000 | 0.956 | 104.56M | 0.920 | 108.71M | 0.927 | 0.97× | 1.01× |
| 1,000,000 | 9.880 | 101.21M | 9.275 | 107.82M | 9.099 | 0.92× | 0.98× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.119 | 0.066 | 0.55× |
| 1 | 5 | 0.244 | 0.226 | 0.93× |
| 1 | 10 | 0.469 | 0.401 | 0.86× |
| 10 | 1 | 0.048 | 0.042 | 0.87× |
| 10 | 5 | 0.238 | 0.214 | 0.90× |
| 10 | 10 | 0.469 | 0.384 | 0.82× |
| 100 | 1 | 0.049 | 0.042 | 0.87× |
| 100 | 5 | 0.206 | 0.176 | 0.86× |
| 100 | 10 | 0.472 | 0.405 | 0.86× |
| 1,000 | 1 | 0.060 | 0.055 | 0.92× |
| 1,000 | 5 | 0.236 | 0.214 | 0.91× |
| 1,000 | 10 | 0.504 | 0.460 | 0.91× |

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.330 | 0.166 | 6.03M | nan | — | — |
| 100,000 | 10 | 1.273 | 0.831 | 12.04M | nan | — | — |
| 100,000 | 1,000 | 13.876 | 10.015 | 99.85M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 85.45M | 94.99M | 1.00× | 2.19M | 3.73M | 1.00× | — |
| 5 | 288.88M | 294.87M | 3.10× | 2.19M | 2.59M | 0.69× | — |
| 10 | 383.49M | 463.56M | 4.88× | 2.03M | 2.60M | 0.70× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
