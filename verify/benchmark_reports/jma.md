# JurikMovingAverage benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.105 | 9.51M | 0.096 | 10.44M | 20.974 | 199.40× | 218.92× |
| 10,000 | 0.957 | 10.45M | 0.957 | 10.45M | 226.238 | 236.49× | 236.38× |
| 100,000 | 10.327 | 9.68M | 11.098 | 9.01M | 2257.225 | 218.58× | 203.40× |
| 1,000,000 | 106.674 | 9.37M | 99.250 | 10.08M | 25447.094 | 238.55× | 256.39× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.127 | 0.186 | 1.46× |
| 1 | 5 | 0.352 | 0.683 | 1.94× |
| 1 | 10 | 0.651 | 1.204 | 1.85× |
| 10 | 1 | 0.065 | 0.511 | 7.87× |
| 10 | 5 | 0.401 | 3.303 | 8.24× |
| 10 | 10 | 0.782 | 7.094 | 9.07× |
| 100 | 1 | 0.077 | 2.570 | 33.20× |
| 100 | 5 | 0.362 | 21.760 | 60.19× |
| 100 | 10 | 0.788 | 33.562 | 42.58× |
| 1,000 | 1 | 0.177 | 26.611 | 150.39× |
| 1,000 | 5 | 0.487 | 164.296 | 337.15× |
| 1,000 | 10 | 1.000 | 384.345 | 384.16× |

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | Reference full µs | vs full |
|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.720 | 0.295 | 3.39M | 2709257.336 | 9184047.70× |
| 100,000 | 10 | 2.284 | 1.790 | 5.59M | 2613216.322 | 1459913.70× |
| 100,000 | 1,000 | 117.753 | 123.931 | 8.07M | 2665410.488 | 21507.21× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | Reference vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 7.89M | 8.09M | 1.00× | 1.85M | 2.33M | 1.00× | 36.80K |
| 5 | 27.79M | 29.23M | 3.61× | 1.35M | 1.35M | 0.58× | 28.30K |
| 10 | 22.60M | 31.77M | 3.93× | 1.12M | 1.36M | 0.59× | 35.71K |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
