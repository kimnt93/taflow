# MathCeil benchmark (`CEIL` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 184.07M | 0.005 | 221.92M | 0.031 | 5.69× | 6.86× |
| 10,000 | 0.028 | 355.48M | 0.024 | 415.21M | 0.044 | 1.55× | 1.81× |
| 100,000 | 0.241 | 414.48M | 0.225 | 443.90M | 0.166 | 0.69× | 0.73× |
| 1,000,000 | 2.764 | 361.74M | 2.263 | 441.80M | 1.534 | 0.56× | 0.68× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.144 | 0.166 | 1.15× |
| 1 | 5 | 0.313 | 0.542 | 1.73× |
| 1 | 10 | 0.510 | 0.955 | 1.87× |
| 10 | 1 | 0.059 | 0.093 | 1.58× |
| 10 | 5 | 0.224 | 0.424 | 1.90× |
| 10 | 10 | 0.483 | 0.937 | 1.94× |
| 100 | 1 | 0.052 | 0.102 | 1.96× |
| 100 | 5 | 0.213 | 0.431 | 2.02× |
| 100 | 10 | 0.475 | 0.917 | 1.93× |
| 1,000 | 1 | 0.053 | 0.092 | 1.73× |
| 1,000 | 5 | 0.249 | 0.505 | 2.03× |
| 1,000 | 10 | 0.503 | 0.961 | 1.91× |

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.287 | 0.176 | 5.68M | 170.328 | 967.82× | 164.62× |
| 100,000 | 10 | 1.000 | 0.529 | 18.91M | 177.355 | 335.41× | 55.36× |
| 100,000 | 1,000 | 7.431 | 3.549 | 281.74M | 179.715 | 50.63× | 9.86× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 179.41M | 138.42M | 1.00× | 1.54M | 2.17M | 1.00× | 191.47M |
| 5 | 233.43M | 368.08M | 2.66× | 1.56M | 1.63M | 0.75× | 173.04M |
| 10 | 589.02M | 472.59M | 3.41× | 1.83M | 2.42M | 1.12× | 326.38M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
